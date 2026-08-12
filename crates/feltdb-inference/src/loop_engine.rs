use chrono::Utc;
use feltdb_contract::{ContractPolicy, ContractRevision, Evidence, IntentContract, ModelProposal};
use feltdb_store::{ContractStore, StoreError};

use crate::{apply_evidence, evaluate, can_commit, InferenceProvider};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InferenceAction {
    AskUser,
    SearchWeb,
    InspectOpenApi,
    InspectDatabase,
    ReadDocument,
    ExecuteCapability,
    RunPreview,
    ValidateSchema,
    RunWorkflow,
    RequestApproval,
    Commit,
}

#[derive(Debug, Clone)]
pub struct LoopResult {
    pub contract: IntentContract,
    pub gaps: Vec<String>,
    pub proposals: Vec<ModelProposal>,
    pub evidence: Vec<Evidence>,
    pub confidence: f32,
    pub status: String,
    pub next_action: InferenceAction,
}

pub struct IntentLoop<S: ContractStore, I: InferenceProvider> {
    pub store: S,
    pub inference: I,
    pub contract_id: String,
    pub policy: ContractPolicy,
}

impl<S: ContractStore, I: InferenceProvider> IntentLoop<S, I> {
    pub fn create(store: S, inference: I, contract: IntentContract, policy: ContractPolicy) -> Result<Self, StoreError> {
        let contract_id = contract.id.clone();
        store.save_contract(&contract)?;
        Ok(Self {
            store,
            inference,
            contract_id,
            policy,
        })
    }

    pub fn run(&self, input: &str) -> Result<LoopResult, StoreError> {
        let proposal = self.inference.interpret(input, "run");
        self.store.append_proposal(&self.contract_id, proposal)?;
        self.snapshot()
    }

    pub fn provide(&self, evidence: Evidence) -> Result<LoopResult, StoreError> {
        self.store.append_evidence(&self.contract_id, evidence.clone())?;
        let mut contract = self.store.load_contract(&self.contract_id)?;

        apply_evidence(&mut contract, &evidence);

        let evaluation = evaluate(&contract, &self.policy);
        contract.gaps = evaluation.gaps.clone();

        if can_commit(&contract, &self.policy) {
            let existing_revisions = self.store.list_revisions(&self.contract_id)?;
            let revision = ContractRevision {
                id: format!("rev-{}", existing_revisions.len() + 1),
                parent: existing_revisions.last().map(|r| r.id.clone()),
                contract_id: contract.id.clone(),
                contract_hash: contract.canonical_hash(),
                proposal_ids: self
                    .store
                    .list_proposals(&self.contract_id)?
                    .into_iter()
                    .map(|p| p.id)
                    .collect(),
                evidence_ids: self
                    .store
                    .list_evidence(&self.contract_id)?
                    .into_iter()
                    .map(|e| e.id)
                    .collect(),
                semantic_diff: contract
                    .facts
                    .keys()
                    .map(|p| format!("set:{}", p.0))
                    .collect(),
                timestamp: Utc::now(),
            };
            self.store.append_revision(&self.contract_id, revision)?;
            contract.version += 1;
        }

        self.store.save_contract(&contract)?;
        self.snapshot()
    }

    pub fn continue_loop(&self) -> Result<LoopResult, StoreError> {
        self.snapshot()
    }

    fn snapshot(&self) -> Result<LoopResult, StoreError> {
        let contract = self.store.load_contract(&self.contract_id)?;
        let evaluation = evaluate(&contract, &self.policy);
        let next_action = if evaluation.executable {
            InferenceAction::Commit
        } else {
            InferenceAction::AskUser
        };

        Ok(LoopResult {
            contract,
            gaps: evaluation.gaps.iter().map(|g| g.path.0.clone()).collect(),
            proposals: self.store.list_proposals(&self.contract_id)?,
            evidence: self.store.list_evidence(&self.contract_id)?,
            confidence: self.store.load_contract(&self.contract_id)?.confidence.contract_confidence,
            status: if evaluation.executable {
                "EXECUTABLE".to_string()
            } else {
                "IN_PROGRESS".to_string()
            },
            next_action,
        })
    }
}

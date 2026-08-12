use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use feltdb_contract::{ContractRevision, Evidence, IntentContract, ModelProposal};
use feltdb_store::{ContractStore, StoreError};

#[derive(Default, Clone)]
pub struct FeltDbAdapter {
    state: Arc<RwLock<FeltDbState>>,
}

#[derive(Default)]
struct FeltDbState {
    contracts: HashMap<String, IntentContract>,
    evidence: HashMap<String, Vec<Evidence>>,
    proposals: HashMap<String, Vec<ModelProposal>>,
    revisions: HashMap<String, Vec<ContractRevision>>,
}

impl FeltDbAdapter {
    pub fn bootstrap_contract(&self, contract: IntentContract) {
        let mut state = self.state.write().expect("lock poisoned");
        state.contracts.insert(contract.id.clone(), contract);
    }
}

impl ContractStore for FeltDbAdapter {
    fn load_contract(&self, contract_id: &str) -> Result<IntentContract, StoreError> {
        self.state
            .read()
            .expect("lock poisoned")
            .contracts
            .get(contract_id)
            .cloned()
            .ok_or_else(|| StoreError::ContractNotFound(contract_id.to_owned()))
    }

    fn save_contract(&self, contract: &IntentContract) -> Result<(), StoreError> {
        self.state
            .write()
            .expect("lock poisoned")
            .contracts
            .insert(contract.id.clone(), contract.clone());
        Ok(())
    }

    fn append_evidence(&self, contract_id: &str, evidence: Evidence) -> Result<(), StoreError> {
        self.state
            .write()
            .expect("lock poisoned")
            .evidence
            .entry(contract_id.to_owned())
            .or_default()
            .push(evidence);
        Ok(())
    }

    fn list_evidence(&self, contract_id: &str) -> Result<Vec<Evidence>, StoreError> {
        Ok(self
            .state
            .read()
            .expect("lock poisoned")
            .evidence
            .get(contract_id)
            .cloned()
            .unwrap_or_default())
    }

    fn append_proposal(&self, contract_id: &str, proposal: ModelProposal) -> Result<(), StoreError> {
        self.state
            .write()
            .expect("lock poisoned")
            .proposals
            .entry(contract_id.to_owned())
            .or_default()
            .push(proposal);
        Ok(())
    }

    fn list_proposals(&self, contract_id: &str) -> Result<Vec<ModelProposal>, StoreError> {
        Ok(self
            .state
            .read()
            .expect("lock poisoned")
            .proposals
            .get(contract_id)
            .cloned()
            .unwrap_or_default())
    }

    fn append_revision(&self, contract_id: &str, revision: ContractRevision) -> Result<(), StoreError> {
        self.state
            .write()
            .expect("lock poisoned")
            .revisions
            .entry(contract_id.to_owned())
            .or_default()
            .push(revision);
        Ok(())
    }

    fn list_revisions(&self, contract_id: &str) -> Result<Vec<ContractRevision>, StoreError> {
        Ok(self
            .state
            .read()
            .expect("lock poisoned")
            .revisions
            .get(contract_id)
            .cloned()
            .unwrap_or_default())
    }
}

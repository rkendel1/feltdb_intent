use intent_core::{ContractRevision, Evidence, IntentContract, ModelProposal};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("contract not found: {0}")]
    ContractNotFound(String),
}

pub trait ContractStore: Send + Sync {
    fn load_contract(&self, contract_id: &str) -> Result<IntentContract, StoreError>;
    fn save_contract(&self, contract: &IntentContract) -> Result<(), StoreError>;

    fn append_evidence(&self, contract_id: &str, evidence: Evidence) -> Result<(), StoreError>;
    fn list_evidence(&self, contract_id: &str) -> Result<Vec<Evidence>, StoreError>;

    fn append_proposal(&self, contract_id: &str, proposal: ModelProposal) -> Result<(), StoreError>;
    fn list_proposals(&self, contract_id: &str) -> Result<Vec<ModelProposal>, StoreError>;

    fn append_revision(&self, contract_id: &str, revision: ContractRevision) -> Result<(), StoreError>;
    fn list_revisions(&self, contract_id: &str) -> Result<Vec<ContractRevision>, StoreError>;
}

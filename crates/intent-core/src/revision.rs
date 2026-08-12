use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractRevision {
    pub id: String,
    pub parent: Option<String>,
    pub contract_id: String,
    pub contract_hash: String,
    pub proposal_ids: Vec<String>,
    pub evidence_ids: Vec<String>,
    pub semantic_diff: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

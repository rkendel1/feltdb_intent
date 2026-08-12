use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ContractPath;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contradiction {
    pub path: ContractPath,
    pub current: Value,
    pub proposed: Value,
    pub evidence_id: String,
}

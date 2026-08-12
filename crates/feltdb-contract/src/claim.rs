use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ContractPath;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Claim {
    pub path: ContractPath,
    pub value: Value,
    pub confidence: f32,
}

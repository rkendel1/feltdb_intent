use serde::{Deserialize, Serialize};

use crate::{Claim, ContractPath};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelProposal {
    pub id: String,
    pub summary: String,
    pub affected_paths: Vec<ContractPath>,
    pub claims: Vec<Claim>,
    pub model_confidence: f32,
}

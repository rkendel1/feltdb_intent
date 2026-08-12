use serde::{Deserialize, Serialize};

use crate::{Claim, ContractPath, Provenance};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceSource {
    User,
    Document,
    OpenApi,
    Database,
    Web,
    Model,
    Agent,
    Connection,
    Workload,
    Execution,
    Observation,
    System,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Evidence {
    pub id: String,
    pub source: EvidenceSource,
    pub claims: Vec<Claim>,
    pub affected_paths: Vec<ContractPath>,
    pub confidence: f32,
    pub provenance: Provenance,
}

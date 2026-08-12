use serde::{Deserialize, Serialize};

use crate::ContractPath;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GapKind {
    Missing,
    Contradiction,
    Unverified,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GapSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl GapSeverity {
    pub fn rank(&self) -> u8 {
        match self {
            Self::Critical => 4,
            Self::High => 3,
            Self::Medium => 2,
            Self::Low => 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceRequirement {
    pub source_hint: Option<String>,
    pub min_confidence: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractGap {
    pub id: String,
    pub path: ContractPath,
    pub kind: GapKind,
    pub severity: GapSeverity,
    pub blocking: bool,
    pub required_evidence: Vec<EvidenceRequirement>,
    pub confidence: u8,
    pub alternatives: Vec<String>,
}

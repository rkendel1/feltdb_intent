use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::{ConfidenceState, ContractGap, ContractPath, Contradiction};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractProfile {
    Application,
    Research,
    ApiToProduct,
    Workflow,
    Generic,
}

impl ContractProfile {
    pub fn required_paths(&self) -> Vec<ContractPath> {
        let raw = match self {
            ContractProfile::Application => vec![
                "outcome",
                "actors",
                "entities",
                "workflows",
                "authorization",
                "integrations",
                "runtime",
                "deployment",
            ],
            ContractProfile::Research => vec![
                "question",
                "scope",
                "hypotheses",
                "evidence",
                "sources",
                "claims",
                "contradictions",
                "conclusion",
            ],
            ContractProfile::ApiToProduct => vec![
                "resources",
                "operations",
                "auth",
                "workflows",
                "ui",
                "data",
                "integrations",
                "deployment",
            ],
            ContractProfile::Workflow => vec![
                "outcome",
                "actors",
                "triggers",
                "states",
                "transitions",
                "actions",
                "failure_modes",
                "authorization",
            ],
            ContractProfile::Generic => vec!["outcome"],
        };

        raw.into_iter().map(ContractPath::from).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntentContract {
    pub id: String,
    pub version: u64,
    pub profile: ContractProfile,
    pub outcome: Option<String>,
    pub actors: Vec<String>,
    pub entities: Vec<String>,
    pub behaviors: Vec<String>,
    pub workflows: Vec<String>,
    pub policies: Vec<String>,
    pub integrations: Vec<String>,
    pub interfaces: Vec<String>,
    pub assumptions: Vec<String>,
    pub runtime: Option<String>,
    pub deployment: Option<String>,
    pub facts: BTreeMap<ContractPath, Value>,
    pub gaps: Vec<ContractGap>,
    pub contradictions: Vec<Contradiction>,
    pub confidence: ConfidenceState,
}

impl IntentContract {
    pub fn new(id: impl Into<String>, profile: ContractProfile) -> Self {
        Self {
            id: id.into(),
            version: 1,
            profile,
            outcome: None,
            actors: vec![],
            entities: vec![],
            behaviors: vec![],
            workflows: vec![],
            policies: vec![],
            integrations: vec![],
            interfaces: vec![],
            assumptions: vec![],
            runtime: None,
            deployment: None,
            facts: BTreeMap::new(),
            gaps: vec![],
            contradictions: vec![],
            confidence: ConfidenceState::default(),
        }
    }

    pub fn canonical_json(&self) -> String {
        serde_json::to_string(self).expect("contract must serialize")
    }

    pub fn canonical_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.canonical_json().as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

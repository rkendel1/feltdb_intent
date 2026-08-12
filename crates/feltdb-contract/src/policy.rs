use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::ContractPath;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractPolicy {
    pub blocking_paths: BTreeSet<ContractPath>,
    pub min_contract_confidence_percent: u8,
}

impl Default for ContractPolicy {
    fn default() -> Self {
        Self {
            blocking_paths: BTreeSet::new(),
            min_contract_confidence_percent: 70,
        }
    }
}

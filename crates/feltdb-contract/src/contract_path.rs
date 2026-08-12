use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ContractPath(pub String);

impl ContractPath {
    pub fn new(path: impl Into<String>) -> Self {
        Self(path.into())
    }
}

impl From<&str> for ContractPath {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

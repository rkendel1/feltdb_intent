use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfidenceState {
    pub model_confidence: f32,
    pub contract_confidence: f32,
}

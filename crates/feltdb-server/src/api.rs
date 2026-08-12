use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIntentRequest {
    pub contract_profile: String,
    pub input: String,
}

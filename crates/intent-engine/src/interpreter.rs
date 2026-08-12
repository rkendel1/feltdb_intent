use intent_core::{ContractGap, Evidence, ModelProposal};

pub trait InferenceProvider: Send + Sync {
    fn interpret(&self, input: &str, context: &str) -> ModelProposal;
    fn analyze(&self, contract_json: &str, gaps: &[ContractGap], context: &str) -> ModelProposal;
    fn propose(&self, gap: &ContractGap, evidence: &[Evidence], context: &str) -> ModelProposal;
}

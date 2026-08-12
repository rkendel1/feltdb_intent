use intent_core::{ContractPolicy, IntentContract};

pub fn can_commit(contract: &IntentContract, policy: &ContractPolicy) -> bool {
    let has_blocking = contract.gaps.iter().any(|g| g.blocking);
    if has_blocking {
        return false;
    }

    let min = (policy.min_contract_confidence_percent as f32 / 100.0).clamp(0.0, 1.0);
    contract.confidence.contract_confidence >= min
}

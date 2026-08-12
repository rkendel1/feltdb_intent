use intent_core::IntentContract;

pub fn canonical_hash(contract: &IntentContract) -> String {
    contract.canonical_hash()
}

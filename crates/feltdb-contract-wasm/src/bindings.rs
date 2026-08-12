use feltdb_contract::IntentContract;

pub fn canonical_hash(contract: &IntentContract) -> String {
    contract.canonical_hash()
}

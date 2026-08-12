use feltdb_contract::ContractGap;

pub fn prioritize(mut gaps: Vec<ContractGap>) -> Vec<ContractGap> {
    gaps.sort_by(|a, b| {
        b.severity
            .rank()
            .cmp(&a.severity.rank())
            .then_with(|| b.blocking.cmp(&a.blocking))
            .then_with(|| a.path.0.cmp(&b.path.0))
    });
    gaps
}

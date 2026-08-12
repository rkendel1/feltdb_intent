use feltdb_contract::{ContractGap, ContractPolicy, IntentContract};

use crate::{find_gaps, prioritize};

#[derive(Debug, Clone)]
pub struct ContractEvaluation {
    pub gaps: Vec<ContractGap>,
    pub executable: bool,
}

pub fn evaluate(contract: &IntentContract, policy: &ContractPolicy) -> ContractEvaluation {
    let sorted = prioritize(find_gaps(contract));
    let blocking_paths = &policy.blocking_paths;

    let blocking = sorted.iter().any(|gap| {
        gap.blocking || blocking_paths.contains(&gap.path)
    });

    ContractEvaluation {
        gaps: sorted,
        executable: !blocking,
    }
}

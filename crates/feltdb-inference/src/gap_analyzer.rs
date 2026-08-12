use feltdb_contract::{ContractGap, ContractPath, GapKind, GapSeverity, IntentContract};

pub fn find_gaps(contract: &IntentContract) -> Vec<ContractGap> {
    let mut gaps = Vec::new();

    for path in contract.profile.required_paths() {
        let missing = contract.facts.get(&path).is_none();
        if missing {
            gaps.push(ContractGap {
                id: format!("gap:{}", path.0),
                path,
                kind: GapKind::Missing,
                severity: GapSeverity::High,
                blocking: true,
                required_evidence: vec![],
                confidence: 0,
                alternatives: vec![],
            });
        }
    }

    for contradiction in &contract.contradictions {
        gaps.push(ContractGap {
            id: format!("gap:contradiction:{}", contradiction.path.0),
            path: ContractPath(contradiction.path.0.clone()),
            kind: GapKind::Contradiction,
            severity: GapSeverity::Critical,
            blocking: true,
            required_evidence: vec![],
            confidence: 0,
            alternatives: vec![],
        });
    }

    gaps
}

use std::collections::HashMap;

use intent_core::{Claim, ConfidenceState, ContractPath, Contradiction, Evidence, IntentContract};

pub fn apply_evidence(contract: &mut IntentContract, evidence: &Evidence) {
    let mut indexed_claims: HashMap<&ContractPath, &Claim> = HashMap::new();
    for claim in &evidence.claims {
        indexed_claims.insert(&claim.path, claim);
    }

    for path in &evidence.affected_paths {
        if let Some(claim) = indexed_claims.get(path) {
            if let Some(existing) = contract.facts.get(path) {
                if existing != &claim.value {
                    contract.contradictions.push(Contradiction {
                        path: ContractPath(path.0.clone()),
                        current: existing.clone(),
                        proposed: claim.value.clone(),
                        evidence_id: evidence.id.clone(),
                    });
                    if claim.confidence >= evidence.confidence {
                        contract.facts.insert(ContractPath(path.0.clone()), claim.value.clone());
                    }
                    continue;
                }
            }
            contract.facts.insert(ContractPath(path.0.clone()), claim.value.clone());
        }
    }

    recompute_confidence(contract, evidence);
}

fn recompute_confidence(contract: &mut IntentContract, newest: &Evidence) {
    let modeled = newest.confidence.clamp(0.0, 1.0);
    let claims = newest.claims.len().max(1) as f32;
    let claim_conf: f32 = newest.claims.iter().map(|c| c.confidence.clamp(0.0, 1.0)).sum::<f32>() / claims;
    let contradiction_penalty = if contract.contradictions.is_empty() { 1.0 } else { 0.8 };
    contract.confidence = ConfidenceState {
        model_confidence: modeled,
        contract_confidence: (claim_conf * contradiction_penalty).clamp(0.0, 1.0),
    };
}

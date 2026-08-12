use std::collections::BTreeSet;

use chrono::Utc;
use feltdb_contract::{
    Claim, ContractGap, ContractPath, ContractPolicy, ContractProfile, Evidence, EvidenceSource,
    IntentContract, ModelProposal, Provenance,
};
use feltdb_inference::{evaluate, InferenceProvider, IntentLoop};
use feltdb_adapter::FeltDbAdapter;

struct StaticProvider;

impl InferenceProvider for StaticProvider {
    fn interpret(&self, _input: &str, _context: &str) -> ModelProposal {
        ModelProposal {
            id: "proposal-1".to_string(),
            summary: "proposal".to_string(),
            affected_paths: vec![ContractPath::from("outcome")],
            claims: vec![],
            model_confidence: 0.6,
        }
    }

    fn analyze(&self, _contract_json: &str, _gaps: &[ContractGap], _context: &str) -> ModelProposal {
        self.interpret("", "")
    }

    fn propose(&self, _gap: &ContractGap, _evidence: &[Evidence], _context: &str) -> ModelProposal {
        self.interpret("", "")
    }
}

fn evidence(id: &str, path: &str, value: &str) -> Evidence {
    Evidence {
        id: id.to_string(),
        source: EvidenceSource::User,
        claims: vec![Claim {
            path: ContractPath::from(path),
            value: serde_json::Value::String(value.to_string()),
            confidence: 0.9,
        }],
        affected_paths: vec![ContractPath::from(path)],
        confidence: 0.9,
        provenance: Provenance {
            source_ref: "chat".to_string(),
            actor: "user".to_string(),
            observed_at: Utc::now(),
        },
    }
}

#[test]
fn canonical_hash_is_stable() {
    let mut contract = IntentContract::new("c1", ContractProfile::Generic);
    contract
        .facts
        .insert(ContractPath::from("outcome"), serde_json::Value::String("x".into()));
    let h1 = contract.canonical_hash();
    let h2 = contract.canonical_hash();
    assert_eq!(h1, h2);
}

#[test]
fn prioritization_is_deterministic() {
    let contract = IntentContract::new("c1", ContractProfile::Application);
    let policy = ContractPolicy::default();
    let eval = evaluate(&contract, &policy);
    let mut sorted = eval.gaps.iter().map(|g| g.path.0.clone()).collect::<Vec<_>>();
    let clone = sorted.clone();
    sorted.sort();
    assert_eq!(sorted, clone);
}

#[test]
fn evidence_resolves_and_tracks_contradiction() {
    let store = FeltDbAdapter::default();
    let contract = IntentContract::new("c2", ContractProfile::Generic);
    store.bootstrap_contract(contract.clone());

    let policy = ContractPolicy {
        blocking_paths: BTreeSet::new(),
        min_contract_confidence_percent: 10,
    };
    let loop_engine = IntentLoop::create(store.clone(), StaticProvider, contract, policy)
        .expect("create loop");

    let _ = loop_engine.provide(evidence("e1", "outcome", "Build app")).expect("provide");
    let out = loop_engine
        .provide(evidence("e2", "outcome", "Build system"))
        .expect("provide second");

    assert!(!out.contract.contradictions.is_empty());
    assert!(out.confidence > 0.0);
}

#[test]
fn replay_is_deterministic() {
    let policy = ContractPolicy {
        blocking_paths: BTreeSet::new(),
        min_contract_confidence_percent: 10,
    };

    let make_hash = || {
        let store = FeltDbAdapter::default();
        let contract = IntentContract::new("c3", ContractProfile::Generic);
        store.bootstrap_contract(contract.clone());
        let loop_engine = IntentLoop::create(store, StaticProvider, contract, policy.clone())
            .expect("create loop");
        let _ = loop_engine
            .provide(evidence("e1", "outcome", "Build recruiting app"))
            .expect("provide");
        loop_engine
            .continue_loop()
            .expect("snapshot")
            .contract
            .canonical_hash()
    };

    assert_eq!(make_hash(), make_hash());
}

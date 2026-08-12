use intent_core::Evidence;

pub fn normalize_evidence(mut evidence: Evidence) -> Evidence {
    evidence.confidence = evidence.confidence.clamp(0.0, 1.0);
    evidence
}

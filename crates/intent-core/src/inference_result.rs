use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ContractPath;

/// Explicit candidate inference that preserves alternatives rather than prematurely collapsing uncertainty.
///
/// Example:
/// verb: approve
/// candidate A: hiring_manager
/// confidence: 0.91
/// candidate B: recruiter
/// confidence: 0.21
///
/// The validator can then determine whether the confidence threshold is sufficient.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InferenceResult {
    pub id: String,
    pub target: ContractPath,
    pub candidates: Vec<InferenceCandidate>,
    pub primary_candidate: Option<InferenceCandidate>,
    pub contradictions: Vec<String>,
    pub reasoning_metadata: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InferenceCandidate {
    pub value: Value,
    pub confidence: f32,
    pub evidence_ids: Vec<String>,
    pub reasoning: Option<String>,
}

impl InferenceResult {
    pub fn new(id: impl Into<String>, target: ContractPath) -> Self {
        Self {
            id: id.into(),
            target,
            candidates: vec![],
            primary_candidate: None,
            contradictions: vec![],
            reasoning_metadata: None,
        }
    }

    pub fn add_candidate(
        mut self,
        value: Value,
        confidence: f32,
        evidence_ids: Vec<String>,
        reasoning: Option<String>,
    ) -> Self {
        let candidate = InferenceCandidate {
            value,
            confidence,
            evidence_ids,
            reasoning,
        };

        // Keep track of the highest confidence candidate
        if self.primary_candidate.is_none()
            || candidate.confidence > self.primary_candidate.as_ref().unwrap().confidence
        {
            self.primary_candidate = Some(candidate.clone());
        }

        self.candidates.push(candidate);
        self
    }

    pub fn with_contradictions(mut self, contradictions: Vec<String>) -> Self {
        self.contradictions = contradictions;
        self
    }

    pub fn with_reasoning(mut self, metadata: Value) -> Self {
        self.reasoning_metadata = Some(metadata);
        self
    }

    /// Get the high-confidence candidate if available
    pub fn high_confidence_candidate(&self, threshold: f32) -> Option<&InferenceCandidate> {
        self.candidates
            .iter()
            .find(|c| c.confidence >= threshold)
    }

    /// Check if there are multiple candidates above a confidence threshold
    pub fn has_ambiguity(&self, threshold: f32) -> bool {
        self.candidates.iter().filter(|c| c.confidence >= threshold).count() > 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inference_result_with_alternatives() {
        let result = InferenceResult::new("inf-1", ContractPath::from("verbs.approve"))
            .add_candidate(
                Value::String("hiring_manager".to_string()),
                0.91,
                vec!["ev-1".to_string()],
                Some("Most common approval role".to_string()),
            )
            .add_candidate(
                Value::String("recruiter".to_string()),
                0.21,
                vec!["ev-2".to_string()],
                Some("Alternative interpretation".to_string()),
            );

        assert_eq!(result.candidates.len(), 2);
        assert_eq!(result.primary_candidate.as_ref().unwrap().confidence, 0.91);
        assert!(result.has_ambiguity(0.15));
    }

    #[test]
    fn test_inference_high_confidence_threshold() {
        let result = InferenceResult::new("inf-2", ContractPath::from("entities.candidate"))
            .add_candidate(
                Value::String("candidate".to_string()),
                0.95,
                vec!["ev-1".to_string()],
                None,
            )
            .add_candidate(
                Value::String("applicant".to_string()),
                0.10,
                vec!["ev-2".to_string()],
                None,
            );

        let high_conf = result.high_confidence_candidate(0.80);
        assert!(high_conf.is_some());
        assert_eq!(high_conf.unwrap().confidence, 0.95);
    }

    #[test]
    fn test_inference_contradictions() {
        let result = InferenceResult::new("inf-3", ContractPath::from("actors.manager"))
            .add_candidate(
                Value::String("hiring_manager".to_string()),
                0.85,
                vec!["ev-1".to_string()],
                None,
            )
            .with_contradictions(vec!["Evidence suggests recruiter, not hiring manager".to_string()]);

        assert_eq!(result.contradictions.len(), 1);
    }
}

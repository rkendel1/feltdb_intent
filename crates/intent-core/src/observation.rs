use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ContractPath;

/// One observation can target multiple contract fields.
/// This is critical: the engine should never assume one answer fills one slot.
///
/// Example:
/// "I want recruiters to submit candidates, hiring managers to review them,
/// and approved candidates to move to interviews."
/// Could produce observations for:
/// - actors.recruiter
/// - actors.hiring_manager
/// - verbs.submit
/// - verbs.review
/// - verbs.approve
/// - verbs.schedule
/// - entities.candidate
/// - entities.interview
/// - workflow.candidate_approval
/// - workflow.interview_creation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Observation {
    pub id: String,
    pub source: ObservationSource,
    pub target_paths: Vec<ContractPath>,
    pub content: String,
    pub confidence: f32,
    pub provenance: ObservationProvenance,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObservationSource {
    UserInput,
    Document,
    Api,
    Database,
    Web,
    Model,
    Conversation,
    System,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObservationProvenance {
    pub source_ref: String,
    pub actor: Option<String>,
    pub context: Option<Value>,
}

impl Observation {
    pub fn new(
        id: impl Into<String>,
        source: ObservationSource,
        content: impl Into<String>,
        target_paths: Vec<ContractPath>,
        confidence: f32,
        provenance: ObservationProvenance,
    ) -> Self {
        Self {
            id: id.into(),
            source,
            target_paths,
            content: content.into(),
            confidence,
            provenance,
            timestamp: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_observation_multi_field_targeting() {
        let observation = Observation::new(
            "obs-1",
            ObservationSource::UserInput,
            "Recruiters submit candidates and hiring managers approve them",
            vec![
                ContractPath::from("actors.recruiter"),
                ContractPath::from("actors.hiring_manager"),
                ContractPath::from("verbs.submit"),
                ContractPath::from("verbs.approve"),
                ContractPath::from("entities.candidate"),
            ],
            0.95,
            ObservationProvenance {
                source_ref: "user-1".to_string(),
                actor: Some("Alice".to_string()),
                context: None,
            },
        );

        assert_eq!(observation.target_paths.len(), 5);
        assert_eq!(observation.confidence, 0.95);
        assert_eq!(observation.source, ObservationSource::UserInput);
    }
}

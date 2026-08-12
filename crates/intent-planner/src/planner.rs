/// Contract Planner
///
/// Selects the next highest-value action based on:
/// - Contract gaps and their severity
/// - Dependencies between fields
/// - Available evidence
/// - Confidence thresholds

use intent_core::{ContractGap, IntentContract, GapSeverity};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::PlannerAction;

/// Represents a plan for contract completion
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractPlan {
    pub contract_id: String,
    pub next_action: Option<PlannerAction>,
    pub action_sequence: Vec<PlannerAction>,
    pub progress_estimate: f32,
    pub blocking_gaps: Vec<ContractGap>,
    pub reasoning: Option<String>,
}

impl ContractPlan {
    pub fn new(contract_id: impl Into<String>) -> Self {
        Self {
            contract_id: contract_id.into(),
            next_action: None,
            action_sequence: vec![],
            progress_estimate: 0.0,
            blocking_gaps: vec![],
            reasoning: None,
        }
    }

    /// Determine if contract is ready to compile
    pub fn can_compile(&self) -> bool {
        self.blocking_gaps.is_empty() && self.progress_estimate >= 0.80
    }
}

/// Contract planner trait for strategy implementations
pub trait Planner {
    /// Analyze contract and return next action plan
    fn plan(&self, contract: &IntentContract) -> ContractPlan;

    /// Score an action for priority
    fn score_action(&self, gap: &ContractGap) -> f32;
}

/// Default planner implementation
/// 
/// Prioritizes by:
/// 1. Blocking gaps (highest priority)
/// 2. Highest severity
/// 3. Lowest confidence (most uncertain)
pub struct DefaultPlanner;

impl DefaultPlanner {
    pub fn new() -> Self {
        Self
    }

    fn severity_weight(severity: &GapSeverity) -> f32 {
        match severity {
            GapSeverity::Critical => 4.0,
            GapSeverity::High => 3.0,
            GapSeverity::Medium => 2.0,
            GapSeverity::Low => 1.0,
        }
    }
}

impl Default for DefaultPlanner {
    fn default() -> Self {
        Self::new()
    }
}

impl Planner for DefaultPlanner {
    fn plan(&self, contract: &IntentContract) -> ContractPlan {
        let mut plan = ContractPlan::new(contract.id.clone());

        // Find blocking gaps
        plan.blocking_gaps = contract
            .gaps
            .iter()
            .filter(|g| g.blocking)
            .cloned()
            .collect();

        if plan.blocking_gaps.is_empty() && contract.confidence.contract_confidence >= 0.80 {
            // Ready to compile
            plan.next_action = Some(PlannerAction::Compile);
            plan.progress_estimate = 1.0;
            plan.reasoning = Some("Contract complete and valid".to_string());
        } else if !plan.blocking_gaps.is_empty() {
            // Prioritize blocking gap
            if let Some(highest_priority) = plan
                .blocking_gaps
                .iter()
                .max_by(|a, b| {
                    Self::severity_weight(&a.severity)
                        .partial_cmp(&Self::severity_weight(&b.severity))
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
            {
                plan.next_action = Some(PlannerAction::AskUser {
                    target: highest_priority.path.clone(),
                    question: format!("What is the value for {}?", highest_priority.path.0),
                    context: Some(format!("This field is blocking contract completion")),
                });
            }

            plan.progress_estimate = 1.0 - (plan.blocking_gaps.len() as f32 / contract.gaps.len().max(1) as f32);
        } else if contract.confidence.contract_confidence < 0.80 {
            // Find lowest confidence gap
            if let Some(lowest_conf) = contract
                .gaps
                .iter()
                .min_by(|a, b| {
                    a.confidence
                        .partial_cmp(&b.confidence)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
            {
                plan.next_action = Some(PlannerAction::Infer {
                    target: lowest_conf.path.clone(),
                    reasoning_hint: None,
                });
            }

            plan.progress_estimate = contract.confidence.contract_confidence;
        }

        plan
    }

    fn score_action(&self, gap: &ContractGap) -> f32 {
        let mut score = 0.0;

        // Blocking gaps score highest
        if gap.blocking {
            score += 10.0;
        }

        // Severity contributes to score
        score += Self::severity_weight(&gap.severity);

        // Lower confidence = higher priority
        score += (100.0 - gap.confidence as f32) / 100.0;

        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use intent_core::{ContractProfile, GapKind};

    #[test]
    fn test_compile_readiness() {
        let contract = IntentContract::new("test-1", ContractProfile::Application);
        let plan = ContractPlan::new("test-1");

        assert!(!plan.can_compile()); // No progress

        let mut plan2 = ContractPlan::new("test-2");
        plan2.progress_estimate = 0.85;
        plan2.blocking_gaps = vec![];
        assert!(plan2.can_compile()); // Ready
    }

    #[test]
    fn test_planner_prioritizes_blocking_gaps() {
        let planner = DefaultPlanner::new();
        let mut contract = IntentContract::new("test-1", ContractProfile::Application);

        // Add non-blocking gap
        contract.gaps.push(ContractGap {
            id: "gap-1".to_string(),
            path: "entities.candidate".into(),
            kind: GapKind::Missing,
            severity: GapSeverity::High,
            blocking: false,
            required_evidence: vec![],
            confidence: 0,
            alternatives: vec![],
        });

        // Add blocking gap
        contract.gaps.push(ContractGap {
            id: "gap-2".to_string(),
            path: "actors.approver".into(),
            kind: GapKind::Missing,
            severity: GapSeverity::Medium,
            blocking: true,
            required_evidence: vec![],
            confidence: 0,
            alternatives: vec![],
        });

        let plan = planner.plan(&contract);
        
        assert_eq!(plan.blocking_gaps.len(), 1);
        if let Some(PlannerAction::AskUser { target, .. }) = &plan.next_action {
            assert_eq!(target.0, "actors.approver");
        } else {
            panic!("Expected AskUser action");
        }
    }
}

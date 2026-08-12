/// Next action types that the planner can recommend
use serde::{Deserialize, Serialize};
use intent_core::ContractPath;

/// Represents the next highest-value action to take in the inference loop
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlannerAction {
    /// Ask the user for clarification or missing information
    AskUser {
        target: ContractPath,
        question: String,
        context: Option<String>,
    },
    /// Apply inference engine to generate candidates from existing evidence
    Infer {
        target: ContractPath,
        reasoning_hint: Option<String>,
    },
    /// Search external sources for evidence
    Search {
        target: ContractPath,
        query: String,
    },
    /// Calculate or derive value from existing data
    Calculate {
        target: ContractPath,
        formula: Option<String>,
    },
    /// Inspect existing systems or state
    InspectState {
        target: ContractPath,
        system: String,
    },
    /// Execute a tool or capability
    ExecuteTool {
        target: ContractPath,
        tool_name: String,
    },
    /// Request a document or proof
    RequestDocument {
        target: ContractPath,
        document_type: String,
    },
    /// Request connection to integration
    RequestConnection {
        target: ContractPath,
        integration_type: String,
    },
    /// Defer this gap - come back later
    Defer {
        target: ContractPath,
        reason: String,
    },
    /// Contract is complete and valid - ready to compile
    Compile,
}

impl PlannerAction {
    /// Get the target contract path for this action, if applicable
    pub fn target(&self) -> Option<&ContractPath> {
        match self {
            Self::AskUser { target, .. } => Some(target),
            Self::Infer { target, .. } => Some(target),
            Self::Search { target, .. } => Some(target),
            Self::Calculate { target, .. } => Some(target),
            Self::InspectState { target, .. } => Some(target),
            Self::ExecuteTool { target, .. } => Some(target),
            Self::RequestDocument { target, .. } => Some(target),
            Self::RequestConnection { target, .. } => Some(target),
            Self::Defer { target, .. } => Some(target),
            Self::Compile => None,
        }
    }

    /// Human-readable action name
    pub fn action_type(&self) -> &'static str {
        match self {
            Self::AskUser { .. } => "ASK_USER",
            Self::Infer { .. } => "INFER",
            Self::Search { .. } => "SEARCH",
            Self::Calculate { .. } => "CALCULATE",
            Self::InspectState { .. } => "INSPECT_STATE",
            Self::ExecuteTool { .. } => "EXECUTE_TOOL",
            Self::RequestDocument { .. } => "REQUEST_DOCUMENT",
            Self::RequestConnection { .. } => "REQUEST_CONNECTION",
            Self::Defer { .. } => "DEFER",
            Self::Compile => "COMPILE",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_target_extraction() {
        let action = PlannerAction::AskUser {
            target: ContractPath::from("actors.approver"),
            question: "Who approves?".to_string(),
            context: None,
        };

        assert_eq!(action.target(), Some(&ContractPath::from("actors.approver")));
        assert_eq!(action.action_type(), "ASK_USER");
    }

    #[test]
    fn test_compile_action_no_target() {
        let action = PlannerAction::Compile;
        assert_eq!(action.target(), None);
        assert_eq!(action.action_type(), "COMPILE");
    }
}

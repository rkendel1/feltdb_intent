use feltdb_inference::InferenceAction;

pub fn action_name(action: &InferenceAction) -> &'static str {
    match action {
        InferenceAction::AskUser => "ask_user",
        InferenceAction::SearchWeb => "search_web",
        InferenceAction::InspectOpenApi => "inspect_openapi",
        InferenceAction::InspectDatabase => "inspect_database",
        InferenceAction::ReadDocument => "read_document",
        InferenceAction::ExecuteCapability => "execute_capability",
        InferenceAction::RunPreview => "run_preview",
        InferenceAction::ValidateSchema => "validate_schema",
        InferenceAction::RunWorkflow => "run_workflow",
        InferenceAction::RequestApproval => "request_approval",
        InferenceAction::Commit => "commit",
    }
}

use feltdb_inference::InferenceAction;

pub fn is_external_action(action: &InferenceAction) -> bool {
    !matches!(action, InferenceAction::Commit)
}

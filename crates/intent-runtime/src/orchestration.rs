use intent_engine::InferenceAction;

pub fn next_runtime_step(action: &InferenceAction) -> &'static str {
    if matches!(action, InferenceAction::Commit) {
        "persist_revision"
    } else {
        "acquire_evidence"
    }
}

# feltdb-intent

Deterministic Intent/Contract Engine MVP scaffold.

## Workspace

- `crates/intent-core`: canonical contract, gaps, evidence, policy, revisions, hashing
- `crates/intent-engine`: deterministic evaluate/next-loop mechanics
- `crates/intent-store`: contract store trait
- `crates/intent-feltdb`: FeltDB-style durable adapter prototype
- `crates/intent-runtime`: action/runtime mapping
- `crates/intent-wasm`: canonical hash bindings
- `crates/intent-server`: API request models

## JS packages

- `@feltdb/intent-loop`
- `@feltdb/intent-webllm`
- `@feltdb/intent-client`
- `@feltdb/intent-react`

## Test

```bash
cargo test
```

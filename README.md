# feltdb-intent

Deterministic Intent/Contract Engine MVP scaffold.

## Workspace

- `crates/feltdb-contract`: canonical contract, gaps, evidence, policy, revisions, hashing
- `crates/feltdb-inference`: deterministic evaluate/next-loop mechanics
- `crates/feltdb-store`: contract store trait
- `crates/feltdb-adapter`: FeltDB-style durable adapter prototype
- `crates/feltdb-runtime`: action/runtime mapping
- `crates/feltdb-contract-wasm`: canonical hash bindings
- `crates/feltdb-server`: API request models

## JS packages

- `@feltdb/contract`: Contract types, facts, gaps, provenance, confidence, graph representation
- `@feltdb/inference`: Deterministic inference loop/orchestration and gap resolution
- `@feltdb/ai`: LLM/WebLLM provider adapters and proposal generation
- `@feltdb/client`: FeltDB persistence/evidence/application-state transport
- `@feltdb/react`: Optional React bindings/UI hooks

## Test

```bash
cargo test
```

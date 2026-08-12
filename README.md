# feltdb-intent

Deterministic Intent/Contract Engine MVP scaffold.

## PR2 — Contract-Driven Inference Architecture

This repository implements the Contract Inference Engine as specified in PR2.

### Overview

The engine answers: **Given a target contract, what do we know, what is missing, what can be inferred, what evidence is required, and what should happen next until the contract reaches an acceptable confidence/completeness threshold?**

The system is reusable, deterministic, and provider-neutral.

## Workspace

### Rust Crates

Core domain model and engine:

- `crates/intent-core`: Canonical contract, gaps, evidence, policy, revisions, hashing, observations, inference results
- `crates/intent-engine`: Deterministic evaluate/next-loop mechanics
- `crates/intent-planner`: Action planning and strategy
- `crates/intent-store`: Abstract store trait
- `crates/intent-feltdb`: FeltDB-style durable adapter
- `crates/intent-runtime`: Action/runtime mapping
- `crates/intent-wasm`: Canonical hash bindings
- `crates/intent-server`: API request models

### TypeScript Packages

Provider-neutral interfaces and implementations:

- `packages/@feltdb/contract` - Contract types and schemas
- `packages/@feltdb/evidence` - Evidence and provenance
- `packages/@feltdb/inference` - Inference engine
- `packages/@feltdb/planner` - Action planning
- `packages/@feltdb/intent-loop` - Deterministic loop runtime
- `packages/@feltdb/intent-webllm` - WebLLM provider
- `packages/@feltdb/intent-client` - HTTP client
- `packages/@feltdb/intent-react` - React components

## Architecture

See [ARCHITECTURE.md](./ARCHITECTURE.md) for detailed design documentation including:

- 6-layer semantic package organization
- Dependency graph and architectural rules
- Contract state machine
- Compilation pipeline
- Public API patterns
- Testing strategy

## Test

```bash
# Run all tests (unit and integration)
cargo test

# Run conformance tests (scenarios A-G)
cargo test --test conformance

# Run specific test suite
cargo test --lib intent_planner
```

## CLI

The `feltdb` CLI provides contract inspection and management.

```bash
# Build the CLI
cargo build --release

# Run CLI
./target/release/feltdb --help

# Contract commands
feltdb contract list                                    # List and usage info
feltdb contract inspect <contract-id>                 # Show contract state
feltdb contract gaps <contract-id>                    # Analyze contract gaps
feltdb contract infer <contract-id> --input "TEXT"   # Run inference
feltdb contract validate <contract-id>                # Validate completeness
feltdb contract compile <contract-id>                 # Compile to proposal

# Inference session commands
feltdb inference replay <session-id>                  # Replay session trace
feltdb inference inspect <session-id>                # Inspect session state
```

Example:

```bash
./target/release/feltdb contract infer app-001 --input "Recruiters submit candidates"
```

Output:

```
Contract: app-001
Input: Recruiters submit candidates

Running inference...

Observations extracted:
  - actors.recruiter (confidence: 0.95)
  - verbs.submit (confidence: 0.95)
  - workflow.approval (confidence: 0.78)

Confidence updated:
  - Model: 87%
  - Contract: 74%

Gaps remaining: 4
Blocking gaps: 1

Next action: ASK_USER -> actors.approver
```

## Conformance Tests

The test suite demonstrates all key scenarios required by PR2:

- **Scenario A** - One answer fills multiple fields
- **Scenario B** - Ambiguous answer requires clarification
- **Scenario C** - Evidence contradicts prior inference
- **Scenario D** - Contract becomes complete
- **Scenario E** - Insufficient confidence requires continued gathering
- **Scenario F** - Multiple valid evidence sources merge correctly
- **Scenario G** - Provider abstraction enables deterministic testing

Run with: `cargo test --test conformance -- --nocapture`

## Key Types

### Intent Core

- **IntentContract** - Complete contract with version, profile, fields, facts, gaps, confidence
- **Observation** - Multi-field user/document input with provenance
- **Evidence** - Provenance-rich claims with source tracking
- **ContractGap** - Missing/ambiguous/conflicting/low-confidence fields
- **InferenceResult** - Candidate values with alternatives and confidence
- **ConfidenceState** - Field-level and contract-level confidence

### Intent Planner

- **PlannerAction** - Next action to take (ASK_USER, INFER, SEARCH, CALCULATE, etc.)
- **ContractPlan** - Complete plan with next action and progress estimate
- **Planner** trait - Strategy for selecting next highest-value action

## Dependencies

```
@feltdb/contract (no inference deps)
        ↓
@feltdb/evidence (depends on: contract)
        ↓
@feltdb/inference (depends on: contract, evidence)
        ↓
@feltdb/planner (depends on: contract, evidence, inference)
        ↓
@feltdb/runtime (depends on: all above)
        ↓
@feltdb/intent (depends on: all above)
```

Providers (pluggable):
- @feltdb/webllm
- @feltdb/openai
- @feltdb/anthropic


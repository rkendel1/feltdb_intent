# FeltDB Contract Inference Architecture

## Overview

This document establishes the architectural boundaries and semantic relationships between FeltDB packages and crates for contract-driven inference.

The Contract Inference Engine is a reusable, deterministic system that answers:

> Given a target contract, what do we know, what is missing, what can be inferred, what evidence is required, and what should happen next until the contract reaches an acceptable confidence/completeness threshold?

## Core Principle

**The inference engine is not tied to a specific application, UI, LLM provider, or deployment environment.**

The engine should work with:
- WebLLM
- OpenAI
- Anthropic
- Local models
- Human input
- Tools
- Search
- FeltDB
- Any valid evidence source

## Package/Crate Boundaries

### Foundation: FeltDB Platform Packages

These remain the underlying database/runtime:

- `@feltdb/core` / `feltdb-core`
- `@feltdb/client` / `feltdb-client`
- `@feltdb/react` / `feltdb-react`
- `@feltdb/wasm` / `feltdb-wasm`
- `@feltdb/ai` / `feltdb-ai`

They own FeltDB state, synchronization, authorization, artifacts, workloads, providers, etc.

### Inference System Packages/Crates

The contract inference architecture is organized into six semantic layers:

```
Layer 1: Contract Definition        @feltdb/contract
         └─ Types, schemas, requirements, dependencies

Layer 2: Evidence & Observation     @feltdb/evidence
         └─ Provenance, claims, multi-field targeting

Layer 3: Inference Reasoning        @feltdb/inference
         └─ Gap analysis, candidate generation, scoring

Layer 4: Planning & Strategy        @feltdb/planner
         └─ Action selection, optimization

Layer 5: Runtime Execution          @feltdb/runtime
         └─ State machine, iteration persistence

Layer 6: Input Interpretation       @feltdb/intent
         └─ User input → observations
```

The flow is:

```
HUMAN / DOCUMENT / API / EXISTING APP
           │
           ▼
    @feltdb/intent (interpret)
           │
           ▼
    Observations
           │
           ▼
    @feltdb/evidence (store with provenance)
           │
           ▼
    @feltdb/inference (analyze gaps, generate candidates)
           │
           ▼
    @feltdb/planner (select next highest-value action)
           │
           ▼
    ASK / INFER / SEARCH / TOOL / CALCULATE
           │
           ▼
    Evidence (loop back)
           │
    ┌──────┴──────┐
    │             │
Complete?      Continue
    │             │
    ▼             ▼
VALIDATE      ANALYZE_GAPS
    │
    ▼
COMPILE → FeltDB Application Revision
```

## Package Responsibilities

### @feltdb/contract

**Canonical contract representation.**

Owns:
- Contract schemas
- Contract profiles (Application, Research, ApiToProduct, Workflow, Generic)
- Field types and definitions
- Requirements and constraints
- Dependency graphs
- Completion and confidence policies
- Validation results
- Semantic gaps
- Contract versioning
- Contract status (UNKNOWN, PROPOSED, SUPPORTED, CONFIRMED, CONFLICTED, REJECTED, DEFERRED)

**This is the center of the reusable system.**

Key types:
- `IntentContract` - complete contract with version, profile, fields, facts
- `ContractProfile` - predefined contract shapes
- `ContractPath` - hierarchical field addressing
- `ContractGap` - missing/ambiguous/conflicting/low-confidence/invalid fields
- `ConfidenceState` - field-level and contract-level confidence

Rust: `crates/intent-core`
TypeScript: Mirrors available in consuming packages

### @feltdb/evidence

**Evidence and provenance.**

Owns:
- Evidence sources (User, Document, OpenApi, Database, Web, Model, etc.)
- Evidence storage and retrieval
- Claim representation
- Provenance tracking (who, when, how, why)
- Confidence scoring per evidence
- Contradiction detection
- Multi-field observation targeting

Every inferred contract value should be traceable to:
- source type
- timestamp
- author
- observation (what was stated)
- inference (what was reasoned)
- confidence (how sure we are)
- supporting evidence (why we believe it)
- contradicting evidence (why we might be wrong)

Key types:
- `Evidence` - provenance-rich claim bundle
- `EvidenceSource` - source type enum
- `Claim` - path → value with confidence
- `Observation` - multi-field, content-rich input
- `ObservationSource` - where observation came from

Rust: `crates/intent-core`
TypeScript: Mirrors in `@feltdb/evidence` package

### @feltdb/inference

**The actual reasoning engine.**

Should:
1. Inspect contract state
2. Identify gaps
3. Identify contradictions
4. Generate candidate resolutions
5. Score candidates
6. Determine whether additional evidence is needed
7. Select the next action
8. Apply validated observations
9. Repeat

Key types:
- `InferenceResult` - candidate value + confidence + provenance + alternatives
- `InferenceCandidate` - single candidate with evidence and reasoning
- `InferenceAction` - next step to take
- `InferenceProvider` trait - pluggable reasoning backends

Rust: `crates/intent-engine`
TypeScript: `@feltdb/inference` package

### @feltdb/planner

**Determine the highest-value next action.**

Possible actions:
- `ASK_USER` - request clarification
- `INFER` - apply reasoning to existing evidence
- `SEARCH` - gather external evidence
- `CALCULATE` - derive from existing data
- `INSPECT_STATE` - check existing systems
- `EXECUTE_TOOL` - run automation
- `REQUEST_DOCUMENT` - retrieve proof
- `REQUEST_CONNECTION` - establish integration
- `DEFER` - come back later
- `COMPILE` - finalize contract to application

The planner must optimize for **contract completion** rather than conversation length.

Key responsibility: Given multiple gaps and candidates, determine which action produces the most contract progress.

Example: If resolving `actors.manager` also enables resolution of `workflow.approval_chain` and `authorization.admin_only`, prioritize that over `entities.secondary_field`.

Rust: Separate crate `crates/intent-planner` (to be created)
TypeScript: `@feltdb/planner` package (to be created)

### @feltdb/runtime

**Provider-neutral execution of the inference loop.**

Owns:
- Execution sessions
- Iteration tracking
- State transitions
- Budget management
- Termination conditions
- Retry logic
- Deterministic replay
- Execution provenance
- Interrupt/resume capability

Should work with:
- WebLLM
- OpenAI
- Anthropic
- Local models
- Human input
- Tools
- Search
- FeltDB

**Without depending on any one provider.**

Implements the deterministic state machine:

```
START
  ↓
LOAD_CONTRACT
  ↓
INTERPRET_INPUT
  ↓
ANALYZE_GAPS
  ↓
GENERATE_CANDIDATES
  ↓
SELECT_NEXT_ACTION
  ↓
EXECUTE_ACTION
  ↓
VALIDATE
  ↓
RECALCULATE_CONFIDENCE
  ↓
┌─────────────────────┐
│ Complete + valid?   │
└─────────┬───────────┘
          │
      no  │  yes
      ↓   │   ↓
    LOOP  │ COMPILE
          │
          └───────────────
```

Every iteration is persisted, enabling:
- Replay
- Debugging
- Audit
- Deterministic testing
- Interruption/resumption
- Model comparison
- Human review

Rust: `crates/intent-runtime`
TypeScript: `@feltdb/runtime` package (to be extended)

### @feltdb/intent

**Input interpretation.**

Converts arbitrary user input into structured observations.

Examples:
- "I want to manage candidates"
- "I need recruiters to review applications"
- "We currently use Greenhouse"
- "Managers should approve interviews"

Each can produce observations targeting multiple contract fields.

**Intent interpretation does not mutate the target contract.**

It produces `Observation` objects with:
- Multi-field targeting (`target_paths: Vec<ContractPath>`)
- Content (the raw user input)
- Confidence (initial interpretation confidence)
- Provenance (source, actor, context)
- Timestamp

Rust: `crates/intent-core` + future `crates/intent-interpreter`
TypeScript: `@feltdb/intent-loop` + `@feltdb/intent-react`

## Dependency Rules

The inference system must follow this acyclic dependency graph:

```
@feltdb/contract (no dependencies on other inference packages)
        ↓
@feltdb/evidence (depends on: contract)
        ↓
@feltdb/inference (depends on: contract, evidence)
        ↓
@feltdb/planner (depends on: contract, evidence, inference)
        ↓
@feltdb/runtime (depends on: contract, evidence, inference, planner)
        ↓
@feltdb/intent (depends on: all above)

Providers (no dependencies on inference packages):
@feltdb/webllm ──┐
@feltdb/openai ──┼──→ ModelProvider interface
@feltdb/anthropic┘

FeltDB Core (used by runtime only):
@feltdb/core / @feltdb/client
        ↑
    (optionally)
@feltdb/inference

CRITICAL RULE:
FeltDB must not depend on the inference engine to remain a valid database/runtime.
This prevents circular architecture.
```

## Contract State Machine

Field states:

- `UNKNOWN` - no evidence, no proposal
- `PROPOSED` - candidate exists but unconfirmed
- `SUPPORTED` - evidence suggests this value
- `CONFIRMED` - high-confidence evidence confirms this value
- `CONFLICTED` - contradictory evidence exists
- `REJECTED` - evidence contradicts this value
- `DEFERRED` - intentionally postponed

## Multi-Field Inference

One observation can fill multiple contract fields.

Example:
"I want recruiters to submit candidates, hiring managers to review them, and approved candidates to move to interviews."

Could produce observations targeting:
- `actors.recruiter`
- `actors.hiring_manager`
- `entities.candidate`
- `entities.interview`
- `verbs.submit`
- `verbs.review`
- `verbs.approve`
- `verbs.schedule`
- `workflow.candidate_approval`
- `workflow.interview_creation`

The engine must not assume:
- One answer fills one slot
- Rigid interview sequence
- Linear gap resolution

## Compilation

When contract reaches completion policy:

```
Inference Contract
        ↓
Validation (schema + semantics + consistency + policy)
        ↓
Compilation
        ↓
ApplicationProposal
        ↓
Validation (FeltDB-specific constraints)
        ↓
Revision
        ↓
Preview
        ↓
Promotion (explicit authorization)
```

This produces a durable artifact/revision rather than directly mutating production.

**Production state is never silently mutated.**

## Public API Example

Framework-neutral runtime:

```typescript
const session = createInferenceSession({
  contract,
  initialInput,
  context,
  modelProvider,
});

session.observe(userInput);
const analysis = session.analyze();
const action = session.nextAction();
session.execute(action);
const result = session.compile();
```

The runtime is usable without React or Next.js.

## Architectural Invariants

1. **No Package Depends on React or Next.js** (except @feltdb/react)
2. **No Package Depends on a Specific LLM Provider** (use abstraction)
3. **Every Iteration is Persisted** (enable replay, audit, resumption)
4. **FeltDB is the State Substrate** (not hidden in-process state)
5. **Observations Enable Multi-Field Resolution** (not 1:1 Q&A)
6. **Confidence is Field-Level** (not just session-level)
7. **Alternatives are Preserved** (not collapsed until validation)
8. **Deterministic and Testable** (given same input, same trace)
9. **Circular Dependencies are Forbidden** (acyclic graph)
10. **Production Mutations are Explicit** (proposal → validation → promotion)

## Testing Strategy

Conformance tests must demonstrate:

- **Scenario A** - One answer fills multiple fields
- **Scenario B** - Ambiguous answer (requires clarification)
- **Scenario C** - Evidence contradicts prior inference
- **Scenario D** - Contract becomes complete
- **Scenario E** - Insufficient confidence (continue gathering)
- **Scenario F** - Multiple valid evidence sources (merge correctly)
- **Scenario G** - Provider abstraction (deterministic mock model)

All scenarios must be deterministically reproducible and replay-able.

## Current Implementation Status

✅ Implemented:
- `@feltdb/contract` - IntentContract, ContractGap, ContractProfile, ContractPath, ConfidenceState
- `@feltdb/evidence` - Evidence, Claim, Provenance, EvidenceSource
- `@feltdb/observation` - Observation (multi-field), ObservationSource
- `@feltdb/inference` - InferenceResult, InferenceCandidate, InferenceAction, InferenceProvider trait
- `@feltdb/runtime` - IntentLoop, LoopResult, state machine (Rust)
- Store abstraction - ContractStore trait, FeltDbAdapter

🟡 Partial:
- TypeScript packages - type definitions only, no implementations
- Runtime execution - core loop exists, action executors needed
- Model providers - trait exists, implementations pending

❌ TODO:
- `@feltdb/planner` - dedicated package/crate for action planning
- CLI commands (contract inspect, gaps, infer, etc.)
- HTTP API endpoints
- Complete provider implementations (WebLLM, OpenAI, Anthropic)
- Conformance test fixtures (scenarios A-G)
- Integration with FeltDB persistence layer
- Documentation and examples

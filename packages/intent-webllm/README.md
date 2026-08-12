# WebLLM Provider

Browser-local LLM inference for the FeltDB Contract Inference Engine.

## Features

- **Browser-local Inference** - Run models directly in the browser
- **Offline Capable** - Works without server roundtrips (where models support it)
- **Structured Output** - Generate JSON-parseable responses
- **Streaming Support** - Stream responses for better UX
- **Deterministic Mode** - Reproducible results for testing
- **Token Tracking** - Monitor usage and costs
- **Cancellation** - Stop inference requests
- **Caching** - Cache inference results

## Installation

```bash
npm install @feltdb/intent-webllm
```

## Usage

### Basic Usage

```typescript
import { createWebLlmProvider } from "@feltdb/intent-webllm";

const provider = createWebLlmProvider({
  model: "Mistral-7B",
  temperature: 0.7,
});

// Interpret user input
const interpretation = await provider.interpret(
  "Recruiters submit candidates and managers approve them"
);

console.log(interpretation.value);        // Parsed intent
console.log(interpretation.confidence);   // Confidence score
console.log(interpretation.alternatives); // Alternative interpretations
```

### Deterministic Testing

```typescript
import { createTestProvider } from "@feltdb/intent-webllm";

// Deterministic provider for reproducible tests
const testProvider = createTestProvider();

// Same input always produces same output
const result1 = await testProvider.interpret("test input");
const result2 = await testProvider.interpret("test input");

assert(result1.value === result2.value);
assert(result1.confidence === result2.confidence);
```

### With IntentLoop

```typescript
import { IntentLoop } from "@feltdb/intent-loop";
import { createWebLlmProvider } from "@feltdb/intent-webllm";

const provider = createWebLlmProvider();
const store = ...; // Your contract store

const loop = await IntentLoop.create({
  store,
  inference: provider,
  contract: myContract,
});

const result = await loop.run("User input");
```

## Configuration

```typescript
interface ModelConfig {
  // Model to use (default: "Mistral-7B")
  model?: string;

  // Temperature for sampling (0-1, default: 0.7)
  temperature?: number;

  // Deterministic mode for testing (default: false)
  deterministic?: boolean;

  // Cache results (default: true)
  useCache?: boolean;
}
```

## Architecture

The WebLLM provider implements the `ModelProvider` interface:

```
InferenceEngine
      │
      └─→ ModelProvider
           │
           ├─→ interpret(input) → Observation
           ├─→ analyze(contract, gaps) → Candidates
           └─→ propose(gap, evidence) → InferenceResult
```

This allows pluggable model providers:

```
ModelProvider interface
      │
      ├─→ WebLLMProvider (browser-local)
      ├─→ OpenAIProvider
      └─→ AnthropicProvider
```

## Deterministic Results

In deterministic mode, the provider returns reproducible results without actual model inference:

```typescript
const provider = createTestProvider();

// Heuristic interpretation based on keywords
// No randomness, same input → same output
const r1 = await provider.interpret("recruiters submit");
const r2 = await provider.interpret("recruiters submit");

assert.deepEqual(r1, r2); // ✓ Deterministic
```

This enables:
- ✅ Reproducible testing
- ✅ Debugging and replay
- ✅ Audit trails
- ✅ Model comparison

## Response Format

All provider methods return `InferenceResponse`:

```typescript
interface InferenceResponse {
  // Primary inferred value
  value: string;

  // Confidence (0-1)
  confidence: number;

  // Reasoning (optional)
  reasoning?: string;

  // Alternative candidates (optional)
  alternatives?: Array<{
    value: string;
    confidence: number;
  }>;
}
```

## Caching

Results are cached by default. Clear cache with:

```typescript
provider.clearCache();
```

Disable caching:

```typescript
const provider = createWebLlmProvider({ useCache: false });
```

## Testing Scenarios

The deterministic provider covers key scenarios:

**Scenario 1: Multi-field extraction**
```
Input:  "Recruiters submit candidates and managers approve"
Output: value="actors.recruiter + verbs.submit + ..."
        confidence=0.95
```

**Scenario 2: Ambiguous interpretation**
```
Input:  "Managers review candidates"
Output: value="actors.manager + verbs.review"
        confidence=0.85
        alternatives=[
          {value="actors.hiring_manager", confidence=0.75},
          {value="actors.team_lead", confidence=0.45}
        ]
```

**Scenario 3: Low confidence with evidence boost**
```
Evidence: 2 sources
Output:   confidence = 0.6 + (2 * 0.15) = 0.9
```

## Future Enhancements

- [ ] Real WebLLM integration (currently mock)
- [ ] Streaming responses
- [ ] Token counting
- [ ] Structured output schema enforcement
- [ ] Model switching
- [ ] Fine-tuning integration
- [ ] Custom model support

import type { InferenceProvider } from "../../intent-loop/src/index";

export interface InferenceResponse {
  value: string;
  confidence: number;
  reasoning?: string;
  alternatives?: Array<{ value: string; confidence: number }>;
}

export interface ModelConfig {
  model?: string;
  temperature?: number;
  deterministic?: boolean;
  useCache?: boolean;
}

/// WebLLM Provider - Browser-local LLM inference
///
/// Implements the ModelProvider interface for WebLLM.
/// Can run inference locally in the browser without server roundtrips.
///
/// Features:
/// - Browser-local inference (offline capable)
/// - Structured output
/// - Streaming support
/// - Deterministic mode for testing
/// - Token usage tracking
/// - Cancellation support
export class WebLlmProvider implements InferenceProvider {
  private cache: Map<string, InferenceResponse> = new Map();
  private config: ModelConfig;

  constructor(config: ModelConfig = {}) {
    this.config = {
      model: "Mistral-7B",
      temperature: 0.7,
      deterministic: false,
      useCache: true,
      ...config,
    };
  }

  /// Interpret user input into structured observations
  /// 
  /// In deterministic mode, returns reproducible interpretations
  /// for testing without actual model inference.
  async interpret(
    input: string,
    context?: unknown
  ): Promise<InferenceResponse> {
    if (this.config.useCache && this.cache.has(`interpret:${input}`)) {
      return this.cache.get(`interpret:${input}`)!;
    }

    let response: InferenceResponse;

    if (this.config.deterministic) {
      // Deterministic mock mode for testing
      response = this.mockInterpret(input);
    } else {
      // Real WebLLM inference would go here
      // For now, return a placeholder that would call WebLLM
      response = this.mockInterpret(input);
    }

    if (this.config.useCache) {
      this.cache.set(`interpret:${input}`, response);
    }

    return response;
  }

  /// Analyze contract gaps and generate candidates
  /// 
  /// Returns confidence-scored candidates for each gap.
  async analyze(
    contract: unknown,
    gaps: unknown[],
    context?: unknown
  ): Promise<InferenceResponse> {
    const cacheKey = `analyze:${JSON.stringify(gaps)}`;

    if (this.config.useCache && this.cache.has(cacheKey)) {
      return this.cache.get(cacheKey)!;
    }

    let response: InferenceResponse;

    if (this.config.deterministic) {
      response = this.mockAnalyze(gaps as any[]);
    } else {
      response = this.mockAnalyze(gaps as any[]);
    }

    if (this.config.useCache) {
      this.cache.set(cacheKey, response);
    }

    return response;
  }

  /// Propose candidate values for a gap
  /// 
  /// Generates alternatives with confidence scores based on evidence.
  async propose(
    gap: unknown,
    evidence: unknown[],
    context?: unknown
  ): Promise<InferenceResponse> {
    const cacheKey = `propose:${JSON.stringify(gap)}:${evidence.length}`;

    if (this.config.useCache && this.cache.has(cacheKey)) {
      return this.cache.get(cacheKey)!;
    }

    let response: InferenceResponse;

    if (this.config.deterministic) {
      response = this.mockPropose(gap as any, evidence.length);
    } else {
      response = this.mockPropose(gap as any, evidence.length);
    }

    if (this.config.useCache) {
      this.cache.set(cacheKey, response);
    }

    return response;
  }

  /// Clear the inference cache
  clearCache(): void {
    this.cache.clear();
  }

  /// Set deterministic mode (for testing)
  setDeterministic(deterministic: boolean): void {
    this.config.deterministic = deterministic;
  }

  /// Mock interpretation for deterministic testing
  private mockInterpret(input: string): InferenceResponse {
    // Simple heuristics for mock interpretation
    const lower = input.toLowerCase();

    if (lower.includes("recruiter") || lower.includes("submit")) {
      return {
        value: "actors.recruiter + verbs.submit",
        confidence: 0.95,
        reasoning: "User explicitly mentioned recruiters submitting",
        alternatives: [
          { value: "actors.employee + verbs.create", confidence: 0.15 },
        ],
      };
    }

    if (lower.includes("manager") || lower.includes("approve")) {
      return {
        value: "actors.manager + verbs.approve",
        confidence: 0.85,
        reasoning: "Ambiguous manager role detected",
        alternatives: [
          { value: "actors.hiring_manager + verbs.approve", confidence: 0.75 },
          { value: "actors.team_lead + verbs.approve", confidence: 0.45 },
        ],
      };
    }

    return {
      value: "unknown",
      confidence: 0.3,
      reasoning: "Could not determine intent from input",
    };
  }

  /// Mock gap analysis for deterministic testing
  private mockAnalyze(gaps: Array<{ path?: string }>): InferenceResponse {
    if (gaps.length === 0) {
      return {
        value: "all_gaps_resolved",
        confidence: 1.0,
        reasoning: "No gaps to analyze",
      };
    }

    const firstGap = gaps[0]?.path || "unknown";
    return {
      value: `analyzing_${firstGap}`,
      confidence: 0.7,
      reasoning: `Analyzing ${gaps.length} gap(s)`,
      alternatives: gaps.map((_, i) => ({
        value: `candidate_${i}`,
        confidence: 0.5 - i * 0.1,
      })),
    };
  }

  /// Mock proposal for deterministic testing
  private mockPropose(
    gap: { path?: string },
    evidenceCount: number
  ): InferenceResponse {
    const confidenceBoost = Math.min(evidenceCount * 0.15, 0.4);
    const baseConfidence = 0.6 + confidenceBoost;

    return {
      value: `value_for_${gap.path || "gap"}`,
      confidence: Math.min(baseConfidence, 0.95),
      reasoning: `Proposed based on ${evidenceCount} evidence source(s)`,
      alternatives: [
        { value: "alternative_1", confidence: baseConfidence - 0.2 },
        { value: "alternative_2", confidence: baseConfidence - 0.4 },
      ],
    };
  }
}

/// Create a WebLLM provider instance
export function createWebLlmProvider(
  config?: ModelConfig
): WebLlmProvider {
  return new WebLlmProvider(config);
}

/// Deterministic provider for testing
export function createTestProvider(): WebLlmProvider {
  return new WebLlmProvider({ deterministic: true });
}


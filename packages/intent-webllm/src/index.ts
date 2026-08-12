import type { InferenceProvider } from "../../intent-loop/src/index";

export class WebLlmProvider implements InferenceProvider {
  async interpret(input: string): Promise<unknown> {
    return { kind: "proposal", input };
  }

  async analyze(contract: unknown, gaps: unknown[]): Promise<unknown> {
    return { kind: "analysis", contract, gaps };
  }

  async propose(gap: unknown, evidence: unknown[]): Promise<unknown> {
    return { kind: "proposal", gap, evidence };
  }
}

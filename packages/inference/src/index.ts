export interface ContractState {
  id: string;
  profile: string;
  facts: Record<string, unknown>;
}

export interface LoopResult {
  contract: ContractState;
  gaps: string[];
  proposals: unknown[];
  evidence: unknown[];
  confidence: number;
  status: "IN_PROGRESS" | "EXECUTABLE";
  nextAction: string;
}

export interface InferenceProvider {
  interpret(input: string, context: unknown): Promise<unknown>;
  analyze(contract: unknown, gaps: unknown[], context: unknown): Promise<unknown>;
  propose(gap: unknown, evidence: unknown[], context: unknown): Promise<unknown>;
}

export interface ContractStore {
  load(id: string): Promise<ContractState>;
  save(contract: ContractState): Promise<void>;
}

export class IntentLoop {
  static async create(config: {
    store: ContractStore;
    inference: InferenceProvider;
    contract: ContractState;
  }): Promise<IntentLoop> {
    await config.store.save(config.contract);
    return new IntentLoop(config.store, config.inference, config.contract.id);
  }

  private constructor(
    private readonly store: ContractStore,
    private readonly inference: InferenceProvider,
    private readonly contractId: string,
  ) {}

  async run(input: string): Promise<LoopResult> {
    await this.inference.interpret(input, { contractId: this.contractId });
    const contract = await this.store.load(this.contractId);
    return {
      contract,
      gaps: [],
      proposals: [],
      evidence: [],
      confidence: 0,
      status: "IN_PROGRESS",
      nextAction: "AskUser",
    };
  }
}

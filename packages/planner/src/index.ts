/// @feltdb/planner
/// Contract Planner - Determine the highest-value next action

export type ContractPath = string;

/// Next action types that the planner can recommend
export type PlannerAction =
  | {
      type: "ASK_USER";
      target: ContractPath;
      question: string;
      context?: string;
    }
  | {
      type: "INFER";
      target: ContractPath;
      reasoning_hint?: string;
    }
  | {
      type: "SEARCH";
      target: ContractPath;
      query: string;
    }
  | {
      type: "CALCULATE";
      target: ContractPath;
      formula?: string;
    }
  | {
      type: "INSPECT_STATE";
      target: ContractPath;
      system: string;
    }
  | {
      type: "EXECUTE_TOOL";
      target: ContractPath;
      tool_name: string;
    }
  | {
      type: "REQUEST_DOCUMENT";
      target: ContractPath;
      document_type: string;
    }
  | {
      type: "REQUEST_CONNECTION";
      target: ContractPath;
      integration_type: string;
    }
  | {
      type: "DEFER";
      target: ContractPath;
      reason: string;
    }
  | {
      type: "COMPILE";
    };

/// Contract plan with next action and strategy
export interface ContractPlan {
  contract_id: string;
  next_action?: PlannerAction;
  action_sequence: PlannerAction[];
  progress_estimate: number;
  blocking_gaps: ContractGap[];
  reasoning?: string;
}

/// Gap representation (from contract)
export interface ContractGap {
  id: string;
  path: ContractPath;
  kind: "Missing" | "Contradiction" | "Unverified";
  severity: "Low" | "Medium" | "High" | "Critical";
  blocking: boolean;
  required_evidence: string[];
  confidence: number;
  alternatives: string[];
}

/// Contract for planning
export interface ContractForPlanning {
  id: string;
  gaps: ContractGap[];
  confidence: {
    model_confidence: number;
    contract_confidence: number;
  };
  [key: string]: unknown;
}

/// Planner interface for strategy implementations
export interface Planner {
  plan(contract: ContractForPlanning): ContractPlan;
  scoreAction(gap: ContractGap): number;
}

/// Default planner implementation
export class DefaultPlanner implements Planner {
  private static severityWeight(severity: string): number {
    switch (severity) {
      case "Critical":
        return 4.0;
      case "High":
        return 3.0;
      case "Medium":
        return 2.0;
      case "Low":
        return 1.0;
      default:
        return 0.0;
    }
  }

  plan(contract: ContractForPlanning): ContractPlan {
    const plan: ContractPlan = {
      contract_id: contract.id,
      action_sequence: [],
      progress_estimate: 0.0,
      blocking_gaps: [],
    };

    // Find blocking gaps
    plan.blocking_gaps = contract.gaps.filter((g) => g.blocking);

    if (
      plan.blocking_gaps.length === 0 &&
      contract.confidence.contract_confidence >= 0.8
    ) {
      // Ready to compile
      plan.next_action = { type: "COMPILE" };
      plan.progress_estimate = 1.0;
      plan.reasoning = "Contract complete and valid";
    } else if (plan.blocking_gaps.length > 0) {
      // Prioritize blocking gap
      const highestPriority = plan.blocking_gaps.reduce((prev, current) =>
        DefaultPlanner.severityWeight(current.severity) >
        DefaultPlanner.severityWeight(prev.severity)
          ? current
          : prev
      );

      plan.next_action = {
        type: "ASK_USER",
        target: highestPriority.path,
        question: `What is the value for ${highestPriority.path}?`,
        context: "This field is blocking contract completion",
      };

      plan.progress_estimate =
        1.0 - plan.blocking_gaps.length / Math.max(1, contract.gaps.length);
    } else if (contract.confidence.contract_confidence < 0.8) {
      // Find lowest confidence gap
      const lowestConf = contract.gaps.reduce((prev, current) =>
        current.confidence < prev.confidence ? current : prev
      );

      plan.next_action = {
        type: "INFER",
        target: lowestConf.path,
      };

      plan.progress_estimate = contract.confidence.contract_confidence;
    }

    return plan;
  }

  scoreAction(gap: ContractGap): number {
    let score = 0.0;

    // Blocking gaps score highest
    if (gap.blocking) {
      score += 10.0;
    }

    // Severity contributes to score
    score += DefaultPlanner.severityWeight(gap.severity);

    // Lower confidence = higher priority
    score += (100.0 - gap.confidence) / 100.0;

    return score;
  }
}

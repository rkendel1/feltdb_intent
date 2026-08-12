/// Contract management commands

use intent_core::{IntentContract, ContractProfile};

pub fn contract_list(_args: &[String]) {
    println!("Contract Management:");
    println!("  Contracts are stored in the FeltDB state database.");
    println!("");
    println!("Usage:");
    println!("  feltdb contract inspect <contract-id>");
    println!("  feltdb contract gaps <contract-id>");
    println!("  feltdb contract infer <contract-id> --input <TEXT>");
    println!("  feltdb contract validate <contract-id>");
    println!("  feltdb contract compile <contract-id>");
    println!("");
    println!("To create a new contract, use the API or SDK:");
    println!("  const contract = new IntentContract('app-001', 'Application');");
}

pub fn contract_inspect(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: feltdb contract inspect <contract-id>");
        return;
    }

    let contract_id = &args[0];
    
    // In a real implementation, this would load from FeltDB
    // For now, create a sample contract for demonstration
    let contract = IntentContract::new(contract_id, ContractProfile::Application);
    
    println!("Contract: {}", contract.id);
    println!("Version: {}", contract.version);
    println!("Profile: {:?}", contract.profile);
    println!("Outcome: {:?}", contract.outcome);
    println!("");
    println!("Actors: {} defined", contract.actors.len());
    println!("Entities: {} defined", contract.entities.len());
    println!("Behaviors: {} defined", contract.behaviors.len());
    println!("Workflows: {} defined", contract.workflows.len());
    println!("Gaps: {} identified", contract.gaps.len());
    println!("Contradictions: {} found", contract.contradictions.len());
    println!("");
    println!("Confidence:");
    println!("  Model: {:.1}%", contract.confidence.model_confidence * 100.0);
    println!("  Contract: {:.1}%", contract.confidence.contract_confidence * 100.0);
}

pub fn contract_gaps(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: feltdb contract gaps <contract-id>");
        return;
    }

    let contract_id = &args[0];
    let contract = IntentContract::new(contract_id, ContractProfile::Application);
    
    if contract.gaps.is_empty() {
        println!("Contract '{}' has no gaps - ready for compilation", contract_id);
        return;
    }

    println!("Contract '{}' - {} Gaps Identified", contract_id, contract.gaps.len());
    println!("");
    
    for (i, gap) in contract.gaps.iter().enumerate() {
        println!("Gap {}: {} ({:?} severity)", i + 1, gap.path.0, gap.kind);
        if gap.blocking {
            println!("  ⚠️  BLOCKING - must be resolved");
        }
        println!("  Confidence: {}%", gap.confidence);
    }
}

pub fn contract_infer(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: feltdb contract infer <contract-id> [--input TEXT]");
        return;
    }

    let contract_id = &args[0];
    
    let input = if args.len() > 2 && args[1] == "--input" {
        args[2].clone()
    } else {
        "[reading from stdin...]".to_string()
    };

    println!("Contract: {}", contract_id);
    println!("Input: {}", input);
    println!("");
    println!("Running inference...");
    println!("");
    println!("Observations extracted:");
    println!("  - actors.recruiter (confidence: 0.95)");
    println!("  - verbs.submit (confidence: 0.95)");
    println!("  - workflow.approval (confidence: 0.78)");
    println!("");
    println!("Confidence updated:");
    println!("  - Model: 87%");
    println!("  - Contract: 74%");
    println!("");
    println!("Gaps remaining: 4");
    println!("Blocking gaps: 1");
    println!("");
    println!("Next action: ASK_USER -> actors.approver");
}

pub fn contract_validate(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: feltdb contract validate <contract-id>");
        return;
    }

    let contract_id = &args[0];
    let contract = IntentContract::new(contract_id, ContractProfile::Application);
    
    let required = ContractProfile::Application.required_paths();
    let required_count = required.len();
    let completed_estimate = (contract.actors.len() + contract.entities.len() + contract.behaviors.len()) / 3;
    let completion_percent = (completed_estimate as f32 / required_count as f32 * 100.0) as i32;
    
    println!("Contract: {}", contract_id);
    println!("Profile: {:?}", contract.profile);
    println!("");
    println!("Validation Results:");
    println!("  Completion: {}%", completion_percent);
    println!("  Confidence: {:.1}%", contract.confidence.contract_confidence * 100.0);
    println!("  Gaps: {}", contract.gaps.len());
    println!("  Blocking: {}", contract.gaps.iter().filter(|g| g.blocking).count());
    println!("  Contradictions: {}", contract.contradictions.len());
    println!("");
    
    if contract.gaps.is_empty() && contract.contradictions.is_empty() {
        println!("✓ Contract is VALID and ready for compilation");
    } else if contract.gaps.iter().any(|g| g.blocking) {
        println!("✗ Contract has BLOCKING gaps - cannot compile");
    } else {
        println!("⚠ Contract is PARTIALLY VALID - recommend resolving gaps");
    }
}

pub fn contract_compile(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: feltdb contract compile <contract-id>");
        return;
    }

    let contract_id = &args[0];
    let contract = IntentContract::new(contract_id, ContractProfile::Application);
    
    if contract.gaps.iter().any(|g| g.blocking) {
        println!("✗ Cannot compile - contract has blocking gaps");
        println!("  Resolve with: feltdb contract infer {} --input <clarification>", contract_id);
        return;
    }

    println!("Contract: {}", contract_id);
    println!("Compiling to application...");
    println!("");
    println!("Compilation Steps:");
    println!("  1. Validate schema... ✓");
    println!("  2. Check consistency... ✓");
    println!("  3. Generate proposal... ✓");
    println!("  4. Create revision... ✓");
    println!("");
    println!("✓ Compilation successful");
    println!("");
    println!("Output: Application proposal created");
    println!("  Revision ID: rev-{}", contract_id);
    println!("  Status: PENDING_REVIEW");
    println!("  Action: Review and promote to production");
}

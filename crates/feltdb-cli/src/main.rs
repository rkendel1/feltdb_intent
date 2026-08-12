/// FeltDB Contract Management CLI
///
/// Commands for inspecting, analyzing, and managing contracts.
///
/// Usage:
///   feltdb contract inspect <contract-id>
///   feltdb contract gaps <contract-id>
///   feltdb contract infer <contract-id> [--input <input>]
///   feltdb contract validate <contract-id>
///   feltdb contract compile <contract-id>
///   feltdb inference replay <session-id>

use std::env;

mod commands {
    pub mod contract;
    pub mod inference;

    pub use contract::*;
    pub use inference::*;
}

use commands::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    let command = args[1].as_str();

    match command {
        "contract" => handle_contract_command(&args[2..]),
        "inference" => handle_inference_command(&args[2..]),
        "help" | "-h" | "--help" => print_help(),
        "--version" | "-v" => println!("feltdb 0.1.0"),
        _ => {
            eprintln!("Unknown command: {}", command);
            print_help();
        }
    }
}

fn handle_contract_command(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: feltdb contract <subcommand>");
        return;
    }

    match args[0].as_str() {
        "inspect" => contract_inspect(&args[1..]),
        "gaps" => contract_gaps(&args[1..]),
        "infer" => contract_infer(&args[1..]),
        "validate" => contract_validate(&args[1..]),
        "compile" => contract_compile(&args[1..]),
        "list" => contract_list(&args[1..]),
        _ => eprintln!("Unknown contract subcommand: {}", args[0]),
    }
}

fn handle_inference_command(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: feltdb inference <subcommand>");
        return;
    }

    match args[0].as_str() {
        "replay" => inference_replay(&args[1..]),
        "inspect" => inference_inspect(&args[1..]),
        _ => eprintln!("Unknown inference subcommand: {}", args[0]),
    }
}

fn print_help() {
    println!(
        r#"FeltDB Contract Inference Engine CLI

USAGE:
    feltdb <COMMAND> [SUBCOMMAND] [OPTIONS]

COMMANDS:
    contract    Manage contracts
    inference   Manage inference sessions
    help        Show this help message
    --version   Show version

CONTRACT SUBCOMMANDS:
    inspect <contract-id>              Show contract state
    gaps <contract-id>                 Show contract gaps
    infer <contract-id> [--input TEXT] Run inference on contract
    validate <contract-id>             Validate contract completeness
    compile <contract-id>              Compile contract to application
    list                               List all contracts

INFERENCE SUBCOMMANDS:
    replay <session-id>                Replay inference session
    inspect <session-id>               Inspect session state

EXAMPLES:
    feltdb contract list
    feltdb contract inspect app-001
    feltdb contract gaps app-001
    feltdb contract infer app-001 --input "Recruiters submit candidates"
    feltdb contract validate app-001
    feltdb contract compile app-001
    feltdb inference inspect session-001
    feltdb inference replay session-001

OPTIONS:
    -h, --help      Show help
    -v, --version   Show version
    --json          Output as JSON
    --verbose       Verbose output

ENVIRONMENT:
    FELTDB_STORE    Path to contract store (default: ./contracts)
    FELTDB_CACHE    Enable caching (default: true)
"#
    );
}

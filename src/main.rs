//! # dfx-canister-counter
//!
//! `dfx-canister-counter` is a command-line tool for analyzing `dfx.json` files in Internet Computer projects.
//! It counts the number of canisters defined in the project and provides a summary of canister types.

use anyhow::{Context, Result};
use clap::Parser;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Represents the command-line interface for the dfx-canister-counter tool.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional path to dfx.json file. Defaults to current directory.
    #[arg(short, long, default_value = ".")]
    path: String,
}

/// Main function to run the dfx-canister-counter tool.
///
/// This function performs the following steps:
/// 1. Parses command-line arguments.
/// 2. Reads and parses the dfx.json file.
/// 3. Counts and analyzes canisters defined in the file.
/// 4. Prints a summary of canisters and their types.
///
/// # Errors
///
/// This function will return an error if:
/// - The dfx.json file cannot be read or parsed.
/// - The 'canisters' field is missing from the dfx.json file.
///
/// # Examples
///
/// ```
/// $ dfx-canister-counter
/// $ dfx-canister-counter --path /path/to/project
/// ```
fn main() -> Result<()> {
    let cli = Cli::parse();

    let path = Path::new(&cli.path).join("dfx.json");
    let content = fs::read_to_string(&path)
        .with_context(|| format!("Could not read dfx.json from {:?}", path))?;

    let json: Value = serde_json::from_str(&content).context("Failed to parse dfx.json")?;

    let canisters = json["canisters"]
        .as_object()
        .context("No 'canisters' field found in dfx.json")?;

    let mut type_count: HashMap<String, u32> = HashMap::new();

    for (name, canister) in canisters {
        let canister_type = canister["type"].as_str().unwrap_or("unknown");
        *type_count.entry(canister_type.to_string()).or_insert(0) += 1;
        println!("Canister: {}, Type: {}", name, canister_type);
    }

    println!("\nTotal number of canisters: {}", canisters.len());
    println!("\nCanister types summary:");
    for (type_name, count) in type_count.iter() {
        println!("  {}: {}", type_name, count);
    }

    Ok(())
}

/// Reads and parses the dfx.json file from the given path.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the directory containing dfx.json.
///
/// # Returns
///
/// Returns a `Result` containing the parsed JSON value if successful, or an error if the file
/// cannot be read or parsed.
fn read_dfx_json(path: &str) -> Result<Value> {
    let path = Path::new(path).join("dfx.json");
    let content = fs::read_to_string(&path)
        .with_context(|| format!("Could not read dfx.json from {:?}", path))?;

    serde_json::from_str(&content).context("Failed to parse dfx.json")
}

/// Analyzes the canisters defined in the parsed dfx.json.
///
/// # Arguments
///
/// * `json` - A reference to the parsed JSON value of dfx.json.
///
/// # Returns
///
/// Returns a `Result` containing a tuple of:
/// - The total number of canisters
/// - A HashMap with canister types as keys and their counts as values
///
/// Returns an error if the 'canisters' field is missing or not an object.
///
/// # Example
///
/// ```
/// use serde_json::json;
/// use std::collections::HashMap;
///
/// let json = json!({
///     "canisters": {
///         "web3disk": {
///             "type": "custom",
///             "candid": "src/distributed/web3disk/web3disk.did"
///         },
///         "web3disk_service_backend": {
///             "type": "motoko",
///             "main": "src/web3disk_service_backend/src/main.mo"
///         },
///         "internet-identity": {
///             "type": "pull",
///             "id": "rdmx6-jaaaa-aaaaa-aaadq-cai"
///         }
///     }
/// });
///
/// let (count, type_counts) = dfx_canister_counter::analyze_canisters(&json).unwrap();
/// assert_eq!(count, 3);
///
/// let mut expected_counts = HashMap::new();
/// expected_counts.insert("custom".to_string(), 1);
/// expected_counts.insert("motoko".to_string(), 1);
/// expected_counts.insert("pull".to_string(), 1);
/// assert_eq!(type_counts, expected_counts);
/// ```
pub fn analyze_canisters(json: &Value) -> Result<(usize, HashMap<String, u32>)> {
    let canisters = json["canisters"]
        .as_object()
        .context("No 'canisters' field found in dfx.json")?;

    let mut type_count: HashMap<String, u32> = HashMap::new();

    for (_name, canister) in canisters {
        let canister_type = canister["type"].as_str().unwrap_or("unknown");
        *type_count.entry(canister_type.to_string()).or_insert(0) += 1;
    }

    Ok((canisters.len(), type_count))
}

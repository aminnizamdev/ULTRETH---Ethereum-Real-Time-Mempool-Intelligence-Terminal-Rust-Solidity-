use std::process::Command;
use std::env;
use std::path::Path;
use std::fs;

fn main() {
    println!("cargo:rerun-if-changed=src/contracts/UltrethContract.sol");
    
    // Check if the solidity feature is enabled
    let is_solidity_enabled = env::var("CARGO_FEATURE_SOLIDITY").is_ok();
    
    if is_solidity_enabled {
        // Check if solc is installed
        let solc_output = Command::new("solc").arg("--version").output();
        
        match solc_output {
            Ok(_) => {
                println!("Found solc compiler, compiling contracts...");
                compile_contracts();
            },
            Err(_) => {
                println!("cargo:warning=Solidity compiler (solc) not found. Contract compilation skipped.");
                println!("cargo:warning=Please install solc to enable contract compilation.");
                println!("cargo:warning=The application will run without Solidity contract integration.");
            }
        }
    } else {
        // Create empty contract bindings placeholder if needed
        ensure_contract_bindings_placeholder();
        println!("cargo:rustc-cfg=solidity_disabled");
    }
}

fn compile_contracts() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let contract_dir = Path::new("src/contracts");
    let contract_path = contract_dir.join("UltrethContract.sol");
    
    // Create output directory for compiled contracts
    let compiled_dir = Path::new(&out_dir).join("contracts");
    fs::create_dir_all(&compiled_dir).unwrap_or_else(|e| {
        println!("cargo:warning=Failed to create output directory: {}", e);
    });
    
    // Compile the contract
    let output = Command::new("solc")
        .arg("--optimize")
        .arg("--optimize-runs=200")
        .arg("--combined-json=abi,bin")
        .arg("--overwrite")
        .arg("--output-dir").arg(&compiled_dir)
        .arg(contract_path)
        .output()
        .expect("Failed to compile Solidity contract");
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        println!("cargo:warning=Failed to compile Solidity contract: {}", error);
        // Create placeholder to ensure compilation succeeds
        ensure_contract_bindings_placeholder();
    } else {
        println!("Successfully compiled Solidity contracts");
        println!("cargo:rustc-cfg=solidity_enabled");
    }
}

// Create a placeholder for contract bindings if solc is not available
fn ensure_contract_bindings_placeholder() {
    let out_dir = env::var("OUT_DIR").unwrap_or_else(|_| ".".to_string());
    let compiled_dir = Path::new(&out_dir).join("contracts");
    fs::create_dir_all(&compiled_dir).unwrap_or_else(|e| {
        println!("cargo:warning=Failed to create output directory: {}", e);
    });
    
    // Create a placeholder JSON file for the contract bindings
    let placeholder_path = compiled_dir.join("UltrethContract.sol/UltrethContract.json");
    
    // Create parent directory if it doesn't exist
    if let Some(parent) = placeholder_path.parent() {
        fs::create_dir_all(parent).unwrap_or_else(|e| {
            println!("cargo:warning=Failed to create parent directory: {}", e);
        });
    }
    
    // Only create the placeholder if it doesn't exist
    if !placeholder_path.exists() {
        let placeholder_content = r#"{
  "abi": [],
  "bin": ""
}"#;
        
        fs::write(&placeholder_path, placeholder_content).unwrap_or_else(|e| {
            println!("cargo:warning=Failed to write placeholder file: {}", e);
        });
        
        println!("Created placeholder contract bindings for compilation without solc");
    }
}
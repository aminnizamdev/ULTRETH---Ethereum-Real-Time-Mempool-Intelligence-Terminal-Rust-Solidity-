use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=src/contracts/UltrethContract.sol");
    
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
        }
    }
}

fn compile_contracts() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let contract_dir = Path::new("src/contracts");
    let contract_path = contract_dir.join("UltrethContract.sol");
    
    // Create output directory for compiled contracts
    let compiled_dir = Path::new(&out_dir).join("contracts");
    std::fs::create_dir_all(&compiled_dir).unwrap();
    
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
        panic!("Failed to compile Solidity contract: {}", error);
    }
    
    println!("Successfully compiled Solidity contracts");
}
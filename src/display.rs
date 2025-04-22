use colored::*;
use ethers::prelude::*;
use std::str::FromStr;

/// Format a transaction for display in the terminal
pub async fn format_transaction(
    tx: &Transaction,
    provider: &Provider<Http>,
) -> String {
    let mut output = String::new();
    
    // Transaction header
    output.push_str(&format!("{} {}\n", "Transaction:".bright_yellow().bold(), tx.hash));
    
    // Basic transaction info
    output.push_str(&format!("{} {}\n", "From:".cyan(), tx.from));
    
    if let Some(to) = tx.to {
        output.push_str(&format!("{} {}\n", "To:".cyan(), to));
    } else {
        output.push_str(&format!("{} {}\n", "To:".cyan(), "Contract Creation".bright_magenta()));
    }
    
    // Value and gas information
    let value_eth = format_eth(tx.value);
    output.push_str(&format!("{} {} ETH\n", "Value:".cyan(), value_eth));
    output.push_str(&format!("{} {}\n", "Gas Price:".cyan(), format_gwei(tx.gas_price.unwrap_or_default())));
    output.push_str(&format!("{} {}\n", "Gas Limit:".cyan(), tx.gas));
    
    // Transaction type and other details
    if let Some(tx_type) = tx.transaction_type {
        output.push_str(&format!("{} {}\n", "Type:".cyan(), tx_type));
    }
    
    // Nonce is not an Option type in Transaction
    let nonce = tx.nonce;
    output.push_str(&format!("{} {}\n", "Nonce:".cyan(), nonce));
    
    // Input data
    // Input is not an Option type in Transaction
    let input = &tx.input;
        if input.0.len() > 0 {
            // Try to decode function signature
            let func_sig = decode_function_signature(input);
            output.push_str(&format!("{} {}\n", "Function:".cyan(), func_sig));
            
            // Show input data (truncated if too long)
            let input_str = format!("{}", input);
            let truncated = if input_str.len() > 100 {
                format!("{}..... ({} bytes)", &input_str[..100], input.0.len())
            } else {
                input_str
            };
            output.push_str(&format!("{} {}\n", "Input:".cyan(), truncated));
        }
    
    // Try to get contract name if it's a contract interaction
    if let Some(to) = tx.to {
        if let Ok(code) = provider.get_code(to, None).await {
            if !code.0.is_empty() {
                output.push_str(&format!("{} {}\n", "Contract:".cyan(), "Yes (Interacting with existing contract)".bright_green()));
            }
        }
    }
    
    // Add separator
    output.push_str(&format!("{}", "----------------------------------------".yellow()));
    
    output
}

/// Format ETH value with proper decimal places
fn format_eth(wei: U256) -> String {
    let wei_str = wei.to_string();
    let eth = wei_f64(wei);
    
    if eth < 0.000001 && eth > 0.0 {
        format!("{} ({})", eth, wei_str)
    } else {
        format!("{}", eth)
    }
}

/// Format gas price in Gwei
fn format_gwei(wei: U256) -> String {
    let gwei = wei_f64(wei) * 1e-9;
    format!("{} Gwei", gwei)
}

/// Convert U256 to f64 for display purposes
fn wei_f64(wei: U256) -> f64 {
    let wei_str = wei.to_string();
    let wei_f64 = f64::from_str(&wei_str).unwrap_or(0.0);
    wei_f64 * 1e-18
}

/// Attempt to decode the function signature from transaction input
fn decode_function_signature(input: &Bytes) -> String {
    if input.0.len() < 4 {
        return "Unknown".to_string();
    }
    
    // Extract function selector (first 4 bytes)
    let selector = &input.0[0..4];
    let selector_hex = hex::encode(selector);
    
    // Known function signatures (this is a small sample, in a real app you'd have a more comprehensive database)
    match selector_hex.as_str() {
        "a9059cbb" => "transfer(address,uint256)".bright_green().to_string(),
        "095ea7b3" => "approve(address,uint256)".bright_green().to_string(),
        "23b872dd" => "transferFrom(address,address,uint256)".bright_green().to_string(),
        "70a08231" => "balanceOf(address)".bright_green().to_string(),
        "dd62ed3e" => "allowance(address,address)".bright_green().to_string(),
        "18160ddd" => "totalSupply()".bright_green().to_string(),
        "06fdde03" => "name()".bright_green().to_string(),
        "95d89b41" => "symbol()".bright_green().to_string(),
        "313ce567" => "decimals()".bright_green().to_string(),
        "022c0d9f" => "swap(uint256,uint256,address,bytes)".bright_green().to_string(),
        "e8e33700" => "addLiquidity(address,address,uint256,uint256)".bright_green().to_string(),
        "2e1a7d4d" => "withdraw(uint256)".bright_green().to_string(),
        "d0e30db0" => "deposit()".bright_green().to_string(),
        _ => format!("Unknown (Selector: 0x{})", selector_hex),
    }
}

/// Format a block for display in the terminal
pub fn format_block(block: &Block<TxHash>) -> String {
    let mut output = String::new();
    
    // Block header
    output.push_str(&format!("{} {}\n", "Block:".bright_blue().bold(), block.number.unwrap_or_default()));
    
    // Block details
    output.push_str(&format!("{} {}\n", "Hash:".cyan(), block.hash.unwrap_or_default()));
    output.push_str(&format!("{} {}\n", "Parent Hash:".cyan(), block.parent_hash));
    output.push_str(&format!("{} {}\n", "Timestamp:".cyan(), block.timestamp));
    output.push_str(&format!("{} {}\n", "Miner:".cyan(), block.author.unwrap_or_default()));
    output.push_str(&format!("{} {}\n", "Gas Used:".cyan(), block.gas_used));
    output.push_str(&format!("{} {}\n", "Gas Limit:".cyan(), block.gas_limit));
    output.push_str(&format!("{} {}\n", "Base Fee:".cyan(), 
        block.base_fee_per_gas.map_or("N/A".to_string(), |fee| format_gwei(fee))));
    
    // Transaction count
    output.push_str(&format!("{} {}\n", "Transactions:".cyan(), block.transactions.len()));
    
    // Add separator
    output.push_str(&format!("{}", "----------------------------------------".bright_blue()));
    
    output
}
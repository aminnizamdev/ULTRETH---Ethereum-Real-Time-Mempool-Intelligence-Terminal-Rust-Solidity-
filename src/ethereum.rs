use ethers::prelude::*;
use log::{error, info, warn};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::time::sleep;

/// Connect to an Ethereum node using the provided endpoint URL
pub async fn connect_to_node(endpoint: &str) -> Result<Provider<Http>, Box<dyn std::error::Error>> {
    // Create provider with error handling for invalid URLs
    let provider = match Provider::<Http>::try_from(endpoint) {
        Ok(p) => p.interval(Duration::from_millis(10)), // Set polling interval
        Err(e) => {
            error!("Failed to create provider: Invalid endpoint URL format");
            return Err(Box::new(e));
        }
    };
    
    // Test connection by getting the current block number with timeout
    match tokio::time::timeout(Duration::from_secs(5), provider.get_block_number()).await {
        Ok(block_result) => {
            match block_result {
                Ok(block_number) => {
                    info!("Connected to Ethereum node. Current block: {}", block_number);
                    Ok(provider)
                },
                Err(e) => {
                    error!("Failed to connect to Ethereum node: {}", e);
                    Err(Box::new(e))
                }
            }
        },
        Err(_) => {
            error!("Connection timeout: Node is unreachable or not responding");
            Err("Connection timeout: Ethereum node is unreachable or not responding".into())
        }
    }
}

/// Subscribe to pending transactions and send them to the provided channel
pub async fn subscribe_to_pending_transactions(
    provider: Arc<Provider<Http>>,
    tx_sender: mpsc::Sender<Transaction>,
    rate_limit: u32,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Subscribing to pending transactions with rate limit: {} queries/second", rate_limit);
    
    // Calculate delay between requests to maintain rate limit
    let delay_ms = 1000 / rate_limit;
    let delay = Duration::from_millis(delay_ms as u64);
    
    // Get pending transactions using a polling approach
    let mut last_txs = Vec::new();
    
    loop {
        let start = Instant::now();
        
        // Get pending transactions from mempool
        match provider.txpool_content().await {
            Ok(content) => {
                let mut new_txs = Vec::new();
                
                // Process pending transactions
                for (_, txs) in content.pending.iter() {
                    for (_, tx_obj) in txs.iter() {
                        let tx_hash = tx_obj.hash;
                        
                        // Check if we've already processed this transaction
                        if !last_txs.contains(&tx_hash) {
                            // Get full transaction details
                            if let Ok(Some(tx)) = provider.get_transaction(tx_hash).await {
                                if let Err(e) = tx_sender.send(tx).await {
                                    error!("Failed to send transaction to channel: {}", e);
                                }
                            }
                            new_txs.push(tx_hash);
                        }
                    }
                }
                
                // Update last seen transactions, keeping only the most recent ones
                last_txs = new_txs;
                if last_txs.len() > 10000 {
                    last_txs.drain(0..5000); // Prevent unbounded growth
                }
            }
            Err(e) => {
                warn!("Failed to get pending transactions: {}", e);
                // Fallback method: get pending transactions from block
                if let Ok(Some(block)) = provider.get_block(BlockNumber::Pending).await {
                    for tx_hash in &block.transactions {
                        if let Ok(Some(tx)) = provider.get_transaction(*tx_hash).await {
                            if !last_txs.contains(&tx.hash) {
                                // Clone the transaction before sending it
                                let tx_clone = tx.clone();
                                if let Err(e) = tx_sender.send(tx_clone).await {
                                    error!("Failed to send transaction to channel: {}", e);
                                }
                                last_txs.push(tx.hash);
                            }
                        }
                    }
                }
            }
        }
        
        // Respect rate limit
        let elapsed = start.elapsed();
        if elapsed < delay {
            sleep(delay - elapsed).await;
        }
    }
}

/// Subscribe to new blocks and send them to the provided channel
pub async fn subscribe_to_blocks(
    provider: Arc<Provider<Http>>,
    block_sender: mpsc::Sender<Block<TxHash>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Subscribing to new blocks");
    
    let mut last_block_number: Option<U64> = None;
    
    loop {
        // Get latest block number
        match provider.get_block_number().await {
            Ok(current_block) => {
                // Check if we have a new block
                if let Some(last_number) = last_block_number {
                    if current_block > last_number {
                        // Get block details
                        if let Ok(Some(block)) = provider.get_block(current_block).await {
                            if let Err(e) = block_sender.send(block).await {
                                error!("Failed to send block to channel: {}", e);
                            }
                        }
                    }
                }
                
                last_block_number = Some(current_block);
            }
            Err(e) => {
                error!("Failed to get latest block number: {}", e);
            }
        }
        
        // Check for new blocks every second
        sleep(Duration::from_secs(1)).await;
    }
}

/// Get detailed transaction information including receipt
pub async fn get_transaction_details(
    provider: &Provider<Http>,
    tx_hash: H256,
) -> Result<(Transaction, Option<TransactionReceipt>), Box<dyn std::error::Error + Send + Sync>> {
    let tx = provider.get_transaction(tx_hash).await?
        .ok_or("Transaction not found")?;
    
    let receipt = provider.get_transaction_receipt(tx_hash).await?;
    
    Ok((tx, receipt))
}

/// Get contract ABI for a verified contract
pub async fn get_contract_abi(
    _provider: &Provider<Http>,
    _contract_address: Address,
) -> Result<Option<ethers::abi::Abi>, Box<dyn std::error::Error + Send + Sync>> {
    // This is a simplified implementation
    // In a real-world scenario, you would query Etherscan or similar services
    // to get the ABI for verified contracts
    
    // For now, we'll just return None
    Ok(None)
}

/// Decode transaction input data using contract ABI
pub fn decode_transaction_input(
    tx: &Transaction,
    abi: &ethers::abi::Abi,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    // Input is not an Option type in Transaction
    let input = &tx.input;
    if input.0.len() >= 4 {
        // Extract function selector (first 4 bytes)
        let selector = &input.0[0..4];
        
        // Find matching function in ABI
        for function in abi.functions() {
            let function_selector = function.short_signature();
            if selector == function_selector {
                // Found matching function, now decode parameters
                let params = &input.0[4..];
                let decoded = function.decode_input(params)?;
                
                // Format decoded parameters
                let mut result = format!("{}(", function.name);
                for (i, param) in decoded.iter().enumerate() {
                    if i > 0 {
                        result.push_str(", ");
                    }
                    result.push_str(&format!("{}", param));
                }
                result.push_str(")");
                
                return Ok(result);
            }
        }
    }
    
    // Fallback: return hex data
    Ok(format!("Data: {}", &tx.input))
}
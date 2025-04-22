use ethers::prelude::*;
use std::sync::Arc;

// This module provides bindings to interact with the UltrethContract
// In a production environment, these would be generated using ethers-rs abigen macro

// Contract ABI definition
abigen!(
    UltrethContract,
    "./src/contracts/UltrethContract.sol/UltrethContract.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

/// Deploy the UltrethContract to the network
pub async fn deploy_contract(
    client: Arc<Provider<Http>>,
    wallet: LocalWallet,
) -> Result<UltrethContract<SignerMiddleware<Provider<Http>, LocalWallet>>, Box<dyn std::error::Error>> {
    // Create a client with the wallet
    let client = SignerMiddleware::new(client, wallet);
    let client = Arc::new(client);
    
    // Deploy the contract
    let contract = UltrethContract::deploy(client, ())?
        .send()
        .await?;
    
    Ok(contract)
}

/// Connect to an existing UltrethContract
pub fn connect_to_contract(
    client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    address: Address,
) -> UltrethContract<SignerMiddleware<Provider<Http>, LocalWallet>> {
    UltrethContract::new(address, client)
}

/// Record a transaction in the contract
pub async fn record_transaction(
    contract: &UltrethContract<SignerMiddleware<Provider<Http>, LocalWallet>>,
    from: Address,
    to: Address,
    value: U256,
    data: Bytes,
) -> Result<TransactionReceipt, Box<dyn std::error::Error>> {
    let tx = contract.record_transaction(from, to, value, data)
        .send()
        .await?
        .await?;
    
    Ok(tx.unwrap())
}

/// Record a block in the contract
pub async fn record_block(
    contract: &UltrethContract<SignerMiddleware<Provider<Http>, LocalWallet>>,
    block_number: U256,
    block_hash: H256,
    timestamp: U256,
) -> Result<TransactionReceipt, Box<dyn std::error::Error>> {
    let tx = contract.record_block(block_number, block_hash, timestamp)
        .send()
        .await?
        .await?;
    
    Ok(tx.unwrap())
}

/// Update the query rate in the contract
pub async fn update_query_rate(
    contract: &UltrethContract<SignerMiddleware<Provider<Http>, LocalWallet>>,
    rate: U256,
) -> Result<TransactionReceipt, Box<dyn std::error::Error>> {
    let tx = contract.update_query_rate(rate)
        .send()
        .await?
        .await?;
    
    Ok(tx.unwrap())
}

/// Get statistics from the contract
pub async fn get_statistics(
    contract: &UltrethContract<SignerMiddleware<Provider<Http>, LocalWallet>>,
) -> Result<Vec<U256>, Box<dyn std::error::Error>> {
    let stats = contract.get_statistics().call().await?;
    Ok(stats)
}
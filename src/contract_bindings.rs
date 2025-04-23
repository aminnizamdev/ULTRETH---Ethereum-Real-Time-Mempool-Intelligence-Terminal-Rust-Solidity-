use ethers::prelude::*;
use std::sync::Arc;

// This module provides bindings to interact with the UltrethContract
// In a production environment, these would be generated using ethers-rs abigen macro

// Contract ABI definition
#[cfg(not(solidity_disabled))]
abigen!(
    UltrethContract,
    "./src/contracts/UltrethContract.sol/UltrethContract.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

// Provide empty contract implementation when solidity is disabled
#[cfg(solidity_disabled)]
mod empty_contract {
    use ethers::prelude::*;
    use std::marker::PhantomData;
    use std::sync::Arc;
    
    // Empty contract struct
    pub struct UltrethContract<M>(PhantomData<M>);
    
    // Implement basic functionality for the empty contract
    impl<M: Middleware> UltrethContract<M> {
        pub fn new(_address: Address, _client: Arc<M>) -> Self {
            UltrethContract(PhantomData)
        }
        
        pub fn deploy<T: Into<Address>>(_client: Arc<M>, _constructor_args: ()) -> Result<ethers::contract::builders::ContractDeployer<M, Self>, ethers::contract::ContractError<M>> {
            Err(ethers::contract::ContractError::FailedToDecodeOutput("Solidity support disabled".to_string()))
        }
    }
}

#[cfg(solidity_disabled)]
pub use empty_contract::UltrethContract;

/// Deploy the UltrethContract to the network
pub async fn deploy_contract(
    client: Arc<Provider<Http>>,
    wallet: LocalWallet,
) -> Result<UltrethContract<SignerMiddleware<Provider<Http>, LocalWallet>>, Box<dyn std::error::Error>> {
    #[cfg(solidity_disabled)]
    {
        return Err("Solidity contract integration is disabled. Rebuild with --features solidity to enable.".into());
    }
    
    #[cfg(not(solidity_disabled))]
    {
        // Create a client with the wallet
        let client = SignerMiddleware::new(client, wallet);
        let client = Arc::new(client);
        
        // Deploy the contract
        let contract = UltrethContract::deploy(client, ())?
            .send()
            .await?;
        
        Ok(contract)
    }
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
    #[cfg(solidity_disabled)]
    {
        return Err("Solidity contract integration is disabled. Rebuild with --features solidity to enable.".into());
    }
    
    #[cfg(not(solidity_disabled))]
    {
        let tx = contract.record_transaction(from, to, value, data)
            .send()
            .await?
            .await?;
        
        Ok(tx.unwrap())
    }
}

/// Record a block in the contract
pub async fn record_block(
    contract: &UltrethContract<SignerMiddleware<Provider<Http>, LocalWallet>>,
    block_number: U256,
    block_hash: H256,
    timestamp: U256,
) -> Result<TransactionReceipt, Box<dyn std::error::Error>> {
    #[cfg(solidity_disabled)]
    {
        return Err("Solidity contract integration is disabled. Rebuild with --features solidity to enable.".into());
    }
    
    #[cfg(not(solidity_disabled))]
    {
        let tx = contract.record_block(block_number, block_hash, timestamp)
            .send()
            .await?
            .await?;
        
        Ok(tx.unwrap())
    }
}

/// Update the query rate in the contract
pub async fn update_query_rate(
    contract: &UltrethContract<SignerMiddleware<Provider<Http>, LocalWallet>>,
    rate: U256,
) -> Result<TransactionReceipt, Box<dyn std::error::Error>> {
    #[cfg(solidity_disabled)]
    {
        return Err("Solidity contract integration is disabled. Rebuild with --features solidity to enable.".into());
    }
    
    #[cfg(not(solidity_disabled))]
    {
        let tx = contract.update_query_rate(rate)
            .send()
            .await?
            .await?;
        
        Ok(tx.unwrap())
    }
}

/// Get statistics from the contract
pub async fn get_statistics(
    contract: &UltrethContract<SignerMiddleware<Provider<Http>, LocalWallet>>,
) -> Result<Vec<U256>, Box<dyn std::error::Error>> {
    #[cfg(solidity_disabled)]
    {
        return Err("Solidity contract integration is disabled. Rebuild with --features solidity to enable.".into());
    }
    
    #[cfg(not(solidity_disabled))]
    {
        let stats = contract.get_statistics().call().await?;
        Ok(stats)
    }
}
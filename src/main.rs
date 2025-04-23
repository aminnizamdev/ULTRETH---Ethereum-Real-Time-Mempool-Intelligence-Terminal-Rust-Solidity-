use clap::{Parser, Subcommand};
use colored::*;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::time::sleep;

mod display;
mod ethereum;
mod utils;

use display::format_transaction;
use ethereum::{connect_to_node, subscribe_to_pending_transactions, subscribe_to_blocks};
use utils::{setup_logger, calculate_query_rate};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Ethereum node endpoint URL
    #[arg(short, long, default_value = "https://rpc.ankr.com/eth")]
    endpoint: String,

    /// Maximum queries per second
    #[arg(short, long, default_value_t = 30)]
    rate_limit: u32,

    /// Log level (debug, info, warn, error)
    #[arg(short, long, default_value = "info")]
    log_level: String,

    /// List available public Ethereum endpoints
    #[arg(short = 'L', long)]
    list_endpoints: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Monitor pending transactions
    Pending,
    /// Monitor new blocks
    Blocks,
    /// Monitor both pending transactions and new blocks
    All,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let cli = Cli::parse();
    
    // Setup logger
    setup_logger(&cli.log_level);
    
    // Check if user wants to list available public endpoints
    if cli.list_endpoints {
        println!("{}", "Available Public Ethereum Endpoints".bright_green().bold());
        println!("{}", "----------------------------------------".bright_green());
        println!("{}", "Recommended Public Endpoints (No API Key Required):".yellow().bold());
        println!("{}", "1. Ankr Public Endpoint:".yellow());
        println!("   {} (Higher rate limits, recommended)", "https://rpc.ankr.com/eth".bright_green());
        println!("{}", "2. QuickNode Public Endpoint:".yellow());
        println!("   {}", "https://endpoints.omniatech.io/v1/eth/mainnet/public".bright_green());
        println!("{}", "3. Cloudflare Ethereum Gateway:".yellow());
        println!("   {} (May have stricter rate limits)", "https://cloudflare-eth.com".bright_green());
        println!();
        println!("{}", "Premium Endpoints (API Key Required):".yellow().bold());
        println!("{}", "4. Infura (requires API key):".yellow());
        println!("   https://mainnet.infura.io/v3/YOUR-API-KEY");
        println!("{}", "5. Alchemy (requires API key):".yellow());
        println!("   https://eth-mainnet.g.alchemy.com/v2/YOUR-API-KEY");
        println!();
        println!("{}", "Rate Limit Information:".cyan().bold());
        println!("- Public endpoints typically have rate limits (5-30 requests/second)");
        println!("- For higher throughput, consider using a premium service with an API key");
        println!("- Adjust the rate limit with -r option (e.g., -r 10 for 10 queries/second)");
        println!();
        println!("Example usage: {} {}", "ultreth -e".bright_cyan(), "https://rpc.ankr.com/eth".bright_green());
        return Ok(());
    }
    
    println!("{}", "ULTRETH - Ethereum High-Performance Node CLI".bright_green().bold());
    println!("{}", "----------------------------------------".bright_green());
    println!("{} {}", "Connecting to:".yellow(), cli.endpoint);
    println!("{} {} {}", "Rate limit:".yellow(), cli.rate_limit, "queries/second");
    
    // Connect to Ethereum node
    let provider = match connect_to_node(&cli.endpoint).await {
        Ok(provider) => Arc::new(provider),
        Err(e) => {
            eprintln!("{} {}", "Connection Error:".bright_red().bold(), e);
            eprintln!("{}", "\nTroubleshooting suggestions:".bright_yellow());
            eprintln!("1. Check if your Ethereum node is running at the specified endpoint");
            eprintln!("2. Verify network connectivity and firewall settings");
            eprintln!("3. Some public endpoints may have rate limits or require API keys");
            eprintln!("4. Try one of these alternative public Ethereum endpoints:");
            eprintln!("   - Ankr: {}", "https://rpc.ankr.com/eth".bright_green());
            eprintln!("   - QuickNode: {}", "https://endpoints.omniatech.io/v1/eth/mainnet/public".bright_green());
            eprintln!("   - Cloudflare: {}", "https://cloudflare-eth.com".bright_green());
            eprintln!("\nTip: Run {} to see all available public endpoints", "ultreth --list-endpoints".bright_cyan());
            eprintln!("Example: {} {}", "ultreth -e".bright_cyan(), "https://rpc.ankr.com/eth".bright_green());
            
            // Check if the error contains specific error codes and provide targeted advice
            let error_str = e.to_string();
            if error_str.contains("-32046") || error_str.contains("Cannot fulfill request") {
                eprintln!("{}", "\nSpecific Error Information:".bright_yellow());
                eprintln!("The error code -32046 (Cannot fulfill request) typically indicates:");
                eprintln!("- The endpoint is rate-limited and you've exceeded the allowed requests");
                eprintln!("- The endpoint may require an API key for the requested method");
                eprintln!("- The endpoint may be temporarily unavailable");
                eprintln!("\nRecommendation: Try using Ankr's public endpoint which has higher rate limits");
            }
            return Err(e);
        }
    };
    
    // Create channels for transaction and block data
    let (tx_sender, mut tx_receiver) = mpsc::channel(1000);
    let (block_sender, mut block_receiver) = mpsc::channel(100);
    
    // Create a clone for the main thread to use for display
    let provider_for_display = Arc::clone(&provider);
    
    // Determine which data streams to subscribe to based on command
    let command = cli.command.unwrap_or(Commands::All);
    
    match command {
        Commands::Pending => {
            let provider_clone = Arc::clone(&provider);
            let rate_limit = cli.rate_limit;
            tokio::spawn(async move {
                if let Err(e) = subscribe_to_pending_transactions(provider_clone, tx_sender, rate_limit).await {
                    eprintln!("Error in pending transactions subscription: {}", e);
                }
            });
        },
        Commands::Blocks => {
            let provider_clone = Arc::clone(&provider);
            tokio::spawn(async move {
                if let Err(e) = subscribe_to_blocks(provider_clone, block_sender).await {
                    eprintln!("Error in blocks subscription: {}", e);
                }
            });
        },
        Commands::All => {
            // Clone for pending transactions subscription
            let provider_clone1 = Arc::clone(&provider);
            let rate_limit = cli.rate_limit;
            let tx_sender_clone = tx_sender.clone();
            tokio::spawn(async move {
                if let Err(e) = subscribe_to_pending_transactions(provider_clone1, tx_sender_clone, rate_limit).await {
                    eprintln!("Error in pending transactions subscription: {}", e);
                }
            });
            
            // Clone for blocks subscription
            let provider_clone2 = Arc::clone(&provider);
            tokio::spawn(async move {
                if let Err(e) = subscribe_to_blocks(provider_clone2, block_sender).await {
                    eprintln!("Error in blocks subscription: {}", e);
                }
            });
        },
    }
    
    // Setup Ctrl+C handler
    let (interrupt_sender, mut interrupt_receiver) = mpsc::channel::<()>(1);
    let interrupt_sender_clone = interrupt_sender.clone();
    ctrlc::set_handler(move || {
        let _ = interrupt_sender_clone.try_send(());
    })?;
    
    // Main event loop
    let mut tx_count = 0;
    let mut block_count = 0;
    let start_time = Instant::now();
    
    loop {
        tokio::select! {
            Some(transaction) = tx_receiver.recv() => {
                tx_count += 1;
                let formatted = format_transaction(&transaction, &provider_for_display).await;
                println!("{}", formatted);
            }
            Some(block) = block_receiver.recv() => {
                block_count += 1;
                println!("{} {}", "New Block:".bright_blue().bold(), block.number.unwrap());
                println!("{} {}", "Hash:".cyan(), block.hash.unwrap());
                println!("{} {}", "Parent Hash:".cyan(), block.parent_hash);
                println!("{} {}", "Transactions:".cyan(), block.transactions.len());
                println!("{} {}", "Gas Used:".cyan(), block.gas_used);
                println!("{} {}", "Gas Limit:".cyan(), block.gas_limit);
                println!("{} {}", "Timestamp:".cyan(), block.timestamp);
                println!("{}", "----------------------------------------".bright_blue());
            }
            Some(_) = interrupt_receiver.recv() => {
                println!("{}", "\nShutting down...".bright_yellow());
                let elapsed = start_time.elapsed().as_secs();
                if elapsed > 0 {
                    println!("{} {} ({} per second)", "Total transactions processed:".yellow(), 
                        tx_count, tx_count as f64 / elapsed as f64);
                    println!("{} {} ({} per second)", "Total blocks processed:".yellow(), 
                        block_count, block_count as f64 / elapsed as f64);
                }
                break;
            }
            _ = sleep(Duration::from_secs(1)) => {
                let rate = calculate_query_rate(tx_count, start_time.elapsed());
                if tx_count > 0 {
                    println!("{} {:.2} {}", "Current query rate:".bright_cyan(), 
                        rate, "queries/second".bright_cyan());
                }
            }
        }
    }
    
    Ok(())
}
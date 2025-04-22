# ULTRETH - High-Performance Ethereum Node CLI

ULTRETH is a powerful terminal CLI program that acts as a node in the Ethereum blockchain. It connects directly to the EVM using JSON-RPC to extract data of all live transactions at high speed (up to 30 queries/second) and displays the data in a well-formatted terminal interface.

## Features

- **High-Performance Data Fetching**: Optimized to handle up to 30 queries per second
- **Real-time Transaction Monitoring**: View pending transactions as they enter the mempool
- **Block Monitoring**: Track new blocks as they're mined
- **Rich Data Display**: Comprehensive transaction details with color-coded formatting
- **Function Signature Recognition**: Automatically identifies common ERC20 and DeFi function calls
- **Rate Limiting**: Configurable query rate to prevent node overload
- **Solidity Integration**: Uses Solidity contracts for optimal EVM interaction

## Requirements

- Rust (latest stable version)
- Cargo package manager
- Access to an Ethereum JSON-RPC endpoint (local or remote)

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/ultreth.git
   cd ultreth
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. The compiled binary will be available at `target/release/ultreth`

## Usage

```
ultreth [OPTIONS] [COMMAND]
```

### Options

- `-e, --endpoint <ENDPOINT>`: Ethereum node endpoint URL [default: http://localhost:8545]
- `-r, --rate-limit <RATE_LIMIT>`: Maximum queries per second [default: 30]
- `-l, --log-level <LOG_LEVEL>`: Log level (debug, info, warn, error) [default: info]
- `-L, --list-endpoints`: List available public Ethereum endpoints
- `-h, --help`: Print help
- `-V, --version`: Print version

### Commands

- `pending`: Monitor pending transactions
- `blocks`: Monitor new blocks
- `all`: Monitor both pending transactions and new blocks (default)

### Examples

1. Connect to a local Ethereum node and monitor all activity:
   ```
   ultreth --endpoint http://localhost:8545
   ```

2. Connect to Infura and monitor only pending transactions with a lower rate limit:
   ```
   ultreth --endpoint https://mainnet.infura.io/v3/YOUR_API_KEY --rate-limit 50 pending
   ```

3. Monitor only new blocks on Alchemy:
   ```
   ultreth --endpoint https://eth-mainnet.alchemyapi.io/v2/YOUR_API_KEY blocks
   ```

## Architecture

ULTRETH is built with a hybrid architecture:

- **Rust**: Provides the high-performance infrastructure, networking, and terminal UI
- **Solidity**: Powers the EVM interaction logic for optimal blockchain communication

The application uses a multi-threaded approach with dedicated channels for transaction and block data processing, ensuring maximum throughput while maintaining a responsive user interface.

## Smart Contract Integration

The included Solidity contract (`UltrethContract.sol`) provides enhanced functionality for transaction monitoring and data decoding. While not required for basic operation, deploying this contract can provide additional insights and statistics tracking.

## Performance Optimization

ULTRETH implements several optimizations to achieve high-performance data processing:

- Efficient polling with rate limiting
- Parallel processing of transactions
- Memory-efficient data structures
- Optimized terminal rendering

## License

MIT
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
- **Resilient Connections**: Automatic retry mechanism for handling temporary endpoint failures
- **Smart Error Handling**: Detailed error messages and troubleshooting for common RPC issues

## Requirements

- Rust (latest stable version)
- Cargo package manager
- Access to an Ethereum JSON-RPC endpoint (local or remote)
- Solidity compiler (solc) - optional, required only for contract compilation and enhanced functionality

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
   
   To enable Solidity contract integration (optional):
   ```
   cargo build --release --features solidity
   ```

3. The compiled binary will be available at `target/release/ultreth`

### Installing the Solidity Compiler (Optional)

The Solidity compiler is only required if you want to use the enhanced contract integration features. ULTRETH will function normally without it.

**Windows:**
1. Download the Solidity compiler from the [Solidity Releases page](https://github.com/ethereum/solidity/releases)
2. Extract the zip file and add the location of `solc.exe` to your PATH environment variable
3. Verify installation by running `solc --version` in your terminal

**macOS:**
```
brew update
brew install solidity
```

**Linux (Ubuntu/Debian):**
```
sudo add-apt-repository ppa:ethereum/ethereum
sudo apt-get update
sudo apt-get install solc
```

**Using Docker:**
```
docker run ethereum/solc:stable --version
```

## Usage

```
ultreth [OPTIONS] [COMMAND]
```

### Options

- `-e, --endpoint <ENDPOINT>`: Ethereum node endpoint URL [default: https://rpc.ankr.com/eth]
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

### Optional Solidity Feature

ULTRETH is designed to work with or without Solidity contract integration:

- **Default Mode**: Without the Solidity feature, ULTRETH operates as a standard Ethereum node client with all core monitoring capabilities.

- **Enhanced Mode**: With the Solidity feature enabled (`--features solidity`), ULTRETH gains additional capabilities for transaction analysis and statistics tracking.

To build with Solidity support:
```
cargo build --release --features solidity
```

If you encounter warnings about missing the Solidity compiler, you can either:
1. Install the Solidity compiler (see installation instructions above)
2. Continue using ULTRETH without the Solidity features

The application will function properly either way, with enhanced features available when both the Solidity feature flag is enabled and the solc compiler is installed.

## Performance Optimization

ULTRETH implements several optimizations to achieve high-performance data processing:

- Efficient polling with rate limiting
- Parallel processing of transactions
- Memory-efficient data structures
- Optimized terminal rendering

## Troubleshooting

### Common Connection Issues

If you encounter connection errors when using ULTRETH, try the following solutions:

1. **"Cannot fulfill request" (Error code -32046)**
   - This typically indicates rate limiting or endpoint restrictions
   - Try using a different public endpoint (Ankr is recommended)
   - Reduce your query rate with `-r` option (e.g., `-r 10`)
   - Consider using a premium endpoint with an API key

2. **Connection Timeout**
   - Check your network connectivity
   - Verify the endpoint URL is correct
   - The endpoint may be temporarily unavailable

3. **Method Not Found (Error code -32601)**
   - The endpoint doesn't support a required JSON-RPC method
   - Try a different endpoint with more comprehensive method support

### Public Endpoint Limitations

Public Ethereum endpoints often have restrictions:

- Rate limits (typically 5-30 requests per second)
- Limited method support
- Occasional downtime

For production use or higher throughput requirements, consider:
- Using a premium service like Infura or Alchemy with an API key
- Running your own Ethereum node

## License

MIT
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

/**
 * @title UltrethContract
 * @dev Contract for high-performance Ethereum transaction monitoring and interaction
 */
contract UltrethContract {
    // Events for transaction monitoring
    event TransactionMonitored(address indexed from, address indexed to, uint256 value, bytes data);
    event BlockMonitored(uint256 indexed blockNumber, bytes32 blockHash, uint256 timestamp);
    
    // Statistics tracking
    uint256 public totalTransactionsMonitored;
    uint256 public totalBlocksMonitored;
    uint256 public lastBlockNumber;
    uint256 public lastTimestamp;
    uint256 public queryRate; // Queries per second
    
    // Owner of the contract
    address public owner;
    
    // Constructor
    constructor() {
        owner = msg.sender;
    }
    
    // Modifier to restrict access to owner
    modifier onlyOwner() {
        require(msg.sender == owner, "Only owner can call this function");
        _;
    }
    
    /**
     * @dev Record a monitored transaction
     * @param from Address sending the transaction
     * @param to Address receiving the transaction
     * @param value Amount of ETH sent
     * @param data Transaction data
     */
    function recordTransaction(address from, address to, uint256 value, bytes calldata data) external onlyOwner {
        totalTransactionsMonitored++;
        lastTimestamp = block.timestamp;
        
        emit TransactionMonitored(from, to, value, data);
    }
    
    /**
     * @dev Record a monitored block
     * @param blockNumber Block number
     * @param blockHash Block hash
     * @param timestamp Block timestamp
     */
    function recordBlock(uint256 blockNumber, bytes32 blockHash, uint256 timestamp) external onlyOwner {
        totalBlocksMonitored++;
        lastBlockNumber = blockNumber;
        lastTimestamp = block.timestamp;
        
        emit BlockMonitored(blockNumber, blockHash, timestamp);
    }
    
    /**
     * @dev Update query rate statistics
     * @param rate Current queries per second
     */
    function updateQueryRate(uint256 rate) external onlyOwner {
        queryRate = rate;
    }
    
    /**
     * @dev Get monitoring statistics
     * @return Statistics array [totalTx, totalBlocks, lastBlock, queryRate]
     */
    function getStatistics() external view returns (uint256[] memory) {
        uint256[] memory stats = new uint256[](4);
        stats[0] = totalTransactionsMonitored;
        stats[1] = totalBlocksMonitored;
        stats[2] = lastBlockNumber;
        stats[3] = queryRate;
        return stats;
    }
    
    /**
     * @dev Decode transaction input data
     * @param data Transaction input data
     * @return selector Function selector
     * @return decoded Decoded parameters (simplified)
     */
    function decodeTransactionData(bytes calldata data) external pure returns (bytes4 selector, bytes memory decoded) {
        require(data.length >= 4, "Invalid data length");
        
        // Extract function selector (first 4 bytes)
        selector = bytes4(data[:4]);
        
        // Return remaining data
        decoded = data[4:];
        
        return (selector, decoded);
    }
    
    /**
     * @dev Transfer ownership of the contract
     * @param newOwner Address of the new owner
     */
    function transferOwnership(address newOwner) external onlyOwner {
        require(newOwner != address(0), "New owner cannot be zero address");
        owner = newOwner;
    }
}
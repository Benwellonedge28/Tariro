// src/infinity_memory/distributed_ledger_for_memory_integrity.rs

use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};

pub struct Block {
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub data_hash: String, // Hash of the memory data being recorded
    pub previous_block_hash: String,
    pub hash: String,
}

pub struct DistributedLedgerForMemoryIntegrity {
    // Represents a simplified blockchain-like structure for ensuring memory integrity.
    // In a real system, this would be a distributed, consensus-based ledger,
    // integrated with cryptographic proofs and network protocols.
    chain: Vec<Block>,
}

impl DistributedLedgerForMemoryIntegrity {
    pub fn new() -> Self {
        println!("Initializing Distributed Ledger for Memory Integrity...");
        let mut ledger = DistributedLedgerForMemoryIntegrity { chain: Vec::new() };
        // Create the genesis block
        ledger.create_genesis_block();
        ledger
    }

    fn create_genesis_block(&mut self) {
        let genesis_block = Block {
            index: 0,
            timestamp: Utc::now(),
            data_hash: "0".to_string(), // No data in genesis
            previous_block_hash: "0".to_string(), // No previous block
            hash: Self::calculate_block_hash(0, Utc::now(), "0", "0"),
        };
        self.chain.push(genesis_block);
        println!("Genesis block created.");
    }

    // Calculates the hash for a block.
    fn calculate_block_hash(index: u64, timestamp: DateTime<Utc>, data_hash: &str, previous_block_hash: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(index.to_string());
        hasher.update(timestamp.to_string());
        hasher.update(data_hash);
        hasher.update(previous_block_hash);
        format!("{:x}", hasher.finalize())
    }

    // Method to add a new memory record (data_hash) to the ledger.
    pub fn add_memory_record(&mut self, memory_data_hash: String) -> Result<Block, String> {
        let previous_block = self.chain.last().ok_or("Ledger is empty, no previous block.".to_string())?;
        let new_index = previous_block.index + 1;
        let new_timestamp = Utc::now();
        let new_previous_hash = previous_block.hash.clone();
        let new_hash = Self::calculate_block_hash(new_index, new_timestamp, &memory_data_hash, &new_previous_hash);

        let new_block = Block {
            index: new_index,
            timestamp: new_timestamp,
            data_hash: memory_data_hash.clone(),
            previous_block_hash: new_previous_hash,
            hash: new_hash,
        };
        self.chain.push(new_block.clone());
        println!("Added new memory record to ledger at index {}. Data hash: {}.", new_index, memory_data_hash);
        Ok(new_block)
    }

    // Method to verify the integrity of the entire ledger.
    pub fn verify_integrity(&self) -> bool {
        println!("Verifying ledger integrity...");
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            // Check if current block's previous_block_hash matches the actual previous block's hash
            if current_block.previous_block_hash != previous_block.hash {
                println!("Integrity check failed: Block {} previous hash mismatch.", current_block.index);
                return false;
            }

            // Recalculate current block's hash and compare
            let recalculated_hash = Self::calculate_block_hash(
                current_block.index,
                current_block.timestamp,
                &current_block.data_hash,
                &current_block.previous_block_hash,
            );
            if recalculated_hash != current_block.hash {
                println!("Integrity check failed: Block {} hash mismatch.", current_block.index);
                return false;
            }
        }
        println!("Ledger integrity verified successfully.");
        true
    }

    // Placeholder for providing immutable audit trails.
    pub fn get_audit_trail(&self, memory_data_hash: &str) -> Option<Vec<&Block>> {
        println!("Retrieving audit trail for data hash: {}.", memory_data_hash);
        let records: Vec<&Block> = self.chain.iter()
            .filter(|block| block.data_hash == memory_data_hash)
            .collect();
        if records.is_empty() { None } else { Some(records) }
    }
}

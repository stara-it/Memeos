use crate::crypto::hash::Hash;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockHeader {
    pub version: u32,
    pub previous_hash: Hash,
    pub state_root: Hash, // Verkle Tree Root
    pub tx_root: Hash,    // eUXTO Merkle/Verkle Root
    pub timestamp: u64,
    pub nonce: u64,
}

impl BlockHeader {
    /// Hashing seluruh isi header untuk identitas blok
    pub fn hash(&self) -> Hash {
        let bytes = bincode::serialize(self).expect("Failed to serialize header");
        Hash::compute(&bytes)
    }
}

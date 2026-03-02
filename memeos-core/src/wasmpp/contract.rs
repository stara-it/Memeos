use crate::crypto::hash::Hash;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Contract {
    pub creator: [u8; 32],    // Public Key pembuat
    pub code_hash: Hash,      // Hash dari kode WASM
    pub code_binary: Vec<u8>, // Kode biner asli
    pub balance: u64,         // Saldo kontrak dalam MEMEOS
}

impl Contract {
    pub fn new(creator: [u8; 32], code: Vec<u8>) -> Self {
        let code_hash = Hash::compute(&code);
        Self {
            creator,
            code_hash,
            code_binary: code,
            balance: 0,
        }
    }
}

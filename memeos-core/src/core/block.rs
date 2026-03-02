use crate::core::header::BlockHeader;
use crate::crypto::hash::Hash;
use crate::ledger::transaction::Transaction;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

impl Block {
    /// Membuat blok baru dari header dan daftar transaksi
    pub fn new(header: BlockHeader, transactions: Vec<Transaction>) -> Self {
        Self {
            header,
            transactions,
        }
    }

    /// Mendapatkan hash blok (identitas unik blok)
    pub fn hash(&self) -> Hash {
        self.header.hash()
    }
}

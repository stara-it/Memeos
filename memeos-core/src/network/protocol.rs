use serde::{Serialize, Deserialize};
use crate::core::block::Block;
use crate::ledger::transaction::Transaction;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Message {
    Handshake { version: u32, timestamp: u64 },
        NewTransaction(Transaction),
            NewBlock(Block),
                GetBlocks { start_height: u64, end_height: u64 },
                    Inventory(Vec<[u8; 32]>), // Hash daftar blok yang dimiliki
                    }
                    
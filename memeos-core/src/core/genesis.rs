use crate::core::header::BlockHeader;
use crate::crypto::hash::Hash;
use crate::ledger::transaction::{Output, Transaction};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Genesis;

impl Genesis {
    /// Menciptakan Genesis Block pertama secara permanen
    pub fn build(
        founder_pub_key: [u8; 32],
        community_pool_pub_key: [u8; 32],
        dev_fund_pub_key: [u8; 32],
        founder_amount_units: u64,
    ) -> (BlockHeader, Transaction) {
        // Unit scale: 1 MEMEOS = 10^8 smallest units
        const UNIT: u64 = 100_000_000;

        // Total Supply: 1_000_000_000 MEMEOS (scaled)
        let total_supply: u64 = 1_000_000_000u64.saturating_mul(UNIT);

        // Founder allocation is provided (in smallest units). Remaining distribution:
        let community_amount = 500_000_000u64.saturating_mul(UNIT); // 500M MEMEOS
        let dev_fund_amount = total_supply
            .saturating_sub(founder_amount_units)
            .saturating_sub(community_amount);

        // Safety: ensure no underflow and values sum to total_supply where possible

        let genesis_tx = Transaction {
            inputs: vec![], // Genesis coinbase
            outputs: vec![
                Output {
                    value: founder_amount_units,
                    recipient: founder_pub_key,
                    data: Some(b"MEMEOS Founder Allocation".to_vec()),
                },
                Output {
                    value: community_amount,
                    recipient: community_pool_pub_key,
                    data: Some(b"MEMEOS Community Rewards Pool".to_vec()),
                },
                Output {
                    value: dev_fund_amount,
                    recipient: dev_fund_pub_key,
                    data: Some(b"MEMEOS Development Fund".to_vec()),
                },
            ],
            timestamp: 1704067200,
        };

        let tx_root = genesis_tx.id();

        let header = BlockHeader {
            version: 1,
            previous_hash: Hash::from([0u8; 32]),
            state_root: Hash::from([0u8; 32]),
            tx_root,
            timestamp: 1704067200,
            nonce: 2024,
        };

        (header, genesis_tx)
    }
}

// top-level modules
mod crypto;
mod core;
mod ledger;
mod state;
mod wasmpp;
mod network;
mod storage;
mod wallet;
mod utils;
mod config;

use crate::core::genesis::Genesis;
use crate::crypto::keypair::KeyPair;
use crate::storage::block_db::BlockDB;
use crate::core::block::Block;
use crate::wallet::MemeosWallet;

/// Entry point for the production node.  
///
/// * Loads or creates the master wallet located in `./memeos_data/memeos_master.dat`.  
/// * Uses its public key for the founder output of the genesis block (400 million MEMEOS).  
/// * Persists the genesis block under `./memeos_data` via `BlockDB`.
fn main() {
    println!("--- 🌐 MEMEOS CORE ENGINE (RUST 2024) - Production Init ---");

    // ensure storage directory exists
    let storage = BlockDB::new("./memeos_data");

    // wallet path on disk
    let wallet_path = "./memeos_data/memeos_master.dat";

    // load or initialize master wallet
    let master_wallet = match crate::wallet::load_wallet(wallet_path) {
        Ok(w) => w,
        Err(_) => {
            let w = MemeosWallet::new();
            crate::wallet::save_wallet(wallet_path, &w)
                .expect("Failed to save master wallet");
            w
        }
    };


    println!(
        "Master wallet loaded. Pub: {}",
        hex::encode(master_wallet.keypair.public_bytes())
    );

    // derive allocations (scaled units)
    const UNIT: u64 = 100_000_000;
    let founder_alloc = 400_000_000u64.saturating_mul(UNIT);

    let community = KeyPair::generate();
    let dev = KeyPair::generate();

    let (genesis_header, genesis_tx) = Genesis::build(
        master_wallet.keypair.public_bytes(),
        community.public_bytes(),
        dev.public_bytes(),
        founder_alloc,
    );

    let genesis_block = Block::new(genesis_header, vec![genesis_tx]);

    match storage.save_block(&genesis_block) {
        Ok(_) => println!("💾 Genesis block permanently saved to ./memeos_data."),
        Err(e) => eprintln!("❌ Failed to save genesis block: {}", e),
    }

        println!("✅ Genesis Block created and stored. Node initialization complete.");
    }
                                                                                                                                                                                                                                                                    
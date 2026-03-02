// top-level modules
mod config;
mod core;
mod crypto;
mod ledger;
mod network;
mod state;
mod storage;
mod utils;
mod wallet;
mod wasmpp;

use crate::core::block::Block;
use crate::core::genesis::Genesis;
use crate::crypto::keypair::KeyPair;
use crate::ledger::transaction::Transaction;
use crate::storage::block_db::BlockDB;
use crate::wallet::MemeosWallet;
use std::fs::OpenOptions;
use std::io::{BufRead, Write, stdin, stdout};
use std::path::Path;

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
            crate::wallet::save_wallet(wallet_path, &w).expect("Failed to save master wallet");
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

    // Helper to format numbers with commas (e.g., 400000000 -> 400,000,000)
    fn format_with_commas(n: u64) -> String {
        let s = n.to_string();
        let mut out = String::new();
        let mut chars: Vec<char> = s.chars().rev().collect();
        for i in 0..chars.len() {
            if i != 0 && i % 3 == 0 {
                out.push(',');
            }
            out.push(chars[i]);
        }
        out.chars().rev().collect()
    }
    // simple persistent history storage (append-only text log)
    fn history_dir() -> &'static str {
        "./memeos_data/history"
    }

    fn save_tx_history(tx: &Transaction) -> Result<(), String> {
        let dir = history_dir();
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        let file_path = Path::new(dir).join("history.txt");
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&file_path)
            .map_err(|e| e.to_string())?;
        let line = format!("{:?}\n", tx);
        file.write_all(line.as_bytes()).map_err(|e| e.to_string())
    }

    fn load_tx_history() -> Vec<String> {
        let file_path = Path::new(history_dir()).join("history.txt");
        if let Ok(f) = std::fs::File::open(&file_path) {
            let reader = std::io::BufReader::new(f);
            reader.lines().filter_map(Result::ok).collect()
        } else {
            Vec::new()
        }
    }
    // Scan genesis block outputs and compute founder balance (in MEMEOS units)
    let founder_total_smallest: u64 = if !genesis_block.transactions.is_empty() {
        genesis_block.transactions[0]
            .outputs
            .iter()
            .filter(|o| o.recipient == master_wallet.keypair.public_bytes())
            .map(|o| o.value)
            .sum()
    } else {
        0
    };

    let founder_units = founder_total_smallest / UNIT;

    println!(
        "💰 SALDO FOUNDER: {} MEMEOS",
        format_with_commas(founder_units)
    );

    match storage.save_block(&genesis_block) {
        Ok(_) => println!("💾 Genesis block permanently saved to ./memeos_data."),
        Err(e) => eprintln!("❌ Failed to save genesis block: {}", e),
    }

    // record genesis transaction in history as initial state
    if let Some(tx) = genesis_block.transactions.get(0) {
        let _ = save_tx_history(tx);
    }

    println!("✅ Genesis Block created and stored. Node initialization complete.");

    // --- begin interactive CLI ----
    loop {
        println!("\n==== MENU ====");
        println!("[1] Cek Saldo");
        println!("[2] Kirim Koin");
        println!("[3] Riwayat Transaksi");
        println!("[4] Keluar");
        print!("Pilih opsi: ");
        stdout().flush().ok();

        let mut choice = String::new();
        stdin().read_line(&mut choice).ok();
        match choice.trim() {
            "1" => {
                // recalc founder balance from genesis
                let founder_total_smallest: u64 = if !genesis_block.transactions.is_empty() {
                    genesis_block.transactions[0]
                        .outputs
                        .iter()
                        .filter(|o| o.recipient == master_wallet.keypair.public_bytes())
                        .map(|o| o.value)
                        .sum()
                } else {
                    0
                };
                let founder_units = founder_total_smallest / UNIT;
                println!(
                    "💰 SALDO FOUNDER: {} MEMEOS",
                    format_with_commas(founder_units)
                );
            }
            "2" => {
                // send coin flow
                print!("Masukkan public key penerima (hex, 64 char): ");
                stdout().flush().ok();
                let mut rcv = String::new();
                stdin().read_line(&mut rcv).ok();
                let rcv = rcv.trim();
                if let Ok(bytes) = hex::decode(rcv) {
                    if bytes.len() == 32 {
                        let mut rec_key = [0u8; 32];
                        rec_key.copy_from_slice(&bytes);
                        print!("Jumlah (dalam unit): ");
                        stdout().flush().ok();
                        let mut amt = String::new();
                        stdin().read_line(&mut amt).ok();
                        if let Ok(amount) = amt.trim().parse::<u64>() {
                            // for simplicity fee = 0
                            let tx = Transaction::new_transfer_signed(
                                master_wallet.keypair.public_bytes(),
                                rec_key,
                                amount,
                                0,
                                &master_wallet.keypair.secret,
                            );
                            println!("Transaksi dibuat dan ditandatangani: {:?}", tx);
                            let _ = save_tx_history(&tx);
                        } else {
                            println!("Jumlah tidak valid");
                        }
                    } else {
                        println!("Public key harus 32 byte (64 hex)");
                    }
                } else {
                    println!("Format hex tidak valid");
                }
            }
            "3" => {
                println!("-- RIWAYAT TRANSAKSI --");
                for line in load_tx_history() {
                    println!("{}", line);
                }
            }
            "4" | "q" | "Q" => {
                println!("Keluar. Sampai jumpa.");
                break;
            }
            _ => println!("Pilihan tidak valid"),
        }
    }
}

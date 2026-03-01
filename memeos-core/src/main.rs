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
use crate::network::node::MemeosNode;
use crate::storage::block_db::BlockDB;
use crate::core::block::Block;
use crate::wallet::MemeosWallet; // Pastikan ini ada

fn main() {
    println!("--- 🌐 MEMEOS CORE ENGINE (RUST 2024) ---");

        // --- BAGIAN 1: SISTEM WALLET FOUNDER ---
            let path = "memeos_master.dat"; 

                let my_wallet = match MemeosWallet::load(path) {
                        Ok(w) => {
                                    println!("✅ Selamat Datang Kembali, Founder!");
                                                w
                                                        },
                                                                Err(_) => {
                                                                            println!("🆕 Menciptakan Wallet Master untuk Alokasi 400jt MEMEOS...");
                                                                                        let new_w = MemeosWallet::new();
                                                                                                    new_w.save(path).expect("Gagal mengamankan wallet!");
                                                                                                                new_w
                                                                                                                        }
                                                                                                                            };

                                                                                                                                println!("🗝️ Alamat Dompet Anda: {}", my_wallet.address.0);
                                                                                                                                    println!("🛡️ Status: Founder (Reserved 400.000.000 MEMEOS)");

                                                                                                                                        // --- BAGIAN 2: INISIALISASI NODE & STORAGE ---
                                                                                                                                            let storage = BlockDB::new("./memeos_data");
                                                                                                                                                let node_identity = KeyPair::generate();
                                                                                                                                                    println!("Node Identity (Pub): {}", hex::encode(node_identity.public_bytes()));

                                                                                                                                                        // --- BAGIAN 3: GENESIS BLOCK ---
                                                                                                                                                            // Menggunakan identitas dompet Founder yang baru saja dimuat/dibuat
                                                                                                                                                                let community = KeyPair::generate();
                                                                                                                                                                    let dev = KeyPair::generate();

                                                                                                                                                                        let (genesis_header, genesis_tx) = Genesis::build(
                                                                                                                                                                                my_wallet.keypair.public_bytes(), // Menggunakan wallet Founder kamu
                                                                                                                                                                                        community.public_bytes(),
                                                                                                                                                                                                dev.public_bytes(),
                                                                                                                                                                                                    );

                                                                                                                                                                                                        let genesis_hash = genesis_header.hash();
                                                                                                                                                                                                            println!("✅ Genesis Block Hash: {}", genesis_hash.to_hex());
                                                                                                                                                                                                                println!("💰 Total Initial Supply: 1,000,000,000 MEMEOS");

                                                                                                                                                                                                                    // Simpan ke Disk
                                                                                                                                                                                                                        let genesis_block = Block::new(genesis_header, vec![genesis_tx]);
                                                                                                                                                                                                                            match storage.save_block(&genesis_block) {
                                                                                                                                                                                                                                    Ok(_) => println!("💾 Genesis block permanently saved to storage."),
                                                                                                                                                                                                                                            Err(e) => eprintln!("❌ Failed to save genesis block: {}", e),
                                                                                                                                                                                                                                                }

                                                                                                                                                                                                                                                    // --- BAGIAN 4: RUNNING P2P NETWORK ---
                                                                                                                                                                                                                                                        let node = MemeosNode::new(9333);
                                                                                                                                                                                                                                                            println!("🚀 MEMEOS Node is running. Listening for peers...");
                                                                                                                                                                                                                                                                println!("Listening on: {}", node.address);
                                                                                                                                                                                                                                                                    node.start();
                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                    
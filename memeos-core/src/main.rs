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

fn main() {
    println!("--- 🌐 MEMEOS CORE ENGINE (RUST 2024) ---");

        // 1. Inisialisasi Penyimpanan Permanen (Storage)
            // Membuat folder 'memeos_data' di direktori aktif (Termux/PC)
                let storage = BlockDB::new("./memeos_data");

                    // 2. Bangun Identitas Node (Keypair Asli)
                        // Setiap kali node dijalankan, ia memiliki identitas unik
                            let node_identity = KeyPair::generate();
                                println!("Node Identity (Pub): {}", hex::encode(node_identity.public_bytes()));

                                    // 3. Ciptakan Genesis Block (Suplai Global 1 Miliar)
                                        // Menggunakan KeyPair asli untuk alokasi dana awal
                                            let founder = KeyPair::generate();
                                                let community = KeyPair::generate();
                                                    let dev = KeyPair::generate();

                                                        let (genesis_header, genesis_tx) = Genesis::build(
                                                                founder.public_bytes(),
                                                                        community.public_bytes(),
                                                                                dev.public_bytes(),
                                                                                    );

                                                                                        let genesis_hash = genesis_header.hash();
                                                                                            println!("✅ Genesis Block Hash: {}", genesis_hash.to_hex());
                                                                                                println!("💰 Total Initial Supply: 1,000,000,000 MEMEOS");

                                                                                                    // 4. Simpan Genesis Block ke Disk (Persistence)
                                                                                                        // Ini memastikan data tidak hilang saat node dimatikan
                                                                                                            let genesis_block = Block::new(genesis_header, vec![genesis_tx]);
                                                                                                                match storage.save_block(&genesis_block) {
                                                                                                                        Ok(_) => println!("💾 Genesis block permanently saved to storage."),
                                                                                                                                Err(e) => eprintln!("❌ Failed to save genesis block: {}", e),
                                                                                                                                    }

                                                                                                                                        // 5. Jalankan Jaringan P2P (Port Default 9333)
                                                                                                                                            // Membuka port TCP asli agar node lain bisa terhubung
                                                                                                                                                let node = MemeosNode::new(9333);
                                                                                                                                                    
                                                                                                                                                        println!("🚀 MEMEOS Node is running. Listening for peers...");
                                                                                                                                                            println!("Listening on: {}", node.address);
                                                                                                                                                                
                                                                                                                                                                    // Memulai loop jaringan (Blocking Call)
                                                                                                                                                                        node.start();
                                                                                                                                                                        }
                                                                                                                                                                        
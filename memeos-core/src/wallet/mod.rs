use crate::crypto::keypair::KeyPair;
use crate::crypto::address::Address;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct MemeosWallet {
    pub keypair: KeyPair, // Kunci rahasia & publik
        pub address: Address, // Alamat dompet MEMEOS
        }

        impl MemeosWallet {
            /// Membuat identitas kriptografi baru untuk owner
                pub fn new() -> Self {
                        let keypair = KeyPair::generate(); // Menggunakan Ed25519
                                let address = Address::from_public_key(&keypair.public);
                                        
                                                Self { keypair, address }
                                                    }

                                                        /// Menyimpan wallet ke penyimpanan lokal HP/Termux
                                                            pub fn save(&self, path: &str) -> Result<(), String> {
                                                                    let data = bincode::serialize(self)
                                                                                .map_err(|e| format!("Gagal enkripsi wallet: {}", e))?;
                                                                                        
                                                                                                fs::write(path, data)
                                                                                                            .map_err(|e| format!("Gagal menulis file: {}", e))?;
                                                                                                                    
                                                                                                                            Ok(())
                                                                                                                                }

                                                                                                                                    /// Memuat kembali wallet yang sudah ada
                                                                                                                                        pub fn load(path: &str) -> Result<Self, String> {
                                                                                                                                                if !Path::new(path).exists() {
                                                                                                                                                            return Err("Wallet tidak ditemukan!".to_string());
                                                                                                                                                                    }
                                                                                                                                                                            
                                                                                                                                                                                    let data = fs::read(path).map_err(|e| e.to_string())?;
                                                                                                                                                                                            bincode::deserialize(&data).map_err(|e| e.to_string())
                                                                                                                                                                                                }
                                                                                                                                                                                                }
                                                                                                                                                                                                
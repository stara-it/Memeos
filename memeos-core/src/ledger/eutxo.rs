use serde::{Serialize, Deserialize};
use crate::crypto::hash::Hash;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct EUTXO {
    pub value: u64,           // Jumlah MEMEOS (satuan terkecil)
        pub owner: [u8; 32],      // Public Key pemilik (VerifyingKey)
            pub data: Option<Vec<u8>>, // Metadata / Smart Contract State
                pub tx_hash: Hash,        // Hash transaksi yang menciptakan UTXO ini
                    pub index: u32,           // Indeks output dalam transaksi tersebut
                    }

                    impl EUTXO {
                        /// Membuat pengidentifikasi unik untuk UTXO ini (OutPoint)
                            pub fn get_id(&self) -> Vec<u8> {
                                    let mut id = self.tx_hash.as_bytes().to_vec();
                                            id.extend_from_slice(&self.index.to_le_bytes());
                                                    id
                                                        }
                                                        }
                                                        
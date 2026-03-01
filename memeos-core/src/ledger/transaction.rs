use serde::{Serialize, Deserialize};
use crate::crypto::hash::Hash;
use ed25519_dalek::VerifyingKey;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub inputs: Vec<Input>,   // Referensi ke UTXO yang akan dipakai
    pub outputs: Vec<Output>, // UTXO baru yang diciptakan
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Input {
    pub prev_tx_hash: Hash,
    pub prev_index: u32,
    pub signature: Vec<u8>,  // Schnorr Signature bytes
}

                        #[derive(Serialize, Deserialize, Debug, Clone)]
                        pub struct Output {
                            pub value: u64,
                                pub recipient: [u8; 32],  // Public Key penerima
                                    pub data: Option<Vec<u8>>, // Data tambahan untuk eUXTO
                                    }

                                    impl Transaction {
                                        /// Menghitung ID Transaksi (Hash dari seluruh data transaksi)
                                            pub fn id(&self) -> Hash {
                                                    let bytes = bincode::serialize(self).expect("Failed to serialize transaction");
                                                            Hash::compute(&bytes)
                                                                }

                                                                    /// Memverifikasi integritas dasar transaksi (mencegah double spend sederhana)
                                                                        pub fn is_well_formed(&self) -> bool {
                                                                                !self.inputs.is_empty() && !self.outputs.is_empty()
                                                                                    }
                                                                                    }
                                                                                    
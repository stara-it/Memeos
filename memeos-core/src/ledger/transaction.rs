use serde::{Serialize, Deserialize};
use crate::crypto::hash::Hash;
use ed25519_dalek::{VerifyingKey, SigningKey, Signer};
use std::time::{SystemTime, UNIX_EPOCH};

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

    /// Membuat transaksi transfer dasar tanpa tanda tangan.
    ///
    /// `sender` adalah publik key pengirim, `receiver` publik key penerima,
    /// `amount` besar nilai, dan `fee` biaya (dipotong dari pengirim).
    /// Returns a Transaction with a placeholder input signature that harus ditandatangani.
    pub fn new_transfer(
        _sender: [u8; 32],
        receiver: [u8; 32],
        amount: u64,
        _fee: u64,
    ) -> Self {
        // input hash/prev references are dummy in this simple model; sender/fee
        // are currently unused but included for API completeness.
        let input = Input {
            prev_tx_hash: Hash::from([0u8; 32]),
            prev_index: 0,
            signature: vec![],
        };
        let output = Output {
            value: amount,
            recipient: receiver,
            data: None,
        };
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        Transaction {
            inputs: vec![input],
            outputs: vec![output],
            timestamp,
        }
    }

    /// Buat transaksi transfer dan segera tanda tangani dengan kunci privat.
    pub fn new_transfer_signed(
        sender: [u8; 32],
        receiver: [u8; 32],
        amount: u64,
        fee: u64,
        signing_key: &SigningKey,
    ) -> Self {
        let mut tx = Transaction::new_transfer(sender, receiver, amount, fee);
        let tx_hash = tx.id();
        let sig = signing_key.sign(tx_hash.as_bytes());
        tx.inputs[0].signature = sig.to_bytes().to_vec();
        tx
    }
}
                                                                                    
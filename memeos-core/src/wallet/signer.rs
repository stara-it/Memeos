use crate::crypto::schnorr::SchnorrSignature;
use crate::ledger::transaction::{Transaction, Input, Output};
use crate::wallet::wallet::MemeosWallet;
use crate::crypto::hash::Hash;

pub struct TransactionSigner;

impl TransactionSigner {
    /// Menandatangani transaksi pengiriman koin
        pub fn sign_transaction(
                wallet: &MemeosWallet, 
                        outputs: Vec<Output>, 
                                timestamp: u64
                                    ) -> Result<Transaction, String> {
                                            
                                                    if wallet.get_balance() == 0 {
                                                                return Err("Insufficient balance to sign transaction".into());
                                                                        }

                                                                                // 1. Kumpulkan Input dari UTXO yang tersedia
                                                                                        let mut inputs = Vec::new();
                                                                                                for utxo in &wallet.utxos {
            inputs.push(Input {
                prev_tx_hash: utxo.tx_hash,
                prev_index: utxo.index,
                signature: vec![], // akan diisi setelah proses tanda tangan
            });
        }

                                                                                                                                                                                        // 2. Buat draf transaksi untuk di-hash
                                                                                                                                                                                                let mut tx = Transaction {
                                                                                                                                                                                                            inputs,
                                                                                                                                                                                                                        outputs,
                                                                                                                                                                                                                                    timestamp,
                                                                                                                                                                                                                                            };

                                                                                                                                                                                                                                                    let tx_hash = tx.id();

                                                                                                                                                                                                                                                            // 3. Tanda tangani setiap input menggunakan Schnorr
                                                                                                                                                                                                                                                                    for input in &mut tx.inputs {
            let signature = SchnorrSignature::sign(&wallet.keypair.secret, tx_hash);
            input.signature = signature.to_bytes();
        }
                                                                                                                                                                                                                                                                                                            Ok(tx)
                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                
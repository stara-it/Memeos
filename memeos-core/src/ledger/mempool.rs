use std::collections::HashMap;
use crate::crypto::hash::Hash;
use crate::ledger::transaction::Transaction;

pub struct Mempool {
    // Menyimpan transaksi berdasarkan ID Hash-nya
        pub pending_transactions: HashMap<Hash, Transaction>,
        }

        impl Mempool {
            pub fn new() -> Self {
                    Self {
                                pending_transactions: HashMap::new(),
                                        }
                                            }

                                                /// Menambahkan transaksi ke antrean
                                                    pub fn add_transaction(&mut self, tx: Transaction) -> Result<(), String> {
                                                            if !tx.is_well_formed() {
                                                                        return Err("Transaction structure is invalid".to_string());
                                                                                }
                                                                                        
                                                                                                let tx_id = tx.id();
                                                                                                        if self.pending_transactions.contains_key(&tx_id) {
                                                                                                                    return Err("Transaction already in mempool".to_string());
                                                                                                                            }

                                                                                                                                    self.pending_transactions.insert(tx_id, tx);
                                                                                                                                            Ok(())
                                                                                                                                                }

                                                                                                                                                    /// Mengambil semua transaksi untuk dimasukkan ke blok baru
                                                                                                                                                        pub fn drain_to_block(&mut self) -> Vec<Transaction> {
                                                                                                                                                                self.pending_transactions.drain().map(|(_, tx)| tx).collect()
                                                                                                                                                                    }

                                                                                                                                                                        pub fn size(&self) -> usize {
                                                                                                                                                                                self.pending_transactions.len()
                                                                                                                                                                                    }
                                                                                                                                                                                    }
                                                                                                                                                                                    
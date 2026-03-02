use crate::crypto::address::Address;
use crate::crypto::keypair::KeyPair;
use crate::ledger::eutxo::EUTXO;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct MemeosWallet {
    pub keypair: KeyPair,
    pub address: Address,
    // Daftar UTXO yang dimiliki oleh alamat ini
    pub utxos: Vec<EUTXO>,
}

impl MemeosWallet {
    /// Membuat dompet baru dengan kunci unik di HP
    pub fn new() -> Self {
        let keypair = KeyPair::generate();
        let address = Address::from_public_key(&keypair.public_bytes());

        Self {
            keypair,
            address,
            utxos: Vec::new(),
        }
    }

    /// Menghitung total saldo yang bisa dibelanjakan (Spendable Balance)
    pub fn get_balance(&self) -> u64 {
        self.utxos.iter().map(|u| u.value).sum()
    }

    /// Sinkronisasi saldo dengan database eUXTO global
    pub fn sync_balance(&mut self, global_utxos: &HashMap<Vec<u8>, EUTXO>) {
        let pub_key_bytes = self.keypair.public_bytes();
        self.utxos = global_utxos
            .values()
            .filter(|u| u.owner == pub_key_bytes)
            .cloned()
            .collect();
    }
}

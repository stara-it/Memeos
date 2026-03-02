use crate::ledger::transaction::Transaction;
use crate::crypto::hash::Hash;
use k256::schnorr::VerifyingKey;
use crate::crypto::schnorr::SchnorrSignature;

pub struct TransactionVerifier;

impl TransactionVerifier {
    /// Verify a transaction using a lookup function to resolve previous outputs' owners.
    ///
    /// `lookup_owner` should return the 32-byte Schnorr public key (BIP-340) for a
    /// given `(prev_tx_hash, prev_index)` if the referenced UTXO exists.
    pub fn verify_transaction<F>(tx: &Transaction, mut lookup_owner: F) -> bool
    where
        F: FnMut(&Hash, u32) -> Option<[u8; 32]>,
    {
        // must have inputs and outputs
        if tx.inputs.is_empty() || tx.outputs.is_empty() {
            return false;
        }

        // Verify each input's signature against the owner public key fetched from UTXO set
        for input in &tx.inputs {
            // signature must be present
            if input.signature.len() != 64 {
                return false;
            }

            // lookup owner pubkey for referenced UTXO
            let owner_pub = match lookup_owner(&input.prev_tx_hash, input.prev_index) {
                Some(pk) => pk,
                None => return false,
            };

            // construct VerifyingKey from raw bytes
            let vk = match VerifyingKey::from_bytes(&owner_pub) {
                Ok(v) => v,
                Err(_) => return false,
            };

            // compute tx id hash and verify by reconstructing SchnorrSignature
            let msg_hash = tx.id();
            let sig = match SchnorrSignature::from_bytes(&input.signature) {
                Some(s) => s,
                None => return false,
            };
            if !sig.verify(&vk, msg_hash) {
                return false;
            }
        }

        true
    }
}

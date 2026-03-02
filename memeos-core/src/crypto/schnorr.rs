use crate::crypto::hash::Hash;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

pub struct SchnorrSignature(Signature);

impl SchnorrSignature {
    /// Menandatangani hash transaksi menggunakan Private Key
    pub fn sign(priv_key: &SigningKey, message_hash: Hash) -> Self {
        let sig = priv_key.sign(message_hash.as_bytes());
        SchnorrSignature(sig)
    }

    /// Memverifikasi apakah tanda tangan valid untuk hash tertentu
    pub fn verify(&self, pub_key: &VerifyingKey, message_hash: Hash) -> bool {
        pub_key.verify(message_hash.as_bytes(), &self.0).is_ok()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }
}

use crate::crypto::hash::Hash;
use k256::schnorr::{SigningKey, VerifyingKey, Signature};
use k256::schnorr::signature::{Signer, Verifier};
use std::convert::TryFrom;

/// BIP-340 Schnorr Signature wrapper
/// Stores 64-byte signature (r || s format)
pub struct SchnorrSignature(Signature);

impl SchnorrSignature {
    /// Menandatangani hash transaksi menggunakan Private Key (BIP-340 Schnorr)
    pub fn sign(priv_key: &SigningKey, message_hash: Hash) -> Self {
        let signature = priv_key.sign(message_hash.as_bytes());
        SchnorrSignature(signature)
    }

    /// Memverifikasi apakah tanda tangan valid untuk hash tertentu
    pub fn verify(&self, pub_key: &VerifyingKey, message_hash: Hash) -> bool {
        pub_key.verify(message_hash.as_bytes(), &self.0).is_ok()
    }

    /// Export signature sebagai 64-byte vector (r || s)
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }

    /// Mencoba membangun `SchnorrSignature` dari byte raw (64 bytes)
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        match Signature::try_from(bytes) {
            Ok(s) => Some(SchnorrSignature(s)),
            Err(_) => None,
        }
    }
}

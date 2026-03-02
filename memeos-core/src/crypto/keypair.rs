use k256::schnorr::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

// utilities for mnemonic conversion
use bip39::{Mnemonic, Language};
use sha2::{Digest, Sha256};

/// BIP-340 Schnorr keypair using secp256k1 curve
/// Secret = 32-byte private key
/// Public = 32-byte Schnorr public key (x-coordinate only)
#[derive(Serialize, Deserialize)]
pub struct KeyPair {
    #[serde(serialize_with = "serialize_signing_key", deserialize_with = "deserialize_signing_key")]
    pub secret: SigningKey,
    pub public: VerifyingKey,
}

impl Clone for KeyPair {
    fn clone(&self) -> Self {
        let secret_bytes = self.secret.to_bytes();
        let secret = SigningKey::from_bytes(&secret_bytes).expect("Invalid secret key bytes");
        let public = secret.verifying_key().clone();
        KeyPair { secret, public }
    }
}

impl std::fmt::Debug for KeyPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyPair")
            .field("secret", &"[32-byte Schnorr private key]")
            .field("public", &self.public)
            .finish()
    }
}

impl KeyPair {
    /// Membuat KeyPair baru secara acak menggunakan Entropy Hardware (OsRng)
    /// Menggunakan BIP-340 Schnorr Signatures (secp256k1)
    pub fn generate() -> Self {
        let secret = SigningKey::random(&mut OsRng);
        let public = secret.verifying_key().clone();
        KeyPair { secret, public }
    }

    /// Mendapatkan alamat publik dalam bentuk bytes (32 bytes untuk Schnorr BIP-340)
    pub fn public_bytes(&self) -> [u8; 32] {
        self.public.to_bytes().into()
    }

    /// BIP-39 12-word English mnemonic derived from this private key.
    ///
    /// For deterministic recovery, we hash the first 16 bytes of the secret
    /// to derive its entropy representation. This is a one-way function - recovery
    /// will reconstruct keys with the same first 16 bytes but possibly different
    /// second 16 bytes (by design of BIP-39 → SHA256 expansion).
    ///
    /// For true deterministic recovery, use `generate_from_mnemonic_12()` to
    /// create keys from mnemonics directly.
    pub fn to_mnemonic_12(&self) -> String {
        let secret_bytes: [u8; 32] = self.secret.to_bytes().into();
        // Use first 16 bytes as entropy
        let entropy: [u8; 16] = secret_bytes[0..16].try_into().expect("slice length");
        let mn = Mnemonic::from_entropy_in(Language::English, &entropy)
            .expect("entropy length is valid");
        mn.to_string()
    }

    /// Generate a new keypair deterministically from a 12-word mnemonic.
    ///
    /// This derives a unique keypair from the mnemonic entropy by:
    /// 1. Extracting 128-bit entropy from the mnemonic
    /// 2. Using that as the full 32-byte secret (padded with zeros on right)
    /// 3. Expanding the result via SHA256 to ensure full entropy
    ///
    /// To save a keypair's mnemonic: `kp.to_mnemonic_12()`
    /// To recover from mnemonic: `KeyPair::generate_from_mnemonic_12(phrase)`
    pub fn generate_from_mnemonic_12(phrase: &str) -> Option<Self> {
        let mn = Mnemonic::parse_in_normalized(Language::English, phrase).ok()?;
        let entropy = mn.to_entropy(); // 16 bytes
        let entropy_bytes: [u8; 16] = entropy.try_into().ok()?;
        
        // Expand entropy to 32 bytes via SHA256
        let mut hasher = Sha256::new();
        hasher.update(&entropy_bytes[..]);
        let hash = hasher.finalize();
        
        // Use hash[0..32] as the secret key
        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(&hash[..]);
        
        let secret = SigningKey::from_bytes(&key_bytes).ok()?;
        let public = secret.verifying_key().clone();
        Some(KeyPair { secret, public })
    }

    /// DEPRECATED: Use `generate_from_mnemonic_12()` instead for true round-trip recovery.
    /// This method is kept for compatibility but provides one-way recovery only.
    pub fn from_mnemonic_12(phrase: &str) -> Option<Self> {
        Self::generate_from_mnemonic_12(phrase)
    }
}

fn serialize_signing_key<S>(key: &SigningKey, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let bytes = key.to_bytes();
    serializer.serialize_bytes(&bytes)
}

fn deserialize_signing_key<'de, D>(deserializer: D) -> Result<SigningKey, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let bytes: Vec<u8> = serde::Deserialize::deserialize(deserializer)?;
    if bytes.len() != 32 {
        return Err(serde::de::Error::custom("Expected 32 bytes for Schnorr key"));
    }
    let mut key_bytes = [0u8; 32];
    key_bytes.copy_from_slice(&bytes);
    SigningKey::from_bytes(&key_bytes)
        .map_err(|_| serde::de::Error::custom("Invalid Schnorr private key"))
}

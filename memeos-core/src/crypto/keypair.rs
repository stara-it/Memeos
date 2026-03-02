use k256::schnorr::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

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

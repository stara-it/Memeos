use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

// Wallets and various other parts of the code often print or debug a KeyPair,
// so the struct needs `Debug` in addition to serialization derives.
#[derive(Serialize, Deserialize, Debug)]
pub struct KeyPair {
    pub secret: SigningKey,
    pub public: VerifyingKey,
}

impl KeyPair {
    /// Membuat KeyPair baru secara acak menggunakan Entropy Hardware (OsRng)
    pub fn generate() -> Self {
        let mut csprng = OsRng;
        let secret = SigningKey::generate(&mut csprng);
        let public = VerifyingKey::from(&secret);
        KeyPair { secret, public }
    }

    /// Mendapatkan alamat publik dalam bentuk bytes
    pub fn public_bytes(&self) -> [u8; 32] {
        *self.public.as_bytes()
    }
}

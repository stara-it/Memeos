use crate::crypto::hash::Hash;
use bech32::{self, Bech32m, Hrp};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// expose the inner string so callers can access it when necessary; we also
// provide convenience accessors for nicer ergonomics.
pub struct Address(pub String);

impl Address {
    /// Mengubah Public Key menjadi Alamat MEMEOS (format: memeos1...)
        pub fn from_public_key(pub_key: &[u8; 32]) -> Self {
                // 1. Hash public key untuk memperpendek/mengamankan (double hashing)
                        let pub_hash = Hash::compute(pub_key);

        // 2. Encode ke Bech32 menggunakan API versi 0.11 (direct slice conversion)
        let hrp = Hrp::parse("memeos").expect("Invalid HRP");
        let encoded = bech32::encode::<Bech32m>(
            hrp,
            pub_hash.as_bytes(),
        )
        .expect("Failed to encode bech32 address");

        Address(encoded)
                                                                                                                }

/// Return the raw inner address string without cloning.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Legacy convenience; clones the string.
    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}
                                                                                                                                
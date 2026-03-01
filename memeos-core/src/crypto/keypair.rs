use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
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
                                                                        
use blake3;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hash([u8; 32]);

impl Hash {
    /// Membuat hash dari slice byte mentah
        pub fn compute(data: &[u8]) -> Self {
                let hash = blake3::hash(data);
                        Hash(*hash.as_bytes())
                            }

                                /// Konversi ke string hex untuk tampilan UI/Log
                                    pub fn to_hex(&self) -> String {
                                            hex::encode(self.0)
                                                }

                                                    pub fn as_bytes(&self) -> &[u8; 32] {
                                                            &self.0
                                                                }
                                                                }

                                                                impl From<[u8; 32]> for Hash {
                                                                    fn from(bytes: [u8; 32]) -> Self {
                                                                            Hash(bytes)
                                                                                }
                                                                                }
                                                                                
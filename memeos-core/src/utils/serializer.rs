use serde::{Serialize, Deserialize};
use bincode::{DefaultOptions, Options};

pub struct MemeosSerializer;

impl MemeosSerializer {
    /// Konfigurasi standar untuk serialisasi biner MEMEOS:
        /// - Little Endian (standar industri)
            /// - Fixint Encoding (lebih stabil untuk ukuran file)
                fn get_config() -> impl Options {
                        DefaultOptions::new()
                                    .with_little_endian()
                                                .with_fixint_encoding()
                                                    }

                                                        /// Mengubah data (Struct/Map) menjadi rangkaian byte (Vec<u8>)
                                                            pub fn serialize<T: Serialize>(data: &T) -> Result<Vec<u8>, String> {
                                                                    Self::get_config()
                                                                                .serialize(data)
                                                                                            .map_err(|e| format!("Serialization error: {}", e))
                                                                                                }

                                                                                                    /// Mengembalikan byte (u8) menjadi data asli (Struct/Map)
                                                                                                        pub fn deserialize<'a, T: Deserialize<'a>>(bytes: &'a [u8]) -> Result<T, String> {
                                                                                                                Self::get_config()
                                                                                                                            .deserialize(bytes)
                                                                                                                                        .map_err(|e| format!("Deserialization error: {}", e))
                                                                                                                                            }
                                                                                                                                            }
                                                                                                                                            
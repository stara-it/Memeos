use crate::core::block::Block;
use crate::crypto::hash::Hash;

pub struct Chain {
    pub blocks: Vec<Block>,
    }

    impl Chain {
        /// Inisialisasi rantai baru
            pub fn new() -> Self {
                    Self {
                                blocks: Vec::new(),
                                        }
                                            }

                                                /// Menambahkan blok ke rantai setelah validasi
                                                    pub fn add_block(&mut self, block: Block) -> Result<(), String> {
                                                            if let Some(last_block) = self.blocks.last() {
                                                                        // Validasi: Previous Hash blok baru harus sama dengan Hash blok terakhir
                                                                                    if block.header.previous_hash != last_block.hash() {
                                                                                                    return Err("Invalid previous block hash".to_string());
                                                                                                                }
                                                                                                                        }
                                                                                                                                
                                                                                                                                        self.blocks.push(block);
                                                                                                                                                Ok(())
                                                                                                                                                    }

                                                                                                                                                        pub fn height(&self) -> usize {
                                                                                                                                                                self.blocks.len()
                                                                                                                                                                    }
                                                                                                                                                                    }
                                                                                                                                                                    
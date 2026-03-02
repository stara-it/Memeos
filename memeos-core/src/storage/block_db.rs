use crate::core::block::Block;
use crate::crypto::hash::Hash;
use std::fs::{File, OpenOptions, create_dir_all};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

pub struct BlockDB {
    pub path: PathBuf,
}

impl BlockDB {
    /// Inisialisasi direktori penyimpanan di smartphone
    pub fn new(data_dir: &str) -> Self {
        let path = PathBuf::from(data_dir);
        if !path.exists() {
            create_dir_all(&path).expect("Failed to create MEMEOS data directory");
        }
        Self { path }
    }

    /// Menyimpan blok secara biner ke disk
    pub fn save_block(&self, block: &Block) -> Result<(), String> {
        let block_hash = block.hash();
        let file_path = self.path.join(format!("{}.bin", block_hash.to_hex()));

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)
            .map_err(|e| e.to_string())?;

        let encoded = bincode::serialize(block).map_err(|e| e.to_string())?;
        file.write_all(&encoded).map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Mengambil blok dari disk berdasarkan hash-nya
    pub fn get_block(&self, block_hash: &Hash) -> Result<Block, String> {
        let file_path = self.path.join(format!("{}.bin", block_hash.to_hex()));

        let mut file = File::open(file_path).map_err(|e| e.to_string())?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;

        let block: Block = bincode::deserialize(&buffer).map_err(|e| e.to_string())?;
        Ok(block)
    }
}

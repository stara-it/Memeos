use crate::ledger::eutxo::EUTXO;
use crate::utils::serializer::MemeosSerializer;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub struct UtxoDB {
    pub data_dir: PathBuf,
}

impl UtxoDB {
    /// Membuat instance database UTXO baru
    pub fn new(data_dir: &str) -> Self {
        // Memastikan folder penyimpanan ada
        let path = PathBuf::from(data_dir);
        if !path.exists() {
            fs::create_dir_all(&path).expect("Gagal membuat direktori UTXO");
        }

        Self { data_dir: path }
    }

    /// Mendapatkan path file database
    fn db_file(&self) -> PathBuf {
        self.data_dir.join("utxo_set.bin")
    }

    /// Menyimpan seluruh state UTXO ke disk secara biner
    pub fn save_utxos(&self, utxos: &HashMap<Vec<u8>, EUTXO>) -> Result<(), String> {
        let serialized = MemeosSerializer::serialize(utxos)?;
        fs::write(self.db_file(), serialized)
            .map_err(|e| format!("Gagal menulis UTXO ke disk: {}", e))
    }

    /// Memuat UTXO dari disk saat Node dinyalakan kembali
    pub fn load_utxos(&self) -> Result<HashMap<Vec<u8>, EUTXO>, String> {
        let file_path = self.db_file();
        if !file_path.exists() {
            return Ok(HashMap::new());
        }

        let bytes = fs::read(file_path).map_err(|e| format!("Gagal membaca file UTXO: {}", e))?;

        MemeosSerializer::deserialize(&bytes)
    }
}

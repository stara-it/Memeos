pub mod wallet;
pub use wallet::MemeosWallet;

use std::fs;
use std::path::Path;

/// Save wallet to disk using bincode
pub fn save_wallet(path: &str, wallet: &MemeosWallet) -> Result<(), String> {
    let data = bincode::serialize(wallet).map_err(|e| e.to_string())?;
    fs::write(path, data).map_err(|e| e.to_string())
}

/// Load wallet from disk using bincode
pub fn load_wallet(path: &str) -> Result<MemeosWallet, String> {
    if !Path::new(path).exists() {
        return Err("Wallet not found".to_string());
    }
    let data = fs::read(path).map_err(|e| e.to_string())?;
    bincode::deserialize(&data).map_err(|e| e.to_string())
}
                                                                                                                                                                                                
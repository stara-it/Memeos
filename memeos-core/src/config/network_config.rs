pub const VERSION: u32 = 1;

/// Satuan terkecil MEMEOS (8 desimal)
pub const COIN: u64 = 100_000_000; 

/// Total Suplai Global: 1 Miliar MEMEOS
pub const TOTAL_SUPPLY: u64 = 1_000_000_000 * COIN; 

/// Alokasi Persentase
pub const FOUNDER_PCT: u64 = 40;
pub const COMMUNITY_PCT: u64 = 50;
pub const DEV_FUND_PCT: u64 = 10;

/// Target waktu antar blok (misal 10 detik untuk smartphone node)
pub const BLOCK_TIME_SECONDS: u64 = 10;

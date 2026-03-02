use crate::ledger::transaction::Transaction;

pub struct TransactionVerifier;

impl TransactionVerifier {
    /// Fungsi utama untuk memvalidasi transaksi
    pub fn verify_transaction(tx: &Transaction) -> bool {
        // 1. Cek apakah transaksi memiliki input dan output (tidak boleh kosong)
        if tx.inputs.is_empty() || tx.outputs.is_empty() {
            return false;
        }

        // 2. Cek apakah jumlah total output masuk akal (tidak negatif)
        // (Logika lebih dalam akan ditambahkan saat sistem saldo aktif)

        // 3. Verifikasi tanda tangan digital dasar
        //    Pastikan setiap input membawa signature 64-byte yang bukan hanya nol.
        let is_signature_valid = Self::verify_signatures(tx);

        is_signature_valid
    }

    fn verify_signatures(tx: &Transaction) -> bool {
        tx.inputs
            .iter()
            .all(|input| input.signature.len() == 64 && input.signature.iter().any(|&b| b != 0))
    }
}

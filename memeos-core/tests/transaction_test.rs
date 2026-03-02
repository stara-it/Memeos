use k256::schnorr::SigningKey;
use k256::schnorr::VerifyingKey;
use rand::rngs::OsRng;
use memeos_core::crypto::hash::Hash;
use memeos_core::ledger::transaction::Transaction;

#[test]
fn transaction_signing_works() {
    let mut rng = OsRng;
    let sk = SigningKey::random(&mut rng);
    let pk_bytes: [u8; 32] = sk.verifying_key().to_bytes().into();

    let tx = Transaction::new_transfer_signed(pk_bytes, pk_bytes, 100, 1, &sk);
    // transaction should contain 1 input signature of length 64
    assert_eq!(tx.inputs.len(), 1);
    assert_eq!(tx.inputs[0].signature.len(), 64);
    // signature verifies
    // The signing routine signs the transaction hash computed before inserting the signature.
    // Recreate the unsigned transaction to derive the hash that was actually signed.
    let unsigned = Transaction::new_transfer(pk_bytes, pk_bytes, 100, 1);
    let tx_hash = unsigned.id();
    let sig = memeos_core::crypto::schnorr::SchnorrSignature::from_bytes(&tx.inputs[0].signature).unwrap();
    assert!(sig.verify(&sk.verifying_key(), tx_hash));
}

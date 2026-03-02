use k256::schnorr::{SigningKey, VerifyingKey};
use memeos_core::crypto::hash::Hash;
use memeos_core::crypto::schnorr::SchnorrSignature;

#[test]
fn schnorr_sign_verify_roundtrip() {
    // generate keys
    let sk = SigningKey::random(&mut rand::rngs::OsRng);
    let pk = sk.verifying_key();

    let message = b"hello schnorr";
    let hash = Hash::compute(message);

    let sig = SchnorrSignature::sign(&sk, hash);
    assert!(sig.verify(&pk, hash));
}


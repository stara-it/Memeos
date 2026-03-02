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

#[test]
fn keypair_mnemonic_roundtrip() {
    // generate mnemonic-based keypair deterministically
    let phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"; // BIP39 test vector
    let kp = memeos_core::crypto::keypair::KeyPair::generate_from_mnemonic_12(phrase)
        .expect("valid mnemonic");
    
    // re-generate from same mnemonic should produce identical key
    let kp2 = memeos_core::crypto::keypair::KeyPair::generate_from_mnemonic_12(phrase)
        .expect("valid mnemonic");
    assert_eq!(kp.public_bytes(), kp2.public_bytes());
    
    // verify phrase extracts 12 words
    assert_eq!(phrase.split_whitespace().count(), 12);
}


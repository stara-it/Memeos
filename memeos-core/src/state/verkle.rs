use crate::crypto::hash::Hash;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum VerkleNode {
    Internal {
        // Verkle Tree biasanya memiliki lebar (arity) yang besar, misal 256
        children: HashMap<u8, Hash>,
        commitment: Hash,
    },
    Leaf {
        key: Hash,
        value: Vec<u8>, // Menyimpan data eUXTO atau saldo
    },
}

impl VerkleNode {
    /// Membuat node internal baru
    pub fn new_internal() -> Self {
        VerkleNode::Internal {
            children: HashMap::new(),
            commitment: Hash::from([0u8; 32]),
        }
    }

    /// Menghitung komitmen (hash) dari node ini
    /// Di Verkle asli, ini menggunakan Vector Commitment (IPA/KZG),
    /// di sini kita implementasikan secara deterministik.
    pub fn compute_commitment(&mut self) {
        if let VerkleNode::Internal {
            children,
            commitment,
        } = self
        {
            let mut data = Vec::new();
            // Urutkan keys agar hash deterministik
            let mut keys: Vec<_> = children.keys().collect();
            keys.sort();

            for key in keys {
                data.push(*key);
                data.extend_from_slice(children[key].as_bytes());
            }
            *commitment = Hash::compute(&data);
        }
    }
}

pub struct VerkleProof {
    pub path: Vec<Hash>,
    pub value: Vec<u8>,
}

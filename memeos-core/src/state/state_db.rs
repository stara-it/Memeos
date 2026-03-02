use crate::crypto::hash::Hash;
use crate::state::verkle::VerkleNode;
use std::collections::HashMap;

pub struct StateDB {
    // Penyimpanan Key-Value untuk semua node di dalam pohon
    pub nodes: HashMap<Hash, VerkleNode>,
    pub root_hash: Hash,
}

impl StateDB {
    pub fn new() -> Self {
        let root_node = VerkleNode::new_internal();
        let initial_root_hash = Hash::from([0u8; 32]);

        let mut nodes = HashMap::new();
        nodes.insert(initial_root_hash, root_node);

        StateDB {
            nodes,
            root_hash: initial_root_hash,
        }
    }

    /// Memasukkan data baru (misal: UTXO baru) ke dalam State
    pub fn update(&mut self, key: Hash, value: Vec<u8>) {
        let leaf = VerkleNode::Leaf { key, value };
        let leaf_hash = Hash::compute(key.as_bytes()); // Sederhananya

        self.nodes.insert(leaf_hash, leaf);

        // Logika update path ke root akan diletakkan di sini
        // Untuk menjaga integritas 1 Miliar suplai global.
        self.root_hash = leaf_hash;
    }

    pub fn get_root(&self) -> Hash {
        self.root_hash
    }
}

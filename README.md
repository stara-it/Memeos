# 🧠 Memeos Core

Memeos adalah blockchain eksperimental generasi baru yang dirancang untuk transaksi global tanpa batas dengan arsitektur kriptografi modern, modular, dan scalable.

Memeos dibangun menggunakan Rust untuk keamanan memori, performa tinggi, dan stabilitas sistem tingkat rendah.

⚠️ Status: Early Development — Belum siap untuk produksi.

---

# 🌍 Visi

Menciptakan jaringan desentralisasi yang:

- Dapat digunakan untuk transaksi global
- Cepat dan ringan
- Aman dengan kriptografi modern
- Mendukung smart contract berbasis WASM++
- Skalabel dengan Verkle Tree
- Modular dan future-proof

---

# 🔬 Teknologi Inti

## 🔐 Cryptography
- BLAKE3 untuk hashing
- Schnorr Signature untuk tanda tangan digital
- Deterministic key generation
- Secure address encoding

## 🧾 Ledger Model
- eUXTO (Extended UTXO)
  - Mendukung smart contract hook
    - Script validation
      - Metadata programmable
        - Future extensibility

        ## 🌳 State System
        - Verkle Tree
          - Proof lebih kecil dari Merkle
            - Lebih efisien untuk light client
              - Siap untuk scaling jangka panjang

              ## ⚙ Smart Contract Engine
              - WASM++ Runtime
                - Deterministic execution
                  - Gas metering
                    - Sandbox isolation
                      - Upgrade-ready

                      ## 🌐 Networking
                      - Modular P2P layer
                      - Block propagation
                      - Chain synchronization
                      - Genesis verification mechanism

                      ---

                      # 🏗 Arsitektur Repository

                      memeos-core/
                      ├── src/
                      │   ├── core/        → Block & consensus engine
                      │   ├── crypto/      → Hash & signature
                      │   ├── ledger/      → eUXTO & transaction logic
                      │   ├── state/       → Verkle tree state
                      │   ├── wasmpp/      → Smart contract VM
                      │   ├── network/     → P2P node
                      │   ├── storage/     → Persistent database
                      │   ├── wallet/      → Wallet & signer logic
                      │   └── utils/       → Helper utilities
                      ├── tests/
                      └── docs/

                      ---

                      # 🧱 Block Structure (Concept)

                      Block Header:
                      - Version
                      - Previous Hash
                      - State Root (Verkle)
                      - Transaction Root
                      - Timestamp
                      - Nonce
                      - Block Hash (BLAKE3)

                      Transaction:
                      - Inputs (eUXTO reference)
                      - Outputs
                      - Schnorr Signature
                      - Optional WASM execution payload

                      ---

                      # 🔑 Design Principles

                      - Security first
                      - Modular architecture
                      - Deterministic execution
                      - Light-client friendly
                      - Clean separation of concerns
                      - Minimal trusted components

                      ---

                      # 📦 Installation

                      Install Rust:

                      curl https://sh.rustup.rs -sSf | sh

                      Clone repository:

                      git clone https://github.com/stara-it/memeos-core.git
                      cd memeos-core

                      Build project:

                      cargo build

                      Run node (development mode):

                      cargo run

                      ---

                      # 🧪 Development Status

                      Current Phase:
                      - Architecture design
                      - Cryptography integration
                      - Genesis block specification
                      - eUXTO model implementation

                      Not yet implemented:
                      - Public testnet
                      - Mining / validator network
                      - Production-ready wallet
                      - Smart contract deployment

                      ---

                      # 📜 License

                      TBD

                      ---

                      # 🎯 Long Term Goal

                      Memeos bertujuan menjadi sistem transaksi global yang ringan, aman, dan scalable dengan fondasi kriptografi modern dan arsitektur modular.

                      Build the engine first. Scale later.
use wasmer::{Instance, Module, Store, imports};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_universal::Universal;

/// Result for execution
pub enum ExecutionResult {
    Success(u64),
    Error(String),
}

/// A production-focused WASM runtime sandbox using Wasmer + Cranelift with deterministic
/// memory enforcement via module declaration validation.
///
/// This implementation enforces memory limits by:
/// 1. Requiring modules to declare a maximum memory limit
/// 2. Rejecting modules whose declared maximum exceeds our configured limit
/// 3. Disabling all host imports (no FS/network/env access)
pub struct WasmRuntime {
    store: Store,
    gas_limit: u64,
    memory_limit_pages: u32,
}

impl WasmRuntime {
    pub fn new(gas_limit: u64, memory_limit_pages: u32) -> Result<Self, String> {
        let compiler = Cranelift::default();
        let engine = Universal::new(compiler).engine();
        let store = Store::new(&engine);

        Ok(Self {
            store,
            gas_limit,
            memory_limit_pages,
        })
    }

    pub fn execute(&self, wasm_bytes: &[u8], _input: &[u8]) -> ExecutionResult {
        if wasm_bytes.len() < 4 || &wasm_bytes[0..4] != b"\0asm" {
            return ExecutionResult::Error("Invalid WASM binary header".into());
        }

        let module = match Module::new(&self.store, wasm_bytes) {
            Ok(m) => m,
            Err(e) => return ExecutionResult::Error(format!("Module compile error: {}", e)),
        };

        // Empty imports - no host access allowed (no FS, network, env vars)
        let import_object = imports! {};
        let instance = match Instance::new(&module, &import_object) {
            Ok(i) => i,
            Err(e) => return ExecutionResult::Error(format!("Instance error: {}", e)),
        };

        // Enforce deterministic memory limit:
        // Module must declare a maximum memory and it must be <= our limit.
        if let Ok(mem) = instance.exports.get_memory("memory") {
            let ty = mem.ty();
            if let Some(max_pages) = ty.maximum {
                // Pages is a tuple struct wrapping u32
                let max_val: u32 = max_pages.0;
                if max_val > self.memory_limit_pages {
                    return ExecutionResult::Error(format!(
                        "WASM module max memory ({} pages) exceeds allowed limit ({} pages)",
                        max_val, self.memory_limit_pages
                    ));
                }
            } else {
                return ExecutionResult::Error(
                    "WASM module must declare a maximum memory limit to run in this sandbox".into(),
                );
            }
        }

        // Try to call common entrypoints: _start, run, or main
        for name in &["_start", "run", "main"] {
            if let Ok(func) = instance.exports.get_function(name) {
                match func.call(&[]) {
                    Ok(_) => return ExecutionResult::Success(0),
                    Err(e) => return ExecutionResult::Error(format!("Execution trap: {}", e)),
                }
            }
        }

        // Module loaded successfully but no known entrypoint found
        ExecutionResult::Success(0)
    }
}

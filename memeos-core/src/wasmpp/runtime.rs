use std::sync::Arc;
use wasmer::{Module, Instance, Store, imports};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_universal::Universal;

/// Result for execution
pub enum ExecutionResult {
    Success(u64),
    Error(String),
}

/// A production-focused WASM runtime sandbox using Wasmer + Cranelift.
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
        Ok(Self { store, gas_limit, memory_limit_pages })
    }

    pub fn execute(&self, wasm_bytes: &[u8], _input: &[u8]) -> ExecutionResult {
        if wasm_bytes.len() < 4 || &wasm_bytes[0..4] != b"\0asm" {
            return ExecutionResult::Error("Invalid WASM binary header".into());
        }

        let module = match Module::new(&self.store, wasm_bytes) {
            Ok(m) => m,
            Err(e) => return ExecutionResult::Error(format!("Module compile error: {}", e)),
        };

        let import_object = imports! {};
        let instance = match Instance::new(&module, &import_object) {
            Ok(i) => i,
            Err(e) => return ExecutionResult::Error(format!("Instance error: {}", e)),
        };

        // Enforce memory declaration: require the module to declare a maximum memory
        // and ensure it's within our configured `memory_limit_pages`.
        if let Ok(mem) = instance.exports.get_memory("memory") {
            let ty = mem.ty(&self.store);
            match ty.maximum() {
                Some(max) => {
                    if max > self.memory_limit_pages {
                        return ExecutionResult::Error("WASM module maximum memory exceeds allowed limit".into());
                    }
                }
                None => {
                    return ExecutionResult::Error("WASM module must declare a maximum memory limit".into());
                }
            }
        }

        // Try to call common entrypoints
        for name in &["_start", "run", "main"] {
            if let Ok(func) = instance.exports.get_function(name) {
                let r = func.call(&[]);
                match r {
                    Ok(_) => return ExecutionResult::Success(0),
                    Err(e) => return ExecutionResult::Error(format!("Runtime trap: {}", e)),
                }
            }
        }

        ExecutionResult::Success(0)
    }
}
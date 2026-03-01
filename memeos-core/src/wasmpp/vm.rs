use serde::{Serialize, Deserialize};
use crate::wasmpp::runtime::{WasmRuntime, ExecutionResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmVM {
        pub gas_limit: u64,
        pub memory_limit_pages: u32,
}

impl WasmVM {
        pub fn new(gas: u64) -> Self {
                Self {
                        gas_limit: gas,
                        memory_limit_pages: 16, // 16 x 64KB = 1MB
                }
        }

        /// Execute a raw wasm module inside the production runtime sandbox.
        pub fn execute_module(&self, wasm_bytes: &[u8], input_data: &[u8]) -> Result<u64, String> {
                let runtime = WasmRuntime::new(self.gas_limit, self.memory_limit_pages)
                        .map_err(|e| format!("Failed to init runtime: {}", e))?;

                match runtime.execute(wasm_bytes, input_data) {
                        ExecutionResult::Success(gas_used) => Ok(gas_used),
                        ExecutionResult::Error(e) => Err(e),
                }
        }
}
                                                                                                                                                                                                
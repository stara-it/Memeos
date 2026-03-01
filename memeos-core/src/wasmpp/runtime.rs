pub struct ExecutionResult {
        pub success: bool,
            pub gas_used: u64,
                pub logs: Vec<String>,
                }

                impl ExecutionResult {
                    pub fn is_rewardable(&self) -> bool {
                            self.success && self.gas_used > 0
                                }
                                }
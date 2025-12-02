//! # 常量值执行器

use crate::context::Context;
use crate::extractor::{ExtractValue, StepExecutor};
use crate::Result;
use serde_json::Value;

/// 常量值执行器
pub struct ConstExecutor {
    value: Value,
}

impl ConstExecutor {
    pub fn new(value: Value) -> Self {
        Self { value }
    }
}

impl StepExecutor for ConstExecutor {
    fn execute(&self, _input: &ExtractValue, _context: &Context) -> Result<ExtractValue> {
        Ok(ExtractValue::from_json(&self.value))
    }
}

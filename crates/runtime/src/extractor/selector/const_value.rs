//! # 常量值执行器

use crate::{
    Result,
    context::{FlowContext, RuntimeContext},
    extractor::value::{ExtractValueData, SharedValue},
};
use serde_json::Value;
use std::sync::Arc;

/// 常量值执行器
pub struct ConstExecutor;

impl ConstExecutor {
    /// 返回常量值
    pub fn execute(
        value: &Value,
        _input: &ExtractValueData,
        _runtime_context: &RuntimeContext,
        _flow_context: &FlowContext,
    ) -> Result<SharedValue> {
        Ok(Arc::new(ExtractValueData::from_json(value)))
    }
}

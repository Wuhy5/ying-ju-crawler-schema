//! # 变量执行器
//!
//! 注意: SetVar 步骤目前仅返回输入值，变量设置逻辑需要在调用方处理
//! 因为 RuntimeContext 和 FlowContext 的 set 方法需要可变引用

use crawler_schema::extract::SetVarStep;
use std::sync::Arc;

use crate::{
    Result,
    context::{FlowContext, RuntimeContext},
    extractor::value::{ExtractValueData, SharedValue},
};

/// 变量执行器
pub struct SetVarExecutor;

impl SetVarExecutor {
    /// 执行设置变量步骤
    ///
    /// 由于上下文只有不可变引用，此方法仅返回包含变量名和值的信息
    /// 实际的变量设置需要在 FlowExecutor 层处理
    pub fn execute(
        _set_var: &SetVarStep,
        input: &ExtractValueData,
        _runtime_context: &RuntimeContext,
        _flow_context: &FlowContext,
    ) -> Result<SharedValue> {
        // TODO: 变量设置逻辑需要在 FlowExecutor 层实现
        // 因为需要可变引用来修改上下文
        // 目前仅透传输入值
        Ok(Arc::new(input.clone()))
    }
}

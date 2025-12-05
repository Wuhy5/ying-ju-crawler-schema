//! # 流程执行器 trait
//!
//! 定义所有流程执行器的通用接口

use crate::{
    Result,
    context::{FlowContext, RuntimeContext},
};
use async_trait::async_trait;

/// 流程执行器 trait
///
/// 所有流程执行器都需要实现此 trait
#[async_trait]
pub trait FlowExecutor: Send + Sync {
    /// 输入类型
    type Input: Send;
    /// 输出类型
    type Output: Send;

    /// 执行流程
    ///
    /// Context 使用 DashMap 实现，内部可变，外部不可变引用
    async fn execute(
        input: Self::Input,
        runtime_context: &RuntimeContext,
        flow_context: &FlowContext,
    ) -> Result<Self::Output>;
}

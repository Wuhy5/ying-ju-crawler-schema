//! # 发现流程执行器

use crate::{
    Result,
    context::{FlowContext, RuntimeContext},
};
use crawler_schema::flow::DiscoveryFlow;

/// 发现请求
#[derive(Debug, Clone)]
pub struct DiscoveryRequest {
    /// 筛选条件
    pub filters: std::collections::HashMap<String, String>,
    /// 页码
    pub page: u32,
}

/// 发现响应
#[derive(Debug, Clone)]
pub struct DiscoveryResponse {
    /// 结果列表
    pub items: Vec<serde_json::Value>,
    /// 是否有下一页
    pub has_next: bool,
}

/// 发现流程执行器
pub struct DiscoveryFlowExecutor;

impl DiscoveryFlowExecutor {
    /// 执行发现流程
    pub async fn execute(
        input: DiscoveryRequest,
        flow: &DiscoveryFlow,
        _runtime_context: &RuntimeContext,
        flow_context: &mut FlowContext,
    ) -> Result<DiscoveryResponse> {
        // 设置上下文变量
        for (key, value) in &input.filters {
            flow_context.set(key, serde_json::json!(value));
        }
        flow_context.set("page", serde_json::json!(input.page));

        // TODO: 实现发现流程
        let _ = flow;

        Ok(DiscoveryResponse {
            items: vec![],
            has_next: false,
        })
    }
}

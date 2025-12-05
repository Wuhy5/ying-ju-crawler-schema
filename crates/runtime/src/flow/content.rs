//! # 内容流程执行器

use crate::{
    Result,
    context::{FlowContext, RuntimeContext},
};
use crawler_schema::flow::ContentFlow;

/// 内容请求
#[derive(Debug, Clone)]
pub struct ContentRequest {
    /// 内容页 URL
    pub url: String,
}

/// 内容响应
#[derive(Debug, Clone)]
pub struct ContentResponse {
    /// 内容数据
    pub data: serde_json::Value,
}

/// 内容流程执行器
pub struct ContentFlowExecutor;

impl ContentFlowExecutor {
    /// 执行内容流程
    pub async fn execute(
        input: ContentRequest,
        flow: &ContentFlow,
        _runtime_context: &RuntimeContext,
        flow_context: &mut FlowContext,
    ) -> Result<ContentResponse> {
        // 设置上下文变量
        flow_context.set("content_url", serde_json::json!(input.url));

        // TODO: 实现内容流程
        let _ = flow;

        Ok(ContentResponse {
            data: serde_json::json!({}),
        })
    }
}

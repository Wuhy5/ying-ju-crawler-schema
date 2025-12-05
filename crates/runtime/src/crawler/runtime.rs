//! # 爬虫运行时
//!
//! 主入口，整合所有模块

use crate::{
    Result,
    context::{FlowContext, RuntimeContext},
    flow::{
        detail::{DetailFlowExecutor, DetailRequest, DetailResponse},
        search::{SearchFlowExecutor, SearchRequest, SearchResponse},
    },
    webview::{SharedWebViewProvider, noop_provider},
};
use crawler_schema::core::CrawlerRule;
use std::sync::Arc;

/// 爬虫运行时
///
/// 整合所有组件，提供统一的爬虫接口
/// Clone 是廉价的，内部使用 Arc 共享资源
#[derive(Clone)]
pub struct CrawlerRuntime {
    /// 运行时上下文（共享资源）
    runtime_context: Arc<RuntimeContext>,
}

impl CrawlerRuntime {
    /// 创建新的运行时实例（不带 WebView 支持）
    ///
    /// 如果规则包含需要 WebView 的配置（如登录、人机验证），
    /// 相关功能将不可用。推荐使用 `builder()` 方法注入 WebView 提供者。
    pub fn new(rule: CrawlerRule, webview_provider: Option<SharedWebViewProvider>) -> Result<Self> {
        let webview_provider = webview_provider.unwrap_or_else(noop_provider);
        // 创建运行时上下文
        let runtime_context = Arc::new(RuntimeContext::with_webview_provider(
            rule,
            webview_provider,
        )?);

        Ok(Self { runtime_context })
    }

    /// 搜索
    pub async fn search(&self, keyword: &str, page: u32) -> Result<SearchResponse> {
        let request = SearchRequest {
            keyword: keyword.to_string(),
            page,
        };
        let flow = &self.runtime_context.rule().search;
        let mut flow_context = FlowContext::new(self.runtime_context.clone());
        SearchFlowExecutor::execute(request, flow, &self.runtime_context, &mut flow_context).await
    }

    /// 获取详情
    pub async fn detail(&self, url: &str) -> Result<DetailResponse> {
        let request = DetailRequest {
            url: url.to_string(),
        };
        let flow = &self.runtime_context.rule().detail;
        let mut flow_context = FlowContext::new(self.runtime_context.clone());
        DetailFlowExecutor::execute(request, flow, &self.runtime_context, &mut flow_context).await
    }

    /// 获取运行时上下文
    pub fn runtime_ctx(&self) -> &Arc<RuntimeContext> {
        &self.runtime_context
    }

    /// 关闭运行时，释放资源
    pub fn shutdown(&self) {
        todo!("实现资源释放逻辑");
    }
}

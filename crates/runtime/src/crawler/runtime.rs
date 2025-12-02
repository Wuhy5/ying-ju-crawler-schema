//! # 爬虫运行时
//!
//! 主入口，整合所有模块

use crate::context::Context;
use crate::error::RuntimeError;
use crate::extractor::ExtractEngine;
use crate::flow::detail::{DetailFlowExecutor, DetailRequest, DetailResponse};
use crate::flow::search::{SearchFlowExecutor, SearchRequest, SearchResponse};
use crate::flow::FlowExecutor;
use crate::http::HttpClient;
use crate::template::TemplateEngine;
use crate::Result;
use crawler_schema::CrawlerRule;
use std::sync::Arc;

/// 爬虫运行时
///
/// 整合所有组件，提供统一的爬虫接口
pub struct CrawlerRuntime {
    /// 爬虫规则
    rule: Arc<CrawlerRule>,
    /// HTTP 客户端
    http_client: Arc<HttpClient>,
    /// 模板引擎
    template_engine: Arc<TemplateEngine>,
    /// 提取引擎
    extract_engine: Arc<ExtractEngine>,
    /// 全局上下文
    context: Context,
}

impl CrawlerRuntime {
    /// 创建新的运行时实例
    pub fn new(rule: CrawlerRule) -> Result<Self> {
        // 创建 HTTP 客户端
        let http_config = rule.http.clone().unwrap_or_default();
        let http_client = Arc::new(HttpClient::new(http_config)?);

        // 创建模板引擎
        let template_engine = Arc::new(TemplateEngine::new()?);

        // 创建提取引擎
        let extract_engine = Arc::new(ExtractEngine::new());

        // 创建全局上下文
        let context = Context::new();

        Ok(Self {
            rule: Arc::new(rule),
            http_client,
            template_engine,
            extract_engine,
            context,
        })
    }

    /// 获取构建器
    pub fn builder() -> crate::crawler::CrawlerRuntimeBuilder {
        crate::crawler::CrawlerRuntimeBuilder::new()
    }

    /// 搜索
    pub async fn search(&self, keyword: &str, page: u32) -> Result<SearchResponse> {
        let executor = SearchFlowExecutor::new(self.rule.search.clone());
        let request = SearchRequest {
            keyword: keyword.to_string(),
            page,
        };

        let mut context = self.context.clone();
        executor.execute(request, &mut context).await
    }

    /// 获取详情
    pub async fn detail(&self, url: &str) -> Result<DetailResponse> {
        let executor = DetailFlowExecutor::new(self.rule.detail.clone());
        let request = DetailRequest {
            url: url.to_string(),
        };

        let mut context = self.context.clone();
        executor.execute(request, &mut context).await
    }

    /// 获取规则元信息
    pub fn meta(&self) -> &crawler_schema::Meta {
        &self.rule.meta
    }

    /// 获取 HTTP 客户端
    pub fn http_client(&self) -> &HttpClient {
        &self.http_client
    }

    /// 获取模板引擎
    pub fn template_engine(&self) -> &TemplateEngine {
        &self.template_engine
    }

    /// 获取提取引擎
    pub fn extract_engine(&self) -> &ExtractEngine {
        &self.extract_engine
    }

    /// 获取全局上下文
    pub fn context(&self) -> &Context {
        &self.context
    }

    /// 获取全局上下文的可变引用
    pub fn context_mut(&mut self) -> &mut Context {
        &mut self.context
    }
}

impl TryFrom<CrawlerRule> for CrawlerRuntime {
    type Error = RuntimeError;

    fn try_from(rule: CrawlerRule) -> Result<Self> {
        Self::new(rule)
    }
}

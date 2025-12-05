//! # 运行时上下文
//!
//! 爬虫实例级的共享资源和全局变量

use crate::{
    http::HttpClient,
    script::{ScriptEngine, ScriptLanguage},
    webview::{SharedWebViewProvider, noop_provider},
};
use crawler_schema::core::CrawlerRule;
use dashmap::DashMap;
use serde_json::{Map, Value};
use std::sync::Arc;

/// 运行时上下文
///
/// 持有爬虫实例级的共享资源，整个爬虫生命周期内有效。
/// 使用 `Arc` 包装以支持跨流程和 Pager 共享。
///
/// # 资源说明
///
/// - `rule`: 爬虫规则定义
/// - `http_client`: HTTP 客户端（连接池复用）
/// - `extract_engine`: 数据提取引擎
/// - `template_engine`: 模板渲染引擎
/// - `globals`: 全局变量（base_url, domain 等）
/// - `webview_provider`: WebView 提供者（可选）
#[derive(Debug)]
pub struct RuntimeContext {
    /// 爬虫规则
    rule: Arc<CrawlerRule>,
    /// HTTP 客户端
    http_client: Arc<HttpClient>,
    /// 全局变量
    globals: Map<String, Value>,
    /// WebView 提供者
    webview_provider: SharedWebViewProvider,
    /// 脚本引擎缓存（按语言类型懒加载）
    script_engines: Arc<DashMap<ScriptLanguage, Arc<dyn ScriptEngine>>>,
}

impl RuntimeContext {
    /// 从爬虫规则创建运行时上下文
    pub fn new(rule: CrawlerRule) -> crate::Result<Self> {
        Self::with_webview_provider(rule, noop_provider())
    }

    /// 创建带 WebView 支持的运行时上下文
    pub fn with_webview_provider(
        rule: CrawlerRule,
        webview_provider: SharedWebViewProvider,
    ) -> crate::Result<Self> {
        // 创建 HTTP 客户端
        let http_config = rule.http.clone().unwrap_or_default();
        let http_client = Arc::new(HttpClient::new(http_config)?);

        // 初始化全局变量
        let mut globals = Map::new();
        globals.insert(
            "base_url".to_string(),
            Value::String(rule.meta.domain.clone()),
        );
        globals.insert(
            "domain".to_string(),
            Value::String(rule.meta.domain.clone()),
        );

        Ok(Self {
            rule: Arc::new(rule),
            http_client,
            globals,
            webview_provider,
            script_engines: Arc::new(DashMap::new()),
        })
    }

    /// 获取爬虫规则
    pub fn rule(&self) -> &CrawlerRule {
        &self.rule
    }

    /// 获取 HTTP 客户端
    pub fn http_client(&self) -> &Arc<HttpClient> {
        &self.http_client
    }

    /// 获取全局变量
    pub fn globals(&self) -> &Map<String, Value> {
        &self.globals
    }

    /// 获取 WebView 提供者
    pub fn webview_provider(&self) -> &SharedWebViewProvider {
        &self.webview_provider
    }

    /// 检查是否支持 WebView
    pub fn has_webview_support(&self) -> bool {
        self.webview_provider.name() != "NoopWebViewProvider"
    }

    /// 获取基础 URL
    pub fn base_url(&self) -> &str {
        &self.rule.meta.domain
    }

    /// 设置全局变量
    pub fn set_global<K: Into<String>>(&mut self, key: K, value: Value) {
        self.globals.insert(key.into(), value);
    }

    /// 获取全局变量
    pub fn get_global(&self, key: &str) -> Option<&Value> {
        self.globals.get(key)
    }
}

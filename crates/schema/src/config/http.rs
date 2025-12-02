//! 全局HTTP配置 (HttpConfig)

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// HTTP 方法
// ============================================================================

/// HTTP 请求方法 (HttpMethod)
/// 用于指定网络请求的 HTTP 方法。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, Copy, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    /// GET 请求，通常用于获取数据。
    #[default]
    Get,
    /// POST 请求，通常用于提交数据。
    Post,
    /// PUT 请求，通常用于更新数据。
    Put,
    /// DELETE 请求，通常用于删除数据。
    Delete,
    /// HEAD 请求，类似于 GET，但只获取响应头。
    Head,
    /// OPTIONS 请求，获取服务器支持的HTTP方法。
    Options,
    /// PATCH 请求，用于部分更新数据。
    Patch,
}

impl HttpMethod {
    /// 是否需要请求体
    pub fn has_body(&self) -> bool {
        matches!(self, Self::Post | Self::Put | Self::Patch)
    }

    /// 获取方法名称
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Delete => "DELETE",
            Self::Head => "HEAD",
            Self::Options => "OPTIONS",
            Self::Patch => "PATCH",
        }
    }
}

// ============================================================================
// HTTP 配置
// ============================================================================

/// 默认User-Agent
pub const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (compatible; YingJuCrawler/1.0)";
/// 默认超时时间（秒）
pub const DEFAULT_TIMEOUT: u32 = 30;
/// 默认是否跟随重定向
pub const DEFAULT_FOLLOW_REDIRECTS: bool = true;
/// 默认最大重定向次数
pub const DEFAULT_MAX_REDIRECTS: u32 = 10;

/// 全局HTTP配置 (HttpConfig)
/// 定义所有网络请求的默认行为。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
#[serde(deny_unknown_fields)]
pub struct HttpConfig {
    /// 全局 User-Agent。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    /// 全局请求超时时间（秒）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
    /// 全局代理地址。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<String>,
    /// 全局请求头。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    /// 是否允许重定向。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_redirects: Option<bool>,
    /// 最大重定向次数。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_redirects: Option<u32>,
    /// 连接超时时间（秒）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_timeout: Option<u32>,
    /// 是否验证SSL证书。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_ssl: Option<bool>,
    /// 请求间隔时间（毫秒），用于限流。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_delay: Option<u32>,
    /// 最大并发请求数。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent: Option<u32>,
    /// 重试次数。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_count: Option<u32>,
    /// 重试间隔（毫秒）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_delay: Option<u32>,
}

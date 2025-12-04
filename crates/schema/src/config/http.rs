//! HTTP 配置模块
//!
//! 定义所有 HTTP 相关的配置结构，包括：
//! - `HttpMethod`: HTTP 请求方法
//! - `RequestConfig`: 请求配置（方法、请求头、请求体）
//! - `ResponseConfig`: 响应配置（编码、内容类型、预处理）
//! - `HttpConfig`: 完整 HTTP 配置（连接参数 + 请求 + 响应）

use crate::{script::Script, template::Template};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// HTTP 方法
// ============================================================================

/// HTTP 请求方法 (HttpMethod)
/// 用于指定网络请求的 HTTP 方法
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, Copy, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    /// GET 请求，通常用于获取数据
    #[default]
    Get,
    /// POST 请求，通常用于提交数据
    Post,
    /// PUT 请求，通常用于更新数据
    Put,
    /// DELETE 请求，通常用于删除数据
    Delete,
    /// HEAD 请求，类似于 GET，但只获取响应头
    Head,
    /// OPTIONS 请求，获取服务器支持的 HTTP 方法
    Options,
    /// PATCH 请求，用于部分更新数据
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
// 请求配置
// ============================================================================

/// 请求配置 (RequestConfig)
///
/// 定义 HTTP 请求的参数，可用于流程级别或全局默认配置
///
/// # 示例
///
/// ```toml
/// [request]
/// method = "POST"
/// content_type = "application/json"
/// body = '{"keyword": "{{ keyword }}"}'
/// headers = { "X-Custom-Header" = "value" }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
#[serde(deny_unknown_fields)]
pub struct RequestConfig {
    /// HTTP 方法，默认为 GET
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<HttpMethod>,

    /// 请求体模板（用于 POST 等请求）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Template>,

    /// 额外的请求头
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, Template>>,

    /// 内容类型（Content-Type），常见值：
    /// - `application/x-www-form-urlencoded`
    /// - `application/json`
    /// - `multipart/form-data`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

// ============================================================================
// 响应配置
// ============================================================================

/// 响应编码
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "lowercase")]
pub enum ResponseEncoding {
    /// 自动检测编码
    #[default]
    Auto,
    /// UTF-8 编码
    #[serde(rename = "utf-8")]
    Utf8,
    /// GBK 编码（简体中文）
    Gbk,
    /// GB2312 编码（简体中文）
    Gb2312,
    /// GB18030 编码（简体中文，GBK 超集）
    Gb18030,
    /// Big5 编码（繁体中文）
    Big5,
    /// Shift_JIS 编码（日文）
    #[serde(rename = "shift_jis")]
    ShiftJis,
    /// EUC-JP 编码（日文）
    #[serde(rename = "euc-jp")]
    EucJp,
    /// EUC-KR 编码（韩文）
    #[serde(rename = "euc-kr")]
    EucKr,
    /// ISO-8859-1 编码（西欧）
    #[serde(rename = "iso-8859-1")]
    Iso8859_1,
    /// Windows-1252 编码（西欧）
    #[serde(rename = "windows-1252")]
    Windows1252,
}

/// 响应内容类型
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ResponseContentType {
    /// HTML 文档
    Html,
    /// JSON 数据
    Json,
    /// XML 数据
    Xml,
    /// 纯文本
    Text,
}

/// 响应处理配置 (ResponseConfig)
///
/// 定义如何处理 HTTP 响应，包括编码检测、内容类型识别和预处理
///
/// # 示例
///
/// ## 指定编码
/// ```toml
/// [response]
/// encoding = "gbk"
/// ```
///
/// ## 自动检测 + 预处理
/// ```toml
/// [response]
/// encoding = "auto"
/// preprocess = { inline = "return decrypt(response.body, 'key');" }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
#[serde(deny_unknown_fields)]
pub struct ResponseConfig {
    /// 响应编码
    ///
    /// - `auto`: 自动检测（默认）
    /// - `utf-8`, `gbk`, `gb2312`, `big5`, `shift_jis` 等
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<ResponseEncoding>,

    /// 响应内容类型（覆盖自动检测）
    ///
    /// 某些网站 Content-Type 不准确，需手动指定
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<ResponseContentType>,

    /// 预处理脚本
    ///
    /// 在解析前对响应体进行处理（解密、解压等）
    /// 输入变量：`response`（包含 body, headers, status）
    /// 返回值：处理后的响应体字符串
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preprocess: Option<Script>,
}

// ============================================================================
// HTTP 配置（完整）
// ============================================================================

/// 默认 User-Agent
pub const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (compatible; YingJuCrawler/1.0)";
/// 默认超时时间（秒）
pub const DEFAULT_TIMEOUT: u32 = 30;
/// 默认是否跟随重定向
pub const DEFAULT_FOLLOW_REDIRECTS: bool = true;
/// 默认最大重定向次数
pub const DEFAULT_MAX_REDIRECTS: u32 = 10;

/// HTTP 配置 (HttpConfig)
///
/// 完整的 HTTP 配置结构，包含连接参数、请求配置和响应配置。
/// 可用于全局配置或流程级配置，流程级配置会覆盖全局配置。
///
/// # 示例
///
/// ## 全局配置
/// ```toml
/// [http]
/// user_agent = "Mozilla/5.0 ..."
/// timeout = 30
///
/// [http.request]
/// headers = { "Accept-Language" = "zh-CN,zh;q=0.9" }
///
/// [http.response]
/// encoding = "utf-8"
/// ```
///
/// ## 流程级配置
/// ```toml
/// [search.http]
/// timeout = 10
/// request.method = "POST"
/// response.encoding = "gbk"
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
#[serde(deny_unknown_fields)]
pub struct HttpConfig {
    // ========== 连接参数 ==========
    /// User-Agent 请求头
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,

    /// 请求超时时间（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,

    /// 连接超时时间（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_timeout: Option<u32>,

    /// 代理地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<String>,

    /// 是否验证 SSL 证书
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_ssl: Option<bool>,

    /// 是否允许重定向
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_redirects: Option<bool>,

    /// 最大重定向次数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_redirects: Option<u32>,

    // ========== 限流与重试 ==========
    /// 请求间隔时间（毫秒），用于限流
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_delay: Option<u32>,

    /// 最大并发请求数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent: Option<u32>,

    /// 重试次数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_count: Option<u32>,

    /// 重试间隔（毫秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_delay: Option<u32>,

    // ========== 请求配置 ==========
    /// 默认请求配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<RequestConfig>,

    // ========== 响应配置 ==========
    /// 默认响应配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<ResponseConfig>,
}

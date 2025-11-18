//! 全局HTTP配置 (HttpConfig)

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 全局HTTP配置 (HttpConfig)
/// 定义所有网络请求的默认行为。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
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
}

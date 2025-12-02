//! 登录流程 (LoginFlow)
//!
//! 支持多种登录方式的模式化设计

use crate::template::Template;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 登录流程 (LoginFlow)
/// 提供三种常见登录模式
///
/// # 示例
///
/// ```yaml
/// # WebView 登录
/// login:
///   webview:
///     url: "https://example.com/login"
///
/// # 表单登录
/// login:
///   form:
///     url: "https://example.com/api/login"
///     method: POST
///     fields:
///       username: "{{ username }}"
///       password: "{{ password }}"
///
/// # 凭证登录
/// login:
///   credential:
///     type: cookie
///     fields:
///       - key: session_id
///         label: "会话ID"
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct LoginFlow {
    /// 流程的功能描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 登录模式（三选一）
    #[serde(flatten)]
    pub mode: LoginMode,
}

/// 登录模式
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum LoginMode {
    /// WebView 登录
    /// App 打开内嵌浏览器，用户手动登录
    Webview(WebViewLogin),

    /// 表单登录
    /// 通过 HTTP 表单提交用户凭证
    Form(FormLogin),

    /// 凭证登录
    /// 用户手动提供认证信息（Cookie/Token等）
    Credential(CredentialLogin),
}

/// WebView 登录配置
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WebViewLogin {
    /// 登录页 URL
    pub url: Template,

    /// Cookie 域名（可选，默认为 URL 的根域名）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_domain: Option<String>,

    /// 登录成功检测选择器（可选）
    /// 如果提供，检测该元素存在则认为登录成功
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_selector: Option<String>,
}

/// 表单登录配置
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FormLogin {
    /// 登录接口 URL
    pub url: Template,

    /// HTTP 方法（默认 POST）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<crate::config::HttpMethod>,

    /// 用户输入字段定义
    pub fields: Vec<LoginField>,

    /// 表单字段映射
    /// key: 接口参数名, value: 用户输入字段key
    pub field_mapping: HashMap<String, Template>,

    /// 登录成功检测
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_check: Option<SuccessCheck>,
}

/// 凭证登录配置
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CredentialLogin {
    /// 凭证类型
    pub credential_type: CredentialType,

    /// 用户需要提供的字段
    pub fields: Vec<LoginField>,
}

/// 凭证类型
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CredentialType {
    /// Cookie
    Cookie,
    /// Token (如 JWT)
    Token,
    /// API Key
    ApiKey,
    /// 自定义
    Custom,
}

/// 登录成功检测
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum SuccessCheck {
    /// JSONPath 检测
    JsonPath {
        /// JSONPath 表达式
        path: String,
        /// 期望值
        expect: serde_json::Value,
    },

    /// 状态码检测
    StatusCode {
        /// 期望的状态码
        expect: u16,
    },

    /// 响应包含检测
    Contains {
        /// 期望包含的字符串
        text: String,
    },
}

/// 登录字段定义
/// 用于 App 生成用户输入表单
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct LoginField {
    /// 字段标识符（变量名）
    pub key: String,
    /// 字段显示名称
    pub label: String,
    /// 字段类型
    #[serde(default)]
    pub field_type: LoginFieldType,
    /// 是否必填
    #[serde(default = "default_true")]
    pub required: bool,
    /// 占位符文本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
}

fn default_true() -> bool {
    true
}

/// 登录字段类型
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LoginFieldType {
    /// 普通文本
    #[default]
    Text,
    /// 密码（隐藏显示）
    Password,
    /// 多行文本（如 Cookie 字符串、Token）
    Textarea,
}

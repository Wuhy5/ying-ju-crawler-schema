//! 共享配置类型
//!
//! 用于多个流程共用的配置结构

use crate::{config::HttpMethod, template::Template};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// 筛选器
// ============================================================================

/// 筛选器组 (FilterGroup)
/// 代表UI上一组相关的筛选选项，如"地区"、"年份"。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FilterGroup {
    /// 筛选器组的显示名称，如 "按类型"。
    pub name: String,
    /// 此筛选器组在URL模板中对应的键 (`key`)。
    pub key: String,
    /// 是否允许多选。
    #[serde(default)]
    pub multiselect: bool,
    /// 此筛选器组下所有可用的选项。
    pub options: Vec<FilterOption>,
}

/// 筛选器选项 (FilterOption)
/// 代表一个具体的筛选选项，如"电影"或"2023年"。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FilterOption {
    /// 选项的显示名称，如 "美国"。
    pub name: String,
    /// 选项的值，将用于替换URL模板中对应的 `key`。
    pub value: String,
}

/// HTTP 请求配置 (RequestConfig)
/// 可选的 HTTP 请求参数配置，用于覆盖默认设置
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
    /// - application/x-www-form-urlencoded
    /// - application/json
    /// - multipart/form-data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

/// 分页配置
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PaginationConfig {
    /// 分页类型
    #[serde(default)]
    pub pagination_type: PaginationType,

    /// 起始页码
    #[serde(default = "default_start_page")]
    pub start_page: u32,

    /// 每页数量（如果适用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,

    /// 最大页数限制
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_pages: Option<u32>,

    /// 页码参数名
    #[serde(default = "default_page_param")]
    pub page_param: String,
}

fn default_start_page() -> u32 {
    1
}

fn default_page_param() -> String {
    "page".to_string()
}

/// 分页类型
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PaginationType {
    /// 页码分页
    #[default]
    PageNumber,
    /// 偏移量分页
    Offset,
    /// 游标分页
    Cursor,
}

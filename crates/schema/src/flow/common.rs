//! 共享配置类型
//!
//! 用于多个流程共用的配置结构

use crate::extract::FieldExtractor;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ============================================================================
// 筛选器
// ============================================================================

/// 筛选器组 (FilterGroup)
/// 代表UI上一组相关的筛选选项，如"地区"、"年份"
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FilterGroup {
    /// 筛选器组的显示名称，如 "按类型"
    pub name: String,
    /// 此筛选器组在URL模板中对应的键 (`key`)
    pub key: String,
    /// 是否允许多选
    #[serde(default)]
    pub multiselect: bool,
    /// 此筛选器组下所有可用的选项
    pub options: Vec<FilterOption>,
}

/// 筛选器选项 (FilterOption)
/// 代表一个具体的筛选选项，如"电影"或"2023年"
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FilterOption {
    /// 选项的显示名称，如 "美国"
    pub name: String,
    /// 选项的值，将用于替换URL模板中对应的 `key`
    pub value: String,
}

// ============================================================================
// 分页配置（枚举类型）
// ============================================================================

/// 分页配置 (Pagination)
///
/// 不同分页类型有不同的结构，通过 `type` 字段区分
///
/// # 示例
///
/// ## 页码分页
/// ```toml
/// [pagination]
/// type = "page_number"
/// start = 1
/// param = "page"
/// ```
///
/// ## 偏移量分页
/// ```toml
/// [pagination]
/// type = "offset"
/// start = 0
/// step = 20
/// param = "offset"
/// ```
///
/// ## 游标分页
/// ```toml
/// [pagination]
/// type = "cursor"
/// param = "cursor"
/// next_cursor.steps = [{ json = "$.data.next_cursor" }]
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Pagination {
    /// 页码分页（最常见）
    ///
    /// URL 示例：`?page=1`, `?page=2`
    PageNumber(PageNumberPagination),

    /// 偏移量分页
    ///
    /// URL 示例：`?offset=0&limit=20`, `?offset=20&limit=20`
    Offset(OffsetPagination),

    /// 游标分页
    ///
    /// URL 示例：`?cursor=abc123`, `?after=xyz789`
    Cursor(CursorPagination),

    /// 无分页（单页）
    None,
}

impl Default for Pagination {
    fn default() -> Self {
        Self::PageNumber(PageNumberPagination::default())
    }
}

/// 页码分页配置
///
/// 适用于传统的页码分页方式
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PageNumberPagination {
    /// 起始页码（默认 1）
    #[serde(default = "default_start_page")]
    pub start: u32,

    /// 页码参数名（默认 "page"）
    #[serde(default = "default_page_param")]
    pub param: String,

    /// 最大页数限制（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_pages: Option<u32>,

    /// 是否有下一页的检测规则（可选）
    ///
    /// 如果不提供，默认当返回结果为空时停止
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_next: Option<FieldExtractor>,
}

impl Default for PageNumberPagination {
    fn default() -> Self {
        Self {
            start: 1,
            param: "page".to_string(),
            max_pages: None,
            has_next: None,
        }
    }
}

/// 偏移量分页配置
///
/// 适用于 API 风格的分页
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct OffsetPagination {
    /// 起始偏移量（默认 0）
    #[serde(default)]
    pub start: u32,

    /// 每次增加的偏移量（即每页数量）
    pub step: u32,

    /// 偏移量参数名（默认 "offset"）
    #[serde(default = "default_offset_param")]
    pub param: String,

    /// 每页数量参数名（可选）
    ///
    /// 如 "limit", "size", "per_page"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_param: Option<String>,

    /// 最大偏移量限制（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_offset: Option<u32>,

    /// 总数量提取规则（可选）
    ///
    /// 用于计算总页数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<FieldExtractor>,
}

/// 游标分页配置
///
/// 适用于基于游标/令牌的分页（如 Twitter, GraphQL）
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CursorPagination {
    /// 游标参数名（默认 "cursor"）
    #[serde(default = "default_cursor_param")]
    pub param: String,

    /// 下一页游标提取规则（必需）
    ///
    /// 从响应中提取下一页的游标值
    pub next_cursor: FieldExtractor,

    /// 是否有下一页的检测规则（可选）
    ///
    /// 如果不提供，默认当 next_cursor 为空时停止
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_next: Option<FieldExtractor>,

    /// 最大请求次数限制（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_requests: Option<u32>,
}

// ============================================================================
// 默认值函数
// ============================================================================

fn default_start_page() -> u32 {
    1
}

fn default_page_param() -> String {
    "page".to_string()
}

fn default_offset_param() -> String {
    "offset".to_string()
}

fn default_cursor_param() -> String {
    "cursor".to_string()
}

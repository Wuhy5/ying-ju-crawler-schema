//! 发现页流程 (DiscoveryFlow)

use crate::{config::HttpConfig, extract::FieldExtractor, fields::ItemFields, template::Template};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::common::{FilterGroup, Pagination};

/// 发现页流程 (DiscoveryFlow)
///
/// 用于列表页、分类页、发现页等场景，支持筛选和分页。
///
/// # 可用变量
///
/// ## Flow 变量（自动注入）
///
/// | 变量 | 类型 | 说明 |
/// |------|------|------|
/// | `page` | u32 | 当前页码 |
/// | `{filter_key}` | String | 各筛选器的 key 对应的选中值 |
///
/// 筛选器变量名由 `filters[].key` 定义，例如：
/// - 定义 `key = "category"` → 可用 `{{ category }}`
/// - 定义 `key = "year"` → 可用 `{{ year }}`
///
/// ## Runtime 全局变量（通过 `$` 前缀访问）
///
/// | 变量 | 说明 |
/// |------|------|
/// | `$.base_url` | 目标网站基础 URL |
/// | `$.domain` | 目标网站域名 |
///
/// # 示例
///
/// ```toml
/// [discovery]
/// url = "{{ $.base_url }}/list?category={{ category }}&year={{ year }}&page={{ page }}"
///
/// [discovery.pagination]
/// type = "page_number"
/// start = 1
///
/// [[discovery.filters]]
/// name = "分类"
/// key = "category"
/// options = [
///     { name = "全部", value = "" },
///     { name = "电影", value = "movie" },
/// ]
///
/// [[discovery.filters]]
/// name = "年份"
/// key = "year"
/// options = [
///     { name = "全部", value = "" },
///     { name = "2024", value = "2024" },
/// ]
///
/// [discovery.fields.title]
/// steps = [{ css = ".title" }, { filter = "trim" }]
///
/// [discovery.fields.url]
/// steps = [{ css = "a" }, { attr = "href" }]
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DiscoveryFlow {
    /// 流程的功能描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 数据源 URL 模板
    ///
    /// 可用变量：`page`（页码）、筛选器 `key` 值、`$.base_url`（全局基础URL）
    pub url: Template,

    /// 流程级 HTTP 配置（可选）
    ///
    /// 覆盖全局 HTTP 配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<HttpConfig>,

    /// 分页配置（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,

    /// 分类列表（可选）
    /// 静态数组或动态获取配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<OptionList>,

    /// 筛选器组列表（可选）
    /// 静态数组或动态获取配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,

    /// list 列表提取规则
    pub list: FieldExtractor,

    /// 将列表项映射为最终数据结构的字段提取规则
    pub fields: ItemFields,
}

// ============================================================================
// 选项列表（分类/筛选选项通用）
// ============================================================================

/// 选项列表
///
/// 支持静态定义或从数据源动态获取
///
/// # 示例
///
/// ## 静态定义
/// ```toml
/// categories = [
///   { key = "movie", label = "电影" },
///   { key = "tv", label = "电视剧" },
/// ]
/// ```
///
/// ## 动态获取（HTML）
/// ```toml
/// [categories]
/// url = "https://example.com/categories"
/// list.steps = [{ css = ".category-item" }]
///
/// [categories.fields]
/// key.steps = [{ attr = "data-id" }]
/// label.steps = [{ css = ".name" }]
/// ```
///
/// ## 动态获取（JSON API）
/// ```toml
/// [categories]
/// url = "https://api.example.com/categories"
/// list.steps = [{ json = "$.data.categories[*]" }]
///
/// [categories.fields]
/// key.steps = [{ json = "$.id" }]
/// label.steps = [{ json = "$.name" }]
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum OptionList {
    /// 静态选项列表
    Static(Vec<OptionItem>),

    /// 动态获取配置
    Dynamic(Box<DynamicOptionList>),
}

/// 选项项（静态定义）
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct OptionItem {
    /// 选项标识（用于 URL 参数）
    pub key: String,
    /// 显示名称
    pub label: String,
    /// 请求值（可选，默认使用 key）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 动态选项列表配置
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DynamicOptionList {
    /// 数据源 URL
    pub url: Template,

    /// 列表提取规则
    /// 提取出选项列表的数组
    pub list: FieldExtractor,

    /// 选项字段提取规则
    pub fields: OptionFields,
}

/// 选项字段定义（用于动态提取）
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct OptionFields {
    /// 选项标识提取规则
    pub key: FieldExtractor,
    /// 显示名称提取规则
    pub label: FieldExtractor,
    /// 请求值提取规则（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<FieldExtractor>,
}

// ============================================================================
// 筛选器组列表
// ============================================================================

/// 筛选器组列表
///
/// 支持静态定义或从数据源动态获取
///
/// # 示例
///
/// ## 静态定义
/// ```toml
/// [[filters]]
/// key = "year"
/// name = "年份"
/// options = [
///   { key = "2024", name = "2024年" },
///   { key = "2023", name = "2023年" },
/// ]
///
/// [[filters]]
/// key = "area"
/// name = "地区"
/// options = [
///   { key = "cn", name = "中国" },
///   { key = "us", name = "美国" },
/// ]
/// ```
///
/// ## 动态获取
/// ```toml
/// [filters]
/// url = "https://example.com/filters"
/// list.steps = [{ css = ".filter-group" }]
///
/// [filters.fields]
/// key.steps = [{ attr = "data-key" }]
/// name.steps = [{ css = ".group-name" }]
///
/// [filters.fields.options]
/// list.steps = [{ css = ".filter-option" }]
///
/// [filters.fields.options.fields]
/// key.steps = [{ attr = "data-value" }]
/// name.steps = [{ css = "text()" }]
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum FilterList {
    /// 静态筛选器组列表
    Static(Vec<FilterGroup>),

    /// 动态获取配置
    Dynamic(Box<DynamicFilterList>),
}

/// 动态筛选器组列表配置
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DynamicFilterList {
    /// 数据源 URL
    pub url: Template,

    /// 筛选组列表提取规则
    pub list: FieldExtractor,

    /// 筛选组字段提取规则
    pub fields: FilterGroupFields,
}

/// 筛选组字段定义（用于动态提取）
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FilterGroupFields {
    /// 筛选组标识提取规则
    pub key: FieldExtractor,
    /// 筛选组名称提取规则
    pub name: FieldExtractor,
    /// 是否多选提取规则（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiselect: Option<FieldExtractor>,
    /// 选项列表配置
    pub options: NestedOptionList,
}

/// 嵌套选项列表（用于筛选组内的选项）
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct NestedOptionList {
    /// 选项列表提取规则
    pub list: FieldExtractor,
    /// 选项字段提取规则
    pub fields: FilterOptionFields,
}

/// 筛选选项字段定义
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FilterOptionFields {
    /// 选项标识/值提取规则
    pub key: FieldExtractor,
    /// 选项名称提取规则
    pub name: FieldExtractor,
}

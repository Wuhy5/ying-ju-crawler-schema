//! 发现页流程 (DiscoveryFlow)

use crate::{fields::ItemFields, template::Template};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::common::{FilterGroup, PaginationConfig};

/// 发现页流程 (DiscoveryFlow)
/// 用于列表页、分类页、发现页等场景
///
/// # 示例
///
/// ```toml
/// [discovery]
/// url = "https://example.com/category/{{ category }}?page={{ page }}"
///
/// [discovery.pagination]
/// pagination_type = "page_number"
/// start_page = 1
///
/// [discovery.fields]
/// title.steps = [{ css = ".title" }, { filter = "trim" }]
/// url.steps = [{ css = "a" }, { attr = "href" }, { filter = "absolute_url" }]
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DiscoveryFlow {
    /// 流程的功能描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 数据源 URL 模板
    /// 支持变量: {{ category }}, {{ page }}, 自定义筛选器变量
    pub url: Template,

    /// 分页配置（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationConfig>,

    /// 分类来源（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<CategorySource>,

    /// 动态筛选器（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FiltersSource>,

    /// 列表项字段定义
    pub fields: ItemFields,
}

/// 分类来源定义
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CategorySource {
    /// 静态分类（手动配置）
    Static { items: Vec<CategoryItem> },
    
    /// 动态分类（从页面提取）
    Dynamic {
        /// 分类数据源 URL
        url: Template,
        /// 分类列表选择器
        selector: String,
        /// 分类项字段定义
        fields: CategoryFields,
    },
}

/// 分类项
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CategoryItem {
    /// 分类唯一标识（用于请求参数）
    pub key: String,
    /// 展示名称
    pub label: String,
    /// 可选：请求使用的值（不提供则默认用 key）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 分类字段定义
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CategoryFields {
    /// 分类标识提取规则
    pub key: String,
    /// 分类名称提取规则
    pub label: String,
    /// 分类值提取规则（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 筛选器来源定义
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FiltersSource {
    /// 静态筛选组
    Static { groups: Vec<FilterGroup> },
    
    /// 动态筛选组（从页面提取）
    Dynamic {
        /// 筛选器数据源 URL
        url: Template,
        /// 筛选组列表选择器
        selector: String,
        /// 筛选组字段定义
        fields: FilterGroupFields,
    },
}

/// 筛选组字段定义
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FilterGroupFields {
    /// 筛选组名称提取规则
    pub name: String,
    /// 筛选组key提取规则
    pub key: String,
    /// 选项列表选择器
    pub options_selector: String,
    /// 选项字段定义
    pub option_fields: FilterOptionFields,
}

/// 筛选选项字段定义
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FilterOptionFields {
    /// 选项名称提取规则
    pub name: String,
    /// 选项值提取规则
    pub value: String,
}

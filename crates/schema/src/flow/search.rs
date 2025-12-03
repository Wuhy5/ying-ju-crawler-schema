//! 搜索流程 (SearchFlow)

use crate::{FieldExtractor, fields::ItemFields, template::Template};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::common::PaginationConfig;

/// 搜索流程 (SearchFlow)
/// 实现搜索功能
///
/// # 示例
///
/// ```toml
/// [search]
/// url = "https://example.com/search?q={{ keyword }}&page={{ page }}"
///
/// [search.pagination]
/// pagination_type = "page_number"
///
/// [search.fields]
/// title.steps = [{ css = ".title" }]
/// url.steps = [{ css = "a" }, { attr = "href" }]
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SearchFlow {
    /// 流程的功能描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 搜索 URL 模板
    /// 约定输入变量: {{ keyword }}, {{ page }}
    pub url: Template,

    /// 分页配置（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationConfig>,

    /// list 列表提取规则
    pub list: FieldExtractor,

    /// 将列表项映射为最终数据结构的字段提取规则
    pub fields: ItemFields,
}

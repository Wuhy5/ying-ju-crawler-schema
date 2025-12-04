//! 搜索流程 (SearchFlow)

use crate::{config::HttpConfig, extract::FieldExtractor, fields::ItemFields, template::Template};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::common::Pagination;

/// 搜索流程 (SearchFlow)
/// 实现搜索功能
///
/// # 示例
///
/// ## 基本 GET 搜索
/// ```toml
/// [search]
/// url = "https://example.com/search?q={{ keyword }}&page={{ page }}"
///
/// [search.pagination]
/// type = "page_number"
///
/// [search.fields]
/// title.steps = [{ css = ".title" }]
/// url.steps = [{ css = "a" }, { attr = "href" }]
/// ```
///
/// ## POST 搜索（使用流程级 HTTP 配置）
/// ```toml
/// [search]
/// url = "https://api.example.com/search"
///
/// [search.http.request]
/// method = "POST"
/// content_type = "application/json"
/// body = '{"keyword": "{{ keyword }}", "page": {{ page }}}'
///
/// [search.http.response]
/// encoding = "utf-8"
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

    /// 流程级 HTTP 配置（可选）
    ///
    /// 覆盖全局 HTTP 配置，支持设置请求方法、请求头、响应编码等
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<HttpConfig>,

    /// 分页配置（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,

    /// list 列表提取规则
    pub list: FieldExtractor,

    /// 将列表项映射为最终数据结构的字段提取规则
    pub fields: ItemFields,
}

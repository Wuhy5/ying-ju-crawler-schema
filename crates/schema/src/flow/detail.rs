//! 详情页流程 (DetailFlow)

use crate::{fields::DetailFields, template::Template};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// 详情页流程 (DetailFlow)
/// 处理单个内容项的详细信息
///
/// # 示例
///
/// ```toml
/// [detail]
/// url = "{{ detail_url }}"
///
/// [detail.fields]
/// media_type = "video"
/// title.steps = [{ css = ".title" }, { filter = "trim" }]
/// cover.steps = [{ css = ".poster img" }, { attr = "src" }, { filter = "absolute_url" }]
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DetailFlow {
    /// 流程的功能描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 详情页 URL 模板
    /// 约定输入变量: {{ detail_url }}
    pub url: Template,

    /// 字段提取规则
    /// 根据媒体类型定义不同的字段集合
    pub fields: DetailFields,
}

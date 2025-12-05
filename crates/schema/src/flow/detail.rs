//! 详情页流程 (DetailFlow)

use crate::{config::HttpConfig, fields::DetailFields, template::Template};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// 详情页流程 (DetailFlow)
///
/// 处理单个内容项的详细信息页面。
///
/// # 可用变量
///
/// ## Flow 变量（自动注入）
///
/// | 变量 | 类型 | 说明 |
/// |------|------|------|
/// | `url` | String | 详情页 URL（从搜索/发现结果传入） |
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
/// [detail]
/// # 直接使用传入的 URL
/// url = "{{ url }}"
///
/// # 或拼接基础 URL
/// # url = "{{ $.base_url }}{{ url }}"
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
    ///
    /// 可用变量：`url`（详情页URL）、`$.base_url`（全局基础URL）
    pub url: Template,

    /// 流程级 HTTP 配置（可选）
    ///
    /// 覆盖全局 HTTP 配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<HttpConfig>,

    /// 字段提取规则
    /// 根据媒体类型定义不同的字段集合
    pub fields: DetailFields,
}

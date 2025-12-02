//! 列表项字段规则
//!
//! 定义列表页/搜索结果中每个项目的字段

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::common::{FieldRule, OptionalFieldRule};

/// 列表项字段规则 (ItemFields)
/// 定义列表页中每个项目需要提取的字段
/// 这些字段将构成 ItemSummary
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ItemFields {
    /// 标题（必需）
    pub title: FieldRule,

    /// 详情页 URL（必需）
    pub url: FieldRule,

    /// 封面图 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: OptionalFieldRule,

    /// 简介/摘要
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: OptionalFieldRule,

    /// 作者/创作者（书籍、漫画、音频常用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: OptionalFieldRule,

    /// 最新章节/更新信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest: OptionalFieldRule,

    /// 评分
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: OptionalFieldRule,

    /// 状态（连载中/已完结等）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: OptionalFieldRule,

    /// 分类/标签（返回字符串，多个标签用分隔符分开）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: OptionalFieldRule,

    /// 扩展字段（用于媒体类型特定的额外信息）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: OptionalFieldRule,
}

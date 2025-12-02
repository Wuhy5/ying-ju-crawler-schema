//! 列表类型的提取规则
//!
//! 用于章节列表、播放线路、音轨列表等嵌套列表结构

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::common::{FieldRule, OptionalFieldRule};

/// 章节列表提取规则 (ChapterListRule)
/// 定义如何提取章节列表
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ChapterListRule {
    /// 章节列表容器的提取流程
    /// 应返回一个包含多个章节元素的数组
    pub list: FieldRule,

    /// 章节标题的提取规则（相对于单个章节元素）
    pub title: FieldRule,

    /// 章节 URL 的提取规则（相对于单个章节元素）
    pub url: FieldRule,
}

/// 播放线路列表提取规则 (PlayLineListRule)
/// 定义如何提取播放线路和剧集
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PlayLineListRule {
    /// 线路列表容器的提取流程
    pub lines: FieldRule,

    /// 线路名称的提取规则（相对于单个线路元素）
    pub line_name: FieldRule,

    /// 剧集列表的提取规则（相对于单个线路元素）
    pub episodes: EpisodeListRule,
}

/// 剧集列表提取规则 (EpisodeListRule)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct EpisodeListRule {
    /// 剧集列表的提取流程
    pub list: FieldRule,

    /// 剧集名称的提取规则
    pub name: FieldRule,

    /// 剧集播放页 URL 的提取规则
    pub url: FieldRule,
}

/// 音轨列表提取规则 (TrackListRule)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TrackListRule {
    /// 音轨列表的提取流程
    pub list: FieldRule,

    /// 音轨名称的提取规则
    pub name: FieldRule,

    /// 音轨 URL 的提取规则
    pub url: FieldRule,

    /// 时长的提取规则（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: OptionalFieldRule,
}

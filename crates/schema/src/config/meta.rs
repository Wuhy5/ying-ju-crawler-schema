//! 元数据 (Meta)

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ============================================================================
// 媒体类型
// ============================================================================

/// 用于指定规则适用的媒体内容类型。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, Copy, Default)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    /// 视频类型，如电影、电视剧等。
    #[default]
    Video,
    /// 音频类型，如音乐、播客等。
    Audio,
    /// 书籍类型，如电子书、小说等。
    Book,
    /// 漫画类型，如漫画、图画书等。
    Manga,
}

impl MediaType {
    /// 获取显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Video => "视频",
            Self::Audio => "音频",
            Self::Book => "书籍",
            Self::Manga => "漫画",
        }
    }
}

// ============================================================================
// 元数据
// ============================================================================

/// 元数据 (Meta)
/// 描述规则的基本信息。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Meta {
    /// 规则名称，如“示例影视站”。
    pub name: String,
    /// 规则作者。
    pub author: String,
    /// 规则版本号，建议遵循 SemVer (如 "1.0.2")。
    pub version: String,
    /// 本规则遵循的规范版本号。
    pub spec_version: String,
    /// 目标网站的主域名。
    pub domain: String,
    /// 规则用于的媒体类型。
    pub media_type: MediaType,
    /// 规则的详细描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 目标网站的编码，默认为 "UTF-8"。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    /// 数据源的图标URL，用于UI展示。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
}

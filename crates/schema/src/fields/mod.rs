//! 字段驱动的规则定义 (Field-Driven Rule Definitions)
//!
//! 本模块定义了"字段即规则"的数据结构，让每个输出字段关联一个提取流程。
//!
//! ## 设计理念
//!
//! **字段驱动**：直接定义每个目标字段的提取流程
//! ```text
//! FieldRules { title: FieldExtractor, cover: FieldExtractor, ... } -> ItemDetail
//! ```
//!
//! 每个 `FieldRule` 包含一个 `FieldExtractor`，通过步骤列表定义提取逻辑：
//! ```toml
//! title.steps = [{ css = ".title" }, { filter = "trim" }]
//! cover.steps = [{ css = "img" }, { attr = "src" }, { filter = "absolute_url" }]
//! ```
//!
//! ## 模块结构
//!
//! - `common`: 通用字段规则类型 (FieldRule)
//! - `item`: 列表项字段规则 (ItemFields)
//! - `list_rules`: 列表类型提取规则 (ChapterListRule, PlayLineListRule 等)
//! - `book`: 书籍字段规则
//! - `video`: 视频字段规则
//! - `audio`: 音频字段规则
//! - `manga`: 漫画字段规则

mod audio;
mod book;
mod common;
mod item;
mod list_rules;
mod manga;
mod video;

// 重新导出所有类型
pub use audio::*;
pub use book::*;
pub use common::*;
pub use item::*;
pub use list_rules::*;
pub use manga::*;
pub use video::*;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ============================================================================
// 统一的详情页字段规则（根据媒体类型选择）
// ============================================================================

/// 详情页字段规则 (DetailFields)
/// 根据媒体类型使用不同的字段定义
///
/// 使用 Box 包装以减少枚举大小差异
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "media_type", rename_all = "lowercase")]
pub enum DetailFields {
    /// 视频详情字段
    Video(Box<VideoDetailFields>),
    /// 音频详情字段
    Audio(Box<AudioDetailFields>),
    /// 书籍详情字段
    Book(Box<BookDetailFields>),
    /// 漫画详情字段
    Manga(Box<MangaDetailFields>),
}

/// 内容页字段规则 (ContentFields)
/// 用于播放页、阅读页等内容消费页面
///
/// 使用 Box 包装以减少枚举大小差异
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "media_type", rename_all = "lowercase")]
pub enum ContentFields {
    /// 视频播放字段
    Video(Box<VideoPlayFields>),
    /// 音频播放字段
    Audio(Box<AudioPlayFields>),
    /// 书籍正文字段
    Book(Box<BookContentFields>),
    /// 漫画阅读字段
    Manga(Box<MangaReadFields>),
}

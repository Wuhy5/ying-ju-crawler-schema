//! 内容页流程 (ContentFlow)

use crate::{fields::ContentFields, template::Template};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// 内容页流程 (ContentFlow)
/// 用于播放页、阅读页等内容消费页面
///
/// 例如：
/// - 视频：解析真实播放地址
/// - 书籍：获取章节正文内容
/// - 漫画：获取章节图片列表
/// - 音频：获取音频播放地址
///
/// # 示例
///
/// ```toml
/// [content]
/// url = "{{ play_url }}"
///
/// [content.fields]
/// media_type = "video"
/// play_url.steps = [{ css = "#player" }, { attr = "src" }, { filter = "absolute_url" }]
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ContentFlow {
    /// 流程的功能描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 内容页 URL 模板
    /// 约定输入变量: {{ play_url }} 或 {{ chapter_url }}
    pub url: Template,

    /// 内容字段提取规则
    pub fields: ContentFields,
}

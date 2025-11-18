//! 辅助枚举类型与标识符

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// 渲染层数据模型
pub mod render;

// 重新导出渲染层类型以便外部使用
pub use render::{
    AudioTrack,
    BookChapter,
    ItemDetail,
    ItemSummary,
    MangaChapter,
    MediaContent,
    VideoEpisode,
    VideoPlayLine,
};

/// 标识符 (Identifier)
/// 用于校验变量名、组件名、流程名等。
/// 必须由字母、数字和下划线组成，且不能以数字开头。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Identifier(#[schemars(regex(pattern = "^[a-zA-Z_][a-zA-Z0-9_]*$"))] pub String);

/// 用于指定规则适用的媒体内容类型。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    /// 视频类型，如电影、电视剧等。
    Video,
    /// 音频类型，如音乐、播客等。
    Audio,
    /// 书籍类型，如电子书、小说等。
    Book,
    /// 漫画类型，如漫画、图画书等。
    Manga,
}

/// HTTP 请求方法 (HttpMethod)
/// 用于指定网络请求的 HTTP 方法。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    /// GET 请求，通常用于获取数据。
    Get,
    /// POST 请求，通常用于提交数据。
    Post,
    /// PUT 请求，通常用于更新数据。
    Put,
    /// DELETE 请求，通常用于删除数据。
    Delete,
    /// HEAD 请求，类似于 GET，但只获取响应头。
    Head,
    /// OPTIONS 请求，获取服务器支持的HTTP方法。
    Options,
}

/// 脚本引擎类型 (ScriptEngine)
/// 用于指定脚本执行环境的类型。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ScriptEngine {
    /// Rhai 脚本引擎。
    Rhai,
    /// JavaScript 脚本引擎。
    JavaScript,
    /// Python 脚本引擎。
    Python,
    /// Lua 脚本引擎。
    Lua,
}

/// 缓存后端 (CacheBackend)
/// 用于指定缓存存储的后端类型。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CacheBackend {
    /// 内存缓存，适合临时数据存储。
    Memory,
    /// SQLite 数据库存储，适合持久化缓存。
    Sqlite,
}

/// 选择器提取方式
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExtractType {
    /// 提取文本内容
    Text,
    /// 提取内部HTML
    Html,
    /// 提取完整HTML
    OuterHtml,
    /// 提取 `href` 属性
    #[serde(rename = "attr:href")]
    AttrHref,
    /// 提取 `src` 属性
    #[serde(rename = "attr:src")]
    AttrSrc,
    /// 提取 `data-src` 属性
    #[serde(rename = "attr:data-src")]
    AttrDataSrc,
    /// 提取自定义属性，格式为 `attr:your-attribute-name`
    #[serde(untagged)]
    CustomAttr(String),
}

//! 可重用组件 (Component)
//!
//! 组件机制允许定义可复用的提取逻辑，避免重复代码。
//!
//! # 设计理念
//!
//! 1. **组件定义**：在 `components` 中以名称为键定义可复用的提取逻辑
//! 2. **组件引用**：在 `ExtractStep` 中通过 `use_component` 引用已定义的组件
//! 3. **参数传递**：引用时可传入参数覆盖组件的默认输入
//!
//! # 示例
//!
//! ```toml
//! # 定义组件
//! [components.parse_encrypted_url]
//! description = "解析加密的视频地址"
//! inputs = { encrypted_url = "" }
//! extractor.steps = [{ script = "decrypt.parse_m3u8" }]
//!
//! [components.extract_cover]
//! description = "提取封面图片"
//! extractor.steps = [
//!     { css = "img" },
//!     { attr = "src" },
//!     { filter = "absolute_url" }
//! ]
//!
//! # 在字段中引用组件
//! cover.steps = [{ use_component = "extract_cover" }]
//!
//! # 带参数引用
//! video_url.steps = [
//!     { use_component = { name = "parse_encrypted_url", args = { encrypted_url = "{{ raw_url }}" } } }
//! ]
//! ```

use crate::extract::FieldExtractor;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 组件集合 (Components)
///
/// 以名称为键的组件映射表，用于在规则文件中定义可复用的提取逻辑。
///
/// # 示例
///
/// ```toml
/// [components.parse_video_url]
/// description = "解析加密的视频地址"
/// extractor.steps = [{ script = "decrypt.parse_m3u8" }]
///
/// [components.extract_cover]
/// description = "提取封面"
/// extractor.steps = [{ css = "img" }, { attr = "src" }]
/// ```
pub type Components = HashMap<String, ComponentDefinition>;

/// 组件定义 (ComponentDefinition)
///
/// 封装可复用的字段提取逻辑。
///
/// # 字段说明
///
/// - `description`: 组件功能描述，便于维护
/// - `inputs`: 组件接收的输入参数及其默认值
/// - `extractor`: 组件的提取逻辑
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ComponentDefinition {
    /// 组件的功能描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 定义组件接收的输入参数 (key: 参数名, value: 默认值)
    ///
    /// 引用组件时，可通过 `args` 覆盖这些参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<HashMap<String, serde_json::Value>>,

    /// 组件的提取逻辑
    pub extractor: FieldExtractor,
}

/// 组件引用 (ComponentRef)
///
/// 在 `ExtractStep` 中用于引用已定义的组件。
///
/// # 示例
///
/// ```toml
/// # 简单引用（无参数）
/// cover.steps = [{ use_component = "extract_cover" }]
///
/// # 带参数引用
/// video_url.steps = [
///     { use_component = { name = "parse_encrypted_url", args = { encrypted_url = "{{ raw_url }}" } } }
/// ]
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ComponentRef {
    /// 简单引用：仅组件名称
    Simple(String),
    /// 带参数引用
    WithArgs {
        /// 组件名称
        name: String,
        /// 传递给组件的参数，会覆盖组件的默认输入
        #[serde(skip_serializing_if = "Option::is_none")]
        args: Option<HashMap<String, serde_json::Value>>,
    },
}

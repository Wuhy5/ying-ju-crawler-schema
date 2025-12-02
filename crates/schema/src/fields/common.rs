//! 通用字段规则定义
//!
//! 包含 FieldRule、TransformRules 等基础类型

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::extract::FieldExtractor;

/// 字段规则 (FieldRule)
/// 定义单个字段的提取方式
///
/// `FieldRule` 是 `FieldExtractor` 的透明包装，提供更语义化的类型名。
///
/// # 示例
///
/// ```toml
/// # 单步骤提取
/// title.steps = [{ css = ".title" }, { filter = "trim" }]
///
/// # 带回退和默认值
/// cover.steps = [{ css = ".poster img" }, { attr = "src" }, { filter = "absolute_url" }]
/// cover.fallback = [[{ css = ".thumbnail" }, { attr = "src" }]]
/// cover.default = "/default-cover.jpg"
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct FieldRule {
    /// 字段提取器
    pub extractor: FieldExtractor,
}

/// 可选字段规则
/// 使用 Option 包装，None 表示不提取该字段
pub type OptionalFieldRule = Option<FieldRule>;

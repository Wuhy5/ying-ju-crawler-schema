//! 脚本调用类型定义
//!
//! 定义脚本调用的通用类型，可用于：
//! - 字段提取流程中的脚本步骤
//! - 登录流程中的脚本逻辑
//! - 其他需要脚本处理的场景
//!
//! # 设计理念
//!
//! 脚本调用是自包含的，无需预定义模块。支持：
//! - 内联代码：直接在配置中写脚本
//! - 外部文件：引用脚本文件
//! - 远程加载：从 URL 加载脚本

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// 脚本引擎
// ============================================================================

/// 脚本引擎类型
///
/// 指定脚本执行环境，默认为 JavaScript。
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase")]
pub enum ScriptEngine {
    /// JavaScript 脚本引擎（默认，使用 Boa）
    #[default]
    JavaScript,
    /// Rhai 脚本引擎（轻量级，Rust 原生）
    Rhai,
    /// Lua 脚本引擎
    Lua,
}

// ============================================================================
// 脚本调用
// ============================================================================

/// 脚本调用步骤
///
/// 自包含的脚本调用定义，支持多种形式：
///
/// # 示例
///
/// ## 最简形式：直接写代码字符串
/// ```yaml
/// script: "return input.trim().toUpperCase()"
/// ```
///
/// ## 内联代码（显式指定引擎）
/// ```yaml
/// script:
///   code: |
///     let result = input.split(",");
///     return result.map(s => s.trim());
///   engine: javascript
/// ```
///
/// ## 引用外部文件
/// ```yaml
/// script:
///   file: "./scripts/login.js"
/// ```
///
/// ## 远程脚本
/// ```yaml
/// script:
///   url: "https://example.com/scripts/utils.js"
///   function: "processData"
/// ```
///
/// ## 带参数调用
/// ```yaml
/// script:
///   code: "return input.replace(params.from, params.to)"
///   params:
///     from: "old"
///     to: "new"
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ScriptStep {
    /// 简单代码字符串
    /// 直接作为脚本代码执行（使用默认引擎）
    Simple(String),

    /// 完整配置
    Full(ScriptConfig),
}

/// 脚本完整配置
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ScriptConfig {
    /// 脚本来源（三选一）
    #[serde(flatten)]
    pub source: ScriptSource,

    /// 脚本引擎（可选，默认 JavaScript）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<ScriptEngine>,

    /// 要调用的函数名（可选）
    /// 如果脚本定义了多个函数，指定要调用的函数
    /// 默认调用 `main` 或直接执行脚本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,

    /// 传递给脚本的参数（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<HashMap<String, serde_json::Value>>,
}

/// 脚本来源
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ScriptSource {
    /// 内联代码
    Code(String),
    /// 本地文件路径（相对于规则文件）
    File(String),
    /// 远程 URL
    Url(String),
}

// ============================================================================
// 实现
// ============================================================================

impl ScriptStep {
    /// 获取脚本来源
    pub fn source(&self) -> ScriptSource {
        match self {
            ScriptStep::Simple(code) => ScriptSource::Code(code.clone()),
            ScriptStep::Full(config) => config.source.clone(),
        }
    }

    /// 获取脚本引擎
    pub fn engine(&self) -> ScriptEngine {
        match self {
            ScriptStep::Simple(_) => ScriptEngine::default(),
            ScriptStep::Full(config) => config.engine.unwrap_or_default(),
        }
    }

    /// 获取函数名
    pub fn function(&self) -> Option<&str> {
        match self {
            ScriptStep::Simple(_) => None,
            ScriptStep::Full(config) => config.function.as_deref(),
        }
    }

    /// 获取参数
    pub fn params(&self) -> Option<&HashMap<String, serde_json::Value>> {
        match self {
            ScriptStep::Simple(_) => None,
            ScriptStep::Full(config) => config.params.as_ref(),
        }
    }
}

impl Default for ScriptStep {
    fn default() -> Self {
        ScriptStep::Simple(String::new())
    }
}

//! Schema 错误类型
//!
//! 包含与 Schema 定义相关的错误类型，主要用于解析和验证阶段。

use thiserror::Error;

/// Schema 错误类型
#[derive(Debug, Error, Clone)]
pub enum SchemaError {
    /// JSON 解析错误
    #[error("JSON 解析错误: {0}")]
    JsonParse(String),

    /// TOML 解析错误
    #[error("TOML 解析错误: {0}")]
    TomlParse(String),

    /// IO 错误
    #[error("IO 错误: {0}")]
    Io(String),
}

impl From<serde_json::Error> for SchemaError {
    fn from(e: serde_json::Error) -> Self {
        SchemaError::JsonParse(e.to_string())
    }
}

impl From<std::io::Error> for SchemaError {
    fn from(e: std::io::Error) -> Self {
        SchemaError::Io(e.to_string())
    }
}

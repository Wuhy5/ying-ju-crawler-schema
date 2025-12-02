//! 运行时模块
//!
//! 包含爬虫规则的运行时实现：
//! - 配置合并 (`config`)
//! - 运行时上下文 (`context`)
//! - 模板渲染 (`template`)
//! - 资源限制检查 (`limits`)
//! - 规则验证 (`validation`)
//!
//! ## 设计理念
//!
//! Schema 模块只包含纯数据结构定义，所有运行时逻辑都在本模块中实现。
//! 通过扩展 trait 模式为 schema 类型添加运行时功能。

pub mod config;
pub mod context;
pub mod error;
pub mod template;
pub mod validation;

pub use config::{ConfigMerge, HttpConfigExt};
pub use context::RuntimeContext;
pub use error::RuntimeError;
pub use template::{escape_html, RenderOptions, TemplateExt};
pub use validation::{RuleValidate, ValidationError};

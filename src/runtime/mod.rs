//! 运行时模块
//!
//! 包含爬虫规则的运行时实现：
//! - 模板渲染 (template)
//! - 管道验证 (pipeline)
//! - 限制检查 (limits)
//! - 规则验证 (validation)
//!
//! ## 设计理念
//!
//! Schema 模块只包含纯数据结构定义，所有运行时逻辑都在本模块中实现。
//! 通过扩展 trait 模式为 schema 类型添加运行时功能。

pub mod limits;
pub mod pipeline;
pub mod template;
pub mod validation;

pub use limits::LimitsExt;
pub use pipeline::PipelineExt;
pub use template::{escape_html, RenderOptions, TemplateExt};
pub use validation::RuleValidate;

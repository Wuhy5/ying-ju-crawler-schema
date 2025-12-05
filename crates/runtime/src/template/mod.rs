//! # 模板模块
//!
//! 提供模板渲染和验证功能

use crate::{Result, RuntimeError, context::FlowContext};
use crawler_schema::template::Template;
use tera::Tera;

/// 模板渲染扩展 trait
///
/// 为 `crawler_schema::Template` 添加运行时渲染能力
pub trait TemplateExt {
    /// 渲染模板
    ///
    /// # 参数
    ///
    /// - `ctx`: 流程上下文，包含 Flow 变量和 Runtime 全局变量
    ///
    /// # 变量查找规则
    ///
    /// | 写法 | 查找逻辑 |
    /// |------|---------|
    /// | `{{ var }}` | 先查 Flow，再查 Runtime |
    /// | `{{ $.var }}` | 仅查 Runtime 全局变量 |
    fn render(&self, flow_context: &FlowContext) -> Result<String>;
}

impl TemplateExt for Template {
    fn render(&self, flow_context: &FlowContext) -> Result<String> {
        Tera::one_off(self.as_str(), &flow_context.to_tera_context()?, true).map_err(|e| {
            RuntimeError::TemplateError {
                error: e.to_string(),
            }
        })
    }
}

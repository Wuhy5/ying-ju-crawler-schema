//! # 步骤执行器
//!
//! 使用无状态静态方法实现各种提取步骤

use crate::{
    Result,
    context::{FlowContext, RuntimeContext},
    extractor::value::{ExtractValueData, SharedValue},
};
use crawler_schema::extract::ExtractStep;

/// 步骤执行器工厂
///
/// 所有执行器都使用静态方法，无需创建实例
pub struct StepExecutorFactory;

impl StepExecutorFactory {
    /// 直接执行步骤
    pub fn execute(
        step: &ExtractStep,
        input: &ExtractValueData,
        runtime_context: &RuntimeContext,
        flow_context: &FlowContext,
    ) -> Result<SharedValue> {
        match step {
            ExtractStep::Css(selector) => {
                crate::extractor::selector::css::CssSelectorExecutor::execute(
                    selector,
                    input,
                    runtime_context,
                    flow_context,
                )
            }
            ExtractStep::Json(selector) => {
                crate::extractor::selector::json::JsonSelectorExecutor::execute(
                    selector,
                    input,
                    runtime_context,
                    flow_context,
                )
            }
            ExtractStep::Regex(regex) => {
                crate::extractor::selector::regex::RegexSelectorExecutor::execute(
                    regex,
                    input,
                    runtime_context,
                    flow_context,
                )
            }
            ExtractStep::Filter(filter) => {
                crate::extractor::filter::executor::FilterExecutor::execute(
                    filter,
                    input,
                    runtime_context,
                    flow_context,
                )
            }
            ExtractStep::Attr(attr) => crate::extractor::selector::attr::AttrExecutor::execute(
                attr,
                input,
                runtime_context,
                flow_context,
            ),
            ExtractStep::Index(index) => crate::extractor::selector::index::IndexExecutor::execute(
                index,
                input,
                runtime_context,
                flow_context,
            ),
            ExtractStep::SetVar(set_var) => {
                crate::extractor::selector::set_var::SetVarExecutor::execute(
                    set_var,
                    input,
                    runtime_context,
                    flow_context,
                )
            }
            ExtractStep::Script(script) => {
                crate::script::ScriptExecutor::execute(script, input, runtime_context, flow_context)
            }
            ExtractStep::UseComponent(component_ref) => {
                crate::extractor::selector::component::ComponentExecutor::execute(
                    component_ref,
                    input,
                    runtime_context,
                    flow_context,
                )
            }
            ExtractStep::Xpath(_selector) => {
                // XPath 需要 JS 环境，暂不支持
                Err(crate::error::RuntimeError::Extraction(
                    "XPath not supported in this context".into(),
                ))
            }
            ExtractStep::Map(steps) => crate::extractor::selector::map::MapExecutor::execute(
                steps,
                input,
                runtime_context,
                flow_context,
            ),
            ExtractStep::Condition(condition) => {
                crate::extractor::selector::condition::ConditionExecutor::execute(
                    condition,
                    input,
                    runtime_context,
                    flow_context,
                )
            }
        }
    }
}

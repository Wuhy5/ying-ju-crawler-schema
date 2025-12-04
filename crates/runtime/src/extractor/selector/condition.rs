//! # 条件执行器
//!
//! 根据条件选择执行不同的提取逻辑

use crate::{
    Result,
    context::Context,
    extractor::{ExtractValue, StepExecutor, StepExecutorFactory},
};
use crawler_schema::extract::{ConditionStep, ExtractStep};

/// 条件执行器
///
/// 根据 `when` 条件选择执行 `then` 或 `otherwise` 步骤
pub struct ConditionExecutor {
    condition: ConditionStep,
}

impl ConditionExecutor {
    pub fn new(condition: ConditionStep) -> Self {
        Self { condition }
    }

    /// 执行一系列步骤
    fn execute_steps(
        &self,
        steps: &[ExtractStep],
        input: ExtractValue,
        context: &Context,
    ) -> Result<ExtractValue> {
        let mut current = input;

        for step in steps {
            let executor = StepExecutorFactory::create(step);
            current = executor.execute(current, context)?;
        }

        Ok(current)
    }

    /// 判断条件是否为真
    ///
    /// 执行 `when` 步骤，如果结果非空/非 null/非 false，则为真
    fn evaluate_condition(&self, input: &ExtractValue, context: &Context) -> bool {
        match self.execute_steps(&self.condition.when, input.clone(), context) {
            Ok(result) => result.is_truthy(),
            Err(_) => false,
        }
    }
}

impl StepExecutor for ConditionExecutor {
    fn execute(&self, input: ExtractValue, context: &Context) -> Result<ExtractValue> {
        if self.evaluate_condition(&input, context) {
            // 条件为真，执行 then 步骤
            self.execute_steps(&self.condition.then, input, context)
        } else if let Some(otherwise) = &self.condition.otherwise {
            // 条件为假，执行 otherwise 步骤
            self.execute_steps(otherwise, input, context)
        } else {
            // 没有 otherwise，返回原输入
            Ok(input)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crawler_schema::extract::{ExtractStep, SelectorStep};

    #[test]
    fn test_condition_true() {
        let condition = ConditionStep {
            when: vec![ExtractStep::Css(SelectorStep::Simple(".vip".to_string()))],
            then: vec![ExtractStep::Const(serde_json::json!("VIP"))],
            otherwise: Some(vec![ExtractStep::Const(serde_json::json!("Normal"))]),
        };

        let executor = ConditionExecutor::new(condition);
        let input = ExtractValue::Html("<div class=\"vip\">VIP User</div>".to_string());
        let context = Context::new();

        let result = executor.execute(input, &context).unwrap();
        assert_eq!(result.as_string(), Some("VIP".to_string()));
    }

    #[test]
    fn test_condition_false() {
        let condition = ConditionStep {
            when: vec![ExtractStep::Css(SelectorStep::Simple(".vip".to_string()))],
            then: vec![ExtractStep::Const(serde_json::json!("VIP"))],
            otherwise: Some(vec![ExtractStep::Const(serde_json::json!("Normal"))]),
        };

        let executor = ConditionExecutor::new(condition);
        let input = ExtractValue::Html("<div class=\"normal\">Normal User</div>".to_string());
        let context = Context::new();

        let result = executor.execute(input, &context).unwrap();
        assert_eq!(result.as_string(), Some("Normal".to_string()));
    }
}

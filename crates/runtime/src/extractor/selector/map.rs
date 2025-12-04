//! # 映射执行器
//!
//! 对数组每个元素应用步骤

use crate::{
    Result,
    context::Context,
    error::RuntimeError,
    extractor::{ExtractValue, StepExecutor, StepExecutorFactory},
};
use crawler_schema::extract::ExtractStep;

/// 映射执行器
///
/// 对输入数组的每个元素执行一系列步骤
pub struct MapExecutor {
    steps: Vec<ExtractStep>,
}

impl MapExecutor {
    pub fn new(steps: Vec<ExtractStep>) -> Self {
        Self { steps }
    }

    /// 对单个值执行所有步骤
    fn execute_steps(&self, input: ExtractValue, context: &Context) -> Result<ExtractValue> {
        let mut current = input;

        for step in &self.steps {
            let executor = StepExecutorFactory::create(step);
            current = executor.execute(current, context)?;
        }

        Ok(current)
    }
}

impl StepExecutor for MapExecutor {
    fn execute(&self, input: ExtractValue, context: &Context) -> Result<ExtractValue> {
        match input {
            ExtractValue::Array(arr) => {
                let results: Vec<ExtractValue> = arr
                    .into_iter()
                    .filter_map(|item| self.execute_steps(item, context).ok())
                    .collect();

                Ok(ExtractValue::Array(results))
            }
            _ => {
                // 非数组输入，直接应用步骤
                Err(RuntimeError::Extraction(
                    "Map step requires array input".to_string(),
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crawler_schema::extract::{ExtractStep, FilterStep};

    #[test]
    fn test_map_filter() {
        let executor = MapExecutor::new(vec![ExtractStep::Filter(FilterStep::Pipeline(
            "trim".to_string(),
        ))]);

        let input = ExtractValue::Array(vec![
            ExtractValue::String("  hello  ".to_string()),
            ExtractValue::String("  world  ".to_string()),
        ]);
        let context = Context::new();

        let result = executor.execute(input, &context).unwrap();

        if let ExtractValue::Array(arr) = result {
            assert_eq!(arr.len(), 2);
            assert_eq!(arr[0].as_string(), Some("hello".to_string()));
            assert_eq!(arr[1].as_string(), Some("world".to_string()));
        } else {
            panic!("Expected array result");
        }
    }
}

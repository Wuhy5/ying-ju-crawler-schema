//! # JSON 选择器执行器

use crate::{
    Result,
    context::Context,
    error::RuntimeError,
    extractor::{ExtractValue, StepExecutor},
};
use crawler_schema::extract::SelectorStep;
use jsonpath_rust::JsonPath;
use serde_json::Value;

/// JSON 选择器执行器
pub struct JsonSelectorExecutor {
    selector: SelectorStep,
}

impl JsonSelectorExecutor {
    pub fn new(selector: SelectorStep) -> Self {
        Self { selector }
    }
}

impl StepExecutor for JsonSelectorExecutor {
    fn execute(&self, input: ExtractValue, _context: &Context) -> Result<ExtractValue> {
        // 获取 JSON 值
        let json: Value = match &input {
            ExtractValue::Json(v) => v.clone(),
            ExtractValue::String(s) => serde_json::from_str(s)
                .map_err(|e| RuntimeError::Extraction(format!("Failed to parse JSON: {}", e)))?,
            ExtractValue::Array(arr) => {
                // 如果是数组，对每个元素应用选择器
                let results: Vec<ExtractValue> = arr
                    .iter()
                    .cloned()
                    .filter_map(|item| self.execute(item, _context).ok())
                    .collect();
                return Ok(ExtractValue::Array(results));
            }
            _ => {
                return Err(RuntimeError::Extraction(
                    "JSON selector requires JSON input".to_string(),
                ));
            }
        };

        let (jsonpath_str, select_all) = match &self.selector {
            SelectorStep::Simple(s) => (s.as_str(), false),
            SelectorStep::WithOptions { expr, all } => (expr.as_str(), *all),
        };

        // 使用 JsonPath trait 的 query 方法
        let results = json.query(jsonpath_str).map_err(|e| {
            RuntimeError::Extraction(format!("Invalid JSONPath '{}': {}", jsonpath_str, e))
        })?;

        // 处理结果
        if results.is_empty() {
            Ok(ExtractValue::Null)
        } else if !select_all && results.len() == 1 {
            Ok(ExtractValue::from_json(results[0]))
        } else {
            Ok(ExtractValue::Array(
                results.into_iter().map(ExtractValue::from_json).collect(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_json_selector_simple() {
        let executor = JsonSelectorExecutor::new(SelectorStep::Simple("$.name".to_string()));
        let input = ExtractValue::Json(json!({"name": "Alice", "age": 30}));
        let context = Context::new();

        let result = executor.execute(input, &context).unwrap();
        assert_eq!(result.as_string(), Some("Alice".to_string()));
    }

    #[test]
    fn test_json_selector_array() {
        let executor = JsonSelectorExecutor::new(SelectorStep::WithOptions {
            expr: "$.items[*].name".to_string(),
            all: true,
        });
        let input = ExtractValue::Json(json!({
            "items": [
                {"name": "A"},
                {"name": "B"},
                {"name": "C"}
            ]
        }));
        let context = Context::new();

        let result = executor.execute(input, &context).unwrap();
        if let ExtractValue::Array(arr) = result {
            assert_eq!(arr.len(), 3);
        } else {
            panic!("Expected array result");
        }
    }

    #[test]
    fn test_json_selector_nested() {
        let executor =
            JsonSelectorExecutor::new(SelectorStep::Simple("$.user.address.city".to_string()));
        let input = ExtractValue::Json(json!({
            "user": {
                "name": "Alice",
                "address": {
                    "city": "Beijing"
                }
            }
        }));
        let context = Context::new();

        let result = executor.execute(input, &context).unwrap();
        assert_eq!(result.as_string(), Some("Beijing".to_string()));
    }
}

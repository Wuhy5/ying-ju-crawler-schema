//! # CSS 选择器执行器

use crate::{
    Result,
    context::Context,
    error::RuntimeError,
    extractor::{ExtractValue, StepExecutor},
};
use crawler_schema::extract::SelectorStep;
use scraper::{Html, Selector};

/// CSS 选择器执行器
pub struct CssSelectorExecutor {
    selector: SelectorStep,
}

impl CssSelectorExecutor {
    pub fn new(selector: SelectorStep) -> Self {
        Self { selector }
    }
}

impl StepExecutor for CssSelectorExecutor {
    fn execute(&self, input: ExtractValue, _context: &Context) -> Result<ExtractValue> {
        // 获取 HTML 字符串
        let html = match &input {
            ExtractValue::String(s) | ExtractValue::Html(s) => s,
            ExtractValue::Array(arr) => {
                // 如果是数组，对每个元素应用选择器
                let results: Vec<ExtractValue> = arr
                    .iter()
                    .filter_map(|item| {
                        if let ExtractValue::Html(h) | ExtractValue::String(h) = item {
                            self.execute_on_html(h).ok()
                        } else {
                            None
                        }
                    })
                    .flat_map(|v| v.into_iter())
                    .collect();
                return Ok(ExtractValue::Array(results));
            }
            _ => {
                return Err(RuntimeError::Extraction(
                    "CSS selector requires HTML input".to_string(),
                ));
            }
        };

        let results = self.execute_on_html(html)?;
        if results.is_empty() {
            Ok(ExtractValue::Null)
        } else if results.len() == 1 && !self.is_select_all() {
            Ok(results.into_iter().next().unwrap())
        } else {
            Ok(ExtractValue::Array(results))
        }
    }
}

impl CssSelectorExecutor {
    /// 在 HTML 上执行选择器
    fn execute_on_html(&self, html: &str) -> Result<Vec<ExtractValue>> {
        let document = Html::parse_fragment(html);

        let (selector_str, select_all) = match &self.selector {
            SelectorStep::Simple(s) => (s.as_str(), false),
            SelectorStep::WithOptions { expr, all } => (expr.as_str(), *all),
        };

        let selector = Selector::parse(selector_str).map_err(|e| {
            RuntimeError::Extraction(format!("Invalid CSS selector '{}': {:?}", selector_str, e))
        })?;

        let elements = document.select(&selector);

        let results: Vec<ExtractValue> = if select_all {
            elements.map(|el| ExtractValue::Html(el.html())).collect()
        } else {
            // 只取第一个匹配
            elements
                .take(1)
                .map(|el| ExtractValue::Html(el.html()))
                .collect()
        };

        Ok(results)
    }

    /// 是否选择所有匹配
    fn is_select_all(&self) -> bool {
        match &self.selector {
            SelectorStep::Simple(_) => false,
            SelectorStep::WithOptions { all, .. } => *all,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_selector_simple() {
        let executor = CssSelectorExecutor::new(SelectorStep::Simple("h1".to_string()));
        let input = ExtractValue::Html("<html><h1>Hello</h1><p>World</p></html>".to_string());
        let context = Context::new();

        let result = executor.execute(input, &context).unwrap();
        assert!(matches!(result, ExtractValue::Html(_)));
    }

    #[test]
    fn test_css_selector_with_class() {
        let executor = CssSelectorExecutor::new(SelectorStep::Simple(".title".to_string()));
        let input = ExtractValue::Html("<div><span class=\"title\">Test</span></div>".to_string());
        let context = Context::new();

        let result = executor.execute(input, &context).unwrap();
        assert!(matches!(result, ExtractValue::Html(_)));
    }

    #[test]
    fn test_css_selector_all() {
        let executor = CssSelectorExecutor::new(SelectorStep::WithOptions {
            expr: "li".to_string(),
            all: true,
        });
        let input = ExtractValue::Html("<ul><li>1</li><li>2</li><li>3</li></ul>".to_string());
        let context = Context::new();

        let result = executor.execute(input, &context).unwrap();
        if let ExtractValue::Array(arr) = result {
            assert_eq!(arr.len(), 3);
        } else {
            panic!("Expected array result");
        }
    }
}

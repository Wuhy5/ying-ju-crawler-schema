//! # 属性提取器

use crate::{
    Result,
    context::Context,
    error::RuntimeError,
    extractor::{ExtractValue, StepExecutor},
};
use scraper::Html;

/// 属性提取器
///
/// 从 HTML 元素中提取属性或文本内容
/// 支持的属性名：
/// - `text` - 提取文本内容
/// - `html` - 提取内部 HTML
/// - `outer_html` - 提取外部 HTML（包含自身标签）
/// - 其他 - 提取指定属性值（如 href, src, class 等）
pub struct AttrExecutor {
    attr_name: String,
}

impl AttrExecutor {
    pub fn new(attr_name: String) -> Self {
        Self { attr_name }
    }
}

impl StepExecutor for AttrExecutor {
    fn execute(&self, input: ExtractValue, _context: &Context) -> Result<ExtractValue> {
        match &input {
            ExtractValue::Html(html) | ExtractValue::String(html) => self.extract_from_html(html),
            ExtractValue::Array(arr) => {
                // 对数组中的每个元素提取属性
                let results: Vec<ExtractValue> = arr
                    .iter()
                    .filter_map(|item| match item {
                        ExtractValue::Html(h) | ExtractValue::String(h) => {
                            self.extract_from_html(h).ok()
                        }
                        _ => None,
                    })
                    .filter(|v| !v.is_empty())
                    .collect();

                if results.is_empty() {
                    Ok(ExtractValue::Null)
                } else if results.len() == 1 {
                    Ok(results.into_iter().next().unwrap())
                } else {
                    Ok(ExtractValue::Array(results))
                }
            }
            _ => Err(RuntimeError::Extraction(
                "Attr executor requires HTML input".to_string(),
            )),
        }
    }
}

impl AttrExecutor {
    fn extract_from_html(&self, html: &str) -> Result<ExtractValue> {
        let document = Html::parse_fragment(html);

        // 获取根元素（第一个非文本元素）
        let root = document
            .root_element()
            .first_child()
            .and_then(scraper::ElementRef::wrap);

        let result = match self.attr_name.as_str() {
            "text" => {
                // 提取所有文本内容
                let text: String = document
                    .root_element()
                    .text()
                    .collect::<Vec<_>>()
                    .join("")
                    .trim()
                    .to_string();
                if text.is_empty() {
                    ExtractValue::Null
                } else {
                    ExtractValue::String(text)
                }
            }
            "html" | "inner_html" => {
                // 提取内部 HTML
                if let Some(el) = root {
                    ExtractValue::String(el.inner_html())
                } else {
                    ExtractValue::Null
                }
            }
            "outer_html" => {
                // 提取外部 HTML
                if let Some(el) = root {
                    ExtractValue::Html(el.html())
                } else {
                    ExtractValue::Null
                }
            }
            attr => {
                // 提取指定属性
                if let Some(el) = root {
                    match el.value().attr(attr) {
                        Some(value) => ExtractValue::String(value.to_string()),
                        None => ExtractValue::Null,
                    }
                } else {
                    ExtractValue::Null
                }
            }
        };

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attr_text() {
        let executor = AttrExecutor::new("text".to_string());
        let input =
            ExtractValue::Html("<div><span>Hello</span> <span>World</span></div>".to_string());
        let context = Context::new();

        let result = executor.execute(input, &context).unwrap();
        assert_eq!(result.as_string(), Some("Hello World".to_string()));
    }

    #[test]
    fn test_attr_href() {
        let executor = AttrExecutor::new("href".to_string());
        let input = ExtractValue::Html("<a href=\"https://example.com\">Link</a>".to_string());
        let context = Context::new();

        let result = executor.execute(input, &context).unwrap();
        assert_eq!(result.as_string(), Some("https://example.com".to_string()));
    }

    #[test]
    fn test_attr_src() {
        let executor = AttrExecutor::new("src".to_string());
        let input = ExtractValue::Html("<img src=\"/image.png\" alt=\"test\">".to_string());
        let context = Context::new();

        let result = executor.execute(input, &context).unwrap();
        assert_eq!(result.as_string(), Some("/image.png".to_string()));
    }

    #[test]
    fn test_attr_array() {
        let executor = AttrExecutor::new("href".to_string());
        let input = ExtractValue::Array(vec![
            ExtractValue::Html("<a href=\"/page1\">1</a>".to_string()),
            ExtractValue::Html("<a href=\"/page2\">2</a>".to_string()),
            ExtractValue::Html("<a href=\"/page3\">3</a>".to_string()),
        ]);
        let context = Context::new();

        let result = executor.execute(input, &context).unwrap();
        if let ExtractValue::Array(arr) = result {
            assert_eq!(arr.len(), 3);
        } else {
            panic!("Expected array result");
        }
    }
}

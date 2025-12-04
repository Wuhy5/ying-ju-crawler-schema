//! # 字符串过滤器

use crate::{
    Result,
    error::RuntimeError,
    extractor::{ExtractValue, filter::Filter},
};
use serde_json::Value;

/// Trim 过滤器
pub struct TrimFilter;

impl Filter for TrimFilter {
    fn apply(&self, input: &ExtractValue, _args: &[Value]) -> Result<ExtractValue> {
        let s = input.as_string().ok_or_else(|| {
            RuntimeError::Extraction("trim filter requires string input".to_string())
        })?;
        Ok(ExtractValue::String(s.trim().to_string()))
    }
}

/// Lower 过滤器
pub struct LowerFilter;

impl Filter for LowerFilter {
    fn apply(&self, input: &ExtractValue, _args: &[Value]) -> Result<ExtractValue> {
        let s = input.as_string().ok_or_else(|| {
            RuntimeError::Extraction("lower filter requires string input".to_string())
        })?;
        Ok(ExtractValue::String(s.to_lowercase()))
    }
}

/// Upper 过滤器
pub struct UpperFilter;

impl Filter for UpperFilter {
    fn apply(&self, input: &ExtractValue, _args: &[Value]) -> Result<ExtractValue> {
        let s = input.as_string().ok_or_else(|| {
            RuntimeError::Extraction("upper filter requires string input".to_string())
        })?;
        Ok(ExtractValue::String(s.to_uppercase()))
    }
}

/// Replace 过滤器
/// 参数: [from, to]
pub struct ReplaceFilter;

impl Filter for ReplaceFilter {
    fn apply(&self, input: &ExtractValue, args: &[Value]) -> Result<ExtractValue> {
        let s = input.as_string().ok_or_else(|| {
            RuntimeError::Extraction("replace filter requires string input".to_string())
        })?;

        if args.len() < 2 {
            return Err(RuntimeError::Extraction(
                "replace filter requires 2 arguments: from, to".to_string(),
            ));
        }

        let from = args[0].as_str().ok_or_else(|| {
            RuntimeError::Extraction("replace: 'from' must be a string".to_string())
        })?;
        let to = args[1].as_str().ok_or_else(|| {
            RuntimeError::Extraction("replace: 'to' must be a string".to_string())
        })?;

        Ok(ExtractValue::String(s.replace(from, to)))
    }
}

/// RegexReplace 过滤器
/// 参数: [pattern, replacement]
pub struct RegexReplaceFilter;

impl Filter for RegexReplaceFilter {
    fn apply(&self, input: &ExtractValue, args: &[Value]) -> Result<ExtractValue> {
        let s = input.as_string().ok_or_else(|| {
            RuntimeError::Extraction("regex_replace filter requires string input".to_string())
        })?;

        if args.len() < 2 {
            return Err(RuntimeError::Extraction(
                "regex_replace filter requires 2 arguments: pattern, replacement".to_string(),
            ));
        }

        let pattern = args[0].as_str().ok_or_else(|| {
            RuntimeError::Extraction("regex_replace: 'pattern' must be a string".to_string())
        })?;
        let replacement = args[1].as_str().ok_or_else(|| {
            RuntimeError::Extraction("regex_replace: 'replacement' must be a string".to_string())
        })?;

        let re = regex::Regex::new(pattern)
            .map_err(|e| RuntimeError::Extraction(format!("Invalid regex pattern: {}", e)))?;

        Ok(ExtractValue::String(
            re.replace_all(&s, replacement).to_string(),
        ))
    }
}

/// Split 过滤器
/// 参数: [separator]
pub struct SplitFilter;

impl Filter for SplitFilter {
    fn apply(&self, input: &ExtractValue, args: &[Value]) -> Result<ExtractValue> {
        let s = input.as_string().ok_or_else(|| {
            RuntimeError::Extraction("split filter requires string input".to_string())
        })?;

        let sep = args.first().and_then(|v| v.as_str()).unwrap_or(" ");

        let parts: Vec<ExtractValue> = s
            .split(sep)
            .map(|p| ExtractValue::String(p.to_string()))
            .collect();

        Ok(ExtractValue::Array(parts))
    }
}

/// Join 过滤器
/// 参数: [separator]
pub struct JoinFilter;

impl Filter for JoinFilter {
    fn apply(&self, input: &ExtractValue, args: &[Value]) -> Result<ExtractValue> {
        let arr = input.as_array().ok_or_else(|| {
            RuntimeError::Extraction("join filter requires array input".to_string())
        })?;

        let sep = args.first().and_then(|v| v.as_str()).unwrap_or("");

        let strings: Vec<String> = arr.iter().filter_map(|v| v.as_string()).collect();

        Ok(ExtractValue::String(strings.join(sep)))
    }
}

/// StripHtml 过滤器
/// 移除所有 HTML 标签
pub struct StripHtmlFilter;

impl Filter for StripHtmlFilter {
    fn apply(&self, input: &ExtractValue, _args: &[Value]) -> Result<ExtractValue> {
        let s = input.as_string().ok_or_else(|| {
            RuntimeError::Extraction("strip_html filter requires string input".to_string())
        })?;

        // 使用正则移除 HTML 标签
        let re = regex::Regex::new(r"<[^>]+>").unwrap();
        let result = re.replace_all(&s, "").to_string();

        Ok(ExtractValue::String(result))
    }
}

/// Substring 过滤器
/// 参数: [start, length?]
pub struct SubstringFilter;

impl Filter for SubstringFilter {
    fn apply(&self, input: &ExtractValue, args: &[Value]) -> Result<ExtractValue> {
        let s = input.as_string().ok_or_else(|| {
            RuntimeError::Extraction("substring filter requires string input".to_string())
        })?;

        let start = args.first().and_then(|v| v.as_i64()).unwrap_or(0) as usize;

        let len = args.get(1).and_then(|v| v.as_i64()).map(|l| l as usize);

        let chars: Vec<char> = s.chars().collect();
        let end = len
            .map(|l| (start + l).min(chars.len()))
            .unwrap_or(chars.len());
        let result: String = chars[start.min(chars.len())..end].iter().collect();

        Ok(ExtractValue::String(result))
    }
}

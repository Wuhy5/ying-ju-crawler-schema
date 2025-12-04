//! # URL 处理过滤器

use crate::{
    Result,
    error::RuntimeError,
    extractor::{ExtractValue, filter::Filter},
};
use serde_json::Value;

/// AbsoluteUrl 过滤器
/// 将相对 URL 转换为绝对 URL
/// 参数: [base_url]
pub struct AbsoluteUrlFilter;

impl Filter for AbsoluteUrlFilter {
    fn apply(&self, input: &ExtractValue, args: &[Value]) -> Result<ExtractValue> {
        let url = input.as_string().ok_or_else(|| {
            RuntimeError::Extraction("absolute_url filter requires string input".to_string())
        })?;

        // 如果已经是绝对 URL，直接返回
        if url.starts_with("http://") || url.starts_with("https://") {
            return Ok(ExtractValue::String(url));
        }

        // 需要 base_url 参数
        let base_url = args.first().and_then(|v| v.as_str()).ok_or_else(|| {
            RuntimeError::Extraction("absolute_url filter requires base_url argument".to_string())
        })?;

        // 拼接 URL
        let absolute = if url.starts_with('/') {
            // 绝对路径
            let base = base_url.trim_end_matches('/');
            // 提取 base 的 origin (scheme + host)
            if let Some(idx) = base.find("://") {
                if let Some(path_start) = base[idx + 3..].find('/') {
                    format!("{}{}", &base[..idx + 3 + path_start], url)
                } else {
                    format!("{}{}", base, url)
                }
            } else {
                format!("{}{}", base, url)
            }
        } else {
            // 相对路径
            let base = base_url.trim_end_matches('/');
            format!("{}/{}", base, url)
        };

        Ok(ExtractValue::String(absolute))
    }
}

/// UrlEncode 过滤器
pub struct UrlEncodeFilter;

impl Filter for UrlEncodeFilter {
    fn apply(&self, input: &ExtractValue, _args: &[Value]) -> Result<ExtractValue> {
        let s = input.as_string().ok_or_else(|| {
            RuntimeError::Extraction("url_encode filter requires string input".to_string())
        })?;

        Ok(ExtractValue::String(urlencoding::encode(&s).to_string()))
    }
}

/// UrlDecode 过滤器
pub struct UrlDecodeFilter;

impl Filter for UrlDecodeFilter {
    fn apply(&self, input: &ExtractValue, _args: &[Value]) -> Result<ExtractValue> {
        let s = input.as_string().ok_or_else(|| {
            RuntimeError::Extraction("url_decode filter requires string input".to_string())
        })?;

        let decoded = urlencoding::decode(&s)
            .map_err(|e| RuntimeError::Extraction(format!("Failed to decode URL: {}", e)))?;

        Ok(ExtractValue::String(decoded.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_absolute_url_already_absolute() {
        let filter = AbsoluteUrlFilter;
        let input = ExtractValue::String("https://example.com/page".to_string());
        let result = filter.apply(&input, &[]).unwrap();
        assert_eq!(
            result.as_string(),
            Some("https://example.com/page".to_string())
        );
    }

    #[test]
    fn test_absolute_url_relative() {
        let filter = AbsoluteUrlFilter;
        let input = ExtractValue::String("/page".to_string());
        let args = vec![Value::String("https://example.com".to_string())];
        let result = filter.apply(&input, &args).unwrap();
        assert_eq!(
            result.as_string(),
            Some("https://example.com/page".to_string())
        );
    }

    #[test]
    fn test_url_encode() {
        let filter = UrlEncodeFilter;
        let input = ExtractValue::String("hello world".to_string());
        let result = filter.apply(&input, &[]).unwrap();
        assert_eq!(result.as_string(), Some("hello%20world".to_string()));
    }
}

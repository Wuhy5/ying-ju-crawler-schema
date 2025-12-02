//! # HTTP 配置扩展
//!
//! 为 HttpConfig 提供合并和转换功能

use crawler_schema::{HttpConfig, RequestConfig};

/// HTTP 配置扩展 trait
pub trait HttpConfigExt {
    /// 合并配置（other 优先级更高）
    fn merge(&self, other: &Self) -> Self;

    /// 合并请求配置
    fn merge_request(&self, request: &RequestConfig) -> Self;
}

impl HttpConfigExt for HttpConfig {
    fn merge(&self, other: &Self) -> Self {
        Self {
            user_agent: other.user_agent.clone().or_else(|| self.user_agent.clone()),
            timeout: other.timeout.or(self.timeout),
            proxy: other.proxy.clone().or_else(|| self.proxy.clone()),
            headers: merge_headers(&self.headers, &other.headers),
            follow_redirects: other.follow_redirects.or(self.follow_redirects),
            max_redirects: other.max_redirects.or(self.max_redirects),
            connect_timeout: other.connect_timeout.or(self.connect_timeout),
            verify_ssl: other.verify_ssl.or(self.verify_ssl),
            request_delay: other.request_delay.or(self.request_delay),
            max_concurrent: other.max_concurrent.or(self.max_concurrent),
            retry_count: other.retry_count.or(self.retry_count),
            retry_delay: other.retry_delay.or(self.retry_delay),
        }
    }

    fn merge_request(&self, _request: &RequestConfig) -> Self {
        // TODO: 实现请求级别的配置合并
        self.clone()
    }
}

/// 合并 headers
fn merge_headers(
    base: &Option<std::collections::HashMap<String, String>>,
    override_headers: &Option<std::collections::HashMap<String, String>>,
) -> Option<std::collections::HashMap<String, String>> {
    match (base, override_headers) {
        (None, None) => None,
        (Some(b), None) => Some(b.clone()),
        (None, Some(o)) => Some(o.clone()),
        (Some(b), Some(o)) => {
            let mut merged = b.clone();
            merged.extend(o.clone());
            Some(merged)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_config() {
        let base = HttpConfig {
            user_agent: Some("Base/1.0".to_string()),
            timeout: Some(30),
            ..Default::default()
        };

        let override_config = HttpConfig {
            user_agent: Some("Override/2.0".to_string()),
            ..Default::default()
        };

        let merged = base.merge(&override_config);
        assert_eq!(merged.user_agent, Some("Override/2.0".to_string()));
        assert_eq!(merged.timeout, Some(30));
    }
}

//! 配置合并模块
//!
//! 提供配置层级合并功能。

use std::collections::HashMap;

use crawler_schema::config::{
    HttpConfig, DEFAULT_FOLLOW_REDIRECTS, DEFAULT_MAX_REDIRECTS, DEFAULT_TIMEOUT,
    DEFAULT_USER_AGENT,
};

/// 配置合并 trait
/// 用于在多层配置（全局/流程/步骤）之间进行值的合并
pub trait ConfigMerge {
    /// 将 `other` 的值合并到 `self`，`other` 中的值优先级更高
    fn merge(&self, other: &Self) -> Self;
}

// 为 Option<T> 实现 ConfigMerge
impl<T: Clone> ConfigMerge for Option<T> {
    fn merge(&self, other: &Self) -> Self {
        other.clone().or_else(|| self.clone())
    }
}

// 为 HashMap 实现 ConfigMerge
impl<K, V> ConfigMerge for HashMap<K, V>
where
    K: Clone + Eq + std::hash::Hash,
    V: Clone,
{
    fn merge(&self, other: &Self) -> Self {
        let mut merged = self.clone();
        for (k, v) in other {
            merged.insert(k.clone(), v.clone());
        }
        merged
    }
}

/// HttpConfig 运行时扩展
pub trait HttpConfigExt {
    /// 合并两个 HttpConfig，`other` 的值优先级更高
    fn merge_with(&self, other: &Self) -> Self;

    /// 创建带有默认值的配置
    fn with_defaults() -> Self;

    /// 获取user_agent，带默认值
    fn user_agent_or_default(&self) -> &str;

    /// 获取timeout，带默认值
    fn timeout_or_default(&self) -> u32;

    /// 获取follow_redirects，带默认值
    fn follow_redirects_or_default(&self) -> bool;
}

impl HttpConfigExt for HttpConfig {
    fn merge_with(&self, other: &Self) -> Self {
        Self {
            user_agent: self.user_agent.merge(&other.user_agent),
            timeout: self.timeout.merge(&other.timeout),
            proxy: self.proxy.merge(&other.proxy),
            headers: match (&self.headers, &other.headers) {
                (Some(base), Some(overlay)) => Some(base.merge(overlay)),
                (None, Some(h)) => Some(h.clone()),
                (Some(h), None) => Some(h.clone()),
                (None, None) => None,
            },
            follow_redirects: self.follow_redirects.merge(&other.follow_redirects),
            max_redirects: self.max_redirects.merge(&other.max_redirects),
            connect_timeout: self.connect_timeout.merge(&other.connect_timeout),
            verify_ssl: self.verify_ssl.merge(&other.verify_ssl),
            request_delay: self.request_delay.merge(&other.request_delay),
            max_concurrent: self.max_concurrent.merge(&other.max_concurrent),
            retry_count: self.retry_count.merge(&other.retry_count),
            retry_delay: self.retry_delay.merge(&other.retry_delay),
        }
    }

    fn with_defaults() -> Self {
        Self {
            user_agent: Some(DEFAULT_USER_AGENT.to_string()),
            timeout: Some(DEFAULT_TIMEOUT),
            follow_redirects: Some(DEFAULT_FOLLOW_REDIRECTS),
            max_redirects: Some(DEFAULT_MAX_REDIRECTS),
            verify_ssl: Some(true),
            ..Default::default()
        }
    }

    fn user_agent_or_default(&self) -> &str {
        self.user_agent.as_deref().unwrap_or(DEFAULT_USER_AGENT)
    }

    fn timeout_or_default(&self) -> u32 {
        self.timeout.unwrap_or(DEFAULT_TIMEOUT)
    }

    fn follow_redirects_or_default(&self) -> bool {
        self.follow_redirects.unwrap_or(DEFAULT_FOLLOW_REDIRECTS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_config_merge() {
        let global = HttpConfig {
            user_agent: Some("GlobalUA".to_string()),
            timeout: Some(60),
            proxy: Some("http://proxy:8080".to_string()),
            ..Default::default()
        };

        let local = HttpConfig {
            timeout: Some(30), // 覆盖全局
            verify_ssl: Some(false),
            ..Default::default()
        };

        let merged = global.merge_with(&local);

        assert_eq!(merged.user_agent, Some("GlobalUA".to_string())); // 保留全局
        assert_eq!(merged.timeout, Some(30)); // 被覆盖
        assert_eq!(merged.proxy, Some("http://proxy:8080".to_string())); // 保留全局
        assert_eq!(merged.verify_ssl, Some(false)); // 新增
    }

    #[test]
    fn test_with_defaults() {
        let config = HttpConfig::with_defaults();
        assert_eq!(config.user_agent_or_default(), DEFAULT_USER_AGENT);
        assert_eq!(config.timeout_or_default(), DEFAULT_TIMEOUT);
        assert!(config.follow_redirects_or_default());
    }
}

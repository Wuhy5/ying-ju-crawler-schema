//! # 运行时构建器
//!
//! 使用构建器模式创建 CrawlerRuntime

use crate::crawler::CrawlerRuntime;
use crate::error::RuntimeError;
use crate::Result;
use crawler_schema::CrawlerRule;
use std::path::Path;

/// 爬虫运行时构建器
///
/// 提供便捷的 API 来创建 CrawlerRuntime 实例
pub struct CrawlerRuntimeBuilder {
    rule: Option<CrawlerRule>,
}

impl CrawlerRuntimeBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self { rule: None }
    }

    /// 从规则对象构建
    pub fn rule(mut self, rule: CrawlerRule) -> Self {
        self.rule = Some(rule);
        self
    }

    /// 从 TOML 文件加载规则
    pub fn rule_file<P: AsRef<Path>>(mut self, path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| RuntimeError::Config(format!("Failed to read rule file: {}", e)))?;

        let rule: CrawlerRule = toml::from_str(&content)
            .map_err(|e| RuntimeError::Config(format!("Failed to parse TOML: {}", e)))?;

        self.rule = Some(rule);
        Ok(self)
    }

    /// 从 TOML 字符串加载规则
    pub fn rule_toml(mut self, toml: &str) -> Result<Self> {
        let rule: CrawlerRule = toml::from_str(toml)
            .map_err(|e| RuntimeError::Config(format!("Failed to parse TOML: {}", e)))?;

        self.rule = Some(rule);
        Ok(self)
    }

    /// 从 JSON 字符串加载规则
    pub fn rule_json(mut self, json: &str) -> Result<Self> {
        let rule: CrawlerRule = serde_json::from_str(json)
            .map_err(|e| RuntimeError::Config(format!("Failed to parse JSON: {}", e)))?;

        self.rule = Some(rule);
        Ok(self)
    }

    /// 构建运行时实例
    pub fn build(self) -> Result<CrawlerRuntime> {
        let rule = self
            .rule
            .ok_or_else(|| RuntimeError::Config("Rule not provided".to_string()))?;

        CrawlerRuntime::new(rule)
    }
}

impl Default for CrawlerRuntimeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let toml = r#"
            [meta]
            name = "Test"
            version = "1.0.0"
            author = "Test Author"

            [detail]
            url = "https://example.com/detail/{{ id }}"

            [search]
            url = "https://example.com/search?q={{ keyword }}"
        "#;

        let runtime = CrawlerRuntimeBuilder::new()
            .rule_toml(toml)
            .unwrap()
            .build();

        assert!(runtime.is_ok());
    }
}

//! 运行时上下文
//!
//! 提供运行时变量存储和配置管理功能。

use std::collections::HashMap;

use serde_json::Value;

use crate::config::HttpConfigExt;
use crawler_schema::HttpConfig;

/// 运行时上下文
///
/// 存储管道执行过程中的变量和配置状态。
#[derive(Debug, Clone, Default)]
pub struct RuntimeContext {
    /// 变量存储
    variables: HashMap<String, Value>,
    /// HTTP 配置（已合并）
    http_config: HttpConfig,
}

impl RuntimeContext {
    /// 创建新的运行时上下文
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            http_config: HttpConfig::with_defaults(),
        }
    }

    /// 使用指定 HTTP 配置创建上下文
    pub fn with_http_config(http_config: HttpConfig) -> Self {
        let merged = HttpConfig::with_defaults().merge_with(&http_config);
        Self {
            variables: HashMap::new(),
            http_config: merged,
        }
    }

    /// 获取变量
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }

    /// 设置变量
    pub fn set(&mut self, name: impl Into<String>, value: Value) {
        self.variables.insert(name.into(), value);
    }

    /// 删除变量
    pub fn remove(&mut self, name: &str) -> Option<Value> {
        self.variables.remove(name)
    }

    /// 检查变量是否存在
    pub fn contains(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }

    /// 获取所有变量（用于模板渲染）
    pub fn variables(&self) -> &HashMap<String, Value> {
        &self.variables
    }

    /// 获取可变变量引用
    pub fn variables_mut(&mut self) -> &mut HashMap<String, Value> {
        &mut self.variables
    }

    /// 获取 HTTP 配置
    pub fn http_config(&self) -> &HttpConfig {
        &self.http_config
    }

    /// 变量数量
    pub fn variable_count(&self) -> usize {
        self.variables.len()
    }

    /// 创建子上下文（继承当前变量，用于循环等场景）
    pub fn child(&self) -> Self {
        Self {
            variables: self.variables.clone(),
            http_config: self.http_config.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_basic() {
        let mut ctx = RuntimeContext::new();

        ctx.set("name", serde_json::json!("test"));
        assert_eq!(ctx.get("name"), Some(&serde_json::json!("test")));
        assert!(ctx.contains("name"));
        assert_eq!(ctx.variable_count(), 1);

        ctx.remove("name");
        assert!(!ctx.contains("name"));
    }

    #[test]
    fn test_context_child() {
        let mut parent = RuntimeContext::new();
        parent.set("inherited", serde_json::json!(true));

        let mut child = parent.child();
        child.set("local", serde_json::json!("child_only"));

        // 子上下文继承父变量
        assert!(child.contains("inherited"));
        assert!(child.contains("local"));

        // 父上下文不受子上下文影响
        assert!(!parent.contains("local"));
    }
}

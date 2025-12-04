//! # 流程状态管理
//!
//! 管理流程执行过程中的状态信息

use std::collections::HashMap;

/// 状态管理器
///
/// 用于跟踪流程执行状态，如当前页码、重试次数等
#[derive(Debug, Clone, Default)]
pub struct StateManager {
    state: HashMap<String, StateValue>,
}

/// 状态值
#[derive(Debug, Clone, PartialEq)]
pub enum StateValue {
    /// 整数
    Int(i64),
    /// 浮点数
    Float(f64),
    /// 字符串
    String(String),
    /// 布尔值
    Bool(bool),
}

impl StateManager {
    /// 创建新的状态管理器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置状态
    pub fn set<K: Into<String>>(&mut self, key: K, value: StateValue) {
        self.state.insert(key.into(), value);
    }

    /// 获取状态
    pub fn get(&self, key: &str) -> Option<&StateValue> {
        self.state.get(key)
    }

    /// 获取整数状态
    pub fn get_int(&self, key: &str) -> Option<i64> {
        self.state.get(key).and_then(|v| match v {
            StateValue::Int(i) => Some(*i),
            _ => None,
        })
    }

    /// 获取字符串状态
    pub fn get_string(&self, key: &str) -> Option<&str> {
        self.state.get(key).and_then(|v| match v {
            StateValue::String(s) => Some(s.as_str()),
            _ => None,
        })
    }

    /// 获取布尔状态
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.state.get(key).and_then(|v| match v {
            StateValue::Bool(b) => Some(*b),
            _ => None,
        })
    }

    /// 增加计数器
    pub fn increment(&mut self, key: &str) -> i64 {
        let current = self.get_int(key).unwrap_or(0);
        let new_value = current + 1;
        self.set(key, StateValue::Int(new_value));
        new_value
    }

    /// 减少计数器
    pub fn decrement(&mut self, key: &str) -> i64 {
        let current = self.get_int(key).unwrap_or(0);
        let new_value = current - 1;
        self.set(key, StateValue::Int(new_value));
        new_value
    }

    /// 合并另一个状态管理器
    pub fn merge(&mut self, other: &StateManager) {
        self.state.extend(other.state.clone());
    }

    /// 清空所有状态
    pub fn clear(&mut self) {
        self.state.clear();
    }

    /// 检查状态是否存在
    pub fn contains(&self, key: &str) -> bool {
        self.state.contains_key(key)
    }

    /// 移除状态
    pub fn remove(&mut self, key: &str) -> Option<StateValue> {
        self.state.remove(key)
    }
}

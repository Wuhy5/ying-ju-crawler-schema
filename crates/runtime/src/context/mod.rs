//! # 执行上下文管理
//!
//! 提供变量存储和流程状态管理

pub mod state;
pub mod variable;

pub use state::StateManager;
pub use variable::VariableStore;

use serde_json::Value;
use std::collections::HashMap;

/// 执行上下文
///
/// 存储运行时变量和状态信息
#[derive(Debug, Clone, Default)]
pub struct Context {
    /// 变量存储
    variables: VariableStore,
    /// 状态管理
    state: StateManager,
}

impl Context {
    /// 创建新的上下文
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置变量
    pub fn set<K: Into<String>>(&mut self, key: K, value: Value) {
        self.variables.set(key, value);
    }

    /// 获取变量
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.variables.get(key)
    }

    /// 获取所有变量（用于模板渲染）
    pub fn all_variables(&self) -> &HashMap<String, Value> {
        self.variables.all()
    }

    /// 获取变量存储的可变引用
    pub fn variables_mut(&mut self) -> &mut VariableStore {
        &mut self.variables
    }

    /// 获取状态管理器
    pub fn state(&self) -> &StateManager {
        &self.state
    }

    /// 获取状态管理器的可变引用
    pub fn state_mut(&mut self) -> &mut StateManager {
        &mut self.state
    }

    /// 合并另一个上下文
    pub fn merge(&mut self, other: &Context) {
        self.variables.merge(&other.variables);
        self.state.merge(&other.state);
    }

    /// 创建子上下文（继承父上下文的变量）
    pub fn child(&self) -> Self {
        Self {
            variables: self.variables.clone(),
            state: StateManager::default(),
        }
    }

    /// 清空上下文
    pub fn clear(&mut self) {
        self.variables.clear();
        self.state.clear();
    }
}

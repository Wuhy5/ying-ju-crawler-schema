//! # 流程上下文
//!
//! 每次流程调用时创建的临时上下文

use super::RuntimeContext;
use crate::Result;
use serde_json::{Map, Value};
use std::sync::Arc;

/// 流程上下文
///
/// 每次流程调用时创建，执行完毕后丢弃。
/// 持有流程变量和对 `RuntimeContext` 的引用。
///
/// # 变量作用域
///
/// - 无前缀变量：先查 Flow，再查 Runtime
/// - `$` 前缀变量：仅查 Runtime 全局变量
///
/// # 示例
///
/// ```rust,ignore
/// let runtime_ctx = Arc::new(RuntimeContext::new(rule)?);
/// let mut flow_ctx = FlowContext::new(runtime_ctx.clone());
///
/// // 设置流程变量
/// flow_ctx.set("keyword", json!("rust"));
/// flow_ctx.set("page", json!(1));
///
/// // 渲染模板
/// let tera_ctx = flow_ctx.to_tera_context()?;
/// let result = runtime_ctx.template_engine().render_str(template, &tera_ctx)?;
/// ```
#[derive(Debug, Clone)]
pub struct FlowContext {
    /// 流程变量
    data: Map<String, Value>,
    /// 运行时上下文引用
    runtime: Arc<RuntimeContext>,
}

impl FlowContext {
    /// 创建新的流程上下文
    pub fn new(runtime: Arc<RuntimeContext>) -> Self {
        Self {
            data: Map::new(),
            runtime,
        }
    }

    /// 设置流程变量
    pub fn set<K: Into<String>>(&mut self, key: K, value: Value) {
        self.data.insert(key.into(), value);
    }

    /// 获取流程变量（仅查 Flow）
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }

    /// 获取变量（先查 Flow，再查 Runtime）
    pub fn resolve(&self, key: &str) -> Option<&Value> {
        self.data
            .get(key)
            .or_else(|| self.runtime.globals().get(key))
    }

    /// 获取运行时上下文
    pub fn runtime(&self) -> &Arc<RuntimeContext> {
        &self.runtime
    }

    /// 获取流程变量 Map
    pub fn data(&self) -> &Map<String, Value> {
        &self.data
    }

    /// 转换为 tera::Context
    ///
    /// 合并两层变量：
    /// 1. 先放 Runtime 全局变量
    /// 2. 再放 Flow 变量（覆盖同名全局变量）
    /// 3. 将全局变量放入 `$` 命名空间，支持 `{{ $.base_url }}` 语法
    pub fn to_tera_context(&self) -> Result<tera::Context> {
        let mut merged = Map::new();

        // 1. 先放 Runtime 全局变量
        for (k, v) in self.runtime.globals() {
            merged.insert(k.clone(), v.clone());
        }

        // 2. 再放 Flow 变量（覆盖同名全局变量）
        for (k, v) in &self.data {
            merged.insert(k.clone(), v.clone());
        }

        // 3. 将全局变量放入 $ 命名空间
        let globals_obj = Value::Object(self.runtime.globals().clone());
        merged.insert("$".to_string(), globals_obj);

        // 使用 from_value 零拷贝转换
        tera::Context::from_value(Value::Object(merged)).map_err(|e| {
            crate::error::RuntimeError::TemplateError {
                error: format!("创建模板上下文失败: {}", e),
            }
        })
    }

    /// 清空流程变量
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// 批量设置流程变量
    pub fn extend<I, K>(&mut self, iter: I)
    where
        I: IntoIterator<Item = (K, Value)>,
        K: Into<String>,
    {
        for (k, v) in iter {
            self.data.insert(k.into(), v);
        }
    }
}

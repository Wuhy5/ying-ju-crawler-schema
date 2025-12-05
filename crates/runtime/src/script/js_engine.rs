//! Boa JavaScript 引擎实现
//!
//! 策略: Boa Context 不支持 Send/Sync (!Send + !Sync)
//! 采用无状态模式 - 每次执行时创建新的 Context

use super::{builtin, context::ScriptContext, engine::ScriptEngine};
use crate::{Result, error::RuntimeError};
use boa_engine::{Context, Source, js_string, object::builtins::JsArray};
use std::time::Duration;

#[derive(Debug)]
pub struct JsScriptEngine {
    /// 执行超时设置
    timeout: Duration,
}

impl JsScriptEngine {
    pub fn new() -> Self {
        Self {
            timeout: Duration::from_secs(5),
        }
    }

    /// 创建新的 Boa Context 并注册内置函数
    fn create_context(&self) -> Result<Context> {
        let mut context = Context::default();

        // 注册内置函数
        builtin::js::register_builtin_functions(&mut context)
            .map_err(|e| RuntimeError::ScriptRuntime(format!("[JS] 注册内置函数失败: {}", e)))?;

        Ok(context)
    }

    /// 将 ScriptContext 中的变量注入到 JS 全局作用域
    fn inject_context(&self, ctx: &mut Context, script_ctx: &ScriptContext) -> Result<()> {
        // 注入 input 变量
        let global = ctx.global_object();
        global
            .set(
                js_string!("input"),
                boa_engine::JsValue::from(js_string!(script_ctx.input.clone())),
                false,
                ctx,
            )
            .map_err(|e| RuntimeError::ScriptRuntime(format!("[JS] 注入 input 失败: {}", e)))?;

        // 注入其他变量
        for (key, value) in &script_ctx.variables {
            let js_value = json_to_js_value(ctx, value)?;
            global
                .set(js_string!(key.clone()), js_value, false, ctx)
                .map_err(|e| {
                    RuntimeError::ScriptRuntime(format!("[JS] 注入变量 {} 失败: {}", key, e))
                })?;
        }

        // 注入 result 别名（指向 input）
        global
            .set(
                js_string!("result"),
                boa_engine::JsValue::from(js_string!(script_ctx.input.clone())),
                false,
                ctx,
            )
            .map_err(|e| RuntimeError::ScriptRuntime(format!("[JS] 注入 result 失败: {}", e)))?;

        Ok(())
    }
}

impl ScriptEngine for JsScriptEngine {
    fn execute(&self, script: &str, context: &ScriptContext) -> Result<String> {
        let mut ctx = self.create_context()?;
        self.inject_context(&mut ctx, context)?;

        let source = Source::from_bytes(script);
        let result = ctx
            .eval(source)
            .map_err(|e| RuntimeError::ScriptRuntime(format!("[JS] {}", e)))?;

        // 将结果转换为字符串
        let result_str = result
            .to_string(&mut ctx)
            .map_err(|e| RuntimeError::ScriptRuntime(format!("[JS] 结果转换失败: {}", e)))?
            .to_std_string_escaped();

        Ok(result_str)
    }

    fn execute_json(&self, script: &str, context: &ScriptContext) -> Result<serde_json::Value> {
        let result = self.execute(script, context)?;
        serde_json::from_str(&result).or(Ok(serde_json::Value::String(result)))
    }

    fn set_timeout(&mut self, duration: Duration) {
        self.timeout = duration;
    }

    fn engine_name(&self) -> &str {
        "javascript"
    }
}

impl Default for JsScriptEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 将 serde_json::Value 转换为 Boa JsValue
fn json_to_js_value(ctx: &mut Context, value: &serde_json::Value) -> Result<boa_engine::JsValue> {
    match value {
        serde_json::Value::Null => Ok(boa_engine::JsValue::null()),
        serde_json::Value::Bool(b) => Ok(boa_engine::JsValue::from(*b)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(boa_engine::JsValue::from(i as i32))
            } else if let Some(f) = n.as_f64() {
                Ok(boa_engine::JsValue::from(f))
            } else {
                Ok(boa_engine::JsValue::null())
            }
        }
        serde_json::Value::String(s) => Ok(boa_engine::JsValue::from(js_string!(s.clone()))),
        serde_json::Value::Array(arr) => {
            let js_arr = JsArray::new(ctx);
            for item in arr {
                let js_item = json_to_js_value(ctx, item)?;
                js_arr.push(js_item, ctx).map_err(|e| {
                    RuntimeError::ScriptRuntime(format!("[JS] 数组操作失败: {}", e))
                })?;
            }
            Ok(js_arr.into())
        }
        serde_json::Value::Object(obj) => {
            let js_obj = boa_engine::JsObject::with_null_proto();
            for (key, val) in obj {
                let js_val = json_to_js_value(ctx, val)?;
                js_obj
                    .set(js_string!(key.clone()), js_val, false, ctx)
                    .map_err(|e| {
                        RuntimeError::ScriptRuntime(format!("[JS] 对象操作失败: {}", e))
                    })?;
            }
            Ok(js_obj.into())
        }
    }
}

//! 脚本执行器 (StepExecutor 实现)
//!
//! 提供脚本步骤的执行能力，支持多种脚本引擎

use crate::{
    Result,
    context::Context,
    error::RuntimeError,
    extractor::{ExtractValue, StepExecutor},
    script::{ScriptContext, ScriptEngine, ScriptEngineFactory, ScriptLanguage},
};
use crawler_schema::script::{Script, ScriptEngine as SchemaScriptEngine, ScriptSource};
use std::{collections::HashMap, sync::Arc};

/// 脚本步骤执行器
///
/// 负责执行提取流程中的脚本步骤，支持：
/// - 多种脚本引擎（Rhai、JavaScript、Lua、Python）
/// - 内联代码、文件引用、URL 引用
/// - 参数传递和上下文变量
pub struct ScriptExecutor {
    /// 脚本配置
    script: Script,
    /// 默认脚本引擎（可被脚本配置覆盖）
    default_engine: Arc<dyn ScriptEngine>,
}

impl ScriptExecutor {
    /// 创建新的脚本执行器
    pub fn new(script: Script) -> Self {
        Self {
            script,
            default_engine: ScriptEngineFactory::create_default(),
        }
    }

    /// 使用自定义默认引擎创建
    pub fn with_engine(script: Script, engine: Arc<dyn ScriptEngine>) -> Self {
        Self {
            script,
            default_engine: engine,
        }
    }

    /// 获取脚本使用的引擎
    fn get_engine(&self) -> Arc<dyn ScriptEngine> {
        // 如果脚本指定了引擎，使用指定的引擎；否则使用默认引擎
        let engine_type = self.script.engine();

        // 检查默认引擎类型是否匹配
        match engine_type {
            SchemaScriptEngine::Rhai => {
                // 如果默认引擎是 Rhai，直接使用
                if self.default_engine.engine_name() == "rhai" {
                    return self.default_engine.clone();
                }
                ScriptEngineFactory::create(ScriptLanguage::Rhai)
            }
            SchemaScriptEngine::JavaScript => {
                if self.default_engine.engine_name() == "javascript" {
                    return self.default_engine.clone();
                }
                ScriptEngineFactory::create(ScriptLanguage::JavaScript)
            }
            SchemaScriptEngine::Lua => {
                if self.default_engine.engine_name() == "lua" {
                    return self.default_engine.clone();
                }
                ScriptEngineFactory::create(ScriptLanguage::Lua)
            }
        }
    }

    /// 加载脚本代码
    fn load_script_code(&self) -> Result<String> {
        match self.script.source() {
            ScriptSource::Code(code) => Ok(code),
            ScriptSource::File(path) => {
                // 从文件加载脚本
                std::fs::read_to_string(&path).map_err(|e| {
                    RuntimeError::ScriptRuntime(format!("无法加载脚本文件 {}: {}", path, e))
                })
            }
            ScriptSource::Url(url) => {
                // URL 加载需要异步，暂时不支持
                Err(RuntimeError::ScriptRuntime(format!(
                    "从 URL 加载脚本暂未实现: {}",
                    url
                )))
            }
        }
    }

    /// 将 ExtractValue 转换为脚本输入字符串
    fn value_to_input(&self, value: &ExtractValue) -> String {
        match value {
            ExtractValue::String(s) => s.clone(),
            ExtractValue::Json(v) => {
                // JSON 值转为字符串
                if let Some(s) = v.as_str() {
                    s.to_string()
                } else {
                    v.to_string()
                }
            }
            ExtractValue::Html(h) => h.clone(),
            ExtractValue::Array(arr) => {
                // 数组序列化为 JSON
                let json_arr: Vec<serde_json::Value> = arr.iter().map(|v| v.as_json()).collect();
                serde_json::to_string(&json_arr).unwrap_or_default()
            }
            ExtractValue::Null => String::new(),
        }
    }

    /// 解析脚本输出为 ExtractValue
    fn parse_output(&self, output: String, input: &ExtractValue) -> ExtractValue {
        // 尝试解析为 JSON
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&output) {
            match json {
                serde_json::Value::String(s) => ExtractValue::String(s),
                serde_json::Value::Array(arr) => {
                    ExtractValue::Array(arr.iter().map(ExtractValue::from_json).collect())
                }
                serde_json::Value::Null => ExtractValue::Null,
                other => ExtractValue::Json(other),
            }
        } else {
            // 如果不是 JSON，根据输入类型决定输出类型
            match input {
                ExtractValue::Html(_) => ExtractValue::Html(output),
                _ => ExtractValue::String(output),
            }
        }
    }
}

impl StepExecutor for ScriptExecutor {
    fn execute(&self, input: ExtractValue, context: &Context) -> Result<ExtractValue> {
        // 1. 加载脚本代码
        let code = self.load_script_code()?;

        // 2. 获取脚本引擎
        let engine = self.get_engine();

        // 3. 转换输入
        let input_str = self.value_to_input(&input);

        // 4. 构建变量上下文
        let mut variables: HashMap<String, serde_json::Value> = HashMap::new();

        // 添加脚本参数
        if let Some(params) = self.script.params() {
            for (key, value) in params {
                variables.insert(key.clone(), value.clone());
            }
        }

        // 添加上下文变量
        for (key, value) in context.variables() {
            variables.insert(key.clone(), value.clone());
        }

        // 5. 创建脚本上下文
        let script_context = ScriptContext::new(input_str, variables);

        // 6. 执行脚本
        let result = engine.execute(&code, &script_context)?;

        // 7. 解析输出
        Ok(self.parse_output(result, &input))
    }
}

// ============================================================================
// 辅助函数
// ============================================================================

/// 执行单个脚本步骤（便捷函数）
///
/// 用于在提取流程外部直接执行脚本
pub fn execute_script(
    script: &Script,
    input: ExtractValue,
    context: &Context,
) -> Result<ExtractValue> {
    let executor = ScriptExecutor::new(script.clone());
    executor.execute(input, context)
}

/// 使用指定引擎执行脚本（便捷函数）
pub fn execute_script_with_engine(
    script: &Script,
    input: ExtractValue,
    context: &Context,
    engine: Arc<dyn ScriptEngine>,
) -> Result<ExtractValue> {
    let executor = ScriptExecutor::with_engine(script.clone(), engine);
    executor.execute(input, context)
}

/// 快速执行简单脚本代码
///
/// 适用于只需要执行一段代码的场景
pub fn execute_code(code: &str, input: &str) -> Result<String> {
    let engine = ScriptEngineFactory::create_default();
    let ctx = ScriptContext::new(input.to_string(), HashMap::new());
    engine.execute(code, &ctx)
}

/// 使用指定引擎快速执行代码
pub fn execute_code_with_engine(
    code: &str,
    input: &str,
    engine: &Arc<dyn ScriptEngine>,
) -> Result<String> {
    let ctx = ScriptContext::new(input.to_string(), HashMap::new());
    engine.execute(code, &ctx)
}

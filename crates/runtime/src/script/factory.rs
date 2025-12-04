//! 脚本引擎工厂

use crate::script::*;
use std::{str::FromStr, sync::Arc};

/// 脚本语言类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScriptLanguage {
    /// Rhai (Rust-like 语法)
    Rhai,
    /// JavaScript (ECMAScript)
    JavaScript,
    /// Lua
    Lua,
    /// Python
    Python,
}

impl ScriptLanguage {
    /// 转为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Rhai => "rhai",
            Self::JavaScript => "javascript",
            Self::Lua => "lua",
            Self::Python => "python",
        }
    }
}

impl FromStr for ScriptLanguage {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rhai" | "rs" => Ok(Self::Rhai),
            "javascript" | "js" | "ecmascript" => Ok(Self::JavaScript),
            "lua" => Ok(Self::Lua),
            "python" | "py" => Ok(Self::Python),
            _ => Err(()),
        }
    }
}

/// 脚本引擎工厂
pub struct ScriptEngineFactory;

impl ScriptEngineFactory {
    /// 创建指定语言的脚本引擎
    pub fn create(language: ScriptLanguage) -> Arc<dyn ScriptEngine> {
        match language {
            ScriptLanguage::Rhai => Arc::new(RhaiScriptEngine::new()),
            ScriptLanguage::JavaScript => Arc::new(JsScriptEngine::new()),
            ScriptLanguage::Lua => Arc::new(LuaScriptEngine::new()),
            ScriptLanguage::Python => {
                Arc::new(PythonScriptEngine::new().expect("Failed to create Python engine"))
            }
        }
    }

    /// 从字符串创建引擎
    pub fn create_from_str(lang: &str) -> Option<Arc<dyn ScriptEngine>> {
        ScriptLanguage::from_str(lang).ok().map(Self::create)
    }

    /// 创建默认引擎 (Rhai)
    pub fn create_default() -> Arc<dyn ScriptEngine> {
        Self::create(ScriptLanguage::Rhai)
    }
}

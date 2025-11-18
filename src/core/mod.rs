//! 核心结构体与顶级规则文件结构

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{CacheConfig, Component, Flow, HttpConfig, Identifier, Meta, ScriptingConfig};

/// 规则文件根结构
/// 这是单个 `.toml | .json` 规则文件的最高层级定义。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RuleFile {
    /// 规则的元数据，用于在软件中识别和展示。
    pub meta: Meta,
    /// 全局的网络请求配置，可被流程局部配置覆盖。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<HttpConfig>,
    /// 脚本引擎的全局配置。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripting: Option<ScriptingConfig>,
    /// 缓存的全局配置。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<CacheConfig>,
    /// 定义可在此规则中复用的“组件”或“函数”。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(with = "HashMap<Identifier, Component>")]
    pub components: Option<HashMap<String, Component>>,
    /// 定义此规则支持的所有可执行流程。
    #[schemars(with = "HashMap<Identifier, Flow>")]
    pub flows: HashMap<String, Flow>,
}

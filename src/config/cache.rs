//! 缓存配置 (CacheConfig)

use crate::CacheBackend;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// 缓存配置 (CacheConfig)
/// 定义规则的缓存策略。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CacheConfig {
    /// 缓存后端类型。
    pub backend: CacheBackend,
    /// 默认的缓存过期时间（秒）。
    pub default_ttl: u32,
}

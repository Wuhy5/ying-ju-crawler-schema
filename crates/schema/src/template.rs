//! 模板字符串类型定义
//!
//! 本模块仅包含模板字符串的 Schema 定义，不包含运行时逻辑。
//! 运行时渲染和验证功能请使用 `crawler_runtime::template` 模块。

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// 模板字符串 (Template)
///
/// 支持 Tera 模板语法的字符串类型，用于在运行时进行变量插值。
///
/// # 变量作用域
///
/// 模板支持两层变量作用域：
///
/// - **流程变量 (Flow)**：每次流程调用时注入的临时变量，如 `keyword`、`page`、`url`
/// - **全局变量 (Runtime)**：爬虫实例级的只读配置，如 `base_url`、`domain`
///
/// ## 变量查找规则
///
/// | 写法 | 查找逻辑 | 示例 |
/// |------|---------|------|
/// | `{{ var }}` | 先查 Flow，再查 Runtime | `{{ keyword }}`、`{{ page }}` |
/// | `{{ $.var }}` | 仅查 Runtime 全局变量 | `{{ $.base_url }}`、`{{ $.domain }}` |
///
/// **注意**：Flow 变量优先级高于 Runtime，同名时 Flow 变量会覆盖 Runtime 变量。
/// 使用 `$` 前缀可以强制访问 Runtime 全局变量。
///
/// ## 各流程可用的 Flow 变量
///
/// | 流程 | 自动注入变量 |
/// |------|------------|
/// | search | `keyword`、`page` |
/// | discovery | `page`、各筛选器的 `key` |
/// | detail | `url`（即 detail_url） |
/// | content | `url`（即 chapter_url 或 play_url） |
///
/// ## Runtime 全局变量
///
/// 以下变量在所有流程中通过 `$` 前缀访问：
///
/// - `$.base_url` - 目标网站的基础 URL
/// - `$.domain` - 目标网站的域名
///
/// # 语法规范
///
/// - 变量插值: `{{ variable }}`
/// - 嵌套访问: `{{ user.name }}`、`{{ items[0] }}`
/// - 全局访问: `{{ $.base_url }}`
/// - 过滤器: `{{ name | upper }}`
/// - 条件: `{% if condition %}...{% endif %}`
/// - 循环: `{% for item in items %}...{% endfor %}`
///
/// # 示例
///
/// ```toml
/// # 搜索 URL - 使用 Flow 变量
/// url = "{{ $.base_url }}/search?q={{ keyword }}&page={{ page }}"
///
/// # 详情 URL - 直接使用传入的 url
/// url = "{{ url }}"
///
/// # 混合使用 - 强制使用全局 base_url
/// url = "{{ $.base_url }}{{ url }}"
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, Default)]
#[serde(transparent)]
pub struct Template(
    #[schemars(pattern(
        "{{\\s*([a-zA-Z_][a-zA-Z0-9_]*(?:\\.[a-zA-Z_][a-zA-Z0-9_]*|\\[[0-9]+\\])*)\\s*}}"
    ))]
    String,
);

impl Template {
    /// 创建新模板
    #[inline]
    pub fn new(template: impl Into<String>) -> Self {
        Self(template.into())
    }

    /// 获取原始字符串
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// 转换为内部字符串
    #[inline]
    pub fn into_string(self) -> String {
        self.0
    }

    /// 检查是否为空
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<String> for Template {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for Template {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl AsRef<str> for Template {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Template {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

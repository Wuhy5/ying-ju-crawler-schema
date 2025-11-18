//! 管道与步骤 (Pipeline & Step)

use crate::{ExtractType, HttpMethod, Identifier};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 管道 (Pipeline)
/// 一个由多个步骤组成的执行序列。
pub type Pipeline = Vec<Step>;

/// 步骤 (Step)
/// 管道中的一个原子操作单元。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Step {
    // --- 核心操作 ---
    /// **HTTP请求**: 发起网络请求。
    HttpRequest(StepHttpRequest),
    /// **CSS选择器**: 从HTML中提取单个元素。
    Selector(StepSelector),
    /// **CSS选择器(全部)**: 从HTML中提取所有匹配的元素数组。
    SelectorAll(StepSelectorAll),
    /// **JSONPath**: 从JSON数据中提取信息。
    JsonPath(StepJsonPath),
    /// **执行脚本**: 调用脚本模块中的函数。
    Script(StepScript),
    /// **调用组件**: 执行一个在 `[components]` 中定义的组件。
    Call(StepCall),

    // --- 数据处理与转换 ---
    /// **字符串操作: 模板**: 使用变量格式化字符串。
    StringTemplate(StepStringTemplate),
    /// **设置常量**: 创建一个值为常量的变量。
    Constant(StepConstant),
    /// **字段映射**: 将解析层字段映射到渲染层标准模型。
    MapField(StepMapField),

    // --- 控制流 ---
    /// **循环: ForEach**: 遍历数组中的每一项并执行子管道。
    LoopForEach(StepLoopForEach),

    // --- 调试 ---
    /// **日志输出**: 打印调试信息。
    Log(StepLog),
}

/// HTTP请求步骤 (StepHttpRequest)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StepHttpRequest {
    /// 要请求的URL。
    /// 模板字符串，详见顶部规范说明。
    pub url: String,
    /// 将包含响应体、状态码、头的响应对象存入此变量。
    #[schemars(with = "Identifier")]
    pub output: String,
    /// HTTP方法。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<HttpMethod>,
    /// POST请求体。
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 模板字符串，详见顶部规范说明。
    pub body: Option<String>,
    /// 本次请求的自定义头。
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 请求头模板字符串，详见顶部规范说明。
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StepSelector {
    /// 输入的HTML字符串。
    /// 模板字符串，详见顶部规范说明。
    pub input: String,
    /// CSS选择器表达式。
    /// 模板字符串，详见顶部规范说明。
    pub query: String,
    /// 提取方式。
    pub extract: ExtractType,
    /// 将提取的结果存入此变量。
    #[schemars(with = "Identifier")]
    pub output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StepSelectorAll {
    /// 输入的HTML字符串。
    /// 模板字符串，详见顶部规范说明。
    pub input: String,
    /// CSS选择器表达式。
    /// 模板字符串，详见顶部规范说明。
    pub query: String,
    /// 提取方式。
    pub extract: ExtractType,
    /// 将提取的字符串数组存入此变量。
    #[schemars(with = "Identifier")]
    pub output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StepJsonPath {
    /// 输入的JSON字符串或对象。
    /// 模板字符串，详见顶部规范说明。
    pub input: String,
    /// JSONPath 查询表达式。
    /// 模板字符串，详见顶部规范说明。
    pub query: String,
    /// 将提取结果存入此变量。
    #[schemars(with = "Identifier")]
    pub output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StepScript {
    /// 要调用的函数，格式为 `模块名.函数名`。
    #[schemars(regex(pattern = r"^[a-zA-Z_][a-zA-Z0-9_]*\.[a-zA-Z_][a-zA-Z0-9_]*$"))]
    pub call: String,
    /// 传递给函数的参数。这是一个动态值，允许在模板中构建复杂的JSON对象。
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 模板字符串，详见顶部规范说明。
    pub with: Option<String>,
    /// 将函数返回值存入此变量。
    #[schemars(with = "Identifier")]
    pub output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StepCall {
    /// 要调用的组件名，必须在 `[components]` 中定义。
    #[schemars(with = "Identifier")]
    pub component: String,
    /// 传递给组件的输入参数。
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 输入参数模板字符串，详见顶部规范说明。
    pub with: Option<HashMap<String, String>>,
    /// 将组件的输出（一个包含所有输出变量的Map）存入此变量。
    #[schemars(with = "Identifier")]
    pub output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StepStringTemplate {
    /// 模板字符串。
    #[serde(rename = "template")]
    pub template_str: String,
    /// 输出结果变量名。
    #[schemars(with = "Identifier")]
    pub output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StepLoopForEach {
    /// 要遍历的数组变量。
    /// 模板字符串，详见顶部规范说明。
    pub input: String,
    /// 在循环中，当前项的变量名。
    #[serde(rename = "as")]
    #[schemars(with = "Identifier")]
    pub r#as: String,
    /// 对每一项执行的子管道。
    pub pipeline: Pipeline,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StepConstant {
    /// 常量值。
    pub value: serde_json::Value,
    /// 将此常量存入的变量名。
    #[schemars(with = "Identifier")]
    pub output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StepLog {
    /// 要打印的消息。
    /// 模板字符串，详见顶部规范说明。
    pub message: String,
}

/// 字段映射步骤 (StepMapField)
/// 用于将解析层的自由格式字段映射到渲染层的标准化模型。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StepMapField {
    /// 输入数据的变量名或模板字符串。
    /// 模板字符串，详见顶部规范说明。
    pub input: String,
    /// 目标模型类型: "item_summary" 或 "item_detail"
    #[schemars(regex(pattern = "^(item_summary|item_detail)$"))]
    pub target: String,
    /// 字段映射规则列表。
    pub mappings: Vec<FieldMapping>,
    /// 将映射结果存入此变量。
    #[schemars(with = "Identifier")]
    pub output: String,
}

/// 字段映射规则 (FieldMapping)
/// 定义源字段到目标字段的映射关系。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FieldMapping {
    /// 源字段路径，支持点号访问嵌套字段，如 "data.title" 或简单字段名 "title"
    pub from: String,
    /// 目标字段名，必须是目标模型中的有效字段名
    /// 如 ItemSummary: id, title, url, media_type, cover, summary, tags, meta
    /// 如 ItemDetail: id, title, url, media_type, cover, description, metadata, tags, content
    pub to: String,
    /// 可选的转换函数（预留），如 "to_lowercase", "trim", "parse_int" 等
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform: Option<String>,
}

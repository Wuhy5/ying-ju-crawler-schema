//! 流程与组件 (Flow & Component)

use crate::{Identifier, pipeline::Pipeline};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 可重用组件 (Component)
/// 一个可被其他管道调用的、封装了特定逻辑的子管道。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Component {
    /// 组件的功能描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 定义组件接收的输入参数 (key: 参数名, value: 默认值)。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(with = "HashMap<Identifier, serde_json::Value>")]
    pub inputs: Option<HashMap<String, serde_json::Value>>,
    /// 组件的核心处理管道。
    pub pipeline: Pipeline,
}

/// 流程 (Flow)
/// 定义一个完整的、可独立执行的任务,如"搜索"、"发现"或"登录"。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Flow {
    /// 流程的功能描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 流程的入口点定义，通常用于构建初始URL。
    pub entry: EntryPoint,
    /// 流程的核心动作管道。
    pub actions: Pipeline,
    /// 输出模型类型（可选），指定流程输出的数据格式。
    /// 可选值: "item_summary" (列表页), "item_detail" (详情页)
    /// 如果未指定，则输出为自由格式的解析结果。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(regex(pattern = "^(item_summary|item_detail)$"))]
    pub output_model: Option<String>,
}

/// ## 入口点 (EntryPoint)
/// 定义流程的起始方式，取代了原有的 `flow_type` 字符串。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum EntryPoint {
    /// **发现/浏览类型**: 用于需要复杂分类和筛选条件的场景。
    Discover {
        /// URL模板。占位符的名称应与 `filters` 中定义的 `key` 相对应。
        /// 例如: `"/list/{{cate_id}}-{{area}}-{{year}}.html?sort_by={{sort}}"`
        url: String,
        /// 定义此发现页面的所有筛选器组，UI将根据此结构动态生成筛选面板。
        filters: Vec<FilterGroup>,
    },
    /// **搜索类型**: 用于简单的关键词搜索场景。
    Search {
        /// URL模板，必须包含 `{{keyword}}` 占位符。
        url: String,
    },
    /// **通用类型**: 用于没有特定UI入口的流程，如“登录”、“签到”等。
    General {
        /// 一个固定的或简单的模板URL。
        url: String,
    },
}

/// ## 筛选器组 (FilterGroup)
/// 代表UI上一组相关的筛选选项，如“地区”、“年份”。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FilterGroup {
    /// 筛选器组的显示名称，如 "按类型"。
    pub name: String,
    /// 此筛选器组在URL模板中对应的键 (`key`)。
    /// 例如，如果 `key` 是 `"cate_id"`，则UI会将用户选择的值替换到URL的 `{{cate_id}}` 位置。
    #[schemars(with = "Identifier")]
    pub key: String,
    /// 是否允许多选。
    #[serde(default)]
    pub multiselect: bool,
    /// 此筛选器组下所有可用的选项。
    pub options: Vec<FilterOption>,
}

/// ## 筛选器选项 (FilterOption)
/// 代表一个具体的筛选选项，如“电影”或“2023年”。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FilterOption {
    /// 选项的显示名称，如 "美国"。
    pub name: String,
    /// 选项的值，将用于替换URL模板中对应的 `key`。
    /// 例如，如果 `key` 是 `"area"`，此 `value` 可能是 `"USA"`。
    pub value: String,
}

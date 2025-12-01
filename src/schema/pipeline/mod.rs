//! 管道与步骤 (Pipeline & Step)
//!
//! 本模块仅包含管道和步骤的数据结构定义。
//! 运行时验证逻辑请使用 `crate::runtime` 模块。

pub mod steps;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub use steps::*;

use crate::schema::template::Template;

/// 步骤Trait (StepTrait)
/// 为步骤提供统一的元数据接口。
/// 运行时验证逻辑请使用 `crate::runtime` 模块中的扩展 trait。
pub trait StepTrait {
    /// 步骤的唯一名称标识
    fn name(&self) -> &'static str;

    /// 步骤的人类可读描述
    fn description(&self) -> &'static str {
        "未提供描述"
    }

    /// 步骤的类别（用于分组显示）
    fn category(&self) -> StepCategory {
        StepCategory::Other
    }

    /// 获取此步骤使用的所有模板（用于静态分析）
    fn templates(&self) -> Vec<&Template> {
        Vec::new()
    }

    /// 获取此步骤的输出变量名
    fn output_variable(&self) -> Option<&str> {
        None
    }
}

/// 步骤类别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum StepCategory {
    /// 核心操作（HTTP请求、选择器等）
    Core,
    /// 数据处理（模板、常量、映射等）
    Data,
    /// 控制流（循环、条件等）
    Control,
    /// 缓存操作
    Cache,
    /// 调试工具
    Debug,
    /// 其他
    Other,
}

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
    HttpRequest(steps::StepHttpRequest),
    /// **CSS选择器**: 从HTML中提取单个元素。
    Selector(steps::StepSelector),
    /// **CSS选择器(全部)**: 从HTML中提取所有匹配的元素数组。
    SelectorAll(steps::StepSelectorAll),
    /// **JSONPath**: 从JSON数据中提取信息。
    JsonPath(steps::StepJsonPath),
    /// **执行脚本**: 调用脚本模块中的函数。
    Script(steps::StepScript),
    /// **调用组件**: 执行一个在 `[components]` 中定义的组件。
    Call(steps::StepCall),

    // --- 数据处理与转换 ---
    /// **字符串操作: 模板**: 使用变量格式化字符串。
    StringTemplate(steps::StepStringTemplate),
    /// **设置常量**: 创建一个值为常量的变量。
    Constant(steps::StepConstant),
    /// **字段映射**: 将解析层字段映射到渲染层标准模型。
    MapField(steps::StepMapField),

    // --- 缓存操作 ---
    /// **缓存获取**: 从缓存中获取值。
    CacheGet(steps::StepCacheGet),
    /// **缓存设置**: 将值存入缓存。
    CacheSet(steps::StepCacheSet),

    // --- 控制流 ---
    /// **循环: ForEach**: 遍历数组中的每一项并执行子管道。
    LoopForEach(steps::StepLoopForEach),

    // --- 调试 ---
    /// **日志输出**: 打印调试信息。
    Log(steps::StepLog),
}

impl Step {
    /// 获取步骤的内部trait实现
    fn as_trait(&self) -> &dyn StepTrait {
        match self {
            Step::HttpRequest(s) => s,
            Step::Selector(s) => s,
            Step::SelectorAll(s) => s,
            Step::JsonPath(s) => s,
            Step::Script(s) => s,
            Step::Call(s) => s,
            Step::StringTemplate(s) => s,
            Step::Constant(s) => s,
            Step::MapField(s) => s,
            Step::CacheGet(s) => s,
            Step::CacheSet(s) => s,
            Step::LoopForEach(s) => s,
            Step::Log(s) => s,
        }
    }
}

impl StepTrait for Step {
    fn name(&self) -> &'static str {
        self.as_trait().name()
    }

    fn description(&self) -> &'static str {
        self.as_trait().description()
    }

    fn category(&self) -> StepCategory {
        self.as_trait().category()
    }

    fn templates(&self) -> Vec<&Template> {
        self.as_trait().templates()
    }

    fn output_variable(&self) -> Option<&str> {
        self.as_trait().output_variable()
    }
}

// PipelineExt trait 已移到 runtime 模块

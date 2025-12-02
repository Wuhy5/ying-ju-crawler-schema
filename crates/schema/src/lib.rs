//! # 影视软件爬虫规则规范 - Schema 定义
//!
//! 该 crate 提供爬虫规则的纯数据结构定义，用于 JSON Schema 生成和序列化。
//!
//! ## 核心设计理念
//!
//! 1. **全流程化 (Step-Based)**：所有数据提取都通过显式步骤定义，无隐式 DSL。
//! 2. **原子化步骤 (Atomic Steps)**：每个 `ExtractStep` 只执行一个最小化的操作。
//! 3. **字段驱动 (Field-Driven)**：每个输出字段直接关联一个 `FieldExtractor`，定义其提取流程。
//! 4. **流程驱动 (Flow-Driven)**：以用户可自定义的"流程 (`Flow`)"为核心组织功能。
//!
//! ## 提取步骤类型
//!
//! - **选择步骤**：`css`、`json`、`xpath`、`regex` - 从文档提取数据
//! - **过滤步骤**：`filter`、`attr`、`index` - 转换和过滤数据
//! - **特殊步骤**：`const`、`var`、`script` - 常量、变量、脚本
//!
//! ## 模板字符串规范
//!
//! 所有输入字段均采用模板字符串格式，支持变量插值。
//!
//! - 语法：`{{ variable }}` 用于插入变量值。
//! - 示例：
//!   - `https://example.com/search?q={{ keyword }}`
//!   - `User-Agent: MyBot/{{ version }}`
//! - 支持表达式与嵌套：如 `{{ user.name }}`、`{{ items[0] }}`

// 通用错误模块
pub mod error;

// Schema 模块
pub mod config;
pub mod core;
pub mod extract;
pub mod fields;
pub mod flow;
pub mod script;
pub mod template;

// 重新导出常用类型
pub use config::*;
pub use core::*;
pub use error::SchemaError;
pub use extract::*;
pub use fields::*;
pub use flow::*;
pub use script::{ScriptConfig, ScriptEngine, ScriptSource, ScriptStep};
pub use template::Template;

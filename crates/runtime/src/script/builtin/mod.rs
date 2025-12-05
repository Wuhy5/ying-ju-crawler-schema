//! 内置函数子模块
//!
//! 架构设计：
//! - `core`: 纯 Rust 实现的所有内置函数，与脚本引擎无关
//! - `rhai`/`js`/`lua`/`python`: 各引擎的适配器，将 core 函数绑定到引擎 API

/// 内置函数核心实现（纯 Rust）
pub mod core;

/// Rhai 引擎适配器
pub mod rhai;

/// JavaScript 引擎适配器
pub mod js;

/// Lua 引擎适配器
pub mod lua;

/// Python 引擎适配器
pub mod python;

// 重新导出核心函数供外部使用
pub use core::*;

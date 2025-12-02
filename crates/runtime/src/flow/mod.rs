//! # 流程执行器模块
//!
//! 实现各种流程的执行逻辑

pub mod content;
pub mod detail;
pub mod discovery;
pub mod executor;
pub mod login;
pub mod search;

pub use executor::FlowExecutor;

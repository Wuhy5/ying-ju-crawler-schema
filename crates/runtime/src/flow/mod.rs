//! # 流程执行器模块
//!
//! 实现各种流程的执行逻辑
//!
//! # 流程类型
//!
//! - `search` - 搜索流程，支持分页
//! - `discovery` - 发现流程，支持筛选和分页
//! - `detail` - 详情流程
//! - `content` - 内容流程
//! - `login` - 登录流程

pub mod content;
pub mod detail;
pub mod discovery;
pub mod executor;
pub mod login;
pub mod pager;
pub mod search;

pub use executor::FlowExecutor;
pub use pager::{
    DiscoveryPager,
    DiscoveryPagerState,
    Pager,
    PagerState,
    SearchPager,
    SearchPagerState,
};

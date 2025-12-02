//! 流程与组件 (Flow & Component)
//!
//! 定义不同类型的流程：
//! - LoginFlow: 登录流程（Pipeline 驱动）
//! - DiscoveryFlow: 发现页流程（筛选和分页）
//! - DetailFlow: 详情页流程（字段驱动）
//! - SearchFlow: 搜索流程（字段驱动）
//! - ContentFlow: 内容页流程（播放页、阅读页）
//! - Component: 可重用组件

pub mod common;
pub mod component;
pub mod content;
pub mod detail;
pub mod discovery;
pub mod login;
pub mod search;

// 重新导出所有公开类型
pub use common::*;
pub use component::*;
pub use content::*;
pub use detail::*;
pub use discovery::*;
pub use login::*;
pub use search::*;

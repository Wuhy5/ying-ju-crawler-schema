//! # 执行上下文管理
//!
//! 提供双层上下文架构：
//!
//! - **RuntimeContext**：爬虫实例级，持有全局变量和共享资源
//! - **FlowContext**：流程级临时上下文，每次流程调用创建并在完成后销毁
//!
//! # 变量作用域
//!
//! | 写法 | 查找逻辑 | 示例 |
//! |------|---------|------|
//! | `{{ var }}` | 先查 Flow，再查 Runtime | `{{ keyword }}`、`{{ page }}` |
//! | `{{ $.var }}` | 仅查 Runtime 全局变量 | `{{ $.base_url }}`、`{{ $.domain }}` |

pub mod flow;
pub mod runtime;

pub use flow::FlowContext;
pub use runtime::RuntimeContext;

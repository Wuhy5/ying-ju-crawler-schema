//! # 人机验证/反爬处理模块
//!
//! 提供验证检测和处理功能，支持多种验证类型：
//! - Cloudflare (JS Challenge, Turnstile, Under Attack Mode)
//! - reCAPTCHA v2/v3
//! - hCaptcha
//! - 自定义验证
//!
//! ## 架构设计
//!
//! ```text
//! HTTP Response
//!      ↓
//! ChallengeDetector (检测是否为验证页面)
//!      ↓
//! ChallengeHandler (处理验证)
//!      ↓ (WebView/Retry/Cookie/External)
//! 验证凭证 (Cookie, Headers)
//! ```

mod detector;
mod handler;
mod manager;

pub use detector::*;
pub use handler::*;
pub use manager::*;

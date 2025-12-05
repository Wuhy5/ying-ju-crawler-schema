//! # 登录流程执行器

use crate::{
    Result,
    context::{FlowContext, RuntimeContext},
};
use crawler_schema::flow::LoginFlow;

/// 登录请求
#[derive(Debug, Clone)]
pub struct LoginRequest {
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
}

/// 登录响应
#[derive(Debug, Clone)]
pub struct LoginResponse {
    /// 是否成功
    pub success: bool,
    /// 会话信息
    pub session: Option<serde_json::Value>,
}

/// 登录流程执行器
pub struct LoginFlowExecutor;

impl LoginFlowExecutor {
    /// 执行登录流程
    pub async fn execute(
        input: LoginRequest,
        flow: &LoginFlow,
        _runtime_context: &RuntimeContext,
        flow_context: &mut FlowContext,
    ) -> Result<LoginResponse> {
        // 设置上下文变量
        flow_context.set("username", serde_json::json!(input.username));
        flow_context.set("password", serde_json::json!(input.password));

        // TODO: 实现登录流程
        let _ = flow;

        Ok(LoginResponse {
            success: false,
            session: None,
        })
    }
}

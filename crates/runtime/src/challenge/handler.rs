//! # 验证处理器
//!
//! 处理检测到的人机验证，支持多种策略

use super::{ChallengeType, DetectionResult, ResponseContext};
use crate::{
    Result,
    RuntimeError,
    webview::{SharedWebViewProvider, WebViewCloseReason, WebViewRequest},
};
use crawler_schema::config::{
    CaptchaProvider,
    ChallengeHandler,
    CookieHandler,
    CookieSource,
    ExternalHandler,
    RetryHandler,
    ScriptHandler,
    WebviewHandler,
};
use std::{collections::HashMap, time::Duration};
use tokio::sync::RwLock;

/// 验证凭证
#[derive(Debug, Clone, Default)]
pub struct ChallengeCredentials {
    /// Cookie 凭证
    pub cookies: HashMap<String, String>,
    /// Header 凭证
    pub headers: HashMap<String, String>,
    /// 其他数据（如 token）
    pub extra: HashMap<String, String>,
    /// 凭证获取时间
    pub obtained_at: Option<std::time::Instant>,
    /// 凭证有效期（秒）
    pub ttl_seconds: Option<u32>,
}

impl ChallengeCredentials {
    /// 创建新凭证
    pub fn new() -> Self {
        Self {
            obtained_at: Some(std::time::Instant::now()),
            ..Default::default()
        }
    }

    /// 添加 Cookie
    pub fn with_cookie(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.cookies.insert(name.into(), value.into());
        self
    }

    /// 添加多个 Cookie
    pub fn with_cookies(mut self, cookies: HashMap<String, String>) -> Self {
        self.cookies.extend(cookies);
        self
    }

    /// 添加 Header
    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }

    /// 设置 TTL
    pub fn with_ttl(mut self, ttl_seconds: u32) -> Self {
        self.ttl_seconds = Some(ttl_seconds);
        self
    }

    /// 检查凭证是否过期
    pub fn is_expired(&self) -> bool {
        if let (Some(obtained_at), Some(ttl)) = (self.obtained_at, self.ttl_seconds) {
            obtained_at.elapsed() > Duration::from_secs(ttl as u64)
        } else {
            false
        }
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.cookies.is_empty() && self.headers.is_empty() && self.extra.is_empty()
    }

    /// 转换为 Cookie 字符串
    pub fn to_cookie_string(&self) -> String {
        self.cookies
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("; ")
    }
}

/// 验证处理器执行上下文
pub struct HandlerContext {
    /// WebView 提供者
    pub webview_provider: SharedWebViewProvider,
    /// 原始请求 URL
    pub url: String,
    /// 检测结果
    pub detection: DetectionResult,
    /// 响应上下文
    pub response: ResponseContext,
    /// HTTP 客户端（用于重试）
    pub http_client: Option<reqwest::Client>,
}

/// 验证处理器 trait
pub trait ChallengeHandlerExt {
    /// 处理验证
    fn handle(
        &self,
        ctx: &HandlerContext,
    ) -> impl std::future::Future<Output = Result<ChallengeCredentials>> + Send;
}

impl ChallengeHandlerExt for ChallengeHandler {
    async fn handle(&self, ctx: &HandlerContext) -> Result<ChallengeCredentials> {
        match self {
            ChallengeHandler::Webview(config) => handle_webview(config, ctx).await,
            ChallengeHandler::Retry(config) => handle_retry(config, ctx).await,
            ChallengeHandler::Cookie(config) => handle_cookie(config, ctx).await,
            ChallengeHandler::External(config) => handle_external(config, ctx).await,
            ChallengeHandler::Script(config) => handle_script(config, ctx).await,
        }
    }
}

// ============================================================================
// WebView 处理器
// ============================================================================

async fn handle_webview(
    config: &WebviewHandler,
    ctx: &HandlerContext,
) -> Result<ChallengeCredentials> {
    let request = build_webview_request(config, ctx);

    let response = ctx.webview_provider.open(request).await?;

    if !response.success {
        return match response.close_reason {
            WebViewCloseReason::Timeout => Err(RuntimeError::WebViewTimeout),
            WebViewCloseReason::UserClosed => Err(RuntimeError::WebViewUserClosed),
            _ => Err(RuntimeError::ChallengeFailed(
                response.error.unwrap_or_else(|| "验证失败".to_string()),
            )),
        };
    }

    let mut credentials = ChallengeCredentials::new().with_cookies(response.cookies);

    // 如果有脚本结果，解析额外数据
    if let Some(script_result) = response.script_result
        && let Ok(extra) = serde_json::from_str::<HashMap<String, String>>(&script_result)
    {
        credentials.extra = extra;
    }

    Ok(credentials)
}

fn build_webview_request(config: &WebviewHandler, ctx: &HandlerContext) -> WebViewRequest {
    let mut request = WebViewRequest::new(&ctx.url)
        .with_timeout(Duration::from_secs(config.timeout_seconds as u64));

    if let Some(title) = &config.tip {
        request = request.with_title(title);
    }

    if let Some(ua) = &config.user_agent {
        request = request.with_user_agent(ua);
    }

    if let Some(check) = &config.success_check {
        request = request.with_success_check(check);
    }

    if let Some(interval) = config.check_interval_ms {
        request = request.with_check_interval(Duration::from_millis(interval as u64));
    }

    if let Some(cookies) = &config.extract_cookies {
        request = request.with_extract_cookies(cookies.clone());
    }

    request
}

// ============================================================================
// 重试处理器
// ============================================================================

async fn handle_retry(config: &RetryHandler, ctx: &HandlerContext) -> Result<ChallengeCredentials> {
    let client = ctx
        .http_client
        .as_ref()
        .ok_or_else(|| RuntimeError::ChallengeFailed("重试处理需要 HTTP 客户端".to_string()))?;

    let mut delay = config.delay_ms;
    let backoff = config.backoff_factor.unwrap_or(1.5);

    for attempt in 0..config.max_retries {
        // 等待
        tokio::time::sleep(Duration::from_millis(delay as u64)).await;

        // 重试请求
        let response = client
            .get(&ctx.url)
            .send()
            .await
            .map_err(|e| RuntimeError::HttpRequest(e.to_string()))?;

        // 检查是否仍然是验证页面
        let status = response.status().as_u16();
        let body = response
            .text()
            .await
            .map_err(|e| RuntimeError::HttpRequest(e.to_string()))?;

        // 简单检查：如果状态码变为 200 且不包含验证特征，认为成功
        if status == 200 && !contains_challenge_patterns(&body) {
            // 成功绕过，但没有额外凭证
            return Ok(ChallengeCredentials::new());
        }

        // 增加延迟
        delay = (delay as f32 * backoff) as u32;

        tracing::debug!("重试验证第 {} 次失败，下次等待 {}ms", attempt + 1, delay);
    }

    Err(RuntimeError::ChallengeMaxAttempts {
        attempts: config.max_retries,
    })
}

/// 检查是否包含验证特征
fn contains_challenge_patterns(body: &str) -> bool {
    const PATTERNS: &[&str] = &[
        "Just a moment",
        "Checking your browser",
        "g-recaptcha",
        "h-captcha",
        "cf-please-wait",
    ];

    PATTERNS.iter().any(|p| body.contains(p))
}

// ============================================================================
// Cookie 处理器
// ============================================================================

async fn handle_cookie(
    config: &CookieHandler,
    _ctx: &HandlerContext,
) -> Result<ChallengeCredentials> {
    let cookies = match &config.source {
        CookieSource::UserInput { tip, cookie_names } => {
            // 这里应该触发用户输入，但在 runtime 层面我们只能返回错误
            // 实际应用中，这应该通过 WebView 或其他 UI 机制处理
            return Err(RuntimeError::ChallengeFailed(format!(
                "需要用户输入 Cookie: {:?}. {}",
                cookie_names,
                tip.as_deref().unwrap_or("")
            )));
        }
        CookieSource::Config { cookies } => parse_cookie_string(cookies),
        CookieSource::Script(_script) => {
            // TODO: 执行脚本获取 Cookie
            return Err(RuntimeError::ChallengeFailed(
                "Cookie 脚本暂未实现".to_string(),
            ));
        }
    };

    Ok(ChallengeCredentials::new().with_cookies(cookies))
}

/// 解析 Cookie 字符串
fn parse_cookie_string(cookie_str: &str) -> HashMap<String, String> {
    let mut cookies = HashMap::new();

    for part in cookie_str.split(';') {
        let part = part.trim();
        if let Some((name, value)) = part.split_once('=') {
            cookies.insert(name.trim().to_string(), value.trim().to_string());
        }
    }

    cookies
}

// ============================================================================
// 外部服务处理器
// ============================================================================

async fn handle_external(
    config: &ExternalHandler,
    ctx: &HandlerContext,
) -> Result<ChallengeCredentials> {
    // 获取必要信息
    let site_key = ctx
        .detection
        .extra_info
        .get("site_key")
        .ok_or_else(|| RuntimeError::ChallengeFailed("缺少 site_key".to_string()))?;

    let challenge_type = ctx
        .detection
        .challenge_type
        .as_ref()
        .ok_or_else(|| RuntimeError::ChallengeFailed("未知验证类型".to_string()))?;

    // 根据提供商调用 API
    let token = match config.provider {
        CaptchaProvider::TwoCaptcha => {
            solve_with_2captcha(config, &ctx.url, site_key, challenge_type).await?
        }
        CaptchaProvider::AntiCaptcha => {
            solve_with_anticaptcha(config, &ctx.url, site_key, challenge_type).await?
        }
        CaptchaProvider::CapSolver => {
            solve_with_capsolver(config, &ctx.url, site_key, challenge_type).await?
        }
        CaptchaProvider::Custom => {
            return Err(RuntimeError::ChallengeFailed(
                "自定义打码服务需要自行实现".to_string(),
            ));
        }
    };

    // 根据验证类型返回不同格式的凭证
    let mut credentials = ChallengeCredentials::new();
    credentials.extra.insert("token".to_string(), token);

    Ok(credentials)
}

async fn solve_with_2captcha(
    config: &ExternalHandler,
    page_url: &str,
    site_key: &str,
    challenge_type: &ChallengeType,
) -> Result<String> {
    let api_key: &str = config.api_key.as_str();
    let endpoint = config.endpoint.as_deref().unwrap_or("https://2captcha.com");

    let method = match challenge_type {
        ChallengeType::RecaptchaV2 => "userrecaptcha",
        ChallengeType::RecaptchaV3 => "userrecaptcha",
        ChallengeType::Hcaptcha => "hcaptcha",
        ChallengeType::CloudflareTurnstile => "turnstile",
        _ => {
            return Err(RuntimeError::ChallengeFailed(
                "不支持的验证类型".to_string(),
            ));
        }
    };

    let client = reqwest::Client::new();

    // 1. 提交任务
    let mut params = vec![
        ("key", api_key),
        ("method", method),
        ("sitekey", site_key),
        ("pageurl", page_url),
        ("json", "1"),
    ];

    if matches!(challenge_type, ChallengeType::RecaptchaV3) {
        params.push(("version", "v3"));
    }

    let submit_url = format!("{}/in.php", endpoint);
    let response = client
        .post(&submit_url)
        .form(&params)
        .send()
        .await
        .map_err(|e| RuntimeError::HttpRequest(e.to_string()))?;

    let result: serde_json::Value = response
        .json()
        .await
        .map_err(|e| RuntimeError::HttpRequest(e.to_string()))?;

    if result["status"].as_i64() != Some(1) {
        return Err(RuntimeError::ChallengeFailed(format!(
            "2captcha 提交失败: {}",
            result["request"]
        )));
    }

    let task_id = result["request"]
        .as_str()
        .ok_or_else(|| RuntimeError::ChallengeFailed("无效的任务 ID".to_string()))?;

    // 2. 轮询结果
    let poll_url = format!(
        "{}/res.php?key={}&action=get&id={}&json=1",
        endpoint, api_key, task_id
    );

    let timeout = Duration::from_secs(config.timeout_seconds as u64);
    let start = std::time::Instant::now();

    while start.elapsed() < timeout {
        tokio::time::sleep(Duration::from_secs(5)).await;

        let response = client
            .get(&poll_url)
            .send()
            .await
            .map_err(|e| RuntimeError::HttpRequest(e.to_string()))?;

        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| RuntimeError::HttpRequest(e.to_string()))?;

        if result["status"].as_i64() == Some(1) {
            return result["request"]
                .as_str()
                .map(|s| s.to_string())
                .ok_or_else(|| RuntimeError::ChallengeFailed("无效的响应".to_string()));
        }

        let request = result["request"].as_str().unwrap_or("");
        if request != "CAPCHA_NOT_READY" {
            return Err(RuntimeError::ChallengeFailed(format!(
                "2captcha 错误: {}",
                request
            )));
        }
    }

    Err(RuntimeError::ExecutionTimeout {
        operation: "2captcha".to_string(),
        elapsed_ms: timeout.as_millis() as u64,
        limit_ms: timeout.as_millis() as u64,
    })
}

async fn solve_with_anticaptcha(
    config: &ExternalHandler,
    page_url: &str,
    site_key: &str,
    challenge_type: &ChallengeType,
) -> Result<String> {
    let api_key = config.api_key.as_str();
    let endpoint = config
        .endpoint
        .as_deref()
        .unwrap_or("https://api.anti-captcha.com");

    let task_type = match challenge_type {
        ChallengeType::RecaptchaV2 => "RecaptchaV2TaskProxyless",
        ChallengeType::RecaptchaV3 => "RecaptchaV3TaskProxyless",
        ChallengeType::Hcaptcha => "HCaptchaTaskProxyless",
        ChallengeType::CloudflareTurnstile => "TurnstileTaskProxyless",
        _ => {
            return Err(RuntimeError::ChallengeFailed(
                "不支持的验证类型".to_string(),
            ));
        }
    };

    let client = reqwest::Client::new();

    // 1. 创建任务
    let create_task = serde_json::json!({
        "clientKey": api_key,
        "task": {
            "type": task_type,
            "websiteURL": page_url,
            "websiteKey": site_key
        }
    });

    let response = client
        .post(format!("{}/createTask", endpoint))
        .json(&create_task)
        .send()
        .await
        .map_err(|e| RuntimeError::HttpRequest(e.to_string()))?;

    let result: serde_json::Value = response
        .json()
        .await
        .map_err(|e| RuntimeError::HttpRequest(e.to_string()))?;

    if result["errorId"].as_i64() != Some(0) {
        return Err(RuntimeError::ChallengeFailed(format!(
            "Anti-Captcha 错误: {}",
            result["errorDescription"]
        )));
    }

    let task_id = result["taskId"]
        .as_i64()
        .ok_or_else(|| RuntimeError::ChallengeFailed("无效的任务 ID".to_string()))?;

    // 2. 获取结果
    let get_result = serde_json::json!({
        "clientKey": api_key,
        "taskId": task_id
    });

    let timeout = Duration::from_secs(config.timeout_seconds as u64);
    let start = std::time::Instant::now();

    while start.elapsed() < timeout {
        tokio::time::sleep(Duration::from_secs(5)).await;

        let response = client
            .post(format!("{}/getTaskResult", endpoint))
            .json(&get_result)
            .send()
            .await
            .map_err(|e| RuntimeError::HttpRequest(e.to_string()))?;

        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| RuntimeError::HttpRequest(e.to_string()))?;

        if result["status"].as_str() == Some("ready") {
            return result["solution"]["gRecaptchaResponse"]
                .as_str()
                .or_else(|| result["solution"]["token"].as_str())
                .map(|s| s.to_string())
                .ok_or_else(|| RuntimeError::ChallengeFailed("无效的响应".to_string()));
        }

        if result["errorId"].as_i64() != Some(0) {
            return Err(RuntimeError::ChallengeFailed(format!(
                "Anti-Captcha 错误: {}",
                result["errorDescription"]
            )));
        }
    }

    Err(RuntimeError::ExecutionTimeout {
        operation: "Anti-Captcha".to_string(),
        elapsed_ms: timeout.as_millis() as u64,
        limit_ms: timeout.as_millis() as u64,
    })
}

async fn solve_with_capsolver(
    config: &ExternalHandler,
    page_url: &str,
    site_key: &str,
    challenge_type: &ChallengeType,
) -> Result<String> {
    let api_key = config.api_key.as_str();
    let endpoint = config
        .endpoint
        .as_deref()
        .unwrap_or("https://api.capsolver.com");

    let task_type = match challenge_type {
        ChallengeType::RecaptchaV2 => "ReCaptchaV2TaskProxyLess",
        ChallengeType::RecaptchaV3 => "ReCaptchaV3TaskProxyLess",
        ChallengeType::Hcaptcha => "HCaptchaTurboTask",
        ChallengeType::CloudflareTurnstile => "AntiTurnstileTaskProxyLess",
        _ => {
            return Err(RuntimeError::ChallengeFailed(
                "不支持的验证类型".to_string(),
            ));
        }
    };

    let client = reqwest::Client::new();

    // 创建任务
    let create_task = serde_json::json!({
        "clientKey": api_key,
        "task": {
            "type": task_type,
            "websiteURL": page_url,
            "websiteKey": site_key
        }
    });

    let response = client
        .post(format!("{}/createTask", endpoint))
        .json(&create_task)
        .send()
        .await
        .map_err(|e| RuntimeError::HttpRequest(e.to_string()))?;

    let result: serde_json::Value = response
        .json()
        .await
        .map_err(|e| RuntimeError::HttpRequest(e.to_string()))?;

    if result["errorId"].as_i64() != Some(0) {
        return Err(RuntimeError::ChallengeFailed(format!(
            "CapSolver 错误: {}",
            result["errorDescription"]
        )));
    }

    let task_id = result["taskId"]
        .as_str()
        .ok_or_else(|| RuntimeError::ChallengeFailed("无效的任务 ID".to_string()))?;

    // 获取结果
    let get_result = serde_json::json!({
        "clientKey": api_key,
        "taskId": task_id
    });

    let timeout = Duration::from_secs(config.timeout_seconds as u64);
    let start = std::time::Instant::now();

    while start.elapsed() < timeout {
        tokio::time::sleep(Duration::from_secs(3)).await;

        let response = client
            .post(format!("{}/getTaskResult", endpoint))
            .json(&get_result)
            .send()
            .await
            .map_err(|e| RuntimeError::HttpRequest(e.to_string()))?;

        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| RuntimeError::HttpRequest(e.to_string()))?;

        if result["status"].as_str() == Some("ready") {
            return result["solution"]["gRecaptchaResponse"]
                .as_str()
                .or_else(|| result["solution"]["token"].as_str())
                .map(|s| s.to_string())
                .ok_or_else(|| RuntimeError::ChallengeFailed("无效的响应".to_string()));
        }

        if result["errorId"].as_i64() != Some(0) {
            return Err(RuntimeError::ChallengeFailed(format!(
                "CapSolver 错误: {}",
                result["errorDescription"]
            )));
        }
    }

    Err(RuntimeError::ExecutionTimeout {
        operation: "CapSolver".to_string(),
        elapsed_ms: timeout.as_millis() as u64,
        limit_ms: timeout.as_millis() as u64,
    })
}

// ============================================================================
// 脚本处理器
// ============================================================================

async fn handle_script(
    _config: &ScriptHandler,
    _ctx: &HandlerContext,
) -> Result<ChallengeCredentials> {
    // TODO: 实现脚本执行
    Err(RuntimeError::ChallengeFailed(
        "脚本处理器暂未实现".to_string(),
    ))
}

// ============================================================================
// 凭证缓存
// ============================================================================

/// 凭证缓存
pub struct CredentialsCache {
    cache: RwLock<HashMap<String, ChallengeCredentials>>,
}

impl Default for CredentialsCache {
    fn default() -> Self {
        Self::new()
    }
}

impl CredentialsCache {
    /// 创建新缓存
    pub fn new() -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
        }
    }

    /// 获取凭证
    pub async fn get(&self, domain: &str) -> Option<ChallengeCredentials> {
        let cache = self.cache.read().await;
        cache.get(domain).and_then(|c| {
            if c.is_expired() {
                None
            } else {
                Some(c.clone())
            }
        })
    }

    /// 存储凭证
    pub async fn set(&self, domain: &str, credentials: ChallengeCredentials) {
        let mut cache = self.cache.write().await;
        cache.insert(domain.to_string(), credentials);
    }

    /// 删除凭证
    pub async fn remove(&self, domain: &str) {
        let mut cache = self.cache.write().await;
        cache.remove(domain);
    }

    /// 清理过期凭证
    pub async fn cleanup_expired(&self) {
        let mut cache = self.cache.write().await;
        cache.retain(|_, v| !v.is_expired());
    }
}

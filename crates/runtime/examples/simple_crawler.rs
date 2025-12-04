//! 简单爬虫示例
//!
//! 使用 YingJu 爬虫规范执行搜索流程，演示反爬检测

use crawler_runtime::{
    challenge::{ChallengeDetectorExt, ResponseContext},
    context::Context,
    extractor::{ExtractEngine, ExtractValue},
    http::HttpClient,
};
use crawler_schema::{
    config::{ChallengeDetector, CustomDetector, HttpConfig, RequestConfig},
    extract::{ExtractStep, FieldExtractor, SelectorStep},
    template::Template,
};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== YingJu 爬虫示例 ===\n");

    // 创建 HTTP 客户端（模拟真实浏览器）
    // 注意：_ok1_ Cookie 是 _guard/auto.js 反爬机制生成的验证 Cookie
    // 在实际应用中，需要通过 WebView 执行 JS 获取此 Cookie
    let mut headers = HashMap::new();
    headers.insert(
        "Accept".to_string(),
        "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8".to_string(),
    );
    headers.insert(
        "Accept-Language".to_string(),
        "zh-CN,zh;q=0.8,zh-TW;q=0.7,zh-HK;q=0.5,en-US;q=0.3,en;q=0.2".to_string(),
    );
    // 不设置 Accept-Encoding，让 reqwest 自动处理解压
    headers.insert("Connection".to_string(), "keep-alive".to_string());
    // 这是从浏览器获取的验证 Cookie（由 _guard/auto.js 生成）
    headers.insert(
        "Cookie".to_string(),
        "_ok1_=Cs6gobeIyxgRvaxLkyOCdu56OdY41tBvdiMMKqisNk3ViP89FmLvUnuUGoQluLoCTT+3Of9vAHXmIXNkocy6ee657qdXagJMDJlOJEzPOtAqNml5ijTLfkeolWtnglrY".to_string(),
    );

    // 将 headers 转换为 Template 格式
    let template_headers: HashMap<String, Template> = headers
        .into_iter()
        .map(|(k, v)| (k, Template::new(v)))
        .collect();

    let http_config = HttpConfig {
        user_agent: Some(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:145.0) Gecko/20100101 Firefox/145.0"
                .to_string(),
        ),
        request: Some(RequestConfig {
            headers: Some(template_headers),
            ..Default::default()
        }),
        timeout: Some(30),
        ..Default::default()
    };
    let http_client = HttpClient::new(http_config)?;
    let extract_engine = ExtractEngine::new();
    let context = Context::new();

    // ============================================
    // 测试1: 直接请求并提取数据
    // ============================================
    println!("测试1: 请求七真书院并提取书籍列表\n");

    let base_url = "http://www.zqb88.cn";
    let keyword = "斗破";

    // 发起 POST 搜索请求
    let search_url = format!("{}/search/", base_url);
    let form_data = vec![("searchkey".to_string(), keyword.to_string())];

    println!("搜索关键词: {}", keyword);
    println!("请求 URL: {}", search_url);
    println!("正在搜索...\n");

    let response = http_client.post_form(&search_url, &form_data).await?;
    let status_code = response.status().as_u16();
    let headers: HashMap<String, String> = response
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect();
    let final_url = response.url().to_string();
    let html = response.text().await?;

    println!("获取到 HTML，长度: {} 字符", html.len());
    println!(
        "HTML 内容前 500 字符:\n{}\n",
        &html.chars().take(500).collect::<String>()
    );

    // ============================================
    // 检测是否为反爬页面
    // ============================================
    let response_ctx = ResponseContext::new(status_code, headers, html.clone(), final_url);

    // 创建自定义检测器（检测 _guard/auto.js 反爬脚本）
    let custom_detector = ChallengeDetector::Custom(Box::new(CustomDetector {
        status_codes: None, // 可能是 200 状态码
        headers: None,
        body_patterns: Some(vec!["_guard/auto.js".to_string(), "/_guard/".to_string()]),
        url_pattern: None,
        detect_script: None,
    }));

    let detection = custom_detector.detect(&response_ctx);

    if detection.detected {
        println!("⚠️  检测到反爬保护！");
        println!("验证类型: {:?}", detection.challenge_type);
        println!("\n该网站使用了 JavaScript 验证机制(_guard/auto.js)");
        println!("需要通过 WebView 执行 JavaScript 才能获取真实内容。");
        println!("\n在实际应用中,可以:");
        println!("1. 配置 ChallengeConfig 启用 WebView 处理器");
        println!("2. 使用 ChallengeManager.handle() 自动处理验证");
        println!("3. 获取验证后的 Cookie 用于后续请求\n");
        return Ok(());
    }

    // 提取书籍列表
    let html_value = ExtractValue::Html(html);

    // 列表选择器（网站使用多个 id="nr" 的 dl 元素，虽不规范但需支持）
    let list_extractor = FieldExtractor {
        steps: vec![ExtractStep::Css(SelectorStep::WithOptions {
            expr: "dl#nr".to_string(),
            all: true,
        })],
        fallback: None,
        default: None,
        nullable: false,
    };

    let list_result = extract_engine.extract_field(&list_extractor, html_value, &context)?;

    // 处理每个列表项
    if let ExtractValue::Array(items) = list_result {
        println!("找到 {} 条结果:\n", items.len());

        for (i, item) in items.into_iter().enumerate().take(10) {
            // 提取标题
            let title_extractor = FieldExtractor {
                steps: vec![
                    ExtractStep::Css(SelectorStep::Simple("h3 a".to_string())),
                    ExtractStep::Attr("text".to_string()),
                ],
                fallback: None,
                default: None,
                nullable: true,
            };

            // 提取链接
            let url_extractor = FieldExtractor {
                steps: vec![
                    ExtractStep::Css(SelectorStep::Simple("a".to_string())),
                    ExtractStep::Attr("href".to_string()),
                ],
                fallback: None,
                default: None,
                nullable: true,
            };

            // 提取作者
            let author_extractor = FieldExtractor {
                steps: vec![
                    ExtractStep::Css(SelectorStep::Simple(".book_other span".to_string())),
                    ExtractStep::Attr("text".to_string()),
                ],
                fallback: None,
                default: None,
                nullable: true,
            };

            // 提取简介
            let intro_extractor = FieldExtractor {
                steps: vec![
                    ExtractStep::Css(SelectorStep::Simple(".book_des".to_string())),
                    ExtractStep::Attr("text".to_string()),
                ],
                fallback: None,
                default: None,
                nullable: true,
            };

            let title = extract_engine
                .extract_field(&title_extractor, item.clone(), &context)
                .ok()
                .and_then(|v| v.as_string())
                .unwrap_or_else(|| "未知标题".to_string());

            let url = extract_engine
                .extract_field(&url_extractor, item.clone(), &context)
                .ok()
                .and_then(|v| v.as_string())
                .map(|u| {
                    if u.starts_with("http") {
                        u
                    } else {
                        format!("{}{}", base_url, u)
                    }
                })
                .unwrap_or_else(|| "无链接".to_string());

            let author = extract_engine
                .extract_field(&author_extractor, item.clone(), &context)
                .ok()
                .and_then(|v| v.as_string());

            let intro = extract_engine
                .extract_field(&intro_extractor, item, &context)
                .ok()
                .and_then(|v| v.as_string());

            println!("{}. {}", i + 1, title);
            println!("   链接: {}", url);
            if let Some(a) = author {
                println!("   作者: {}", a);
            }
            if let Some(desc) = intro {
                let short = if desc.len() > 80 {
                    format!("{}...", &desc.chars().take(80).collect::<String>())
                } else {
                    desc
                };
                println!("   简介: {}", short);
            }
            println!();
        }
    } else {
        println!("未找到结果");
    }

    println!("\n=== 测试完成 ===");
    Ok(())
}

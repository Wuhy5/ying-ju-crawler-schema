//! TOML 规则爬虫示例
//!
//! 演示 CrawlerRuntime 的简洁使用方式

use crawler_runtime::crawler::CrawlerRuntime;
use crawler_schema::core::CrawlerRule;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== YingJu 爬虫运行时示例 ===\n");

    // ==========================================
    // 第一步：加载规则
    // ==========================================
    let rule_path = std::env::current_dir()?.join("crates/runtime/examples/rules/17xiaoshuo.toml");
    let rule: CrawlerRule = toml::from_str(&std::fs::read_to_string(&rule_path)?)?;

    println!("✓ 加载规则: {}", rule.meta.name);
    println!("  作者: {}", rule.meta.author);
    println!("  域名: {}", rule.meta.domain);
    println!("  类型: {:?}\n", rule.meta.media_type);

    // ==========================================
    // 第二步：创建运行时
    // ==========================================
    let runtime = CrawlerRuntime::new(rule)?;
    println!("✓ 运行时就绪\n");

    // ==========================================
    // 第三步：搜索（一行代码！）
    // ==========================================
    println!("--- 搜索 \"斗破\" ---\n");

    let result = runtime.search("斗破", 1).await?;

    println!("✓ 找到 {} 条结果:\n", result.items.len());

    for (i, item) in result.items.iter().enumerate().take(5) {
        println!("{}. {}", i + 1, item.title);
        println!("   链接: {}", item.url);
        if let Some(author) = &item.author {
            println!("   作者: {}", author);
        }
        if let Some(summary) = &item.summary {
            let short: String = summary.chars().take(60).collect();
            println!(
                "   简介: {}{}",
                short,
                if summary.len() > 60 { "..." } else { "" }
            );
        }
        println!();
    }

    // ==========================================
    // 第四步：获取详情（可选）
    // ==========================================
    if let Some(first) = result.items.first() {
        println!("--- 获取第一本书的详情 ---\n");

        // 构造完整 URL
        let detail_url = if first.url.starts_with("http://") || first.url.starts_with("https://") {
            first.url.clone()
        } else if first.url.starts_with('/') {
            format!("https://{}{}", runtime.meta().domain, first.url)
        } else {
            // URL 可能已经包含域名（如 www.1qxs.com/...）
            format!("https://{}", first.url)
        };

        match runtime.detail(&detail_url).await {
            Ok(detail) => {
                println!("✓ 获取详情成功:");
                println!("  标题: {}", detail.title());
                println!("  作者: {}", detail.author());
                if let Some(intro) = detail.intro() {
                    let short: String = intro.chars().take(100).collect();
                    println!(
                        "  简介: {}{}",
                        short,
                        if intro.len() > 100 { "..." } else { "" }
                    );
                }
                // 如果是 Book 类型，显示更多信息
                if let crawler_runtime::flow::detail::DetailResponse::Book(book) = &detail {
                    if let Some(status) = &book.status {
                        println!("  状态: {}", status);
                    }
                    if let Some(category) = &book.category {
                        println!("  分类: {}", category);
                    }
                    if !book.chapters.is_empty() {
                        println!("  章节数: {}", book.chapters.len());
                        println!(
                            "  最新章节: {}",
                            book.chapters
                                .last()
                                .map(|c| &c.title)
                                .unwrap_or(&"".to_string())
                        );
                    }
                }
            }
            Err(e) => {
                println!("✗ 获取详情失败: {}", e);
            }
        }
    }

    println!("\n=== 完成 ===");
    Ok(())
}

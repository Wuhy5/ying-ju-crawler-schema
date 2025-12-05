//! # 详情流程执行器

use crate::{
    Result,
    context::{FlowContext, RuntimeContext},
    error::RuntimeError,
    extractor::{ExtractEngine, SharedValue, value::ExtractValueData},
    model::{BookDetail, ChapterItem},
    template::TemplateExt,
};
use crawler_schema::{
    fields::{BookDetailFields, ChapterListRule, DetailFields},
    flow::DetailFlow,
};
use std::sync::Arc;

/// 详情请求
#[derive(Debug, Clone)]
pub struct DetailRequest {
    /// 详情页 URL
    pub url: String,
}

/// 详情响应（通用）
#[derive(Debug, Clone)]
pub enum DetailResponse {
    /// 书籍详情
    Book(Box<BookDetail>),
    /// 其他类型（暂用 JSON）
    Other(serde_json::Value),
}

impl DetailResponse {
    /// 获取标题
    pub fn title(&self) -> &str {
        match self {
            Self::Book(b) => &b.title,
            Self::Other(v) => v.get("title").and_then(|t| t.as_str()).unwrap_or(""),
        }
    }

    /// 获取作者
    pub fn author(&self) -> &str {
        match self {
            Self::Book(b) => &b.author,
            Self::Other(v) => v.get("author").and_then(|t| t.as_str()).unwrap_or(""),
        }
    }

    /// 获取简介
    pub fn intro(&self) -> Option<&str> {
        match self {
            Self::Book(b) => b.intro.as_deref(),
            Self::Other(v) => v.get("intro").and_then(|t| t.as_str()),
        }
    }
}

/// 详情流程执行器
pub struct DetailFlowExecutor;

impl DetailFlowExecutor {
    /// 提取字符串字段
    fn extract_string(
        extractor: &crawler_schema::extract::FieldExtractor,
        input: &SharedValue,
        runtime_context: &RuntimeContext,
        flow_context: &FlowContext,
    ) -> Option<String> {
        ExtractEngine::extract_field(extractor, input.as_ref(), runtime_context, flow_context)
            .ok()
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
    }

    /// 提取书籍详情
    fn extract_book_detail(
        fields: &BookDetailFields,
        html: &SharedValue,
        runtime_context: &RuntimeContext,
        flow_context: &FlowContext,
    ) -> Result<BookDetail> {
        // 提取必需字段
        let title =
            Self::extract_string(&fields.title.extractor, html, runtime_context, flow_context)
                .ok_or_else(|| RuntimeError::Extraction("无法提取标题".to_string()))?;

        let author = Self::extract_string(
            &fields.author.extractor,
            html,
            runtime_context,
            flow_context,
        )
        .ok_or_else(|| RuntimeError::Extraction("无法提取作者".to_string()))?;

        // 提取可选字段
        let cover = fields
            .cover
            .as_ref()
            .and_then(|f| Self::extract_string(&f.extractor, html, runtime_context, flow_context));

        let intro = fields
            .intro
            .as_ref()
            .and_then(|f| Self::extract_string(&f.extractor, html, runtime_context, flow_context));

        let category = fields
            .category
            .as_ref()
            .and_then(|f| Self::extract_string(&f.extractor, html, runtime_context, flow_context));

        let status = fields
            .status
            .as_ref()
            .and_then(|f| Self::extract_string(&f.extractor, html, runtime_context, flow_context));

        let last_chapter = fields
            .last_chapter
            .as_ref()
            .and_then(|f| Self::extract_string(&f.extractor, html, runtime_context, flow_context));

        let word_count = fields
            .word_count
            .as_ref()
            .and_then(|f| Self::extract_string(&f.extractor, html, runtime_context, flow_context));

        // 提取章节列表
        let chapters = if let Some(chapter_rule) = &fields.chapters {
            Self::extract_chapters(chapter_rule, html, runtime_context, flow_context)?
        } else {
            vec![]
        };

        Ok(BookDetail {
            title,
            author,
            cover,
            intro,
            category,
            status,
            tags: None,
            last_chapter,
            update_time: None,
            word_count,
            toc_url: None,
            chapters,
            raw: serde_json::json!({}),
        })
    }

    /// 提取章节列表
    fn extract_chapters(
        rule: &ChapterListRule,
        html: &SharedValue,
        runtime_context: &RuntimeContext,
        flow_context: &FlowContext,
    ) -> Result<Vec<ChapterItem>> {
        // 先提取列表容器
        let list_result = ExtractEngine::extract_field(
            &rule.list.extractor,
            html.as_ref(),
            runtime_context,
            flow_context,
        )?;

        let items = match list_result.as_ref() {
            ExtractValueData::Array(arr) => arr,
            _ => return Ok(vec![]),
        };

        let mut chapters = Vec::new();
        for item in items.iter() {
            let title =
                Self::extract_string(&rule.title.extractor, item, runtime_context, flow_context);
            let url =
                Self::extract_string(&rule.url.extractor, item, runtime_context, flow_context);

            if let (Some(title), Some(url)) = (title, url) {
                chapters.push(ChapterItem { title, url });
            }
        }

        Ok(chapters)
    }

    /// 执行详情流程
    pub async fn execute(
        input: DetailRequest,
        flow: &DetailFlow,
        runtime_context: &RuntimeContext,
        flow_context: &mut FlowContext,
    ) -> Result<DetailResponse> {
        // 1. 设置上下文变量
        flow_context.set("detail_url", serde_json::json!(&input.url));

        // 2. 渲染 URL
        let url = flow.url.render(flow_context)?;

        // 3. 发起 HTTP 请求
        let response = runtime_context.http_client().get(&url).await?;
        let html_text = response
            .text()
            .await
            .map_err(|e| RuntimeError::HttpRequest(format!("读取响应失败: {}", e)))?;
        let html = Arc::new(ExtractValueData::Html(Arc::from(
            html_text.into_boxed_str(),
        )));

        // 4. 根据媒体类型提取字段
        match &flow.fields {
            DetailFields::Book(fields) => {
                let detail =
                    Self::extract_book_detail(fields, &html, runtime_context, flow_context)?;
                Ok(DetailResponse::Book(Box::new(detail)))
            }
            DetailFields::Video(_) => {
                // TODO: 实现视频详情提取
                Ok(DetailResponse::Other(serde_json::json!({"type": "video"})))
            }
            DetailFields::Audio(_) => {
                // TODO: 实现音频详情提取
                Ok(DetailResponse::Other(serde_json::json!({"type": "audio"})))
            }
            DetailFields::Manga(_) => {
                // TODO: 实现漫画详情提取
                Ok(DetailResponse::Other(serde_json::json!({"type": "manga"})))
            }
        }
    }
}

//! # 搜索流程执行器

use crate::{
    Result,
    context::{FlowContext, RuntimeContext},
    error::RuntimeError,
    extractor::{ExtractEngine, SharedValue, value::ExtractValueData},
    model::SearchItem,
    template::TemplateExt,
};
use crawler_schema::{extract::FieldExtractor, fields::ItemFields, flow::SearchFlow};
use serde_json::{Map, Value};
use std::sync::Arc;

/// 搜索请求
#[derive(Debug, Clone)]
pub struct SearchRequest {
    /// 搜索关键词
    pub keyword: String,
    /// 页码
    pub page: u32,
}

/// 搜索结果
#[derive(Debug, Clone)]
pub struct SearchResponse {
    /// 搜索结果列表
    pub items: Vec<SearchItem>,
    /// 是否有下一页
    pub has_next: bool,
    /// 原始数据
    pub raw_items: Vec<Value>,
}

/// 搜索流程执行器
pub struct SearchFlowExecutor;

impl SearchFlowExecutor {
    /// 提取字段值为字符串
    fn extract_string(
        extractor: &FieldExtractor,
        input: &SharedValue,
        runtime_context: &RuntimeContext,
        flow_context: &FlowContext,
    ) -> Option<String> {
        ExtractEngine::extract_field(extractor, input.as_ref(), runtime_context, flow_context)
            .ok()
            .and_then(|v| v.as_str().map(|s| s.to_string()))
    }

    /// 从列表项提取搜索结果
    fn extract_item(
        fields: &ItemFields,
        item_html: &SharedValue,
        runtime_context: &RuntimeContext,
        flow_context: &FlowContext,
        base_url: &str,
    ) -> Result<SearchItem> {
        // 提取必需字段
        let title = Self::extract_string(
            &fields.title.extractor,
            item_html,
            runtime_context,
            flow_context,
        )
        .ok_or_else(|| RuntimeError::Extraction("Failed to extract title".to_string()))?;

        let url = Self::extract_string(
            &fields.url.extractor,
            item_html,
            runtime_context,
            flow_context,
        )
        .ok_or_else(|| RuntimeError::Extraction("Failed to extract url".to_string()))?;

        // 处理相对 URL
        let url = if !url.starts_with("http") && !base_url.is_empty() {
            if url.starts_with('/') {
                format!("{}{}", base_url.trim_end_matches('/'), url)
            } else {
                format!("{}/{}", base_url.trim_end_matches('/'), url)
            }
        } else {
            url
        };

        // 提取可选字段
        let cover = fields.cover.as_ref().and_then(|f| {
            Self::extract_string(&f.extractor, item_html, runtime_context, flow_context)
        });

        let summary = fields.summary.as_ref().and_then(|f| {
            Self::extract_string(&f.extractor, item_html, runtime_context, flow_context)
        });

        let author = fields.author.as_ref().and_then(|f| {
            Self::extract_string(&f.extractor, item_html, runtime_context, flow_context)
        });

        let latest = fields.latest.as_ref().and_then(|f| {
            Self::extract_string(&f.extractor, item_html, runtime_context, flow_context)
        });

        // 构建原始数据
        let mut raw: Map<String, Value> = Map::new();
        raw.insert("title".to_string(), Value::String(title.clone()));
        raw.insert("url".to_string(), Value::String(url.clone()));
        if let Some(ref c) = cover {
            raw.insert("cover".to_string(), Value::String(c.clone()));
        }
        if let Some(ref s) = summary {
            raw.insert("summary".to_string(), Value::String(s.clone()));
        }
        if let Some(ref a) = author {
            raw.insert("author".to_string(), Value::String(a.clone()));
        }
        if let Some(ref l) = latest {
            raw.insert("latest".to_string(), Value::String(l.clone()));
        }

        Ok(SearchItem {
            title,
            url,
            cover,
            summary,
            author,
            latest,
            score: None,
            status: None,
            category: None,
            raw: Value::Object(raw),
        })
    }

    /// 执行搜索流程
    pub async fn execute(
        input: SearchRequest,
        flow: &SearchFlow,
        runtime_context: &RuntimeContext,
        flow_context: &mut FlowContext,
    ) -> Result<SearchResponse> {
        // 获取 base_url
        let base_url = runtime_context
            .globals()
            .get("base_url")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        // 设置上下文变量
        flow_context.set("keyword", serde_json::json!(input.keyword));
        flow_context.set("page", serde_json::json!(input.page));
        flow_context.set("base_url", serde_json::json!(&base_url));

        // 1. 渲染 URL
        let url = flow.url.render(flow_context)?;
        let full_url = if !url.starts_with("http") && !base_url.is_empty() {
            format!("{}{}", base_url.trim_end_matches('/'), url)
        } else {
            url
        };

        // 2. 发起 HTTP 请求
        let response = runtime_context
            .http_client()
            .get(&full_url)
            .await
            .map_err(|e| RuntimeError::HttpRequest(format!("Request failed: {}", e)))?;

        let html = response
            .text()
            .await
            .map_err(|e| RuntimeError::HttpRequest(format!("Failed to read response: {}", e)))?;

        // 3. 提取列表
        let html_value = Arc::new(ExtractValueData::Html(Arc::from(html.into_boxed_str())));
        let list_result = ExtractEngine::extract_field(
            &flow.list,
            html_value.as_ref(),
            runtime_context,
            flow_context,
        )?;

        // 4. 遍历列表项，提取字段
        let mut items = Vec::new();
        let mut raw_items = Vec::new();

        match list_result.as_ref() {
            ExtractValueData::Array(arr) => {
                for item_value in arr.iter() {
                    match Self::extract_item(
                        &flow.fields,
                        item_value,
                        runtime_context,
                        flow_context,
                        &base_url,
                    ) {
                        Ok(item) => {
                            raw_items.push(item.raw.clone());
                            items.push(item);
                        }
                        Err(e) => {
                            // 记录错误但继续处理
                            eprintln!("Warning: Failed to extract item: {}", e);
                        }
                    }
                }
            }
            ExtractValueData::Html(h) => {
                // 单个结果
                let item_value = Arc::new(ExtractValueData::Html(Arc::clone(h)));
                if let Ok(item) = Self::extract_item(
                    &flow.fields,
                    &item_value,
                    runtime_context,
                    flow_context,
                    &base_url,
                ) {
                    raw_items.push(item.raw.clone());
                    items.push(item);
                }
            }
            _ => {}
        }

        // 5. 判断是否有下一页（简单实现：有结果就认为可能有下一页）
        let has_next = !items.is_empty();

        Ok(SearchResponse {
            items,
            has_next,
            raw_items,
        })
    }
}

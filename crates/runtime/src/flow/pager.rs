//! # 分页器
//!
//! 为流程结果提供链式分页能力

use crate::{Result, context::RuntimeContext};
use crawler_schema::flow::common::Pagination;
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};

/// 分页状态 trait
///
/// 不同流程（Search/Discovery）实现不同的状态结构
pub trait PagerState: Clone + Send + Sync {
    /// 获取当前页码
    fn current_page(&self) -> u32;

    /// 设置页码
    fn with_page(&self, page: u32) -> Self;

    /// 转换为 Flow 变量
    fn to_flow_vars(&self) -> HashMap<String, Value>;
}

/// 搜索分页状态
#[derive(Debug, Clone)]
pub struct SearchPagerState {
    /// 搜索关键词
    pub keyword: String,
    /// 当前页码
    pub page: u32,
    /// 游标（用于游标分页）
    pub cursor: Option<String>,
}

impl SearchPagerState {
    /// 创建新的搜索分页状态
    pub fn new(keyword: impl Into<String>, page: u32) -> Self {
        Self {
            keyword: keyword.into(),
            page,
            cursor: None,
        }
    }

    /// 设置游标
    pub fn with_cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }
}

impl PagerState for SearchPagerState {
    fn current_page(&self) -> u32 {
        self.page
    }

    fn with_page(&self, page: u32) -> Self {
        Self {
            keyword: self.keyword.clone(),
            page,
            cursor: None, // 切换页码时清除游标
        }
    }

    fn to_flow_vars(&self) -> HashMap<String, Value> {
        let mut vars = HashMap::new();
        vars.insert("keyword".into(), Value::String(self.keyword.clone()));
        vars.insert("page".into(), Value::Number(self.page.into()));
        if let Some(ref cursor) = self.cursor {
            vars.insert("cursor".into(), Value::String(cursor.clone()));
        }
        vars
    }
}

/// 发现分页状态
#[derive(Debug, Clone)]
pub struct DiscoveryPagerState {
    /// 筛选条件
    pub filters: HashMap<String, String>,
    /// 当前页码
    pub page: u32,
    /// 游标（用于游标分页）
    pub cursor: Option<String>,
}

impl DiscoveryPagerState {
    /// 创建新的发现分页状态
    pub fn new(filters: HashMap<String, String>, page: u32) -> Self {
        Self {
            filters,
            page,
            cursor: None,
        }
    }

    /// 设置游标
    pub fn with_cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }
}

impl PagerState for DiscoveryPagerState {
    fn current_page(&self) -> u32 {
        self.page
    }

    fn with_page(&self, page: u32) -> Self {
        Self {
            filters: self.filters.clone(),
            page,
            cursor: None, // 切换页码时清除游标
        }
    }

    fn to_flow_vars(&self) -> HashMap<String, Value> {
        let mut vars = HashMap::new();
        // 筛选条件
        for (k, v) in &self.filters {
            vars.insert(k.clone(), Value::String(v.clone()));
        }
        // 页码
        vars.insert("page".into(), Value::Number(self.page.into()));
        // 游标
        if let Some(ref cursor) = self.cursor {
            vars.insert("cursor".into(), Value::String(cursor.clone()));
        }
        vars
    }
}

/// 通用分页器
///
/// 持有分页状态和执行分页所需的资源
#[derive(Clone)]
pub struct Pager<S: PagerState> {
    /// 运行时上下文（共享资源）
    runtime: Arc<RuntimeContext>,
    /// 分页配置
    pagination: Option<Pagination>,
    /// 分页状态
    state: S,
    /// 下一页游标（从响应中提取）
    next_cursor: Option<String>,
}

impl<S: PagerState> Pager<S> {
    /// 创建新的分页器
    pub fn new(runtime: Arc<RuntimeContext>, pagination: Option<Pagination>, state: S) -> Self {
        Self {
            runtime,
            pagination,
            state,
            next_cursor: None,
        }
    }

    /// 获取运行时上下文
    #[inline]
    pub fn runtime(&self) -> &RuntimeContext {
        &self.runtime
    }

    /// 获取运行时上下文（Arc 引用）
    #[inline]
    pub fn runtime_arc(&self) -> Arc<RuntimeContext> {
        Arc::clone(&self.runtime)
    }

    /// 获取分页配置
    #[inline]
    pub fn pagination(&self) -> Option<&Pagination> {
        self.pagination.as_ref()
    }

    /// 获取当前状态
    #[inline]
    pub fn state(&self) -> &S {
        &self.state
    }

    /// 获取当前页码
    #[inline]
    pub fn current_page(&self) -> u32 {
        self.state.current_page()
    }

    /// 设置下一页游标
    pub fn set_next_cursor(&mut self, cursor: Option<String>) {
        self.next_cursor = cursor;
    }

    /// 获取下一页游标
    #[inline]
    pub fn next_cursor(&self) -> Option<&str> {
        self.next_cursor.as_deref()
    }

    /// 创建下一页的分页器
    pub fn next_page_pager(&self) -> Option<Self> {
        // 如果是游标分页，需要有游标才能翻页
        if let Some(Pagination::Cursor(_)) = &self.pagination {
            let cursor = self.next_cursor.clone()?;
            let new_state = self.state.with_page(self.state.current_page() + 1);
            // 需要在 state 中存储 cursor，这里通过重新创建来实现
            return Some(Self {
                runtime: Arc::clone(&self.runtime),
                pagination: self.pagination.clone(),
                state: new_state,
                next_cursor: Some(cursor),
            });
        }

        // 页码分页或偏移分页，直接增加页码
        Some(Self {
            runtime: Arc::clone(&self.runtime),
            pagination: self.pagination.clone(),
            state: self.state.with_page(self.state.current_page() + 1),
            next_cursor: None,
        })
    }

    /// 创建上一页的分页器
    pub fn prev_page_pager(&self) -> Option<Self> {
        let current = self.state.current_page();
        if current <= 1 {
            return None;
        }

        Some(Self {
            runtime: Arc::clone(&self.runtime),
            pagination: self.pagination.clone(),
            state: self.state.with_page(current - 1),
            next_cursor: None,
        })
    }

    /// 创建指定页的分页器
    pub fn goto_page_pager(&self, page: u32) -> Result<Self> {
        // 游标分页不支持跳页
        if let Some(Pagination::Cursor(_)) = &self.pagination {
            return Err(crate::error::RuntimeError::Pagination(
                "游标分页不支持跳页".to_string(),
            ));
        }

        Ok(Self {
            runtime: Arc::clone(&self.runtime),
            pagination: self.pagination.clone(),
            state: self.state.with_page(page),
            next_cursor: None,
        })
    }

    /// 获取 Flow 变量（用于模板渲染）
    pub fn to_flow_vars(&self) -> HashMap<String, Value> {
        let mut vars = self.state.to_flow_vars();

        // 如果有游标，覆盖 cursor 变量
        if let Some(ref cursor) = self.next_cursor {
            vars.insert("cursor".into(), Value::String(cursor.clone()));
        }

        vars
    }
}

/// 搜索分页器类型别名
pub type SearchPager = Pager<SearchPagerState>;

/// 发现分页器类型别名
pub type DiscoveryPager = Pager<DiscoveryPagerState>;

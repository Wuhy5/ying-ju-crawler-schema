# Ying-Ju Crawler Schema & Runtime – APM Implementation Plan
**Memory Strategy:** Dynamic-MD
**Last Modification:** Plan creation by the Setup Agent.
**Project Overview:** 高性能、可扩展的媒体爬虫规则定义和运行时执行库，为 Ying-Ju-App（Tauri 应用）提供支持。包含 Schema（纯数据结构定义）和 Runtime（运行时执行逻辑）两个核心 crate。项目重点：激进性能优化（零拷贝、并发安全）、模块化架构重构（Context 携带服务、依赖注入统一）、完整流程实现（Content/Discovery/Login）、脚本引擎完善（Lua/Python + 安全沙箱）、并发爬取（聚合搜索）、70%+ 测试覆盖率。


---

## Phase 1: Foundation Refactoring

### Task 1.1 – 添加脚本安全配置到 Schema - Agent_Schema
**Objective:** 为 Schema 添加脚本执行安全配置支持（内存限制、文件访问限制、网络访问限制、超时限制）。
**Output:** 更新的 `script.rs` 和 `meta.rs` 文件，包含 `ScriptSecurityConfig` 定义和集成。
**Guidance:** 在 `Script` 类型中添加可选的 `security` 字段，在 `Meta` 中添加全局默认配置。确保所有字段使用正确的 Serde 标注（`#[serde(skip_serializing_if = "Option::is_none")]`）。

1. 在 `crates/schema/src/script.rs` 中定义 `ScriptSecurityConfig` 结构体（字段：`max_memory_mb: Option<u64>`、`allow_file_access: Option<bool>`、`allow_network: Option<bool>`、`timeout_seconds: Option<u64>`）
2. 为 `Script` 类型添加 `security: Option<ScriptSecurityConfig>` 字段
3. 在 `crates/schema/src/config/meta.rs` 中为 `Meta` 添加 `default_script_security: Option<ScriptSecurityConfig>` 字段
4. 运行 `cargo test --package crawler-schema` 验证序列化正常
5. 运行 `cargo run --bin generate_schema` 验证 JSON Schema 生成

### Task 1.2 – 重构 ExtractValue 使用借用模式 - Agent_Core
**Objective:** 重构 `ExtractValue` 类型引入生命周期参数，使用 `Cow<'a, str>` 支持零拷贝。
**Output:** 重构后的 `crates/runtime/src/extractor/value.rs` 文件和单元测试。
**Guidance:** 添加生命周期参数 `<'a>`，修改 `String` 和 `Html` 变体为 `Cow<'a, str>`，更新所有方法签名使用借用返回。优化 `from_json` 和 `From` 实现优先使用 `Cow::Borrowed`。

1. 为 `ExtractValue` 添加生命周期参数：`pub enum ExtractValue<'a>`
2. 修改变体定义：`String(Cow<'a, str>)`、`Html(Cow<'a, str>)`，保持 `Json(Value)` 和 `Array(Vec<ExtractValue<'a>>)` 不变
3. 更新所有方法签名使用借用返回：`as_string(&self) -> Option<Cow<'a, str>>`、`as_json(&self) -> &Value`
4. 优化 `from_json` 和 `From` 实现避免不必要拷贝（优先使用 `Cow::Borrowed`）
5. 运行 `cargo check --package crawler-runtime` 确保编译通过
6. 为新 API 编写单元测试（验证借用和拥有两种场景）

### Task 1.3 – 重构 Context 携带 Services 并支持父子引用 - Agent_Core
**Objective:** 重构 `Context` 类型添加 Services 引用、父 Context 引用，切换到 DashMap 支持并发。
**Output:** 重构后的 `crates/runtime/src/context/mod.rs` 文件。
**Guidance:** 添加生命周期参数 `<'a>`，添加 `services: Arc<Services>` 和 `parent: Option<&'a Context<'a>>` 字段。将 `variables` 改为 `Arc<DashMap<String, Value>>`（需添加 `dashmap` 依赖）。实现变量查找链（先查当前，再递归查父）。

1. 为 `Context` 添加生命周期参数：`pub struct Context<'a>`，添加字段 `services: Arc<Services>`、`parent: Option<&'a Context<'a>>`
2. 将 `variables: VariableStore` 改为 `variables: Arc<DashMap<String, Value>>`（添加 `dashmap` 依赖到 `Cargo.toml`）
3. 更新 `new()` 方法接受 `services: Arc<Services>` 参数
4. 实现 `child(&'a self) -> Context<'a>` 方法（共享 services，新建 variables，设置 parent）
5. 重构 `get(&self, key: &str) -> Option<Value>` 实现变量查找链（先查当前 `variables`，未找到则递归查询 `parent`）
6. 运行 `cargo check --package crawler-runtime` 确保编译通过

### Task 1.4 – 创建 Services 容器并实现依赖注入 - Agent_Core
**Objective:** 创建统一的 Services 容器，包含所有运行时服务，实现从规则检测并初始化脚本引擎。
**Output:** 新文件 `crates/runtime/src/crawler/services.rs` 和更新的 `CrawlerRuntime`。
**Guidance:** 定义 `Services` 结构体包含所有服务（HttpClient、ExtractEngine、TemplateEngine、脚本引擎 DashMap、WebViewProvider、ChallengeManager）。实现 `detect_required_engines` 函数递归扫描规则中的脚本定义。实现 `from_rule` 构建方法。**Depends on: Task 1.3 Output**

1. 创建 `crates/runtime/src/crawler/services.rs`，定义 `pub struct Services` 包含字段：`http_client: Arc<HttpClient>`、`extract_engine: Arc<ExtractEngine>`、`template_engine: Arc<TemplateEngine>`、`script_engines: DashMap<ScriptEngineType, Arc<dyn ScriptEngine>>`、`webview_provider: SharedWebViewProvider`、`challenge_manager: Arc<ChallengeManager>`
2. 实现 `detect_required_engines(rule: &CrawlerRule) -> HashSet<ScriptEngineType>` 函数（递归扫描 rule 中所有 Script 字段）
3. 实现 `Services::from_rule(rule: &CrawlerRule, webview_provider: SharedWebViewProvider) -> Result<Arc<Self>>` 方法（初始化所有服务，根据检测结果初始化脚本引擎）
4. 修改 `CrawlerRuntime::with_webview_provider` 使用 `Services::from_rule` 并将 services 传递给 Context
5. 运行 `cargo check --package crawler-runtime` 确保编译通过

### Task 1.5 – 迁移现有代码到新 API - Agent_Core
**Objective:** 更新所有模块的代码以适配新的 Context 和 ExtractValue API。
**Output:** 所有 runtime 模块更新完成，编译通过。
**Guidance:** 系统地更新 ExtractEngine、Flow Executors、脚本执行器、HTTP 模块、模板渲染器等所有使用旧 API 的代码。添加必要的生命周期参数，修改服务访问方式通过 `context.services`。**Depends on: Task 1.2 Output** **Depends on: Task 1.3 Output** **Depends on: Task 1.4 Output**

1. 更新 `crates/runtime/src/extractor/engine.rs` 的 `extract_field` 方法签名添加生命周期参数，修改内部调用适配新 `ExtractValue<'a>`
2. 更新所有 Flow Executors（`search.rs`、`detail.rs`、`content.rs`、`discovery.rs`、`login.rs`）的 `execute` 方法接受 `Context<'_>`，通过 `context.services` 访问服务
3. 更新 `crates/runtime/src/script/executor.rs` 的脚本执行逻辑，通过 `Context` 的 services 获取脚本引擎
4. 更新 `crates/runtime/src/http/` 模块中所有使用 Context 的地方
5. 更新 `crates/runtime/src/template/renderer.rs` 中模板渲染逻辑
6. 运行 `cargo build --workspace` 确保全部编译通过，修复所有编译错误

### Task 1.6 – 更新示例代码适配新 API - Agent_Core
**Objective:** 更新示例代码以使用新的 API，确保可运行。
**Output:** 更新的 `crates/runtime/examples/toml_crawler.rs` 文件。
**Guidance:** 适配新的 API 调用方式，确保示例代码编译通过并正常运行。**Depends on: Task 1.5 Output**

- 更新 `toml_crawler.rs` 中 `CrawlerRuntime::new` 的调用（如果 API 有变化）
- 确保所有方法调用符合新的 API 签名
- 运行 `cargo run --example toml_crawler` 验证示例正常工作，输出与之前一致

## Phase 2: Flow Implementation - Part 1

### Task 2.1 – 补充 Detail 流程的媒体类型字段定义 - Agent_Schema
**Objective:** 在 Schema 中补充 Video/Audio/Manga 详情字段定义，遵循通用命名约定。
**Output:** 更新的 `video.rs`、`audio.rs`、`manga.rs` 字段定义文件。
**Guidance:** 使用通用字段名（title/author/cover/intro/category/status），添加媒体特定字段（Video: episodes, Audio: tracks, Manga: chapters）。确保与 BookDetailFields 命名一致性。

- 在 `crates/schema/src/fields/` 中补充 `video.rs`、`audio.rs`、`manga.rs` 的字段定义，使用通用字段名（title/author/cover/intro/category/status）
- 添加媒体特定字段：Video（episodes: Vec<EpisodeItem>）、Audio（tracks: Vec<TrackItem>）、Manga（chapters: Vec<ChapterItem>）
- 运行 `cargo test --package crawler-schema` 验证定义正确

### Task 2.2 – 实现 Detail 流程的 Video/Audio/Manga 提取逻辑 - Agent_Flow
**Objective:** 实现三种媒体类型的详情提取逻辑，复用 Book 的提取框架。
**Output:** 更新的 `crates/runtime/src/flow/detail.rs` 和新的响应类型。
**Guidance:** 扩展 `DetailResponse` 枚举，根据 `media_type` 分发到不同提取器。复用现有提取框架，只需映射到不同字段。创建测试用例验证。**Depends on: Task 2.1 Output by Agent_Schema**

1. 扩展 `DetailResponse` 枚举添加 `Video(VideoDetailResponse)`、`Audio(AudioDetailResponse)`、`Manga(MangaDetailResponse)` 变体
2. 在 `DetailFlowExecutor::execute` 中根据 `detail.fields.media_type` 分发到不同提取逻辑
3. 实现 `extract_video_detail` 方法（复用 Book 的提取框架，映射到 Video 字段）
4. 实现 `extract_audio_detail` 和 `extract_manga_detail` 方法（同上）
5. 创建测试用例验证不同媒体类型的提取（使用静态 HTML 文件）

### Task 2.3 – 实现 Content 流程 - 书籍章节内容提取 - Agent_Flow
**Objective:** 实现书籍章节内容提取的完整 Content 流程。
**Output:** 完整的 `ContentFlowExecutor` 实现（Book 类型）。
**Guidance:** 实现 HTTP 请求、HTML 解析、内容提取、分页处理。使用 `ExtractEngine` 根据 `ContentFlow` 规范提取字段。

1. 定义 `BookContentResponse` 结构体（字段：content: String, title: Option<String>, next_url: Option<String>）
2. 实现 `ContentFlowExecutor::execute` 方法：渲染 URL 模板 → 发起 HTTP 请求 → 解析 HTML
3. 根据 `ContentFlow` 的字段定义提取章节内容（使用 `ExtractEngine`）
4. 处理分页逻辑（如果有 `next_page` 字段定义，递归或返回 next_url）
5. 创建测试用例验证提取（使用 `17xiaoshuo.toml` 规则）

### Task 2.4 – 实现 Content 流程 - 视频播放地址解析 - Agent_Flow
**Objective:** 实现视频播放地址解析的 Content 流程，支持脚本解密。
**Output:** Video Content 提取逻辑。
**Guidance:** 实现基础 URL 提取，集成脚本引擎支持解密算法，处理多种播放地址格式（直链、M3U8、DASH）。

1. 定义 `VideoContentResponse` 结构体（字段：play_url: String, quality: Option<String>, format: Option<String>）
2. 实现基础提取逻辑（从 HTML 中提取视频 URL）
3. 集成脚本引擎支持（如果 ContentFlow 定义了 Script 步骤，执行脚本解密）
4. 处理多种播放地址格式（直链、M3U8、DASH 等）
5. 创建测试用例（使用模拟的视频规则）

### Task 2.5 – 实现 Content 流程 - 音频地址解析 - Agent_Flow
**Objective:** 实现音频播放地址解析的 Content 流程。
**Output:** Audio Content 提取逻辑。
**Guidance:** 复用 Task 2.4 的框架，适配音频字段和格式。

- 定义 `AudioContentResponse` 结构体（字段：audio_url: String, quality: Option<String>, format: Option<String>）
- 复用 Task 2.4 的提取框架，适配音频字段
- 创建测试用例验证

### Task 2.6 – 实现 Content 流程 - 漫画图片列表提取 - Agent_Flow
**Objective:** 实现漫画章节图片列表提取的 Content 流程。
**Output:** Manga Content 提取逻辑。
**Guidance:** 提取图片列表，处理懒加载图片（data-src 等属性），可能需要脚本支持。

1. 定义 `MangaContentResponse` 结构体（字段：images: Vec<String>, chapter_title: Option<String>）
2. 实现图片列表提取（使用 CSS 选择器提取所有 img 标签）
3. 处理懒加载图片（提取 data-src、data-original 等属性）
4. 创建测试用例验证

### Task 2.7 – 实现 Discovery 流程 - 分类列表和筛选 - Agent_Flow
**Objective:** 实现完整的 Discovery 流程，支持分类、筛选、排序、分页。
**Output:** 完整的 `DiscoveryFlowExecutor` 实现。
**Guidance:** 定义请求响应结构，实现分类列表提取、筛选条件应用、排序、分页逻辑。创建测试验证。

1. 定义 `DiscoveryRequest`（category: String, filters: HashMap<String, String>, sort: Option<String>, page: u32）和 `DiscoveryResponse`（items: Vec<SearchItem>, has_next: bool）
2. 实现分类列表提取（根据 DiscoveryFlow 的 categories 定义）
3. 实现筛选条件应用（构建查询参数或 URL 路径）
4. 实现排序选项（根据 sort 参数调整 URL）
5. 实现分页逻辑（类似 SearchFlow）
6. 创建测试用例验证（使用静态数据或真实网站）

## Phase 3: Advanced Features

### Task 3.1 – 实现 Login 流程 - Script 模式 - Agent_Flow
**Objective:** 实现通过执行脚本获取登录凭证的 Login 流程。
**Output:** 完整的 `LoginFlowExecutor` 实现（Script 模式）。
**Guidance:** 执行脚本获取 Cookie/Headers，存储到 ChallengeManager，更新 HTTP 客户端配置。脚本应返回 JSON 格式的凭证数据。

1. 定义 `LoginRequest` 和 `LoginResponse` 结构体
2. 实现 `LoginFlowExecutor::execute` 方法处理 `LoginFlow::Script` 变体
3. 执行脚本并解析返回的凭证数据（使用 `context.services.script_engines`）
4. 将凭证存储到 `ChallengeManager`（通过 `context.services.challenge_manager`）
5. 创建测试用例验证

### Task 3.2 – 实现 Login 流程 - WebView 模式 - Agent_Flow
**Objective:** 实现通过 WebView 让用户手动登录并获取凭证的 Login 流程。
**Output:** WebView 模式的 Login 流程实现。
**Guidance:** 调用 `WebViewProvider` 打开登录页面，等待用户完成登录，提取 Cookie/Headers，存储凭证。

1. 实现 `LoginFlowExecutor` 处理 `LoginFlow::WebView` 变体
2. 构建 `WebViewRequest`（URL、title、tip 从 LoginFlow 配置中获取）
3. 调用 `context.services.webview_provider.open()` 并等待响应
4. 从 `WebViewResponse` 中提取 cookies 和 headers
5. 存储凭证到 `ChallengeManager`
6. 创建测试用例（使用 mock WebViewProvider）

### Task 3.3 – 实现 Login 流程 - Credential 模式和 OAuth - Agent_Flow
**Objective:** 实现表单提交登录和 OAuth 第三方登录支持。
**Output:** Credential 和 OAuth 模式的 Login 流程实现。
**Guidance:** Credential 模式发送 POST 请求提交用户名密码，OAuth 模式打开授权页面并处理回调。

1. 实现 `LoginFlow::Credential` 处理：构建 POST 请求，提交用户名密码，解析响应获取 Cookie
2. 实现 `LoginFlow::OAuth` 处理：构建授权 URL，打开 WebView，处理回调获取 token
3. 将获取的凭证存储到 `ChallengeManager`
4. 创建测试用例验证不同模式

### Task 3.4 – 实现 Lua 脚本引擎 - Agent_Script
**Objective:** 完善 Lua 脚本引擎的执行逻辑和内置函数。
**Output:** 完整的 `LuaScriptEngine` 实现。
**Guidance:** 使用 `mlua` 库实现脚本执行，注册内置函数（HTTP、编码、JSON、正则等），实现超时控制。

1. 完善 `crates/runtime/src/script/lua_engine.rs` 的 `execute` 和 `execute_json` 方法
2. 注册内置函数（参考 `builtin/lua.rs`）：http_get、json_parse、regex_match、base64_encode 等
3. 实现超时控制（使用 Lua 的 hook 机制或 tokio::time::timeout）
4. 实现错误处理和日志记录
5. 创建测试用例验证脚本执行和内置函数

### Task 3.5 – 实现 Python 脚本引擎 - Agent_Script
**Objective:** 完善 Python 脚本引擎的执行逻辑和内置函数。
**Output:** 完整的 `PythonScriptEngine` 实现。
**Guidance:** 使用 `rustpython-vm` 实现脚本执行，注册内置函数，实现超时控制。注意性能限制。

1. 完善 `crates/runtime/src/script/python_engine.rs` 的 `execute` 和 `execute_json` 方法
2. 注册内置函数（参考 `builtin/python.rs`）
3. 实现超时控制（可能需要研究 rustpython 的解决方案）
4. 实现错误处理和日志记录
5. 创建测试用例验证

### Task 3.6 – 实现脚本安全限制 - Agent_Script
**Objective:** 实现脚本执行的安全限制（内存、文件访问、网络访问、超时）。
**Output:** 所有脚本引擎的安全限制实现。
**Guidance:** 根据 `ScriptSecurityConfig`（Task 1.1）实施限制。内存限制、文件系统访问禁止、网络访问仅允许通过提供的 HTTP 客户端、强制超时。

1. 为所有脚本引擎（Rhai/JS/Lua/Python）实现内存限制检查（根据 `security.max_memory_mb`）
2. 禁用文件系统访问（移除 fs 相关函数，除非 `security.allow_file_access` 为 true）
3. 禁用网络访问（移除 network 相关函数，除非 `security.allow_network` 为 true，但鼓励使用内置的 http 函数）
4. 强制超时限制（使用 `security.timeout_seconds` 或默认值）
5. 创建测试用例验证各种限制是否生效
6. 添加 tracing 日志记录安全事件

## Phase 4: Concurrency & Performance

### Task 4.1 – 设计并实现并发控制器 API - Agent_Concurrent
**Objective:** 设计并实现可配置的并发控制器 API。
**Output:** `ConcurrentConfig` 和 `ConcurrentCrawler` 结构体及方法。
**Guidance:** 定义配置结构（并发数、超时、重试、限流、失败策略、按域名限流），实现动态配置更新。

1. 创建 `crates/runtime/src/concurrent/mod.rs` 和子模块
2. 定义 `ConcurrentConfig` 结构体（包含所有配置项）
3. 定义 `FailureStrategy`、`RetryConfig`、`RateLimitConfig` 枚举和结构体
4. 实现 `ConcurrentCrawler` 结构体，包含 `config: Arc<RwLock<ConcurrentConfig>>`
5. 实现 `update_config` 方法支持动态调整
6. 添加 Builder 模式简化配置

### Task 4.2 – 实现任务调度和并发执行 - Agent_Concurrent
**Objective:** 实现基于 tokio 的任务调度器，支持并发限制和超时控制。
**Output:** 任务调度核心逻辑。
**Guidance:** 使用 `tokio::task::JoinSet` 和 `Semaphore` 实现并发控制，支持任务优先级、超时、取消。

1. 实现任务调度器（使用 `JoinSet` 管理异步任务）
2. 使用 `Semaphore` 限制并发数（根据 `config.max_concurrent`）
3. 实现全局超时和单任务超时控制
4. 实现失败策略处理（FailFast/ContinueOnError/WaitAll）
5. 创建测试用例验证调度逻辑

### Task 4.3 – 实现限流器 - Agent_Concurrent
**Objective:** 实现全局和按域名的限流器。
**Output:** 限流器实现。
**Guidance:** 使用令牌桶算法实现限流，支持按域名独立限流。使用 `governor` 或手动实现。

1. 实现全局限流器（根据 `RateLimitConfig`）
2. 实现按域名限流器（使用 `HashMap<Domain, RateLimiter>`）
3. 集成到 `HttpClient`（发请求前检查限流）
4. 支持动态调整限流配置
5. 创建测试用例验证限流效果

### Task 4.4 – 实现重试机制 - Agent_Concurrent
**Objective:** 实现自动重试机制，支持指数退避。
**Output:** 重试逻辑实现。
**Guidance:** 根据 `RetryConfig` 实现重试，支持指数退避、固定延迟等策略。

1. 定义 `BackoffStrategy` 枚举（Exponential/Fixed/Linear）
2. 实现重试逻辑（包装 HTTP 请求和流程执行）
3. 集成到 `ConcurrentCrawler` 的任务执行中
4. 添加 tracing 日志记录重试事件
5. 创建测试用例验证

### Task 4.5 – 实现聚合搜索功能 - Agent_Concurrent
**Objective:** 实现聚合搜索功能，同时向多个源发起搜索并合并结果。
**Output:** `ConcurrentCrawler::aggregate_search` 方法实现。
**Guidance:** 使用并发控制器并发调用多个 `CrawlerRuntime::search`，合并、去重、排序结果。**Depends on: Task 4.2 Output** **Depends on: Task 4.3 Output** **Depends on: Task 4.4 Output**

1. 实现 `aggregate_search(sources: Vec<Arc<CrawlerRuntime>>, keyword: &str, page: u32) -> Result<AggregatedSearchResult>` 方法
2. 并发调用所有 sources 的 `search` 方法
3. 合并结果（去重、按相关性或时间排序）
4. 处理部分失败（根据 `FailureStrategy`）
5. 创建测试用例验证（使用多个 mock CrawlerRuntime）

### Task 4.6 – 流程结果缓存集成 - Agent_Concurrent
**Objective:** 在高层流程（Search/Detail/Content）中集成缓存，缓存流程执行结果。
**Output:** 更新的流程执行器，支持结果缓存。
**Guidance:** 在 `CrawlerRuntime` 的 `search`、`detail`、`content` 方法中添加结果缓存逻辑。使用已实现的 `CacheStorage`（Task 1.8）。**Depends on: Task 1.8 Output**

1. 在 `CrawlerRuntime::search` 中添加缓存逻辑：
   - 生成缓存 key（keyword + page + 规则 ID）
   - 执行前查询缓存，命中则直接返回
   - 执行后将结果存入缓存（根据配置的 TTL）
2. 在 `CrawlerRuntime::detail` 中添加缓存逻辑（缓存 key: URL + 规则 ID）
3. 在 `CrawlerRuntime::content` 中添加缓存逻辑（缓存 key: URL + 规则 ID）
4. 添加配置选项控制各流程是否启用缓存（在 `CacheConfig` 中）
5. 创建测试用例验证流程缓存功能
6. 添加缓存命中率统计（可选）

### Task 4.7 – 性能优化扫尾 - Agent_Core
**Objective:** 审查并优化剩余的性能热点。
**Output:** 性能优化报告和代码改进。
**Guidance:** 使用 profiler 识别热点，优化剩余的 clone、分配、锁竞争。验证 Phase 1 的零拷贝优化效果。

1. 使用 `cargo flamegraph` 或 `perf` 分析性能热点
2. 优化识别出的热点（减少 clone、优化循环、减少分配）
3. 审查 DashMap 使用是否合理，是否有锁竞争
4. 对比优化前后的 Benchmark 结果
5. 记录优化项和性能提升数据

## Phase 5: Quality Assurance

### Task 5.1 – 建立测试框架和静态测试数据 - Agent_Quality
**Objective:** 建立测试框架，准备静态 HTML/JSON 测试数据。
**Output:** 测试框架和 `crates/runtime/tests/` 目录结构。
**Guidance:** 创建测试数据目录，录制真实网站的 HTML 响应作为测试数据。建立测试辅助函数。

1. 创建 `crates/runtime/tests/fixtures/` 目录存放测试数据
2. 录制真实网站的 HTML 响应（17xiaoshuo 等）
3. 创建测试辅助模块（`tests/common/mod.rs`）提供 mock 函数
4. 实现 mock `WebViewProvider` 和 mock `HttpClient`
5. 编写示例测试验证框架可用

### Task 5.2 – 编写单元测试 - Extractor 模块 - Agent_Quality
**Objective:** 为 Extractor 模块编写单元测试，覆盖所有选择器和过滤器。
**Output:** `crates/runtime/src/extractor/` 的单元测试。
**Guidance:** 测试 CSS 选择器、JSONPath、Regex、所有过滤器函数。使用静态数据。

1. 为 `selector/css.rs` 编写测试（各种 CSS 选择器场景）
2. 为 `selector/json.rs` 编写测试（JSONPath 表达式）
3. 为 `selector/regex.rs` 编写测试（正则捕获组）
4. 为 `filter/` 下所有过滤器编写测试
5. 为 `engine.rs` 编写集成测试（完整的提取流程）
6. 运行 `cargo test` 确保通过

### Task 5.3 – 编写单元测试 - 脚本引擎模块 - Agent_Quality
**Objective:** 为所有脚本引擎编写单元测试，验证执行和安全限制。
**Output:** `crates/runtime/src/script/` 的单元测试。
**Guidance:** 测试脚本执行、内置函数、错误处理、安全限制（内存、超时、文件访问）。

1. 为每个脚本引擎（Rhai/JS/Lua/Python）编写基础执行测试
2. 测试所有内置函数（HTTP、JSON、Base64、正则等）
3. 测试安全限制（超时、内存、文件访问禁止）
4. 测试错误处理（脚本语法错误、运行时错误）
5. 运行 `cargo test` 确保通过

### Task 5.4 – 编写集成测试 - 流程端到端测试 - Agent_Quality
**Objective:** 编写端到端集成测试，验证完整的爬虫流程。
**Output:** `crates/runtime/tests/` 的集成测试文件。
**Guidance:** 测试 Search → Detail → Content 完整流程。使用静态数据和真实网站（可选）。

1. 创建 `tests/integration_test.rs` 文件
2. 测试 `search` 流程（使用 17xiaoshuo 规则和静态 HTML）
3. 测试 `detail` 流程（Book/Video/Audio/Manga）
4. 测试 `content` 流程（各种媒体类型）
5. 测试 `discovery` 和 `login` 流程
6. （可选）测试真实网站（标记为 `#[ignore]`，手动运行）
7. 运行 `cargo test` 确保通过

### Task 5.5 – 编写 Benchmark 性能测试 - Agent_Quality
**Objective:** 使用 criterion 编写 Benchmark 测试，衡量性能指标。
**Output:** `crates/runtime/benches/` 的 Benchmark 文件。
**Guidance:** Benchmark 提取速度、并发吞吐量、内存使用。对比优化前后的性能。

1. 添加 `criterion` 依赖到 `Cargo.toml` 的 `[dev-dependencies]`
2. 创建 `benches/extract_bench.rs`（Benchmark 提取器性能）
3. 创建 `benches/flow_bench.rs`（Benchmark 流程执行性能）
4. 创建 `benches/concurrent_bench.rs`（Benchmark 并发爬取吞吐量）
5. 运行 `cargo bench` 生成性能报告
6. 记录关键指标（ops/sec、内存使用）

### Task 5.6 – 配置 CI/CD - GitHub Actions - Agent_Quality
**Objective:** 配置 GitHub Actions 自动运行测试、Clippy、格式检查。
**Output:** `.github/workflows/ci.yml` 文件。
**Guidance:** 配置多个 job：test、clippy、fmt、benchmark（可选）。支持多平台（Linux/Windows/macOS）。

1. 创建 `.github/workflows/ci.yml` 文件
2. 配置 `test` job（运行 `cargo test --workspace`）
3. 配置 `clippy` job（运行 `cargo clippy -- -D warnings`）
4. 配置 `fmt` job（运行 `cargo +nightly fmt -- --check`）
5. （可选）配置 `benchmark` job（运行 `cargo bench` 并上传结果）
6. 配置矩阵测试（Rust stable/nightly，OS: ubuntu/windows/macos）
7. 提交并验证 CI 流水线运行成功

### Task 5.7 – 补全 Rustdoc 文档注释 - Agent_Quality
**Objective:** 为所有公开 API 补全 Rustdoc 文档注释，包含示例代码。
**Output:** 完整的 Rustdoc 文档。
**Guidance:** 为所有 pub 函数、结构体、枚举添加文档注释。包含 `# Examples` 代码示例。生成文档验证。

1. 审查 `crawler-schema` crate 的所有公开 API，补全文档注释
2. 审查 `crawler-runtime` crate 的所有公开 API，补全文档注释
3. 为关键 API 添加 `# Examples` 代码示例
4. 为复杂结构添加 `# Note` 说明使用注意事项
5. 运行 `cargo doc --open` 验证文档生成正确
6. 运行 `cargo test --doc` 验证示例代码可编译

### Task 5.8 – 编写架构文档 - Agent_Quality
**Objective:** 编写项目架构文档，说明设计理念、模块职责、扩展指南。
**Output:** `docs/architecture.md` 文件。
**Guidance:** 包含核心设计理念、模块结构图、数据流图、扩展指南（如何添加新过滤器/脚本引擎）、性能优化指南。

1. 创建 `docs/` 目录和 `architecture.md` 文件
2. 编写核心设计理念章节（Schema/Runtime 分离、原子步骤、零拷贝等）
3. 绘制模块职责图和数据流图（使用 Mermaid 或 ASCII）
4. 编写扩展指南（如何添加新的过滤器、选择器、脚本引擎）
5. 编写性能优化指南（生命周期使用、DashMap 最佳实践、并发控制）
6. 更新 README.md 链接到架构文档

### Task 5.9 – 集成 tracing 日志系统 - Agent_Quality
**Objective:** 在所有模块中集成 tracing 日志，支持结构化日志和性能追踪。
**Output:** 全模块 tracing 集成。
**Guidance:** 使用 `tracing` 和 `tracing-subscriber`。在关键路径添加 span 和 event。支持日志级别配置。

1. 添加 `tracing` 和 `tracing-subscriber` 依赖
2. 在 `CrawlerRuntime::new` 中初始化 tracing subscriber
3. 在所有流程执行器中添加 span（`#[instrument]` 宏）
4. 在关键操作添加 event（HTTP 请求、脚本执行、提取步骤）
5. 在错误处理中添加 error event
6. 在示例代码中演示日志输出
7. 添加文档说明如何配置日志级别

### Task 5.10 – 错误处理增强 - anyhow 集成 - Agent_Quality
**Objective:** 增强错误处理，使用 anyhow 提供错误上下文链。
**Output:** 改进的错误处理。
**Guidance:** 使用 `anyhow::Context` 为错误添加上下文信息。提供用户友好的错误消息，指向规则文件的具体位置。

1. 为 `RuntimeError` 添加更多变体（细化错误类型）
2. 在错误返回时使用 `.context()` 添加上下文信息
3. 在流程执行器中添加规则路径信息到错误（如 "Task search.fields.title.steps[2] failed"）
4. 实现错误格式化，提供用户友好的错误消息
5. 创建测试用例验证错误消息质量


---

## Phase 1 补充：缓存系统设计

### Task 1.7 – 添加缓存配置到 Schema - Agent_Schema
**Objective:** 为 Schema 添加缓存配置支持，包括缓存策略、TTL、存储类型选择。
**Output:** 新文件 `crates/schema/src/config/cache.rs` 和更新的 `extract.rs`。
**Guidance:** 定义 `CacheConfig` 结构体（策略、TTL、最大容量），在 `Meta` 中添加全局缓存配置，在 `ExtractStep` 中添加 `cache_get` 和 `cache_set` 步骤类型。

1. 创建 `crates/schema/src/config/cache.rs`，定义 `CacheConfig` 结构体（字段：`ttl_seconds: Option<u64>`、`max_capacity: Option<usize>`、`storage_type: StorageType`）
2. 定义 `StorageType` 枚举：`Memory`（内存缓存）、`Persistent`（持久化缓存，通过 trait 提供）
3. 在 `crates/schema/src/config/meta.rs` 中为 `Meta` 添加 `cache: Option<CacheConfig>` 字段
4. 在 `crates/schema/src/extract.rs` 的 `ExtractStep` 枚举中添加两个新变体：`CacheGet { key: Template }` 和 `CacheSet { key: Template, ttl: Option<u64> }`
5. 运行 `cargo test --package crawler-schema` 验证序列化正常
6. 更新 JSON Schema 生成

### Task 1.8 – 实现缓存存储 Trait 和内存实现 - Agent_Core
**Objective:** 定义缓存存储 trait，实现内存 LRU 缓存。
**Output:** 新文件 `crates/runtime/src/cache/mod.rs`、`storage.rs`、`memory.rs`。
**Guidance:** 定义 `CacheStorage` trait（get/set/delete/clear 方法），实现 `MemoryCacheStorage` 使用 `quick_cache` 或 `moka`。支持 TTL 和容量限制。**Depends on: Task 1.7 Output**

1. 创建 `crates/runtime/src/cache/mod.rs` 定义模块结构
2. 创建 `crates/runtime/src/cache/storage.rs`，定义 `CacheStorage` trait：
   - `async fn get(&self, key: &str) -> Result<Option<Value>>`
   - `async fn set(&self, key: &str, value: Value, ttl: Option<Duration>) -> Result<()>`
   - `async fn delete(&self, key: &str) -> Result<()>`
   - `async fn clear(&self) -> Result<()>`
3. 创建 `crates/runtime/src/cache/memory.rs`，实现 `MemoryCacheStorage` 使用 `quick_cache` 的 `Cache` 类型
4. 支持 TTL（使用过期时间戳）和容量限制
5. 实现线程安全（使用 `Arc` 包装）
6. 创建单元测试验证内存缓存功能

### Task 1.9 – 实现缓存存储的提取步骤执行器 - Agent_Core
**Objective:** 为 `CacheGet` 和 `CacheSet` 步骤实现执行器。
**Output:** 更新的 `crates/runtime/src/extractor/executor.rs`。
**Guidance:** 在 `StepExecutorFactory` 中添加 `CacheGet` 和 `CacheSet` 的处理。通过 `Context` 访问缓存服务。**Depends on: Task 1.8 Output**

1. 在 `crates/runtime/src/extractor/executor.rs` 的 `StepExecutorFactory::create` 中添加 `ExtractStep::CacheGet` 和 `ExtractStep::CacheSet` 分支
2. 实现 `CacheGetExecutor`：
   - 渲染 key 模板
   - 通过 `context.services.cache_storage.get(key)` 获取缓存值
   - 如果缓存命中，返回 `ExtractValue::Json(value)`；否则返回 `ExtractValue::Null`
3. 实现 `CacheSetExecutor`：
   - 渲染 key 模板
   - 将当前 input 转换为 JSON Value
   - 通过 `context.services.cache_storage.set(key, value, ttl)` 存储缓存
   - 返回原始 input（透传）
4. 在 `Services` 结构体中添加 `cache_storage: Arc<dyn CacheStorage>` 字段
5. 创建测试用例验证缓存步骤功能

### Task 1.10 – 添加持久化缓存 Provider Trait - Agent_Core
**Objective:** 定义持久化缓存 Provider trait，供 App 层实现。
**Output:** 新文件 `crates/runtime/src/cache/persistent.rs`。
**Guidance:** 定义 `PersistentCacheProvider` trait，提供依赖注入接口。类似 `WebViewProvider` 模式。**Depends on: Task 1.8 Output**

1. 创建 `crates/runtime/src/cache/persistent.rs`
2. 定义 `PersistentCacheProvider` trait，继承 `CacheStorage`，添加持久化特定方法：
   - `async fn flush(&self) -> Result<()>` - 强制刷新到磁盘
   - `async fn size(&self) -> Result<u64>` - 获取缓存大小
3. 定义 `SharedPersistentCacheProvider` 类型别名：`Arc<dyn PersistentCacheProvider + Send + Sync>`
4. 实现 `noop_persistent_cache_provider()` 函数返回一个空实现（不支持持久化时使用）
5. 在 `Services::from_rule` 中根据 `CacheConfig.storage_type` 选择内存或持久化缓存
6. 更新 `CrawlerRuntime` 的构建器支持注入 `PersistentCacheProvider`
7. 添加文档注释说明如何在 App 层实现持久化缓存

### Task 1.11 – 集成缓存到 HTTP 客户端 - Agent_Core
**Objective:** 在 HTTP 客户端层集成缓存，自动缓存 HTTP 响应。
**Output:** 更新的 `crates/runtime/src/http/client.rs`。
**Guidance:** 在 `HttpClient` 中添加可选的响应缓存功能。根据 URL 和参数生成缓存 key，请求前查缓存，响应后存缓存。**Depends on: Task 1.8 Output**

1. 在 `HttpClient` 结构体中添加 `cache_storage: Option<Arc<dyn CacheStorage>>` 字段
2. 实现缓存 key 生成函数：`fn cache_key(url: &str, method: &str, body: &Option<String>) -> String`（使用 URL + method + body hash）
3. 在 `HttpClient::request` 方法中添加缓存逻辑：
   - 对于 GET 请求，先查询缓存
   - 如果缓存命中且未过期，直接返回缓存响应
   - 如果缓存未命中，发起 HTTP 请求，响应后存入缓存（根据配置的 TTL）
4. 支持通过 HTTP 头（`Cache-Control`）控制是否缓存
5. 添加配置选项控制是否启用 HTTP 缓存
6. 创建测试用例验证 HTTP 缓存功能

# Ying-Ju Crawler Schema

这个包提供了 Ying-Ju 媒体爬虫规范的 Rust 类型定义和 JSON Schema 自动生成工具。

## 项目概述

Ying-Ju 是一个专为可视化规则编辑器设计的媒体爬虫框架，强调结构的明确性、步骤的原子性和流程的模块化。其核心理念包括：

1. **原子化步骤 (Atomic Steps)**：每个 `Step` 只执行一个最小化的、单一的操作。
2. **显式数据流 (Explicit Data Flow)**：每个步骤都明确定义其输入 (`input`) 和输出 (`output`)，数据在变量上下文中的流动清晰可见。
3. **可重用组件 (Reusable Components)**：允许将常用的管道封装成可复用的组件。
4. **流程驱动 (Flow-Driven)**：以用户可自定义的"流程 (`Flow`)"为核心，替代固定的 `discover` 和 `search` 入口。

## 功能特性

### 1. Rust 类型定义

`src/lib.rs` 及其子模块包含完整的 Rust 结构定义：

- `RuleFile` - 规则文件根结构
- `Meta` - 元数据
- `HttpConfig` - HTTP 配置
- `Step` - 管道步骤类型
- `Flow` - 可执行流程
- `Component` - 可重用组件
- 以及其他所有支持类型

### 2. JSON Schema 自动生成

使用 `schemars` 库自动从 Rust 类型生成 JSON Schema：

```bash
cd ying-ju-crawler-schema
cargo run --bin generate_schema
```

这会在 `../ying-ju-crawler-docs/docs/schema/schema.json` 生成最新的 Schema 文件。

## 优势

✅ **单一真实来源 (Single Source of Truth)**
- 所有类型定义在 Rust 中
- Schema 自动从 Rust 代码生成
- 永远不会产生不一致

✅ **易于维护**
- 修改 Rust 类型定义后，运行生成命令即可更新 Schema
- 无需手动编辑 JSON Schema

✅ **类型安全**
- Rust 编译器保证类型一致性
- Serde 自动处理序列化

✅ **自动文档**
- Rust 文档注释自动转为 JSON Schema 描述

## 模板字符串规范

所有输入字段均采用模板字符串格式，支持变量插值。

- 语法：`{{ variable }}` 用于插入变量值。
- 示例：
  - `https://example.com/search?q={{ keyword }}`
  - `User-Agent: MyBot/{{ version }}`
- 支持表达式与嵌套：如 `{{ user.name }}`、`{{ items[0] }}`
- 运行时会自动渲染所有模板字符串。

## 使用流程

### 1. 修改类型定义

在相应的 Rust 文件中修改类型定义，例如在 `src/pipeline/mod.rs` 中添加新的步骤类型：

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Step {
    // 现有类型...
    
    // 新类型
    NewType {
        param1: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        param2: Option<String>,
    },
}
```

### 2. 生成新的 Schema

```bash
cd ying-ju-crawler-schema
cargo run --bin generate_schema
```

Schema 会自动从更新的 Rust 类型生成。

## 验证规则文件

使用生成的 Schema 验证 TOML 规则文件：

```bash
# 将 TOML 转换为 JSON
python3 -c "import toml, json; \
    data = toml.load('rule.toml'); \
    json.dump(data, open('rule.json', 'w'))"

# 使用 ajv 验证
npm install -g ajv-cli
ajv validate -s ../ying-ju-crawler-docs/docs/schema/schema.json -d rule.json
```

或在 Rust 中使用：

```rust
use crawler_schema::RuleFile;

let content = std::fs::read_to_string("rule.toml")?;
let rule: RuleFile = toml::from_str(&content)?;
// 类型检查保证了有效性
```

## 文件结构

```
ying-ju-crawler-schema/
├── Cargo.toml                      # 包定义
├── src/
│   ├── lib.rs                     # 主库文件和模块声明
│   ├── bin/
│   │   └── generate_schema.rs     # Schema 生成工具
│   ├── core/                      # 核心结构体
│   ├── config/                    # 配置相关类型
│   │   ├── cache.rs
│   │   ├── http.rs
│   │   ├── meta.rs
│   │   └── scripting.rs
│   ├── flow/                      # 流程和组件
│   ├── pipeline/                  # 管道和步骤
│   └── types/                     # 辅助类型和枚举
└── README.md                      # 本文件
```

## 版本同步

包版本和规范版本保持一致：
- `Cargo.toml` 中的版本 = 规范版本
- 当更新规范时，同时更新此包版本

## 扩展建议

未来可以考虑：
1. **更多步骤类型** - 随规范演进添加
2. **动态 Schema** - 基于运行时配置生成
3. **多语言支持** - 为不同语言生成相应的类型定义（Python、TypeScript 等）
4. **自动化测试** - 验证 Schema 的完整性和一致性

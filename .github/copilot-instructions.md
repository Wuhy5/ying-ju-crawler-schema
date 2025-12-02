# Ying-Ju Crawler Schema - Copilot 开发指南

## 项目概述

这是 **Ying-Ju 媒体爬虫 APP** 的核心爬虫规范库，使用 Rust 编写。提供 JSON/TOML 格式的爬虫规则定义及运行时验证。

**核心设计理念**：
- **原子化步骤 (Atomic Steps)**：每个 `Step` 只执行一个最小化操作
- **显式数据流**：通过 `input`/`output` 变量在上下文中传递数据
- **字段驱动**：`FieldRule` 直接定义每个输出字段的提取流程
- **关键模式**：Schema 模块只包含 `#[derive(Serialize, Deserialize, JsonSchema)]` 的纯数据结构，所有运行时逻辑通过扩展 trait 在 `runtime` 模块实现。

## 开发约定

### Serde 标注规范

- 所有结构体使用 `#[serde(deny_unknown_fields)]` 确保严格解析
- 可选字段使用 `#[serde(skip_serializing_if = "Option::is_none")]`
- 枚举使用 `#[serde(rename_all = "snake_case")]` 或 `#[serde(tag = "type")]`

## 模板字符串规范

所有 `Template` 类型字段支持 Tera 模板语法：
- 变量插值：`{{ variable }}`
- 嵌套访问：`{{ user.name }}`、`{{ items[0] }}`
- 运行时通过 `TemplateExt::render()` 渲染

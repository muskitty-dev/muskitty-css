# muskitty-css

[English](README.md) | [简体中文](README.zh-CN.md)

使用安全的 Rust 从零实现的 [CSS Syntax Module Level 3][spec]
词法分析器（tokenizer）与解析器（parser）。这是一个门面（facade）crate，
为方便使用而重新导出 [`muskitty-css-tokenizer`][tok] 与
[`muskitty-css-parser`][par]。它是 [MusKitty][workspace]
浏览器引擎项目的一部分。

[spec]: https://drafts.csswg.org/css-syntax-3/
[tok]: https://github.com/muskitty-dev/muskitty-css-tokenizer
[par]: https://github.com/muskitty-dev/muskitty-css-parser
[workspace]: https://github.com/Ink-dark/MusKitty

## 状态

| 阶段 | 规范覆盖范围 | 测试 |
|-------|---------------|-------|
| Tokenizer (§4.3) | §4.3.1–§4.3.13 全部完成 | 单元测试 + html5lib 风格的 fixture 测试 |
| Parser (§5) | §5.2、§5.3、§5.4.1–§5.4.10、§5.5.1–§5.5.11 全部完成 | 74 个单元测试 |

## 安装

```toml
[dependencies]
muskitty-css = "0.4"
```

MSRV：Rust 1.82+（使用了 `Option::is_none_or` 及其他近期才稳定的 stdlib API）。

## 快速上手

```rust
use muskitty_css::{parse_stylesheet, tokenize};
use muskitty_css::parser::Rule;
use muskitty_css::tokenizer::Token;

// 仅进行词法分析。
let tokens = tokenize("color: red;");
assert!(matches!(tokens[0], Token::Ident(_)));

// 解析完整的样式表。
let ss = parse_stylesheet("a { color: red; } @media print { p { font: 16px } }");
assert_eq!(ss.rules.len(), 2);

// 检视第一条规则的 prelude。
if let Rule::QualifiedRule(qr) = &ss.rules[0] {
    println!("selector had {} component values", qr.prelude.len());
    println!("rule has {} declarations", qr.declarations.len());
}
```

其他顶层入口点：[`parse_rule`](crate::parse_rule)、
[`parse_declaration`](crate::parse_declaration)、
[`parse_component_value`](crate::parse_component_value)、
[`parse_list_of_component_values`](crate::parse_list_of_component_values)、
[`parse_comma_separated_list_of_component_values`](crate::parse_comma_separated_list_of_component_values)。

## 架构

本 crate 遵循 CSS Syntax §3.1 的两阶段模型：

1. **词法分析（Tokenization）**（`tokenizer` 模块）——消费一串 Unicode
   码点（在经过 §5.3 预处理之后），并按照 §4.3 "Tokenizer Algorithms"
   产出 `Token`：`consume_a_token` (§4.3.1) 会分派到若干子算法，例如
   `consume_an_ident_like_token` (§4.3.4)、`consume_a_numeric_token`
   (§4.3.3)、`consume_a_string_token` (§4.3.5) 以及
   `consume_a_url_token` (§4.3.6)。
2. **解析（Parsing）**（`parser` 模块）——消费 token 流，并依据 §5
   生成 CSS 对象（Stylesheet、Rule、Declaration、ComponentValue）。
   - `parser::types` —— §5.2 数据结构。
   - `parser::token_stream` —— §5.3 `TokenStream`，带有用于回溯的
     mark/restore 能力。
   - `parser::algorithms` —— §5.5 共 11 个解析器算法。
   - `parser::entry_points` —— §5.4 共 9 个入口点（§5.4.1 / §5.4.2
     的 grammar hooks 暂未实现）。

## 规范来源

本实现所依据的权威规范来源：
`D:\CSSWG\css-syntax-3\Overview.md`（编辑草案的 Markdown 渲染版本）。
代码注释中所有的行号均引用自此文件。

公开规范：<https://drafts.csswg.org/css-syntax-3/>

## 许可证

Apache-2.0，与 workspace 根仓库一致。

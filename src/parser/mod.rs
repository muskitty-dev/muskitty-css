//! CSS Syntax parser — re-export shim.
//!
//! The parser implementation has been extracted into the standalone
//! `muskitty-css-parser` crate (independent git repository, published
//! to crates.io). This module re-exports the public API so that existing
//! `crate::parser::*` references inside `muskitty-css` (and downstream
//! crates using `muskitty_css::parser::*`) continue to resolve without
//! modification.
//!
//! See `crates/muskitty-css-parser/` for the implementation.
//! Spec: CSS Syntax Module Level 3 §5 "Parser Algorithms".

pub use muskitty_css_parser::{
    algorithms::BlockContents,
    entry_points::{
        parse_a_blocks_contents, parse_a_comma_separated_list_of_component_values,
        parse_a_component_value, parse_a_declaration, parse_a_list_of_component_values,
        parse_a_rule, parse_a_stylesheet, parse_a_stylesheets_contents,
    },
    grammar::{parse_a_comma_separated_list_with_grammar, parse_a_grammar, Grammar},
    token_stream::TokenStream,
    types::{
        AtRule, BlockKind, ComponentValue, Declaration, Function, ParseError, QualifiedRule, Rule,
        SimpleBlock, Stylesheet,
    },
};

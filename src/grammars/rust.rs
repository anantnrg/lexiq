use crate::grammar::{Grammar, Rule, Syntax};
use crate::rule;

pub struct RustLang;

impl RustLang {
    pub fn grammar() -> Grammar {
        Grammar {
            name: String::from("Rust"),
            extensions: vec![String::from("rs")],
            syntax: Syntax {
                keywords: Self::keywords(),
                punctuation: Self::punctuation(),
                data: Self::data(),
                comments: Self::comments(),
            },
        }
    }
    pub fn keywords() -> Vec<Rule> {
        vec![
            rule!(r#"\b[a-z_][a-z0-9_]*\b"#, "keywords.identifier"),
            rule!(
                r#"\b[A-Z][a-z0-9]*([A-Z][a-z0-9]*)*\b"#,
                "keywords.camelcase-ident"
            ),
            rule!(r#"\b[A-Z0-9_]+\b"#, "keywords.upper-snakecase"),
            rule!(r#"\b[iu](?:8|16|32|64|128|size)\b"#, "keywords.integer"),
            rule!(r#"\bf(32|64)\b"#, "keywords.float"),
            rule!(r#"\b[A-Z][a-zA-Z0-9_]*\b"#, "keywords.type"),
        ]
    }
    pub fn punctuation() -> Vec<Rule> {
        vec![]
    }
    pub fn data() -> Vec<Rule> {
        vec![
            rule!(
                r#"\b\d+(\_\d+)*([uUiIfF](8|16|32|64|size)?)?\b"#,
                "data.decimal"
            ),
            rule!(r#"\b\d+(\.\d+)?[eE][+-]?\d+\b"#, "data.float"),
            rule!(r#"'\w+"#, "data.lifetime"),
        ]
    }
    pub fn comments() -> Vec<Rule> {
        vec![
            rule!(r#"\/\/.*"#, "comments.line"),
            rule!(r#"\/\*[\s\S]*?\*\/"#, "comments.block"),
        ]
    }
}

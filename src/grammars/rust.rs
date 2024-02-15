use crate::grammar::{Grammar, Rule};
use crate::rule;

pub struct RustLang;

const COMMENTS: usize = 0;
const STRINGS: usize = 1;
const KEYWORDS: usize = 2;
const TYPES: usize = 3;
const LIFETIMES: usize = 4;
const IDENTIFIERS: usize = 5;
const NUMBERS: usize = 6;
const PUNCTUATION: usize = 7;

impl RustLang {
    pub fn grammar() -> Grammar {
        let mut rules = Vec::new();
        rules.extend(Self::keywords());
        rules.extend(Self::comments());
        rules.extend(Self::data());
        rules.extend(Self::punctuation());
        Grammar {
            name: String::from("Rust"),
            extensions: vec![String::from("rs")],
            rules,
        }
    }
    pub fn keywords() -> Vec<Rule> {
        vec![
            rule!(
                r#"\b[a-z_][a-z0-9_]*\b"#,
                "keywords.identifier",
                IDENTIFIERS
            ),
            rule!(
                r#"\b[A-Z][a-z0-9]*([A-Z][a-z0-9]*)*\b"#,
                "keywords.camelcase-ident",
                IDENTIFIERS
            ),
            rule!(r#"\b[A-Z0-9_]+\b"#, "keywords.upper-snakecase", IDENTIFIERS),
            rule!(
                r#"\b[iu](?:8|16|32|64|128|size)\b"#,
                "keywords.integer",
                TYPES
            ),
            rule!(r#"\bf(32|64)\b"#, "keywords.float", TYPES),
            rule!(r#"\b[A-Z][a-zA-Z0-9_]*\b"#, "keywords.type", TYPES),
        ]
    }
    pub fn punctuation() -> Vec<Rule> {
        vec![]
    }
    pub fn data() -> Vec<Rule> {
        vec![
            rule!(
                r#"\b\d+(\_\d+)*([uUiIfF](8|16|32|64|size)?)?\b"#,
                "data.decimal",
                NUMBERS
            ),
            rule!(r#"\b\d+(\.\d+)?[eE][+-]?\d+\b"#, "data.float", NUMBERS),
            rule!(r#"'\w+"#, "data.lifetime", LIFETIMES),
        ]
    }
    pub fn comments() -> Vec<Rule> {
        vec![
            rule!(r#"\/\/.*"#, "comments.line", COMMENTS),
            rule!(r#"\/\*[\s\S]*?\*\/"#, "comments.block", COMMENTS),
        ]
    }
}

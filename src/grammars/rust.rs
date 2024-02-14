use crate::grammar::{Grammar, Rule, Syntax};
use crate::rule;

pub struct RustLang;

const NON_RAW_INDENTIFIER: &'static str = r#"[[:alpha:]][_[:alnum:]]*|_[_[:alnum:]]+"#;
const IDENTIFIER: &'static str = r#"(?:(?:(?:r\#)?{{non_raw_ident}})\b)"#;
const CAMEL_CASE_IDENTIFIER: &'static str = r#"\b_*[A-Z][a-zA-Z0-9_]*[a-z][a-zA-Z0-9_]*\b"#;
const LIFETIME: &'static str = r#"'(?:_|{{non_raw_ident}})(?!\')\b"#;
const ESCAPED_BYTE: &'static str = r#"\\([nrt0\"'\\]|x\h{2})"#;
const ESCAPED_CHAR: &'static str = r#"\\([nrt0\"''\\]|x[0-7]\h|u\{(?:\h_*){1,6}\})"#;
const INTEGER_SUFFIXES: &'static str = r#"[iu](?:8|16|32|64|128|size)"#;
const FLOAT_SUFFIXES: &'static str = r#"f(32|64)"#;
const FLOAT_EXPONENT: &'static str = r#"[eE][+-]?[0-9_]*[0-9][0-9_]*"#;
const DECIMAL_LITERAL: &'static str = r#"[0-9](?:[0-9_])*"#;
const TYPE_IDENTIFIER: &'static str =
    r#"\b(?:[[:upper:]]|_*[[:upper:]][[:alnum:]_]*[[:lower:]][[:alnum:]_]*)\b"#;

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
    pub fn keywords() -> Vec<Rule> {}
    pub fn punctuation() -> Vec<Rule> {}
    pub fn data() -> Vec<Rule> {}
    pub fn comments() -> Vec<Rule> {}
}

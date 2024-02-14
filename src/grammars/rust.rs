use crate::grammar::{Grammar, Rule, Syntax};

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
    pub fn keywords() -> Vec<Rule> {}
    pub fn punctuation() -> Vec<Rule> {}
    pub fn data() -> Vec<Rule> {}
    pub fn comments() -> Vec<Rule> {}
}

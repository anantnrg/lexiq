use crate::grammar::{Grammar, Rule, Syntax};

pub struct RustLang;

const NonRawIndentifier: &'static str = "";
const Identifier: &'static str = "";
const CamelCaseIdentifier: &'static str = "";
const Lifetime: &'static str = "";
const EscapedByte: &'static str = "";
const IntegerSuffixes: &'static str = "";
const FloatSuffixes: &'static str = "";
const FloatExponent: &'static str = "";
const DecimalLiteral: &'static str = "";
const TypeIdentifier: &'static str = "";

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

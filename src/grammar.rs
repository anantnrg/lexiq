use std::{fs::File, io::Read, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Grammar {
    pub name: String,
    pub extensions: Vec<String>,
    pub syntax: Syntax,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Syntax {
    pub keywords: Vec<Rule>,
    pub punctuation: Vec<Rule>,
    pub data: Vec<Rule>,
    pub comments: Vec<Rule>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Rule {
    #[serde(rename = "match")]
    pub regex: String,
    pub scope: String,
}

#[derive(Debug, Clone)]
pub struct Match {
    pub start: usize,
    pub end: usize,
    pub scope: String,
}

impl Grammar {
    pub fn new(name: String, extensions: Vec<String>, syntax: Syntax) -> Self {
        Grammar {
            name,
            extensions,
            syntax,
        }
    }

    pub fn get_comments(&self, code: String) -> Vec<Match> {
        vec![]
    }

    pub fn default() -> Self {
        Grammar {
            name: String::from("Rust"),
            extensions: vec!["rs".to_string()],
            syntax: Syntax {
                data: vec![],
                keywords: vec![],
                punctuation: vec![],
                comments: vec![],
            },
        }
    }
}

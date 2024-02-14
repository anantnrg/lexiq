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
    pub fn new(grammar: PathBuf) -> Self {
        let mut file = String::new();
        File::open(grammar)
            .expect("Couldn't find the specified grammar file")
            .read_to_string(&mut file)
            .expect("Couldn't parse the given file");

        let grammar = serde_yaml::from_str(&file)
            .expect("Failed to parse into struct Grammar. Is the file a YAML file?");

        println!("{:?}", grammar);

        grammar
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

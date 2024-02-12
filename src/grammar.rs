use std::{fs::File, io::Read, path::PathBuf};

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Grammar {
    pub name: String,
    pub extensions: Vec<String>,
    pub groups: Vec<Group>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Group {
    pub name: String,
    pub rules: Vec<Rule>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Rule {
    pub regex: String,
    pub scope: String,
}

impl Grammar {
    pub fn new(name: String, grammar: PathBuf) -> Self {
        let mut file = String::new();
        File::open(grammar)
            .expect("Couldn't find the specified grammar file")
            .read_to_string(&mut file)
            .expect("Couldn't parse the given file");

        let grammar: Grammar = serde_yaml::from_str(&file)
            .expect("Failed to parse into struct Grammar. Is the file a YAML file?");

        println!("{:?}", grammar);

        grammar
    }
}

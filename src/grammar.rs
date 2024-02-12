use std::{fs::File, io::Read, path::PathBuf};

use serde::Deserialize;
use serde::Deserializer;

#[derive(Deserialize, Debug, Clone)]
pub struct Grammar {
    pub name: String,
    pub extensions: Vec<String>,
    pub syntax: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct Group {
    pub rules: Vec<Rule>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Rule {
    pub regex: String,
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
}

impl<'de> Deserialize<'de> for Group {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Inner {
            #[serde(flatten)]
            rules: Vec<Rule>,
        }

        let inner = Inner::deserialize(deserializer)?;
        Ok(Group { rules: inner.rules })
    }
}

use std::{fs::File, io::Read, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Grammar {
    pub name: String,
    pub extensions: Vec<String>,
    pub rules: Vec<Rule>,
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

#[macro_export]
macro_rules! rule {
    ($regex:expr, $scope:expr) => {
        Rule {
            regex: $regex.to_string(),
            scope: $scope.to_string(),
        }
    };
}

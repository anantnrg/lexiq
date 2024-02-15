use regex::Regex;

#[derive(Debug, Clone)]
pub struct Grammar {
    pub name: String,
    pub extensions: Vec<String>,
    pub rules: Vec<Rule>,
    pub sorted: bool,
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub regex: String,
    pub scope: String,
    pub precedence: usize,
}

#[derive(Debug, Clone)]
pub struct Match {
    pub start: usize,
    pub end: usize,
    pub scope: String,
}

#[derive(Debug, Clone)]
pub(crate) struct CompiledGrammar {
    pub(crate) rules: Vec<(String, Regex)>,
}

#[macro_export]
macro_rules! rule {
    ($regex:expr, $scope:expr, $precedence:expr) => {
        Rule {
            regex: $regex.to_string(),
            scope: $scope.to_string(),
            precedence: $precedence,
        }
    };
}

impl Grammar {
    pub fn sort(&mut self) {
        self.rules.sort_by_key(|rule| rule.precedence);
    }

    #[allow(private_interfaces)]
    pub fn compile(&mut self) -> CompiledGrammar {
        let mut compiled_rules = Vec::new();

        for rule in &self.rules {
            if let Ok(regex) = Regex::new(&rule.regex) {
                compiled_rules.push((rule.scope.clone(), regex))
            }
        }

        CompiledGrammar {
            rules: compiled_rules,
        }
    }
}

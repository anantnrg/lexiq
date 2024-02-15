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
        println!("before sort: {:?}", self.rules);
        self.rules.sort_by_key(|rule| rule.precedence);
        println!("after sort: {:?}", self.rules);
    }
}

#[derive(Debug, Clone)]
pub struct Grammar {
    pub name: String,
    pub extensions: Vec<String>,
    pub groups: Vec<Group>,
}

#[derive(Debug, Clone)]
pub struct Group {
    pub name: String,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub regex: String,
    pub scope: String,
}

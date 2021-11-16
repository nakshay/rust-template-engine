use crate::pair::Pair;
use std::collections::HashMap;
use std::fmt::Display;

pub enum TemplateVariable {
    Boolean(bool),
    Displayable(Box<dyn Display>),
    Double(f64),
    Integer(isize),
    String(String),
    SubContext(Context),
}

pub struct Context {
    pub vars: HashMap<String, TemplateVariable>,
}

impl Context {
    pub fn with(pairs: Vec<Pair>) -> Self {
        let mut vars = HashMap::new();
        for pair in pairs {
            vars.insert(pair.0, pair.1);
        }
        Self { vars }
    }
}

use crate::template::{Context, TemplateVariable};
use std::fmt::Display;

pub struct Pair(pub String, pub TemplateVariable);

pub trait PairGenerator<T> {
    fn of(key: &str, value: T) -> Pair;
}

impl PairGenerator<bool> for Pair {
    fn of(key: &str, value: bool) -> Pair {
        Pair(String::from(key), TemplateVariable::Boolean(value))
    }
}

impl PairGenerator<Box<dyn Display>> for Pair {
    fn of(key: &str, value: Box<dyn Display>) -> Pair {
        Pair(String::from(key), TemplateVariable::Displayable(value))
    }
}

impl PairGenerator<f64> for Pair {
    fn of(key: &str, value: f64) -> Pair {
        Pair(String::from(key), TemplateVariable::Double(value))
    }
}

impl PairGenerator<Context> for Pair {
    fn of(key: &str, value: Context) -> Pair {
        Pair(String::from(key), TemplateVariable::SubContext(value))
    }
}

impl PairGenerator<isize> for Pair {
    fn of(key: &str, value: isize) -> Pair {
        Pair(String::from(key), TemplateVariable::Integer(value))
    }
}

impl PairGenerator<String> for Pair {
    fn of(key: &str, value: String) -> Pair {
        Pair(String::from(key), TemplateVariable::String(value))
    }
}

impl PairGenerator<&str> for Pair {
    fn of(key: &str, value: &str) -> Pair {
        Pair(
            String::from(key),
            TemplateVariable::String(String::from(value)),
        )
    }
}

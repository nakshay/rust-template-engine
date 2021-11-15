use std::collections::HashMap;
use std::fmt::{self, Display};

pub enum TemplateVariable {
    Boolean(bool),
    Displayable(Box<dyn Display>),
    Double(f64),
    Integer(isize),
    String(String),
    SubContext(Context),
}

trait PairGenerator<T> {
    fn of(key: &str, value: T) -> Pair;
}

pub struct Pair(String, TemplateVariable);

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

pub struct Context {
    vars: HashMap<String, TemplateVariable>,
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

pub fn render(template: String, context: Context) -> String {
    let mut output = template.clone();

    for (_key, value) in &context.vars {
        let key = &format!("{{{{ {} }}}}", _key);
        match value {
            TemplateVariable::Displayable(value) => {
                output = output.replace(key, &format!("{}", value));
            },
            TemplateVariable::SubContext(_) => {},
            TemplateVariable::Boolean(value) => {
                output = output.replace(key, &format!("{}", value));
            },
            TemplateVariable::Double(value) => {
                output = output.replace(key, &format!("{}", value));
            },
            TemplateVariable::Integer(value) => {
                output = output.replace(key, &format!("{}", value));
            },
            TemplateVariable::String(value) => {
                output = output.replace(key, &format!("{}", value));
            },
        };
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_pass_a_string_to_the_template_and_print_it() {
        let context = Context::with(vec![Pair::of("name", "Rust")]);
        let output = render(s("name: {{ name }}"), context);
        assert_eq!(output, s("name: Rust"));
    }

    #[test]
    fn can_pass_a_boolean_to_the_template_and_print_it() {
        let context = Context::with(vec![Pair::of("is awesome", true)]);
        let output = render(s("is awesome: {{ is awesome }}"), context);
        assert_eq!(output, s("is awesome: true"));
    }

    #[test]
    fn can_pass_a_displayable_object_to_the_template_and_print_it() {
        let context = Context::with(vec![Pair::of(
            "displayable object",
            get_displayable_object(),
        )]);
        let output = render(s("displayable object: {{ displayable object }}"), context);
        assert_eq!(output, s("displayable object: bar"));
    }

    #[test]
    fn can_pass_a_double_to_the_template_and_print_it() {
        let context = Context::with(vec![Pair::of("a double", 1.2)]);
        let output = render(s("a double: {{ a double }}"), context);
        assert_eq!(output, s("a double: 1.2"));
    }


    fn can_pass_a_hashmap_to_the_template_and_print_it() {
        let context = Context::with(vec![Pair::of(
            "a map",
            Context::with(vec![Pair::of(
                "nested",
                Context::with(vec![Pair::of("nested nested", "foo")]),
            )]),
        )]);
        let output = render(s("a map: {{ a map.nested.nested nested }}"), context);
        assert_eq!(output, s("a map: foo"));
    }

    #[test]
    fn can_pass_an_integer_to_the_template_and_print_it() {
        let context = Context::with(vec![Pair::of("an integer", 100)]);
        let output = render(s("an integer: {{ an integer }}"), context);
        assert_eq!(output, s("an integer: 100"));
    }

    struct CanBeDisplayed {
        foo: String,
    }

    impl Display for CanBeDisplayed {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.foo)
        }
    }

    fn get_displayable_object() -> Box<dyn Display> {
        Box::new(CanBeDisplayed { foo: s("bar") })
    }

    /// Creates a Sring instance
    fn s(input: &str) -> String {
        String::from(input)
    }
}

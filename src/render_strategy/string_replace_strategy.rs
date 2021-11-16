use crate::template::{Context, TemplateVariable};

pub fn strint_replace_strategy(template: String, context: Context) -> String {
    let mut output = template.clone();

    for (_key, value) in &context.vars {
        let key = &format!("{{{{ {} }}}}", _key);
        match value {
            TemplateVariable::Displayable(value) => {
                output = output.replace(key, &format!("{}", value));
            }
            TemplateVariable::SubContext(_) => {}
            TemplateVariable::Boolean(value) => {
                output = output.replace(key, &format!("{}", value));
            }
            TemplateVariable::Double(value) => {
                output = output.replace(key, &format!("{}", value));
            }
            TemplateVariable::Integer(value) => {
                output = output.replace(key, &format!("{}", value));
            }
            TemplateVariable::String(value) => {
                output = output.replace(key, &format!("{}", value));
            }
        };
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pair::{Pair, PairGenerator};
    use std::fmt::{Display, Formatter, Result};

    #[test]
    fn can_pass_a_string_to_the_template_and_print_it() {
        let context = Context::with(vec![Pair::of("name", "Rust")]);
        let output = strint_replace_strategy(s("name: {{ name }}"), context);
        assert_eq!(output, s("name: Rust"));
    }

    #[test]
    fn can_pass_a_boolean_to_the_template_and_print_it() {
        let context = Context::with(vec![Pair::of("is awesome", true)]);
        let output = strint_replace_strategy(s("is awesome: {{ is awesome }}"), context);
        assert_eq!(output, s("is awesome: true"));
    }

    #[test]
    fn can_pass_a_displayable_object_to_the_template_and_print_it() {
        let context = Context::with(vec![Pair::of(
            "displayable object",
            get_displayable_object(),
        )]);
        let output =
            strint_replace_strategy(s("displayable object: {{ displayable object }}"), context);
        assert_eq!(output, s("displayable object: bar"));
    }

    #[test]
    fn can_pass_a_double_to_the_template_and_print_it() {
        let context = Context::with(vec![Pair::of("a double", 1.2)]);
        let output = strint_replace_strategy(s("a double: {{ a double }}"), context);
        assert_eq!(output, s("a double: 1.2"));
    }

    #[test]
    fn can_pass_a_hashmap_to_the_template_and_print_it() {
        let context = Context::with(vec![Pair::of(
            "a map",
            Context::with(vec![Pair::of(
                "nested",
                Context::with(vec![Pair::of("nested nested", "foo")]),
            )]),
        )]);
        let output = strint_replace_strategy(s("a map: {{ a map.nested.nested nested }}"), context);
        assert_eq!(output, s("a map: foo"));
    }

    #[test]
    fn can_pass_an_integer_to_the_template_and_print_it() {
        let context = Context::with(vec![Pair::of("an integer", 100)]);
        let output = strint_replace_strategy(s("an integer: {{ an integer }}"), context);
        assert_eq!(output, s("an integer: 100"));
    }

    struct CanBeDisplayed {
        foo: String,
    }

    impl Display for CanBeDisplayed {
        fn fmt(&self, f: &mut Formatter) -> Result {
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

pub mod pair;
pub mod render_strategy;
pub mod template;

use template::Context;

pub fn render(
    template: String,
    context: Context,
    strategy: fn(String, Context) -> String,
) -> String {
    strategy(template, context)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pair::{Pair, PairGenerator};
    use render_strategy::string_replace_strategy;
    use std::fmt::{Display, Formatter, Result};

    #[test]
    fn can_pass_a_string_to_the_template_and_print_it() {
        let context = Context::with(vec![Pair::of("name", "Rust")]);
        let output = render(s("name: {{ name }}"), context, string_replace_strategy);
        assert_eq!(output, s("name: Rust"));
    }

    #[test]
    fn can_pass_a_boolean_to_the_template_and_print_it() {
        let context = Context::with(vec![Pair::of("is awesome", true)]);
        let output = render(
            s("is awesome: {{ is awesome }}"),
            context,
            string_replace_strategy,
        );
        assert_eq!(output, s("is awesome: true"));
    }

    #[test]
    fn can_pass_a_displayable_object_to_the_template_and_print_it() {
        let context = Context::with(vec![Pair::of(
            "displayable object",
            get_displayable_object(),
        )]);
        let output = render(
            s("displayable object: {{ displayable object }}"),
            context,
            string_replace_strategy,
        );
        assert_eq!(output, s("displayable object: bar"));
    }

    #[test]
    fn can_pass_a_double_to_the_template_and_print_it() {
        let context = Context::with(vec![Pair::of("a double", 1.2)]);
        let output = render(
            s("a double: {{ a double }}"),
            context,
            string_replace_strategy,
        );
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
        let output = render(
            s("a map: {{ a map.nested.nested nested }}"),
            context,
            string_replace_strategy,
        );
        assert_eq!(output, s("a map: foo"));
    }

    #[test]
    fn can_pass_an_integer_to_the_template_and_print_it() {
        let context = Context::with(vec![Pair::of("an integer", 100)]);
        let output = render(
            s("an integer: {{ an integer }}"),
            context,
            string_replace_strategy,
        );
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

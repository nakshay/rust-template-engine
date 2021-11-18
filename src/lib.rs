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
    use render_strategy;
    use std::fmt::{Display, Formatter, Result};

    #[test]
    fn can_pass_a_string_to_the_template_and_print_it() {
        let context = Context::with(vec![Pair::of("name", "Rust")]);
        let output = render(s("name: {{ name }}"), context, render_strategy::string_replace);
        assert_eq!(output, s("name: Rust"));
    }

    #[test]
    fn can_pass_a_boolean_to_the_template_and_print_it() {
        let context = Context::with(vec![Pair::of("is awesome", true)]);
        let output = render(
            s("is awesome: {{ is awesome }}"),
            context,
            render_strategy::string_replace,
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
            render_strategy::string_replace,
        );
        assert_eq!(output, s("displayable object: bar"));
    }

    #[test]
    fn can_pass_a_double_to_the_template_and_print_it() {
        let context = Context::with(vec![Pair::of("a double", 1.2)]);
        let output = render(
            s("a double: {{ a double }}"),
            context,
            render_strategy::string_replace,
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
            render_strategy::string_replace,
        );
        assert_eq!(output, s("a map: foo"));
    }

    #[test]
    fn can_pass_an_integer_to_the_template_and_print_it() {
        let context = Context::with(vec![Pair::of("an integer", 100)]);
        let output = render(
            s("an integer: {{ an integer }}"),
            context,
            render_strategy::string_replace,
        );
        assert_eq!(output, s("an integer: 100"));
    }

    #[test]
    fn can_print_multiple_variables_inside_the_same_template() {
        let context = Context::with(vec![
            Pair::of(
                "context",
                Context::with(vec![
                    Pair::of(
                        "context",
                        Context::with(vec![
                            Pair::of("displayable", get_displayable_object()),
                            Pair::of("bool", true),
                            Pair::of("f64", 1.23),
                            Pair::of("isize", 123456),
                            Pair::of("string", "foo"),
                            Pair::of("string reference", "bar"),
                        ]),
                    ),
                    Pair::of("displayable", get_displayable_object()),
                    Pair::of("bool", true),
                    Pair::of("f64", 1.23),
                    Pair::of("isize", 123456),
                    Pair::of("string", "foo"),
                    Pair::of("string reference", "bar"),
                ]),
            ),
            Pair::of("displayable", get_displayable_object()),
            Pair::of("bool", true),
            Pair::of("f64", 1.23),
            Pair::of("isize", 123456),
            Pair::of("string", "foo"),
            Pair::of("string reference", "bar"),
        ]);
        let template = s("
            displayable: {{ displayable }}
            bool: {{ bool }}
            f64: {{ f64 }}
            isize: {{ isize }}
            string: {{ string }}
            string reference: {{ string reference }}

            context.displayable: {{ context.displayable }}
            context.bool: {{ context.bool }}
            context.f64: {{ context.f64 }}
            context.isize: {{ context.isize }}
            context.string: {{ context.string }}
            context.string reference: {{ context.string reference }}

            context.context.displayable: {{ context.context.displayable }}
            context.context.bool: {{ context.context.bool }}
            context.context.f64: {{ context.context.f64 }}
            context.context.isize: {{ context.context.isize }}
            context.context.string: {{ context.context.string }}
            context.context.string reference: {{ context.context.string reference }}
        ");
        let output = render(template, context, render_strategy::string_replace);
        let expected_output = s("
            displayable: bar
            bool: true
            f64: 1.23
            isize: 123456
            string: foo
            string reference: bar

            context.displayable: bar
            context.bool: true
            context.f64: 1.23
            context.isize: 123456
            context.string: foo
            context.string reference: bar

            context.context.displayable: bar
            context.context.bool: true
            context.context.f64: 1.23
            context.context.isize: 123456
            context.context.string: foo
            context.context.string reference: bar
        ");
        assert_eq!(output, expected_output);
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

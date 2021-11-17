use crate::template::{Context, TemplateVariable};

pub fn string_replace_strategy(template: String, context: Context) -> String {
    let mut output = template.clone();

    for (_key, value) in &context.vars {
        let key = &format!("{{{{ {} }}}}", _key);
        output = match value {
            TemplateVariable::Displayable(value) => {
                output.replace(key, &format!("{}", value))
            },
            TemplateVariable::SubContext(value) => {
                parse_sub_context(value)
            },
            TemplateVariable::Boolean(value) => {
                output.replace(key, &format!("{}", value))
            },
            TemplateVariable::Double(value) => {
                output.replace(key, &format!("{}", value))
            },
            TemplateVariable::Integer(value) => {
                output.replace(key, &format!("{}", value))
            },
            TemplateVariable::String(value) => {
                output.replace(key, &format!("{}", value))
            },
        };
    }

    output
}

fn parse_sub_context(context: &Context) -> String {
    "".into()
}

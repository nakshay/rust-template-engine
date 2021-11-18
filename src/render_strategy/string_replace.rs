use crate::template::{Context, TemplateVariable};

pub fn string_replace(template: String, context: Context) -> String {
    replace(&template, context, "".into())
}

fn replace(template: &String, context: Context, prepend: String) -> String {
    let mut output = template.clone();

    for (variable_name, wrapper) in context.vars {
        let key = String::from(format!("{{{{ {}{} }}}}", prepend, variable_name));
        output = match wrapper {
            TemplateVariable::Displayable(value) => {
                output.replace(&key, &format!("{}", value))
            },
            TemplateVariable::SubContext(value) => {
                replace(&output, value, format!("{}{}.", prepend, variable_name))
            },
            TemplateVariable::Boolean(value) => {
                output.replace(&key, &format!("{}", value))
            },
            TemplateVariable::Double(value) => {
                output.replace(&key, &format!("{}", value))
            },
            TemplateVariable::Integer(value) => {
                output.replace(&key, &format!("{}", value))
            },
            TemplateVariable::String(value) => {
                output.replace(&key, &format!("{}", value))
            },
        };
    }

    output
}

use crate::template::{Context, TemplateVariable};

pub fn string_replace_strategy(template: String, context: Context) -> String {
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

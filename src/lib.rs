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

use crate::prelude::*;
use biome_css_syntax::{CssAnyFunction, CssAnyFunctionFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssAnyFunction;
impl FormatNodeRule<CssAnyFunction> for FormatCssAnyFunction {
    // TODO: This is really `AnyCssFunction` and will probably get moved to be that later.
    fn fmt_fields(&self, node: &CssAnyFunction, f: &mut CssFormatter) -> FormatResult<()> {
        let CssAnyFunctionFields {
            css_simple_function,
        } = node.as_fields();

        write!(f, [css_simple_function.format()])
    }
}

use crate::prelude::*;
use biome_css_syntax::{CssUrlFunction, CssUrlFunctionFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUrlFunction;
impl FormatNodeRule<CssUrlFunction> for FormatCssUrlFunction {
    fn fmt_fields(&self, node: &CssUrlFunction, f: &mut CssFormatter) -> FormatResult<()> {
        let CssUrlFunctionFields {
            name,
            l_paren_token,
            value,
            modifiers,
            r_paren_token,
        } = node.as_fields();

        let should_insert_space = f.options().delimiter_spacing().value();

        if should_insert_space {
            write!(
                f,
                [
                    name.format(),
                    l_paren_token.format(),
                    space(),
                    value.format(),
                    modifiers.format(),
                    space(),
                    r_paren_token.format()
                ]
            )
        } else {
            write!(
                f,
                [
                    name.format(),
                    l_paren_token.format(),
                    value.format(),
                    modifiers.format(),
                    r_paren_token.format()
                ]
            )
        }
    }
}

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


        let maybe_space = format_with(|f| {
            if f.options().delimiter_spacing().value() {
                write!(f, [space()])?;
            }
            Ok(())
        });


        write!(f, [name.format(), l_paren_token.format(), maybe_space, value.format()])?;

        if value.is_some() && modifiers.iter().next().is_some() {
            write!(f, [space()])?;
        }

        write!(f, [maybe_space, modifiers.format(), r_paren_token.format()])
    }
}

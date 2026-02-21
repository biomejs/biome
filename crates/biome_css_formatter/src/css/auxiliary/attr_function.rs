use crate::prelude::*;
use biome_css_syntax::{CssAttrFunction, CssAttrFunctionFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssAttrFunction;

impl FormatNodeRule<CssAttrFunction> for FormatCssAttrFunction {
    fn fmt_fields(&self, node: &CssAttrFunction, f: &mut CssFormatter) -> FormatResult<()> {
        let CssAttrFunctionFields {
            name_token,
            l_paren_token,
            attr_name,
            attr_type,
            fallback_value,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                name_token.format(),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent(&format_with(|f| {
                        write!(f, [group(&attr_name.format())])?;

                        if let Some(attr_type) = &attr_type {
                            write!(f, [soft_line_break_or_space(), attr_type.format()])?;
                        }

                        write!(f, [fallback_value.format()])?;

                        Ok(())
                    })),
                    r_paren_token.format()
                ])
            ]
        )
    }
}

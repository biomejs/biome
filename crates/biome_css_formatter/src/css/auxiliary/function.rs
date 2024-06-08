use crate::css::value::identifier::FormatCssIdentifierOptions;
use crate::prelude::*;
use biome_css_syntax::{CssFunction, CssFunctionFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssFunction;
impl FormatNodeRule<CssFunction> for FormatCssFunction {
    fn fmt_fields(&self, node: &CssFunction, f: &mut CssFormatter) -> FormatResult<()> {
        let CssFunctionFields {
            name,
            l_paren_token,
            items,
            r_paren_token,
        } = node.as_fields();

        if let Ok(name) = name {
            write!(
                f,
                [name
                    .format()
                    .with_options(FormatCssIdentifierOptions::default().prevent_lowercase(true))]
            )?;
        }

        write!(
            f,
            [group(&format_args![
                l_paren_token.format(),
                soft_block_indent(&items.format()),
                r_paren_token.format()
            ])]
        )
    }
}

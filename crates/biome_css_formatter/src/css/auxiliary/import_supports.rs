use crate::prelude::*;
use biome_css_syntax::{CssImportSupports, CssImportSupportsFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssImportSupports;
impl FormatNodeRule<CssImportSupports> for FormatCssImportSupports {
    fn fmt_fields(&self, node: &CssImportSupports, f: &mut CssFormatter) -> FormatResult<()> {
        let CssImportSupportsFields {
            supports_token,
            l_paren_token,
            condition,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                supports_token.format(),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent(&condition.format()),
                    r_paren_token.format()
                ])
            ]
        )
    }
}

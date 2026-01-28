use biome_css_syntax::{CssCommaSeparatedValue, CssCommaSeparatedValueFields};
use biome_formatter::{format_args, write};

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCommaSeparatedValue;

impl FormatNodeRule<CssCommaSeparatedValue> for FormatCssCommaSeparatedValue {
    fn fmt_fields(&self, node: &CssCommaSeparatedValue, f: &mut CssFormatter) -> FormatResult<()> {
        let CssCommaSeparatedValueFields {
            l_curly_token,
            items,
            r_curly_token,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                l_curly_token.format(),
                soft_block_indent(&items.format()),
                r_curly_token.format()
            ])]
        )
    }
}

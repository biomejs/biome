use biome_css_syntax::{ScssParameterList, ScssParameterListFields};
use biome_formatter::{format_args, write};

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssParameterList;

impl FormatNodeRule<ScssParameterList> for FormatScssParameterList {
    fn fmt_fields(&self, node: &ScssParameterList, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssParameterListFields {
            l_paren_token,
            items,
            r_paren_token,
        } = node.as_fields();

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

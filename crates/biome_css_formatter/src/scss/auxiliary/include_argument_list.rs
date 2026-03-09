use crate::prelude::*;
use biome_css_syntax::{ScssIncludeArgumentList, ScssIncludeArgumentListFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssIncludeArgumentList;

impl FormatNodeRule<ScssIncludeArgumentList> for FormatScssIncludeArgumentList {
    fn fmt_fields(&self, node: &ScssIncludeArgumentList, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssIncludeArgumentListFields {
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

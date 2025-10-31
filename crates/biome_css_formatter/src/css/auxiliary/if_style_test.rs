use crate::prelude::*;
use biome_css_syntax::{CssIfStyleTest, CssIfStyleTestFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIfStyleTest;

impl FormatNodeRule<CssIfStyleTest> for FormatCssIfStyleTest {
    fn fmt_fields(&self, node: &CssIfStyleTest, f: &mut CssFormatter) -> FormatResult<()> {
        let CssIfStyleTestFields {
            style_token,
            l_paren_token,
            test,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                style_token.format(),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent(&test.format()),
                    r_paren_token.format()
                ])
            ]
        )
    }
}

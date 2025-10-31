use crate::prelude::*;
use biome_css_syntax::{CssIfMediaTest, CssIfMediaTestFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIfMediaTest;

impl FormatNodeRule<CssIfMediaTest> for FormatCssIfMediaTest {
    fn fmt_fields(&self, node: &CssIfMediaTest, f: &mut CssFormatter) -> FormatResult<()> {
        let CssIfMediaTestFields {
            media_token,
            l_paren_token,
            test,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                media_token.format(),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent(&test.format()),
                    r_paren_token.format()
                ])
            ]
        )
    }
}

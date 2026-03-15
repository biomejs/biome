use crate::prelude::*;
use biome_css_syntax::{CssIfSassTest, CssIfSassTestFields};
use biome_formatter::{format_args, write};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIfSassTest;
impl FormatNodeRule<CssIfSassTest> for FormatCssIfSassTest {
    fn fmt_fields(&self, node: &CssIfSassTest, f: &mut CssFormatter) -> FormatResult<()> {
        let CssIfSassTestFields {
            sass_token,
            l_paren_token,
            test,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                sass_token.format(),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent(&test.format()),
                    r_paren_token.format()
                ])
            ]
        )
    }
}

use crate::prelude::*;
use biome_css_syntax::{CssIfSupportsTest, CssIfSupportsTestFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIfSupportsTest;

impl FormatNodeRule<CssIfSupportsTest> for FormatCssIfSupportsTest {
    fn fmt_fields(&self, node: &CssIfSupportsTest, f: &mut CssFormatter) -> FormatResult<()> {
        let CssIfSupportsTestFields {
            supports_token,
            l_paren_token,
            test,
            r_paren_token,
        } = node.as_fields();

        let should_insert_space = f.options().delimiter_spacing().value();

        write!(
            f,
            [
                supports_token.format(),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent_with_maybe_space(&test.format(), should_insert_space),
                    r_paren_token.format()
                ])
            ]
        )
    }
}

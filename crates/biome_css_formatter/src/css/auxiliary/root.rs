use crate::prelude::*;
use biome_css_syntax::{CssRoot, CssRootFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRoot;

impl FormatNodeRule<CssRoot> for FormatCssRoot {
    fn fmt_fields(&self, node: &CssRoot, f: &mut CssFormatter) -> FormatResult<()> {
        let CssRootFields {
            bom_token,
            rules,
            eof_token,
        } = node.as_fields();

        write!(
            f,
            [
                bom_token.format(),
                rules.format(),
                hard_line_break(),
                format_removed(&eof_token?),
            ]
        )
    }
}

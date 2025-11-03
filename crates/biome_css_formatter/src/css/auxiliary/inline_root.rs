use crate::prelude::*;
use biome_css_syntax::{CssInlineRoot, CssInlineRootFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssInlineRoot;

impl FormatNodeRule<CssInlineRoot> for FormatCssInlineRoot {
    fn fmt_fields(&self, node: &CssInlineRoot, f: &mut CssFormatter) -> FormatResult<()> {
        let CssInlineRootFields { items, eof_token } = node.as_fields();

        write!(
            f,
            [
                items.format(),
                hard_line_break(),
                format_removed(&eof_token?),
            ]
        )
    }
}

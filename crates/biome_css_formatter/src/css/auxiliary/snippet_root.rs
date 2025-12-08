use crate::prelude::*;
use biome_css_syntax::{CssSnippetRoot, CssSnippetRootFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSnippetRoot;

impl FormatNodeRule<CssSnippetRoot> for FormatCssSnippetRoot {
    fn fmt_fields(&self, node: &CssSnippetRoot, f: &mut CssFormatter) -> FormatResult<()> {
        let CssSnippetRootFields { items, eof_token } = node.as_fields();

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

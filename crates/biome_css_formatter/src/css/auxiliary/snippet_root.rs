use crate::prelude::*;
use biome_css_syntax::{CssSnippetRoot, CssSnippetRootFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSnippetRoot;

impl FormatNodeRule<CssSnippetRoot> for FormatCssSnippetRoot {
    fn fmt_fields(&self, node: &CssSnippetRoot, f: &mut CssFormatter) -> FormatResult<()> {
        let CssSnippetRootFields { items, eof_token } = node.as_fields();

        // The value of a `style` attribute is followed by the closing quote of
        // the attribute rather than by the end of a file, and a hard break
        // here would force it onto its own line even when it fits beside the
        // tag.
        if f.options().is_html_style_attribute() {
            return write!(f, [items.format(), format_removed(&eof_token?)]);
        }

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

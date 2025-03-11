use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{HtmlRoot, HtmlRootFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlRoot;
impl FormatNodeRule<HtmlRoot> for FormatHtmlRoot {
    fn fmt_fields(&self, node: &HtmlRoot, f: &mut HtmlFormatter) -> FormatResult<()> {
        let HtmlRootFields {
            bom_token,
            directive,
            html,
            eof_token,
        } = node.as_fields();

        write!(
            f,
            [
                bom_token.format(),
                directive.format(),
                html.format(),
                hard_line_break(),
                format_removed(&eof_token?),
            ]
        )
    }
}

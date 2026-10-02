use crate::FormatEmbedded;
use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_html_syntax::HtmlEmbeddedContent;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlEmbeddedContent;
impl FormatNodeRule<HtmlEmbeddedContent> for FormatHtmlEmbeddedContent {
    fn fmt_fields(&self, node: &HtmlEmbeddedContent, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_verbatim_skipped(node.syntax()).fmt(f)
    }

    fn wrap_embed(
        &self,
        _node: &HtmlEmbeddedContent,
        embedded: &FormatEmbedded,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        // The content of `<script>` and `<style>` tags starts on its own line.
        if f.options().indent_script_and_style().value() {
            write!(
                f,
                [
                    hard_line_break(),
                    indent(&format_args![hard_line_break(), embedded]),
                    hard_line_break()
                ]
            )
        } else {
            write!(f, [hard_line_break(), embedded, hard_line_break()])
        }
    }
}

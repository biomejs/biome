use crate::{FormatBogusNodeRule, prelude::*};
use biome_formatter::FormatError;
use biome_html_syntax::HtmlBogusTextExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlBogusTextExpression;
impl FormatBogusNodeRule<HtmlBogusTextExpression> for FormatHtmlBogusTextExpression {
    fn fmt(&self, node: &HtmlBogusTextExpression, f: &mut HtmlFormatter) -> FormatResult<()> {
        if f.options().file_source().is_svelte() {
            // Without a reliable closing brace, the following markup may still be guest code.
            return Err(FormatError::SyntaxError);
        }
        format_bogus_node(node.syntax()).fmt(f)
    }
}

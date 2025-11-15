use crate::prelude::*;
use biome_html_syntax::GlimmerMustacheComment;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGlimmerMustacheComment;
impl FormatNodeRule<GlimmerMustacheComment> for FormatGlimmerMustacheComment {
    fn fmt_fields(&self, node: &GlimmerMustacheComment, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}

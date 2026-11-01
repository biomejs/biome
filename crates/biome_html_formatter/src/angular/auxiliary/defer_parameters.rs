use crate::prelude::*;
use biome_html_syntax::AngularDeferParameters;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularDeferParameters;
impl FormatNodeRule<AngularDeferParameters> for FormatAngularDeferParameters {
    fn fmt_fields(&self, node: &AngularDeferParameters, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}

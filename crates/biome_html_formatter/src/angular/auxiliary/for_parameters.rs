use crate::prelude::*;
use biome_html_syntax::AngularForParameters;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularForParameters;
impl FormatNodeRule<AngularForParameters> for FormatAngularForParameters {
    fn fmt_fields(&self, node: &AngularForParameters, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}

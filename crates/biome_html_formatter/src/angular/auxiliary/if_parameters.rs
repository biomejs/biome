use crate::prelude::*;
use biome_html_syntax::AngularIfParameters;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularIfParameters;
impl FormatNodeRule<AngularIfParameters> for FormatAngularIfParameters {
    fn fmt_fields(&self, node: &AngularIfParameters, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}

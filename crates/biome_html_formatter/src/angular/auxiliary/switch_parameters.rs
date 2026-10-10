use crate::prelude::*;
use biome_html_syntax::AngularSwitchParameters;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularSwitchParameters;
impl FormatNodeRule<AngularSwitchParameters> for FormatAngularSwitchParameters {
    fn fmt_fields(
        &self,
        node: &AngularSwitchParameters,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}

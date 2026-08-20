use crate::prelude::*;
use biome_html_syntax::AngularPlaceholderParameters;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularPlaceholderParameters;
impl FormatNodeRule<AngularPlaceholderParameters> for FormatAngularPlaceholderParameters {
    fn fmt_fields(
        &self,
        node: &AngularPlaceholderParameters,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}

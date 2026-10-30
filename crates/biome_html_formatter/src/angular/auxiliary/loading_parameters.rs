use crate::prelude::*;
use biome_html_syntax::AngularLoadingParameters;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularLoadingParameters;
impl FormatNodeRule<AngularLoadingParameters> for FormatAngularLoadingParameters {
    fn fmt_fields(
        &self,
        node: &AngularLoadingParameters,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}

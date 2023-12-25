use crate::prelude::*;
use biome_js_syntax::TsTypeEmptyParameters;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsTypeEmptyParameters;
impl FormatNodeRule<TsTypeEmptyParameters> for FormatTsTypeEmptyParameters {
    fn fmt_fields(&self, node: &TsTypeEmptyParameters, f: &mut JsFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}

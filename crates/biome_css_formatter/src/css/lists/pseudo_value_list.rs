use crate::prelude::*;
use biome_css_syntax::CssPseudoValueList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoValueList;
impl FormatRule<CssPseudoValueList> for FormatCssPseudoValueList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssPseudoValueList, f: &mut CssFormatter) -> FormatResult<()> {
        let mut joiner = f.join_nodes_with_soft_line();

        for (rule, formatted) in node.elements().zip(node.format_separated(",")) {
            joiner.entry(rule.node()?.syntax(), &formatted);
        }

        joiner.finish()
    }
}

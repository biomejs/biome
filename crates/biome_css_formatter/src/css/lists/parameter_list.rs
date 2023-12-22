use crate::prelude::*;
use biome_css_syntax::CssParameterList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssParameterList;
impl FormatRule<CssParameterList> for FormatCssParameterList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssParameterList, f: &mut CssFormatter) -> FormatResult<()> {
        let mut joiner = f.join_nodes_with_soft_line();

        for (rule, formatted) in node.elements().zip(node.format_separated(",")) {
            joiner.entry(rule.node()?.syntax(), &formatted);
        }

        joiner.finish()
    }
}

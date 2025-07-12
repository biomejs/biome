use crate::prelude::*;
use biome_css_syntax::CssPseudoElementFunctionParameterList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoElementFunctionParameterList;
impl FormatRule<CssPseudoElementFunctionParameterList>
    for FormatCssPseudoElementFunctionParameterList
{
    type Context = CssFormatContext;
    fn fmt(
        &self,
        node: &CssPseudoElementFunctionParameterList,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        f.join_with(&soft_line_break_or_space())
            .entries(node.iter().formatted())
            .finish()
    }
}

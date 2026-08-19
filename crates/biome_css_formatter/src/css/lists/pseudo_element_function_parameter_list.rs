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
        let separator = soft_line_break_or_space();
        let mut joiner = f.join_with(&separator);

        for parameter in node.iter() {
            joiner.entry(&parameter.format().with_text_case(CssCase::Preserve));
        }

        joiner.finish()
    }
}

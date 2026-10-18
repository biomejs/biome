use crate::prelude::*;
use biome_css_syntax::CssSassIndentedDeclarationOrAtRuleBlock;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSassIndentedDeclarationOrAtRuleBlock;
impl FormatNodeRule<CssSassIndentedDeclarationOrAtRuleBlock>
    for FormatCssSassIndentedDeclarationOrAtRuleBlock
{
    fn fmt_fields(
        &self,
        node: &CssSassIndentedDeclarationOrAtRuleBlock,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let items = node.items();

        write!(f, [block_indent(&items.format())])
    }
}

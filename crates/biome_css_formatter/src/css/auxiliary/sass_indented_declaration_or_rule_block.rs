use crate::prelude::*;
use biome_css_syntax::CssSassIndentedDeclarationOrRuleBlock;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSassIndentedDeclarationOrRuleBlock;
impl FormatNodeRule<CssSassIndentedDeclarationOrRuleBlock>
    for FormatCssSassIndentedDeclarationOrRuleBlock
{
    fn fmt_fields(
        &self,
        node: &CssSassIndentedDeclarationOrRuleBlock,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let items = node.items();

        write!(f, [block_indent(&items.format())])
    }
}

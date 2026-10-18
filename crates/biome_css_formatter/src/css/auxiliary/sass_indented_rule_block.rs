use crate::prelude::*;
use biome_css_syntax::CssSassIndentedRuleBlock;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSassIndentedRuleBlock;
impl FormatNodeRule<CssSassIndentedRuleBlock> for FormatCssSassIndentedRuleBlock {
    fn fmt_fields(
        &self,
        node: &CssSassIndentedRuleBlock,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let rules = node.rules();

        write!(f, [block_indent(&rules.format())])
    }
}

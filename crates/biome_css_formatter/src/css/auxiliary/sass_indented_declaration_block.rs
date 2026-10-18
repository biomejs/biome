use crate::prelude::*;
use biome_css_syntax::CssSassIndentedDeclarationBlock;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSassIndentedDeclarationBlock;
impl FormatNodeRule<CssSassIndentedDeclarationBlock> for FormatCssSassIndentedDeclarationBlock {
    fn fmt_fields(
        &self,
        node: &CssSassIndentedDeclarationBlock,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let declarations = node.declarations();

        write!(f, [block_indent(&declarations.format())])
    }
}

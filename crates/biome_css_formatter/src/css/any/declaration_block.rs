//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssDeclarationBlock;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssDeclarationBlock;
impl FormatRule<AnyCssDeclarationBlock> for FormatAnyCssDeclarationBlock {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssDeclarationBlock, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssDeclarationBlock::CssBogusBlock(node) => node.format().fmt(f),
            AnyCssDeclarationBlock::CssDeclarationBlock(node) => node.format().fmt(f),
        }
    }
}

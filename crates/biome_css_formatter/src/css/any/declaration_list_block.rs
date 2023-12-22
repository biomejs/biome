//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssDeclarationListBlock;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssDeclarationListBlock;
impl FormatRule<AnyCssDeclarationListBlock> for FormatAnyCssDeclarationListBlock {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssDeclarationListBlock, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssDeclarationListBlock::CssDeclarationListBlock(node) => node.format().fmt(f),
            AnyCssDeclarationListBlock::CssBogusBlock(node) => node.format().fmt(f),
        }
    }
}

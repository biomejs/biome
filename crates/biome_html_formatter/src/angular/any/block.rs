//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnyAngularBlock;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyAngularBlock;
impl FormatRule<AnyAngularBlock> for FormatAnyAngularBlock {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnyAngularBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnyAngularBlock::AngularDeferBlock(node) => node.format().fmt(f),
            AnyAngularBlock::AngularForBlock(node) => node.format().fmt(f),
            AnyAngularBlock::AngularIfBlock(node) => node.format().fmt(f),
            AnyAngularBlock::AngularLetBlock(node) => node.format().fmt(f),
            AnyAngularBlock::AngularSwitchBlock(node) => node.format().fmt(f),
        }
    }
}

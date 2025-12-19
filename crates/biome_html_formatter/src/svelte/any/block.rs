//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnySvelteBlock;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySvelteBlock;
impl FormatRule<AnySvelteBlock> for FormatAnySvelteBlock {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnySvelteBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnySvelteBlock::SvelteBogusBlock(node) => node.format().fmt(f),
            AnySvelteBlock::SvelteConstBlock(node) => node.format().fmt(f),
            AnySvelteBlock::SvelteDebugBlock(node) => node.format().fmt(f),
            AnySvelteBlock::SvelteEachBlock(node) => node.format().fmt(f),
            AnySvelteBlock::SvelteHtmlBlock(node) => node.format().fmt(f),
            AnySvelteBlock::SvelteIfBlock(node) => node.format().fmt(f),
            AnySvelteBlock::SvelteKeyBlock(node) => node.format().fmt(f),
            AnySvelteBlock::SvelteRenderBlock(node) => node.format().fmt(f),
        }
    }
}

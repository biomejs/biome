//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssFontFeatureValuesBlock;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssFontFeatureValuesBlock;
impl FormatRule<AnyCssFontFeatureValuesBlock> for FormatAnyCssFontFeatureValuesBlock {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssFontFeatureValuesBlock, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssFontFeatureValuesBlock::CssBogusBlock(node) => node.format().fmt(f),
            AnyCssFontFeatureValuesBlock::CssFontFeatureValuesBlock(node) => node.format().fmt(f),
        }
    }
}

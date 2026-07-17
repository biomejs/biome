//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssPseudoClassNthValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssPseudoClassNthValue;
impl FormatRule<AnyCssPseudoClassNthValue> for FormatAnyCssPseudoClassNthValue {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssPseudoClassNthValue, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssPseudoClassNthValue::CssNumber(node) => node.format().fmt(f),
            AnyCssPseudoClassNthValue::ScssInterpolatedNthValue(node) => node.format().fmt(f),
            AnyCssPseudoClassNthValue::ScssInterpolation(node) => node.format().fmt(f),
        }
    }
}

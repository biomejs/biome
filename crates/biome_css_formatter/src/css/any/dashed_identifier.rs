//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssDashedIdentifier;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssDashedIdentifier;
impl FormatRule<AnyCssDashedIdentifier> for FormatAnyCssDashedIdentifier {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssDashedIdentifier, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssDashedIdentifier::CssDashedIdentifier(node) => node.format().fmt(f),
            AnyCssDashedIdentifier::ScssInterpolatedDashedIdentifier(node) => node.format().fmt(f),
        }
    }
}

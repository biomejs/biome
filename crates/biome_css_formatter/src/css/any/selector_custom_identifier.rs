//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssSelectorCustomIdentifier;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssSelectorCustomIdentifier;
impl FormatRule<AnyCssSelectorCustomIdentifier> for FormatAnyCssSelectorCustomIdentifier {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssSelectorCustomIdentifier, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssSelectorCustomIdentifier::CssCustomIdentifier(node) => node.format().fmt(f),
            AnyCssSelectorCustomIdentifier::ScssInterpolatedIdentifier(node) => {
                node.format().fmt(f)
            }
        }
    }
}

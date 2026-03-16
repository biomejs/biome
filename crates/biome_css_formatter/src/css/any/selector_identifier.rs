//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssSelectorIdentifier;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssSelectorIdentifier;
impl FormatRule<AnyCssSelectorIdentifier> for FormatAnyCssSelectorIdentifier {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssSelectorIdentifier, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssSelectorIdentifier::CssIdentifier(node) => node.format().fmt(f),
            AnyCssSelectorIdentifier::ScssInterpolatedIdentifier(node) => node.format().fmt(f),
        }
    }
}

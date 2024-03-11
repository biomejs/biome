//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssUrlModifier;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssUrlModifier;
impl FormatRule<AnyCssUrlModifier> for FormatAnyCssUrlModifier {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssUrlModifier, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssUrlModifier::CssBogusUrlModifier(node) => node.format().fmt(f),
            AnyCssUrlModifier::CssFunction(node) => node.format().fmt(f),
            AnyCssUrlModifier::CssIdentifier(node) => node.format().fmt(f),
        }
    }
}

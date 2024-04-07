//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssCustomIdentifier;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssCustomIdentifier;
impl FormatRule<AnyCssCustomIdentifier> for FormatAnyCssCustomIdentifier {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssCustomIdentifier, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssCustomIdentifier::CssBogusCustomIdentifier(node) => node.format().fmt(f),
            AnyCssCustomIdentifier::CssCustomIdentifier(node) => node.format().fmt(f),
        }
    }
}

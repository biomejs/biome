//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssAttrName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssAttrName;
impl FormatRule<AnyCssAttrName> for FormatAnyCssAttrName {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssAttrName, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssAttrName::CssBogusAttrName(node) => node.format().fmt(f),
            AnyCssAttrName::CssIdentifier(node) => node.format().fmt(f),
        }
    }
}

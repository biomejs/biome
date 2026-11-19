//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssNamespacePrefixName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssNamespacePrefixName;
impl FormatRule<AnyCssNamespacePrefixName> for FormatAnyCssNamespacePrefixName {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssNamespacePrefixName, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssNamespacePrefixName::CssBogusCustomIdentifier(node) => node.format().fmt(f),
            AnyCssNamespacePrefixName::CssIdentifier(node) => node.format().fmt(f),
            AnyCssNamespacePrefixName::ScssInterpolatedIdentifier(node) => node.format().fmt(f),
        }
    }
}

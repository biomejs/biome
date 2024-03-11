//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssDeclarationName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssDeclarationName;
impl FormatRule<AnyCssDeclarationName> for FormatAnyCssDeclarationName {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssDeclarationName, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssDeclarationName::CssDashedIdentifier(node) => node.format().fmt(f),
            AnyCssDeclarationName::CssIdentifier(node) => node.format().fmt(f),
        }
    }
}

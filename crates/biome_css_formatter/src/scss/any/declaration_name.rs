//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyScssDeclarationName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyScssDeclarationName;
impl FormatRule<AnyScssDeclarationName> for FormatAnyScssDeclarationName {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyScssDeclarationName, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyScssDeclarationName::ScssIdentifier(node) => node.format().fmt(f),
            AnyScssDeclarationName::ScssNamespacedIdentifier(node) => node.format().fmt(f),
        }
    }
}

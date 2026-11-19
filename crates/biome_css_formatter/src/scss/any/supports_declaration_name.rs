//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyScssSupportsDeclarationName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyScssSupportsDeclarationName;
impl FormatRule<AnyScssSupportsDeclarationName> for FormatAnyScssSupportsDeclarationName {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyScssSupportsDeclarationName, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyScssSupportsDeclarationName::ScssExpression(node) => node.format().fmt(f),
            AnyScssSupportsDeclarationName::ScssVariable(node) => node.format().fmt(f),
        }
    }
}

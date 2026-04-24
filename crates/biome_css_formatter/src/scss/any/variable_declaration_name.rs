//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyScssVariableDeclarationName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyScssVariableDeclarationName;
impl FormatRule<AnyScssVariableDeclarationName> for FormatAnyScssVariableDeclarationName {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyScssVariableDeclarationName, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyScssVariableDeclarationName::ScssNamespacedVariable(node) => node.format().fmt(f),
            AnyScssVariableDeclarationName::ScssVariable(node) => node.format().fmt(f),
        }
    }
}

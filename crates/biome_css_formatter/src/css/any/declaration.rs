//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssDeclaration;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssDeclaration;
impl FormatRule<AnyCssDeclaration> for FormatAnyCssDeclaration {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssDeclaration, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssDeclaration::CssDeclarationWithSemicolon(node) => node.format().fmt(f),
            AnyCssDeclaration::CssEmptyDeclaration(node) => node.format().fmt(f),
        }
    }
}

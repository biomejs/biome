//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssDeclarationWithSemicolon;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssDeclarationWithSemicolon;
impl FormatRule<AnyCssDeclarationWithSemicolon> for FormatAnyCssDeclarationWithSemicolon {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssDeclarationWithSemicolon, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssDeclarationWithSemicolon::CssDeclarationWithSemicolon(node) => {
                node.format().fmt(f)
            }
            AnyCssDeclarationWithSemicolon::CssEmptyDeclaration(node) => node.format().fmt(f),
        }
    }
}

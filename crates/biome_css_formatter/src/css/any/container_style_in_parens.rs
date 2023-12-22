//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssContainerStyleInParens;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssContainerStyleInParens;
impl FormatRule<AnyCssContainerStyleInParens> for FormatAnyCssContainerStyleInParens {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssContainerStyleInParens, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssContainerStyleInParens::AnyCssContainerStyleQuery(node) => node.format().fmt(f),
            AnyCssContainerStyleInParens::CssDeclaration(node) => node.format().fmt(f),
        }
    }
}

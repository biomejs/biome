//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssContainerQueryInParens;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssContainerQueryInParens;
impl FormatRule<AnyCssContainerQueryInParens> for FormatAnyCssContainerQueryInParens {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssContainerQueryInParens, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssContainerQueryInParens::CssContainerQueryInParens(node) => node.format().fmt(f),
            AnyCssContainerQueryInParens::CssContainerSizeFeatureInParens(node) => {
                node.format().fmt(f)
            }
            AnyCssContainerQueryInParens::CssContainerStyleQueryInParens(node) => {
                node.format().fmt(f)
            }
        }
    }
}

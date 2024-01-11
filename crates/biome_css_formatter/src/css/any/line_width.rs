//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssLineWidth;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssLineWidth;
impl FormatRule<AnyCssLineWidth> for FormatAnyCssLineWidth {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssLineWidth, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssLineWidth::CssRegularDimension(node) => node.format().fmt(f),
            AnyCssLineWidth::CssLineWidthKeyword(node) => node.format().fmt(f),
        }
    }
}

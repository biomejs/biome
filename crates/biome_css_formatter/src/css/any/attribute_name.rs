//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssAttributeName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssAttributeName;
impl FormatRule<AnyCssAttributeName> for FormatAnyCssAttributeName {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssAttributeName, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssAttributeName::CssIdentifier(node) => node.format().fmt(f),
            AnyCssAttributeName::ScssInterpolatedIdentifier(node) => node.format().fmt(f),
        }
    }
}

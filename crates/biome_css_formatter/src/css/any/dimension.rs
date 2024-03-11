//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssDimension;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssDimension;
impl FormatRule<AnyCssDimension> for FormatAnyCssDimension {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssDimension, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssDimension::CssPercentage(node) => node.format().fmt(f),
            AnyCssDimension::CssRegularDimension(node) => node.format().fmt(f),
            AnyCssDimension::CssUnknownDimension(node) => node.format().fmt(f),
        }
    }
}

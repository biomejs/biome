//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssContainerName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssContainerName;
impl FormatRule<AnyCssContainerName> for FormatAnyCssContainerName {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssContainerName, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssContainerName::CssCustomIdentifier(node) => node.format().fmt(f),
            AnyCssContainerName::ScssInterpolatedIdentifier(node) => node.format().fmt(f),
        }
    }
}

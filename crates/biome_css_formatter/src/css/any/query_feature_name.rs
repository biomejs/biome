//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssQueryFeatureName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssQueryFeatureName;
impl FormatRule<AnyCssQueryFeatureName> for FormatAnyCssQueryFeatureName {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssQueryFeatureName, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssQueryFeatureName::CssIdentifier(node) => node.format().fmt(f),
            AnyCssQueryFeatureName::ScssInterpolatedIdentifier(node) => node.format().fmt(f),
        }
    }
}

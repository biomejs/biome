//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssMediaQueryType;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssMediaQueryType;
impl FormatRule<AnyCssMediaQueryType> for FormatAnyCssMediaQueryType {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssMediaQueryType, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssMediaQueryType::CssIdentifier(node) => node.format().fmt(f),
            AnyCssMediaQueryType::CssMediaQueryFeature(node) => node.format().fmt(f),
        }
    }
}

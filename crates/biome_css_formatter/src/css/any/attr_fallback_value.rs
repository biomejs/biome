//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssAttrFallbackValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssAttrFallbackValue;
impl FormatRule<AnyCssAttrFallbackValue> for FormatAnyCssAttrFallbackValue {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssAttrFallbackValue, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssAttrFallbackValue::CssGenericComponentValueList(node) => node.format().fmt(f),
            AnyCssAttrFallbackValue::ScssExpression(node) => node.format().fmt(f),
        }
    }
}

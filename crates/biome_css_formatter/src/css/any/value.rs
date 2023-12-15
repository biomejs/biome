//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssValue;
impl FormatRule<AnyCssValue> for FormatAnyCssValue {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssValue, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssValue::CssIdentifier(node) => node.format().fmt(f),
            AnyCssValue::CssString(node) => node.format().fmt(f),
            AnyCssValue::CssNumber(node) => node.format().fmt(f),
            AnyCssValue::AnyCssDimension(node) => node.format().fmt(f),
            AnyCssValue::CssRatio(node) => node.format().fmt(f),
            AnyCssValue::CssAnyFunction(node) => node.format().fmt(f),
            AnyCssValue::CssCustomProperty(node) => node.format().fmt(f),
        }
    }
}

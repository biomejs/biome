//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssZIndexPropertyValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssZIndexPropertyValue;
impl FormatRule<AnyCssZIndexPropertyValue> for FormatAnyCssZIndexPropertyValue {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssZIndexPropertyValue, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssZIndexPropertyValue::CssAuto(node) => node.format().fmt(f),
            AnyCssZIndexPropertyValue::CssNumber(node) => node.format().fmt(f),
            AnyCssZIndexPropertyValue::CssWideKeyword(node) => node.format().fmt(f),
            AnyCssZIndexPropertyValue::CssUnknownPropertyValue(node) => node.format().fmt(f),
            AnyCssZIndexPropertyValue::CssBogusPropertyValue(node) => node.format().fmt(f),
        }
    }
}

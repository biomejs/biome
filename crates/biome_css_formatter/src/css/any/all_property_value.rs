//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssAllPropertyValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssAllPropertyValue;
impl FormatRule<AnyCssAllPropertyValue> for FormatAnyCssAllPropertyValue {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssAllPropertyValue, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssAllPropertyValue::CssWideKeyword(node) => node.format().fmt(f),
            AnyCssAllPropertyValue::CssUnknownPropertyValue(node) => node.format().fmt(f),
            AnyCssAllPropertyValue::CssBogusPropertyValue(node) => node.format().fmt(f),
        }
    }
}

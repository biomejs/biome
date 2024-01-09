//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssBorderPropertyValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssBorderPropertyValue;
impl FormatRule<AnyCssBorderPropertyValue> for FormatAnyCssBorderPropertyValue {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssBorderPropertyValue, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssBorderPropertyValue::CssBorder(node) => node.format().fmt(f),
            AnyCssBorderPropertyValue::CssWideKeyword(node) => node.format().fmt(f),
            AnyCssBorderPropertyValue::CssUnknownPropertyValue(node) => node.format().fmt(f),
            AnyCssBorderPropertyValue::CssBogusPropertyValue(node) => node.format().fmt(f),
        }
    }
}

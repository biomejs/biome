//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssUnicodeValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssUnicodeValue;
impl FormatRule<AnyCssUnicodeValue> for FormatAnyCssUnicodeValue {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssUnicodeValue, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssUnicodeValue::CssBogusUnicodeRangeValue(node) => node.format().fmt(f),
            AnyCssUnicodeValue::CssUnicodeCodepoint(node) => node.format().fmt(f),
            AnyCssUnicodeValue::CssUnicodeRangeInterval(node) => node.format().fmt(f),
            AnyCssUnicodeValue::CssUnicodeRangeWildcard(node) => node.format().fmt(f),
        }
    }
}

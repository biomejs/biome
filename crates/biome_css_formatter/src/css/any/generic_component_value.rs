//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssGenericComponentValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssGenericComponentValue;
impl FormatRule<AnyCssGenericComponentValue> for FormatAnyCssGenericComponentValue {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssGenericComponentValue, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssGenericComponentValue::AnyCssValue(node) => node.format().fmt(f),
            AnyCssGenericComponentValue::CssGenericDelimiter(node) => node.format().fmt(f),
        }
    }
}

//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssUrlValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssUrlValue;
impl FormatRule<AnyCssUrlValue> for FormatAnyCssUrlValue {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssUrlValue, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssUrlValue::CssString(node) => node.format().fmt(f),
            AnyCssUrlValue::CssUrlValueRaw(node) => node.format().fmt(f),
        }
    }
}

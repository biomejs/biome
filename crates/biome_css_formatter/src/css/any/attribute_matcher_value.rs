//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssAttributeMatcherValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssAttributeMatcherValue;
impl FormatRule<AnyCssAttributeMatcherValue> for FormatAnyCssAttributeMatcherValue {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssAttributeMatcherValue, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssAttributeMatcherValue::CssIdentifier(node) => node.format().fmt(f),
            AnyCssAttributeMatcherValue::CssString(node) => node.format().fmt(f),
        }
    }
}

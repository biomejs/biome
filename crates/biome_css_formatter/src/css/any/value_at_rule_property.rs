//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssValueAtRuleProperty;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssValueAtRuleProperty;
impl FormatRule<AnyCssValueAtRuleProperty> for FormatAnyCssValueAtRuleProperty {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssValueAtRuleProperty, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssValueAtRuleProperty::CssBogusProperty(node) => node.format().fmt(f),
            AnyCssValueAtRuleProperty::CssValueAtRuleGenericProperty(node) => node.format().fmt(f),
        }
    }
}

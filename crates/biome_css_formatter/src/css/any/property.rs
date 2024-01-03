//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssProperty;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssProperty;
impl FormatRule<AnyCssProperty> for FormatAnyCssProperty {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssProperty, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssProperty::CssGenericProperty(node) => node.format().fmt(f),
            AnyCssProperty::CssBogusProperty(node) => node.format().fmt(f),
            AnyCssProperty::CssAllProperty(node) => node.format().fmt(f),
            AnyCssProperty::CssZIndexProperty(node) => node.format().fmt(f),
        }
    }
}

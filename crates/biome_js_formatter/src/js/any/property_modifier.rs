//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsPropertyModifier;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsPropertyModifier;
impl FormatRule<AnyJsPropertyModifier> for FormatAnyJsPropertyModifier {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsPropertyModifier, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsPropertyModifier::JsAccessorModifier(node) => node.format().fmt(f),
            AnyJsPropertyModifier::JsDecorator(node) => node.format().fmt(f),
            AnyJsPropertyModifier::JsStaticModifier(node) => node.format().fmt(f),
            AnyJsPropertyModifier::TsAccessibilityModifier(node) => node.format().fmt(f),
            AnyJsPropertyModifier::TsOverrideModifier(node) => node.format().fmt(f),
            AnyJsPropertyModifier::TsReadonlyModifier(node) => node.format().fmt(f),
        }
    }
}

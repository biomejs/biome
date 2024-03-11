//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsMethodModifier;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsMethodModifier;
impl FormatRule<AnyJsMethodModifier> for FormatAnyJsMethodModifier {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsMethodModifier, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsMethodModifier::JsDecorator(node) => node.format().fmt(f),
            AnyJsMethodModifier::JsStaticModifier(node) => node.format().fmt(f),
            AnyJsMethodModifier::TsAccessibilityModifier(node) => node.format().fmt(f),
            AnyJsMethodModifier::TsOverrideModifier(node) => node.format().fmt(f),
        }
    }
}

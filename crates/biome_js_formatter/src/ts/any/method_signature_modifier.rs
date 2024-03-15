//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyTsMethodSignatureModifier;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTsMethodSignatureModifier;
impl FormatRule<AnyTsMethodSignatureModifier> for FormatAnyTsMethodSignatureModifier {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyTsMethodSignatureModifier, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyTsMethodSignatureModifier::JsDecorator(node) => node.format().fmt(f),
            AnyTsMethodSignatureModifier::JsStaticModifier(node) => node.format().fmt(f),
            AnyTsMethodSignatureModifier::TsAbstractModifier(node) => node.format().fmt(f),
            AnyTsMethodSignatureModifier::TsAccessibilityModifier(node) => node.format().fmt(f),
            AnyTsMethodSignatureModifier::TsOverrideModifier(node) => node.format().fmt(f),
        }
    }
}

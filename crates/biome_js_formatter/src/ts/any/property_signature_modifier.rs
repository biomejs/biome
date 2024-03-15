//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyTsPropertySignatureModifier;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTsPropertySignatureModifier;
impl FormatRule<AnyTsPropertySignatureModifier> for FormatAnyTsPropertySignatureModifier {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyTsPropertySignatureModifier, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyTsPropertySignatureModifier::JsAccessorModifier(node) => node.format().fmt(f),
            AnyTsPropertySignatureModifier::JsDecorator(node) => node.format().fmt(f),
            AnyTsPropertySignatureModifier::JsStaticModifier(node) => node.format().fmt(f),
            AnyTsPropertySignatureModifier::TsAbstractModifier(node) => node.format().fmt(f),
            AnyTsPropertySignatureModifier::TsAccessibilityModifier(node) => node.format().fmt(f),
            AnyTsPropertySignatureModifier::TsDeclareModifier(node) => node.format().fmt(f),
            AnyTsPropertySignatureModifier::TsOverrideModifier(node) => node.format().fmt(f),
            AnyTsPropertySignatureModifier::TsReadonlyModifier(node) => node.format().fmt(f),
        }
    }
}

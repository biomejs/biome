//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyTsIndexSignatureModifier;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTsIndexSignatureModifier;
impl FormatRule<AnyTsIndexSignatureModifier> for FormatAnyTsIndexSignatureModifier {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyTsIndexSignatureModifier, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyTsIndexSignatureModifier::JsStaticModifier(node) => node.format().fmt(f),
            AnyTsIndexSignatureModifier::TsReadonlyModifier(node) => node.format().fmt(f),
        }
    }
}

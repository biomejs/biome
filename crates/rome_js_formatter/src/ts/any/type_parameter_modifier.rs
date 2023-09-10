//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyTsTypeParameterModifier;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTsTypeParameterModifier;
impl FormatRule<AnyTsTypeParameterModifier> for FormatAnyTsTypeParameterModifier {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyTsTypeParameterModifier, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyTsTypeParameterModifier::TsConstModifier(node) => node.format().fmt(f),
            AnyTsTypeParameterModifier::TsInModifier(node) => node.format().fmt(f),
            AnyTsTypeParameterModifier::TsOutModifier(node) => node.format().fmt(f),
        }
    }
}

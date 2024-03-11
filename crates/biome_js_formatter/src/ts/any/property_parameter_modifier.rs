//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyTsPropertyParameterModifier;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTsPropertyParameterModifier;
impl FormatRule<AnyTsPropertyParameterModifier> for FormatAnyTsPropertyParameterModifier {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyTsPropertyParameterModifier, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyTsPropertyParameterModifier::TsAccessibilityModifier(node) => node.format().fmt(f),
            AnyTsPropertyParameterModifier::TsOverrideModifier(node) => node.format().fmt(f),
            AnyTsPropertyParameterModifier::TsReadonlyModifier(node) => node.format().fmt(f),
        }
    }
}

use crate::prelude::*;
use crate::utils::format_modifiers::FormatModifiers;
use biome_js_syntax::TsMethodSignatureModifierList;

#[derive(Debug, Clone, Default)]
pub struct FormatTsMethodSignatureModifierList;

impl FormatRule<TsMethodSignatureModifierList> for FormatTsMethodSignatureModifierList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &TsMethodSignatureModifierList, f: &mut JsFormatter) -> FormatResult<()> {
        FormatModifiers::from(node.clone()).fmt(f)
    }
}

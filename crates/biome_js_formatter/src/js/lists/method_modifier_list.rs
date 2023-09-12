use crate::prelude::*;
use crate::utils::format_modifiers::FormatModifiers;
use biome_js_syntax::JsMethodModifierList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsMethodModifierList;

impl FormatRule<JsMethodModifierList> for FormatJsMethodModifierList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsMethodModifierList, f: &mut JsFormatter) -> FormatResult<()> {
        FormatModifiers::from(node.clone()).fmt(f)
    }
}

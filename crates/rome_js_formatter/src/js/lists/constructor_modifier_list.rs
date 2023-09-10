use crate::prelude::*;
use biome_rowan::AstNodeList;
use rome_js_syntax::JsConstructorModifierList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsConstructorModifierList;

impl FormatRule<JsConstructorModifierList> for FormatJsConstructorModifierList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsConstructorModifierList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&space())
            .entries(node.iter().formatted())
            .finish()
    }
}

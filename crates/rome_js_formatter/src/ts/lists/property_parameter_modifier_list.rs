use crate::prelude::*;
use crate::utils::sort_modifiers_by_precedence;
use biome_js_syntax::TsPropertyParameterModifierList;

#[derive(Debug, Clone, Default)]
pub struct FormatTsPropertyParameterModifierList;

impl FormatRule<TsPropertyParameterModifierList> for FormatTsPropertyParameterModifierList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &TsPropertyParameterModifierList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&space())
            .entries(sort_modifiers_by_precedence(node).into_iter().formatted())
            .finish()
    }
}

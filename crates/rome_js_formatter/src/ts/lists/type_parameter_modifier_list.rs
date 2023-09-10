use crate::prelude::*;
use biome_js_syntax::TsTypeParameterModifierList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsTypeParameterModifierList;

impl FormatRule<TsTypeParameterModifierList> for FormatTsTypeParameterModifierList {
    type Context = JsFormatContext;
    fn fmt(&self, node: &TsTypeParameterModifierList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&space())
            .entries(node.iter().formatted())
            .finish()
    }
}

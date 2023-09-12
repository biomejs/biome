use crate::prelude::*;
use crate::utils::sort_modifiers_by_precedence;

use biome_js_syntax::TsIndexSignatureModifierList;

#[derive(Debug, Clone, Default)]
pub struct FormatTsIndexSignatureModifierList;

impl FormatRule<TsIndexSignatureModifierList> for FormatTsIndexSignatureModifierList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &TsIndexSignatureModifierList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&space())
            .entries(sort_modifiers_by_precedence(node).into_iter().formatted())
            .finish()
    }
}

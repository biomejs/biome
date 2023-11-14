use crate::prelude::*;
use biome_js_syntax::{
    AnyTsTypeParameterModifier, TsTypeParameterModifierList, TypeParameterModifiers,
};
use biome_rowan::AstNodeList;
use smallvec::SmallVec;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsTypeParameterModifierList;

impl FormatRule<TsTypeParameterModifierList> for FormatTsTypeParameterModifierList {
    type Context = JsFormatContext;
    fn fmt(&self, node: &TsTypeParameterModifierList, f: &mut JsFormatter) -> FormatResult<()> {
        let modifiers = sort_modifiers_by_precedence(node);
        f.join_with(&space())
            .entries(modifiers.into_iter().formatted())
            .finish()
    }
}

/// This function consumes a list of modifiers and applies a predictable sorting.
fn sort_modifiers_by_precedence(
    list: &TsTypeParameterModifierList,
) -> SmallVec<[AnyTsTypeParameterModifier; 3]> {
    let mut result = list.iter().collect::<SmallVec<_>>();
    result.sort_unstable_by_key(|node| TypeParameterModifiers::from(node));
    result
}

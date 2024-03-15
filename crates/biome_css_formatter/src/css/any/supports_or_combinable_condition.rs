//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssSupportsOrCombinableCondition;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssSupportsOrCombinableCondition;
impl FormatRule<AnyCssSupportsOrCombinableCondition> for FormatAnyCssSupportsOrCombinableCondition {
    type Context = CssFormatContext;
    fn fmt(
        &self,
        node: &AnyCssSupportsOrCombinableCondition,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        match node {
            AnyCssSupportsOrCombinableCondition::AnyCssSupportsInParens(node) => {
                node.format().fmt(f)
            }
            AnyCssSupportsOrCombinableCondition::CssSupportsOrCondition(node) => {
                node.format().fmt(f)
            }
        }
    }
}

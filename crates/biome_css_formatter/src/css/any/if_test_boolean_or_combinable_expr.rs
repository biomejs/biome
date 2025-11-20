//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssIfTestBooleanOrCombinableExpr;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssIfTestBooleanOrCombinableExpr;
impl FormatRule<AnyCssIfTestBooleanOrCombinableExpr> for FormatAnyCssIfTestBooleanOrCombinableExpr {
    type Context = CssFormatContext;
    fn fmt(
        &self,
        node: &AnyCssIfTestBooleanOrCombinableExpr,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        match node {
            AnyCssIfTestBooleanOrCombinableExpr::AnyCssIfTestBooleanExprGroup(node) => {
                node.format().fmt(f)
            }
            AnyCssIfTestBooleanOrCombinableExpr::CssIfTestBooleanOrExpr(node) => {
                node.format().fmt(f)
            }
        }
    }
}

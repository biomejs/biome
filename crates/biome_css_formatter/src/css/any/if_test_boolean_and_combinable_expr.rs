//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssIfTestBooleanAndCombinableExpr;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssIfTestBooleanAndCombinableExpr;
impl FormatRule<AnyCssIfTestBooleanAndCombinableExpr>
    for FormatAnyCssIfTestBooleanAndCombinableExpr
{
    type Context = CssFormatContext;
    fn fmt(
        &self,
        node: &AnyCssIfTestBooleanAndCombinableExpr,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        match node {
            AnyCssIfTestBooleanAndCombinableExpr::AnyCssIfTestBooleanExprGroup(node) => {
                node.format().fmt(f)
            }
            AnyCssIfTestBooleanAndCombinableExpr::CssIfTestBooleanAndExpr(node) => {
                node.format().fmt(f)
            }
        }
    }
}

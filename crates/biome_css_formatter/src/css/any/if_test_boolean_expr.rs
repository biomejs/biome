//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssIfTestBooleanExpr;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssIfTestBooleanExpr;
impl FormatRule<AnyCssIfTestBooleanExpr> for FormatAnyCssIfTestBooleanExpr {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssIfTestBooleanExpr, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssIfTestBooleanExpr::AnyCssIfTestBooleanAndCombinableExpr(node) => {
                node.format().fmt(f)
            }
            AnyCssIfTestBooleanExpr::AnyCssIfTestBooleanOrCombinableExpr(node) => {
                node.format().fmt(f)
            }
            AnyCssIfTestBooleanExpr::CssIfTestBooleanNotExpr(node) => node.format().fmt(f),
        }
    }
}

//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssIfCondition;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssIfCondition;
impl FormatRule<AnyCssIfCondition> for FormatAnyCssIfCondition {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssIfCondition, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssIfCondition::AnyCssIfTestBooleanExpr(node) => node.format().fmt(f),
            AnyCssIfCondition::CssElseKeyword(node) => node.format().fmt(f),
        }
    }
}

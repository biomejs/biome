//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssIfTestBooleanExprGroup;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssIfTestBooleanExprGroup;
impl FormatRule<AnyCssIfTestBooleanExprGroup> for FormatAnyCssIfTestBooleanExprGroup {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssIfTestBooleanExprGroup, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssIfTestBooleanExprGroup::AnyCssIfTest(node) => node.format().fmt(f),
            AnyCssIfTestBooleanExprGroup::CssIfTestBooleanExprInParens(node) => {
                node.format().fmt(f)
            }
        }
    }
}

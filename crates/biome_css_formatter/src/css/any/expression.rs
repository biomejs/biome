//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssExpression;
impl FormatRule<AnyCssExpression> for FormatAnyCssExpression {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssExpression, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssExpression::CssBinaryExpression(node) => node.format().fmt(f),
            AnyCssExpression::CssListOfComponentValuesExpression(node) => node.format().fmt(f),
            AnyCssExpression::CssParenthesizedExpression(node) => node.format().fmt(f),
        }
    }
}

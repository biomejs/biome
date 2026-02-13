//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyScssExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyScssExpression;
impl FormatRule<AnyScssExpression> for FormatAnyScssExpression {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyScssExpression, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyScssExpression::AnyCssValue(node) => node.format().fmt(f),
            AnyScssExpression::ScssArbitraryArgument(node) => node.format().fmt(f),
            AnyScssExpression::ScssBinaryExpression(node) => node.format().fmt(f),
            AnyScssExpression::ScssExpression(node) => node.format().fmt(f),
            AnyScssExpression::ScssKeywordArgument(node) => node.format().fmt(f),
            AnyScssExpression::ScssListExpression(node) => node.format().fmt(f),
            AnyScssExpression::ScssMapExpression(node) => node.format().fmt(f),
            AnyScssExpression::ScssParenthesizedExpression(node) => node.format().fmt(f),
            AnyScssExpression::ScssUnaryExpression(node) => node.format().fmt(f),
        }
    }
}

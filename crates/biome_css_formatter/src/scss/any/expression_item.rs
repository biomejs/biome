//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyScssExpressionItem;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyScssExpressionItem;
impl FormatRule<AnyScssExpressionItem> for FormatAnyScssExpressionItem {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyScssExpressionItem, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyScssExpressionItem::AnyCssValue(node) => node.format().fmt(f),
            AnyScssExpressionItem::CssGenericDelimiter(node) => node.format().fmt(f),
            AnyScssExpressionItem::ScssBinaryExpression(node) => node.format().fmt(f),
            AnyScssExpressionItem::ScssListExpression(node) => node.format().fmt(f),
            AnyScssExpressionItem::ScssMapExpression(node) => node.format().fmt(f),
            AnyScssExpressionItem::ScssParenthesizedExpression(node) => node.format().fmt(f),
            AnyScssExpressionItem::ScssUnaryExpression(node) => node.format().fmt(f),
        }
    }
}

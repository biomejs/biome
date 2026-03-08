//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssGenericPropertyValueOrExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssGenericPropertyValueOrExpression;
impl FormatRule<AnyCssGenericPropertyValueOrExpression>
    for FormatAnyCssGenericPropertyValueOrExpression
{
    type Context = CssFormatContext;
    fn fmt(
        &self,
        node: &AnyCssGenericPropertyValueOrExpression,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        match node {
            AnyCssGenericPropertyValueOrExpression::CssGenericComponentValueList(node) => {
                node.format().fmt(f)
            }
            AnyCssGenericPropertyValueOrExpression::ScssExpression(node) => node.format().fmt(f),
        }
    }
}

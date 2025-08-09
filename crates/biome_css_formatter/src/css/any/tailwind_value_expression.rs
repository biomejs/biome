//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssTailwindValueExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssTailwindValueExpression;
impl FormatRule<AnyCssTailwindValueExpression> for FormatAnyCssTailwindValueExpression {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssTailwindValueExpression, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssTailwindValueExpression::CssIdentifier(node) => node.format().fmt(f),
            AnyCssTailwindValueExpression::CssString(node) => node.format().fmt(f),
            AnyCssTailwindValueExpression::CssTailwindValueArbitraryType(node) => {
                node.format().fmt(f)
            }
            AnyCssTailwindValueExpression::CssTailwindValueThemeReference(node) => {
                node.format().fmt(f)
            }
        }
    }
}

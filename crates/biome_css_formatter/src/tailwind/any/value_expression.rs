//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyTwValueExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTwValueExpression;
impl FormatRule<AnyTwValueExpression> for FormatAnyTwValueExpression {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyTwValueExpression, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyTwValueExpression::CssIdentifier(node) => node.format().fmt(f),
            AnyTwValueExpression::CssString(node) => node.format().fmt(f),
            AnyTwValueExpression::TwValueArbitraryType(node) => node.format().fmt(f),
            AnyTwValueExpression::TwValueThemeReference(node) => node.format().fmt(f),
        }
    }
}

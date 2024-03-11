//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssSupportsInParens;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssSupportsInParens;
impl FormatRule<AnyCssSupportsInParens> for FormatAnyCssSupportsInParens {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssSupportsInParens, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssSupportsInParens::AnyCssValue(node) => node.format().fmt(f),
            AnyCssSupportsInParens::CssFunction(node) => node.format().fmt(f),
            AnyCssSupportsInParens::CssSupportsConditionInParens(node) => node.format().fmt(f),
            AnyCssSupportsInParens::CssSupportsFeatureDeclaration(node) => node.format().fmt(f),
            AnyCssSupportsInParens::CssSupportsFeatureSelector(node) => node.format().fmt(f),
        }
    }
}

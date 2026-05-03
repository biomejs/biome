//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyScssForwardVisibilityClause;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyScssForwardVisibilityClause;
impl FormatRule<AnyScssForwardVisibilityClause> for FormatAnyScssForwardVisibilityClause {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyScssForwardVisibilityClause, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyScssForwardVisibilityClause::ScssHideClause(node) => node.format().fmt(f),
            AnyScssForwardVisibilityClause::ScssShowClause(node) => node.format().fmt(f),
        }
    }
}

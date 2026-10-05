//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyScssAtRootQuery;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyScssAtRootQuery;
impl FormatRule<AnyScssAtRootQuery> for FormatAnyScssAtRootQuery {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyScssAtRootQuery, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyScssAtRootQuery::ScssAtRootQueryClause(node) => node.format().fmt(f),
            AnyScssAtRootQuery::ScssExpression(node) => node.format().fmt(f),
        }
    }
}

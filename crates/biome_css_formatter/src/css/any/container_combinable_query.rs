//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssContainerCombinableQuery;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssContainerCombinableQuery;
impl FormatRule<AnyCssContainerCombinableQuery> for FormatAnyCssContainerCombinableQuery {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssContainerCombinableQuery, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssContainerCombinableQuery::CssContainerAndQuery(node) => node.format().fmt(f),
            AnyCssContainerCombinableQuery::CssContainerOrQuery(node) => node.format().fmt(f),
            AnyCssContainerCombinableQuery::AnyCssContainerQueryInParens(node) => {
                node.format().fmt(f)
            }
        }
    }
}

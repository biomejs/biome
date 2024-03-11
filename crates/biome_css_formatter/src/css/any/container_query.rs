//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssContainerQuery;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssContainerQuery;
impl FormatRule<AnyCssContainerQuery> for FormatAnyCssContainerQuery {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssContainerQuery, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssContainerQuery::AnyCssContainerQueryInParens(node) => node.format().fmt(f),
            AnyCssContainerQuery::CssContainerAndQuery(node) => node.format().fmt(f),
            AnyCssContainerQuery::CssContainerNotQuery(node) => node.format().fmt(f),
            AnyCssContainerQuery::CssContainerOrQuery(node) => node.format().fmt(f),
        }
    }
}

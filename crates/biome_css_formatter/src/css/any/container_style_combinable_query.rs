//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssContainerStyleCombinableQuery;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssContainerStyleCombinableQuery;
impl FormatRule<AnyCssContainerStyleCombinableQuery> for FormatAnyCssContainerStyleCombinableQuery {
    type Context = CssFormatContext;
    fn fmt(
        &self,
        node: &AnyCssContainerStyleCombinableQuery,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        match node {
            AnyCssContainerStyleCombinableQuery::CssContainerStyleAndQuery(node) => {
                node.format().fmt(f)
            }
            AnyCssContainerStyleCombinableQuery::CssContainerStyleOrQuery(node) => {
                node.format().fmt(f)
            }
            AnyCssContainerStyleCombinableQuery::CssContainerStyleInParens(node) => {
                node.format().fmt(f)
            }
        }
    }
}

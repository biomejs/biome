//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssContainerStyleOrCombinableQuery;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssContainerStyleOrCombinableQuery;
impl FormatRule<AnyCssContainerStyleOrCombinableQuery>
    for FormatAnyCssContainerStyleOrCombinableQuery
{
    type Context = CssFormatContext;
    fn fmt(
        &self,
        node: &AnyCssContainerStyleOrCombinableQuery,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        match node {
            AnyCssContainerStyleOrCombinableQuery::CssContainerStyleInParens(node) => {
                node.format().fmt(f)
            }
            AnyCssContainerStyleOrCombinableQuery::CssContainerStyleOrQuery(node) => {
                node.format().fmt(f)
            }
        }
    }
}

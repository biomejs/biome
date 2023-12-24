//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssContainerStyleAndCombinableQuery;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssContainerStyleAndCombinableQuery;
impl FormatRule<AnyCssContainerStyleAndCombinableQuery>
    for FormatAnyCssContainerStyleAndCombinableQuery
{
    type Context = CssFormatContext;
    fn fmt(
        &self,
        node: &AnyCssContainerStyleAndCombinableQuery,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        match node {
            AnyCssContainerStyleAndCombinableQuery::CssContainerStyleAndQuery(node) => {
                node.format().fmt(f)
            }
            AnyCssContainerStyleAndCombinableQuery::CssContainerStyleInParens(node) => {
                node.format().fmt(f)
            }
        }
    }
}

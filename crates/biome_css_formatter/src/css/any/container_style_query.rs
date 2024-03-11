//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssContainerStyleQuery;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssContainerStyleQuery;
impl FormatRule<AnyCssContainerStyleQuery> for FormatAnyCssContainerStyleQuery {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssContainerStyleQuery, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssContainerStyleQuery::CssContainerStyleAndQuery(node) => node.format().fmt(f),
            AnyCssContainerStyleQuery::CssContainerStyleInParens(node) => node.format().fmt(f),
            AnyCssContainerStyleQuery::CssContainerStyleNotQuery(node) => node.format().fmt(f),
            AnyCssContainerStyleQuery::CssContainerStyleOrQuery(node) => node.format().fmt(f),
            AnyCssContainerStyleQuery::CssDeclaration(node) => node.format().fmt(f),
        }
    }
}

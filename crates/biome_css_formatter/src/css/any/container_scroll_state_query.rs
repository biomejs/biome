//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssContainerScrollStateQuery;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssContainerScrollStateQuery;
impl FormatRule<AnyCssContainerScrollStateQuery> for FormatAnyCssContainerScrollStateQuery {
    type Context = CssFormatContext;
    fn fmt(
        &self,
        node: &AnyCssContainerScrollStateQuery,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        match node {
            AnyCssContainerScrollStateQuery::AnyCssQueryFeature(node) => node.format().fmt(f),
            AnyCssContainerScrollStateQuery::CssContainerScrollStateAndQuery(node) => {
                node.format().fmt(f)
            }
            AnyCssContainerScrollStateQuery::CssContainerScrollStateInParens(node) => {
                node.format().fmt(f)
            }
            AnyCssContainerScrollStateQuery::CssContainerScrollStateNotQuery(node) => {
                node.format().fmt(f)
            }
            AnyCssContainerScrollStateQuery::CssContainerScrollStateOrQuery(node) => {
                node.format().fmt(f)
            }
        }
    }
}

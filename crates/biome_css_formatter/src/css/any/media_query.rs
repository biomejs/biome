//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssMediaQuery;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssMediaQuery;
impl FormatRule<AnyCssMediaQuery> for FormatAnyCssMediaQuery {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssMediaQuery, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssMediaQuery::AnyCssMediaTypeQuery(node) => node.format().fmt(f),
            AnyCssMediaQuery::CssBogusMediaQuery(node) => node.format().fmt(f),
            AnyCssMediaQuery::CssMediaConditionQuery(node) => node.format().fmt(f),
            AnyCssMediaQuery::CssMetavariable(node) => node.format().fmt(f),
        }
    }
}

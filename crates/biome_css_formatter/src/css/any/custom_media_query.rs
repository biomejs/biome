//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssCustomMediaQuery;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssCustomMediaQuery;
impl FormatRule<AnyCssCustomMediaQuery> for FormatAnyCssCustomMediaQuery {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssCustomMediaQuery, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssCustomMediaQuery::CssBooleanMediaQuery(node) => node.format().fmt(f),
            AnyCssCustomMediaQuery::CssMediaQueryList(node) => node.format().fmt(f),
        }
    }
}

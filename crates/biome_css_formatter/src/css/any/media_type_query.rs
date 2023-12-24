//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssMediaTypeQuery;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssMediaTypeQuery;
impl FormatRule<AnyCssMediaTypeQuery> for FormatAnyCssMediaTypeQuery {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssMediaTypeQuery, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssMediaTypeQuery::CssMediaAndTypeQuery(node) => node.format().fmt(f),
            AnyCssMediaTypeQuery::CssMediaTypeQuery(node) => node.format().fmt(f),
        }
    }
}

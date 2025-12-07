//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssIfMediaTestQuery;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssIfMediaTestQuery;
impl FormatRule<AnyCssIfMediaTestQuery> for FormatAnyCssIfMediaTestQuery {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssIfMediaTestQuery, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssIfMediaTestQuery::AnyCssMediaCondition(node) => node.format().fmt(f),
            AnyCssIfMediaTestQuery::AnyCssQueryFeature(node) => node.format().fmt(f),
        }
    }
}

//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnyAngularDefaultClauseBody;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyAngularDefaultClauseBody;
impl FormatRule<AnyAngularDefaultClauseBody> for FormatAnyAngularDefaultClauseBody {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnyAngularDefaultClauseBody, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnyAngularDefaultClauseBody::AngularBlockBody(node) => node.format().fmt(f),
            AnyAngularDefaultClauseBody::AngularDefaultExpressionClause(node) => {
                node.format().fmt(f)
            }
        }
    }
}

//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnyAngularLoadingParameterClause;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyAngularLoadingParameterClause;
impl FormatRule<AnyAngularLoadingParameterClause> for FormatAnyAngularLoadingParameterClause {
    type Context = HtmlFormatContext;
    fn fmt(
        &self,
        node: &AnyAngularLoadingParameterClause,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        match node {
            AnyAngularLoadingParameterClause::AngularAfterTimeClause(node) => node.format().fmt(f),
            AnyAngularLoadingParameterClause::AngularMinimumTimeClause(node) => {
                node.format().fmt(f)
            }
        }
    }
}

//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnyAngularDeferClause;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyAngularDeferClause;
impl FormatRule<AnyAngularDeferClause> for FormatAnyAngularDeferClause {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnyAngularDeferClause, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnyAngularDeferClause::AngularDeferHydrateNeverClause(node) => node.format().fmt(f),
            AnyAngularDeferClause::AngularDeferHydrateOnClause(node) => node.format().fmt(f),
            AnyAngularDeferClause::AngularDeferHydrateWhenClause(node) => node.format().fmt(f),
            AnyAngularDeferClause::AngularDeferOnClause(node) => node.format().fmt(f),
            AnyAngularDeferClause::AngularDeferPrefetchOnClause(node) => node.format().fmt(f),
            AnyAngularDeferClause::AngularDeferPrefetchWhenClause(node) => node.format().fmt(f),
            AnyAngularDeferClause::AngularDeferWhenClause(node) => node.format().fmt(f),
        }
    }
}

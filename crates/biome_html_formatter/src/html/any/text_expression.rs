//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnyHtmlTextExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyHtmlTextExpression;
impl FormatRule<AnyHtmlTextExpression> for FormatAnyHtmlTextExpression {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnyHtmlTextExpression, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnyHtmlTextExpression::HtmlTextExpression(node) => node.format().fmt(f),
            AnyHtmlTextExpression::SvelteTextExpression(node) => node.format().fmt(f),
        }
    }
}

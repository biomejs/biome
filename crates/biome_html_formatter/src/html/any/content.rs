//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnyHtmlContent;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyHtmlContent;
impl FormatRule<AnyHtmlContent> for FormatAnyHtmlContent {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnyHtmlContent, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnyHtmlContent::AnyHtmlTextExpression(node) => node.format().fmt(f),
            AnyHtmlContent::HtmlContent(node) => node.format().fmt(f),
            AnyHtmlContent::HtmlEmbeddedContent(node) => node.format().fmt(f),
        }
    }
}

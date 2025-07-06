//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnyHtmlAstroFrontmatterElement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyHtmlAstroFrontmatterElement;
impl FormatRule<AnyHtmlAstroFrontmatterElement> for FormatAnyHtmlAstroFrontmatterElement {
    type Context = HtmlFormatContext;
    fn fmt(
        &self,
        node: &AnyHtmlAstroFrontmatterElement,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        match node {
            AnyHtmlAstroFrontmatterElement::HtmlAstroFrontmatterElement(node) => {
                node.format().fmt(f)
            }
            AnyHtmlAstroFrontmatterElement::HtmlBogusFrontmatter(node) => node.format().fmt(f),
        }
    }
}

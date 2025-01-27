//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnyHtmlElement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyHtmlElement;
impl FormatRule<AnyHtmlElement> for FormatAnyHtmlElement {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnyHtmlElement, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnyHtmlElement::HtmlBogusElement(node) => node.format().fmt(f),
            AnyHtmlElement::HtmlCdataSection(node) => node.format().fmt(f),
            AnyHtmlElement::HtmlComment(node) => node.format().fmt(f),
            AnyHtmlElement::HtmlContent(node) => node.format().fmt(f),
            AnyHtmlElement::HtmlElement(node) => node.format().fmt(f),
            AnyHtmlElement::HtmlSelfClosingElement(node) => node.format().fmt(f),
        }
    }
}

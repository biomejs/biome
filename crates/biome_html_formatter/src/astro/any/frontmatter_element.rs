//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnyAstroFrontmatterElement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyAstroFrontmatterElement;
impl FormatRule<AnyAstroFrontmatterElement> for FormatAnyAstroFrontmatterElement {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnyAstroFrontmatterElement, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnyAstroFrontmatterElement::AstroBogusFrontmatter(node) => node.format().fmt(f),
            AnyAstroFrontmatterElement::AstroFrontmatterElement(node) => node.format().fmt(f),
        }
    }
}

//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnySvelteTemplateElement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySvelteTemplateElement;
impl FormatRule<AnySvelteTemplateElement> for FormatAnySvelteTemplateElement {
    type Context = HtmlFormatContext;
    fn fmt(
        &self,
        node: &AnySvelteTemplateElement,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        match node {
            AnySvelteTemplateElement::HtmlAttributeSingleTextExpression(node) => {
                node.format().fmt(f)
            }
            AnySvelteTemplateElement::SvelteTemplateChunkElement(node) => {
                node.format().fmt(f)
            }
        }
    }
}

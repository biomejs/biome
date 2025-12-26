//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnyVueVForBindingListElement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyVueVForBindingListElement;
impl FormatRule<AnyVueVForBindingListElement> for FormatAnyVueVForBindingListElement {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnyVueVForBindingListElement, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnyVueVForBindingListElement::AnyVueVForDestructuredBinding(node) => {
                node.format().fmt(f)
            }
            AnyVueVForBindingListElement::VueVForIdentifierBinding(node) => node.format().fmt(f),
            AnyVueVForBindingListElement::VueVForRestBinding(node) => node.format().fmt(f),
        }
    }
}

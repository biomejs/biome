//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnyVueVForBinding;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyVueVForBinding;
impl FormatRule<AnyVueVForBinding> for FormatAnyVueVForBinding {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnyVueVForBinding, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnyVueVForBinding::VueVForSimpleBinding(node) => node.format().fmt(f),
            AnyVueVForBinding::VueVForTupleBinding(node) => node.format().fmt(f),
        }
    }
}

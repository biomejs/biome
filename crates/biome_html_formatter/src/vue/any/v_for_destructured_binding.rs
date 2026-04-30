//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnyVueVForDestructuredBinding;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyVueVForDestructuredBinding;
impl FormatRule<AnyVueVForDestructuredBinding> for FormatAnyVueVForDestructuredBinding {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnyVueVForDestructuredBinding, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnyVueVForDestructuredBinding::VueVForArrayBinding(node) => node.format().fmt(f),
            AnyVueVForDestructuredBinding::VueVForObjectBinding(node) => node.format().fmt(f),
        }
    }
}

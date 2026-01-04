//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnySvelteBindingAssignmentBinding;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySvelteBindingAssignmentBinding;
impl FormatRule<AnySvelteBindingAssignmentBinding> for FormatAnySvelteBindingAssignmentBinding {
    type Context = HtmlFormatContext;
    fn fmt(
        &self,
        node: &AnySvelteBindingAssignmentBinding,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        match node {
            AnySvelteBindingAssignmentBinding::SvelteName(node) => node.format().fmt(f),
            AnySvelteBindingAssignmentBinding::SvelteRestBinding(node) => node.format().fmt(f),
        }
    }
}

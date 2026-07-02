//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnyAngularBinding;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyAngularBinding;
impl FormatRule<AnyAngularBinding> for FormatAnyAngularBinding {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnyAngularBinding, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnyAngularBinding::AngularEventBinding(node) => node.format().fmt(f),
            AnyAngularBinding::AngularPropertyBinding(node) => node.format().fmt(f),
            AnyAngularBinding::AngularTwoWayBinding(node) => node.format().fmt(f),
        }
    }
}

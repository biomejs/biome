//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnyAngularAttribute;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyAngularAttribute;
impl FormatRule<AnyAngularAttribute> for FormatAnyAngularAttribute {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnyAngularAttribute, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnyAngularAttribute::AngularEventBinding(node) => node.format().fmt(f),
            AnyAngularAttribute::AngularPropertyBinding(node) => node.format().fmt(f),
            AnyAngularAttribute::AngularStructuralDirective(node) => node.format().fmt(f),
            AnyAngularAttribute::AngularTemplateRefVariable(node) => node.format().fmt(f),
            AnyAngularAttribute::AngularTwoWayBinding(node) => node.format().fmt(f),
        }
    }
}

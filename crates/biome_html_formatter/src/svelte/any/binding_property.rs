//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnySvelteBindingProperty;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySvelteBindingProperty;
impl FormatRule<AnySvelteBindingProperty> for FormatAnySvelteBindingProperty {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnySvelteBindingProperty, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnySvelteBindingProperty::SvelteLiteral(node) => node.format().fmt(f),
            AnySvelteBindingProperty::SvelteName(node) => node.format().fmt(f),
        }
    }
}

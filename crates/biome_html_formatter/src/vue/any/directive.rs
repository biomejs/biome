//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnyVueDirective;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyVueDirective;
impl FormatRule<AnyVueDirective> for FormatAnyVueDirective {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnyVueDirective, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnyVueDirective::VueBogusDirective(node) => node.format().fmt(f),
            AnyVueDirective::VueDirective(node) => node.format().fmt(f),
            AnyVueDirective::VueVBindShorthandDirective(node) => node.format().fmt(f),
            AnyVueDirective::VueVOnShorthandDirective(node) => node.format().fmt(f),
            AnyVueDirective::VueVSlotShorthandDirective(node) => node.format().fmt(f),
        }
    }
}

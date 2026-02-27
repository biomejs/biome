//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnySvelteDirective;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySvelteDirective;
impl FormatRule<AnySvelteDirective> for FormatAnySvelteDirective {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnySvelteDirective, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnySvelteDirective::SvelteAnimateDirective(node) => node.format().fmt(f),
            AnySvelteDirective::SvelteBindDirective(node) => node.format().fmt(f),
            AnySvelteDirective::SvelteClassDirective(node) => node.format().fmt(f),
            AnySvelteDirective::SvelteInDirective(node) => node.format().fmt(f),
            AnySvelteDirective::SvelteOutDirective(node) => node.format().fmt(f),
            AnySvelteDirective::SvelteStyleDirective(node) => node.format().fmt(f),
            AnySvelteDirective::SvelteTransitionDirective(node) => node.format().fmt(f),
            AnySvelteDirective::SvelteUseDirective(node) => node.format().fmt(f),
        }
    }
}

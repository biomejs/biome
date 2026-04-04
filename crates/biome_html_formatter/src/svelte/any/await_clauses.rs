//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnySvelteAwaitClauses;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySvelteAwaitClauses;
impl FormatRule<AnySvelteAwaitClauses> for FormatAnySvelteAwaitClauses {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnySvelteAwaitClauses, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnySvelteAwaitClauses::SvelteAwaitCatchBlock(node) => node.format().fmt(f),
            AnySvelteAwaitClauses::SvelteAwaitThenBlock(node) => node.format().fmt(f),
            AnySvelteAwaitClauses::SvelteBogusBlock(node) => node.format().fmt(f),
        }
    }
}

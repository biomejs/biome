//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnySvelteDestructuredName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySvelteDestructuredName;
impl FormatRule<AnySvelteDestructuredName> for FormatAnySvelteDestructuredName {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnySvelteDestructuredName, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnySvelteDestructuredName::SvelteCurlyDestructuredName(node) => node.format().fmt(f),
            AnySvelteDestructuredName::SvelteSquareDestructuredName(node) => node.format().fmt(f),
        }
    }
}

//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnySvelteInterpolatedStringPart;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySvelteInterpolatedStringPart;
impl FormatRule<AnySvelteInterpolatedStringPart> for FormatAnySvelteInterpolatedStringPart {
    type Context = HtmlFormatContext;
    fn fmt(
        &self,
        node: &AnySvelteInterpolatedStringPart,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        match node {
            AnySvelteInterpolatedStringPart::HtmlAttributeSingleTextExpression(node) => {
                node.format().fmt(f)
            }
            AnySvelteInterpolatedStringPart::SvelteInterpolatedStringChunk(node) => {
                node.format().fmt(f)
            }
        }
    }
}

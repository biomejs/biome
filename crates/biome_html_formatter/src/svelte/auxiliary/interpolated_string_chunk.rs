use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteInterpolatedStringChunk, SvelteInterpolatedStringChunkFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteInterpolatedStringChunk;
impl FormatNodeRule<SvelteInterpolatedStringChunk> for FormatSvelteInterpolatedStringChunk {
    fn fmt_fields(
        &self,
        node: &SvelteInterpolatedStringChunk,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let SvelteInterpolatedStringChunkFields {
            html_string_literal_token,
        } = node.as_fields();
        write!(f, [html_string_literal_token.format()])
    }
}

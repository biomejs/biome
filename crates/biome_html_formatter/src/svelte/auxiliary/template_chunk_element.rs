use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteTemplateChunkElement, SvelteTemplateChunkElementFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteTemplateChunkElement;
impl FormatNodeRule<SvelteTemplateChunkElement> for FormatSvelteTemplateChunkElement {
    fn fmt_fields(
        &self,
        node: &SvelteTemplateChunkElement,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let SvelteTemplateChunkElementFields {
            html_template_chunk_token,
        } = node.as_fields();
        write!(f, [html_template_chunk_token.format()])
    }
}

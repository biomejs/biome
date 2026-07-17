use crate::prelude::*;
use biome_formatter::write;
use biome_js_syntax::{JsSvelteSnippetRoot, JsSvelteSnippetRootFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsSvelteSnippetRoot;
impl FormatNodeRule<JsSvelteSnippetRoot> for FormatJsSvelteSnippetRoot {
    fn fmt_fields(&self, node: &JsSvelteSnippetRoot, f: &mut JsFormatter) -> FormatResult<()> {
        let JsSvelteSnippetRootFields {
            name,
            eof_token,
            parameters,
        } = node.as_fields();

        write!(f, [name.format(), parameters.format(), eof_token.format()])
    }
}

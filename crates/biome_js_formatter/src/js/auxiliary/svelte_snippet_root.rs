use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_js_syntax::{JsSvelteSnippetRoot, JsSvelteSnippetRootFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsSvelteSnippetRoot;
impl FormatNodeRule<JsSvelteSnippetRoot> for FormatJsSvelteSnippetRoot {
    fn fmt_fields(&self, node: &JsSvelteSnippetRoot, f: &mut JsFormatter) -> FormatResult<()> {
        let JsSvelteSnippetRootFields {
            name,
            type_parameters,
            parameters,
            eof_token,
        } = node.as_fields();

        write!(
            f,
            [
                group(&format_args![
                    name.format(),
                    type_parameters.format(),
                    parameters.format()
                ]),
                eof_token.format()
            ]
        )
    }
}

use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_js_syntax::{JsSvelteEachBindingRoot, JsSvelteEachBindingRootFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsSvelteEachBindingRoot;

impl FormatNodeRule<JsSvelteEachBindingRoot> for FormatJsSvelteEachBindingRoot {
    fn fmt_fields(&self, node: &JsSvelteEachBindingRoot, f: &mut JsFormatter) -> FormatResult<()> {
        let JsSvelteEachBindingRootFields {
            context,
            type_annotation,
            index,
            key,
            eof_token,
        } = node.as_fields();

        write!(
            f,
            [
                group(&format_args![
                    context.format(),
                    type_annotation.format(),
                    index.format(),
                    key.format(),
                ]),
                eof_token.format(),
            ]
        )
    }
}

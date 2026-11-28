use crate::prelude::*;
use biome_formatter::write;
use biome_js_syntax::{JsSvelteEachKey, JsSvelteEachKeyFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsSvelteEachKey;

impl FormatNodeRule<JsSvelteEachKey> for FormatJsSvelteEachKey {
    fn fmt_fields(&self, node: &JsSvelteEachKey, f: &mut JsFormatter) -> FormatResult<()> {
        let JsSvelteEachKeyFields {
            l_paren_token,
            expression,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                space(),
                l_paren_token.format(),
                expression.format(),
                r_paren_token.format(),
            ]
        )
    }
}

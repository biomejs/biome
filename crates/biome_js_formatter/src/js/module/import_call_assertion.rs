use crate::prelude::*;
use biome_formatter::write;
use biome_js_syntax::JsImportCallAssertion;
use biome_js_syntax::JsImportCallAssertionFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsImportCallAssertion;
impl FormatNodeRule<JsImportCallAssertion> for FormatJsImportCallAssertion {
    fn fmt_fields(&self, node: &JsImportCallAssertion, f: &mut JsFormatter) -> FormatResult<()> {
        let JsImportCallAssertionFields {
            with_token,
            colon_token,
            l_curly_token,
            assertions,
            r_curly_token,
        } = node.as_fields();

        write![
            f,
            [
                with_token.format(),
                colon_token.format(),
                space(),
                l_curly_token.format(),
                space(),
                assertions.format(),
                space(),
                r_curly_token.format()
            ]
        ]
    }
}

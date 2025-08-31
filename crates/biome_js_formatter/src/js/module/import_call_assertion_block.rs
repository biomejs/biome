use crate::prelude::*;
use biome_formatter::write;
use biome_js_syntax::JsImportCallAssertionBlock;
use biome_js_syntax::JsImportCallAssertionBlockFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsImportCallAssertionBlock;
impl FormatNodeRule<JsImportCallAssertionBlock> for FormatJsImportCallAssertionBlock {
    fn fmt_fields(
        &self,
        node: &JsImportCallAssertionBlock,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsImportCallAssertionBlockFields {
            l_curly_token,
            assertion,
            r_curly_token,
        } = node.as_fields();

        write![
            f,
            [
                l_curly_token.format(),
                space(),
                assertion.format(),
                space(),
                r_curly_token.format()
            ]
        ]
    }
}

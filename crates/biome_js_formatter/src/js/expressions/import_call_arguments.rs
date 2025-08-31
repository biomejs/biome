use crate::prelude::*;
use biome_formatter::write;
use biome_js_syntax::JsImportCallArguments;
use biome_js_syntax::JsImportCallArgumentsFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsImportCallArguments;
impl FormatNodeRule<JsImportCallArguments> for FormatJsImportCallArguments {
    fn fmt_fields(&self, node: &JsImportCallArguments, f: &mut JsFormatter) -> FormatResult<()> {
        let JsImportCallArgumentsFields {
            l_paren_token,
            argument,
            comma_token,
            js_import_call_assertion_block,
            r_paren_token,
        } = node.as_fields();

        write![f, [l_paren_token.format(), argument.format()]]?;

        if let Some(comma) = comma_token {
            write![f, [comma.format()]]?;
            if let Some(assertion_block) = js_import_call_assertion_block {
                write![f, [space(), assertion_block.format()]]?;
            }
        }

        write![f, [r_paren_token.format()]]
    }
}

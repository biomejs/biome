use crate::prelude::*;
use biome_formatter::write;
use biome_formatter::FormatError::SyntaxError;
use biome_js_syntax::{TsImportTypeArguments, TsImportTypeArgumentsFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsImportTypeArguments;
impl FormatNodeRule<TsImportTypeArguments> for FormatTsImportTypeArguments {
    fn fmt_fields(&self, node: &TsImportTypeArguments, f: &mut JsFormatter) -> FormatResult<()> {
        let TsImportTypeArgumentsFields {
            l_paren_token,
            argument,
            comma_token,
            ts_import_type_assertion_block,
            r_paren_token,
        } = node.as_fields();

        if comma_token.is_some() && ts_import_type_assertion_block.is_some() {
            write!(
                f,
                [
                    l_paren_token.format(),
                    argument.format(),
                    comma_token.format(),
                    space(),
                    ts_import_type_assertion_block.format(),
                    space(),
                    r_paren_token.format()
                ]
            )
        } else {
            Err(SyntaxError)
        }
    }
}

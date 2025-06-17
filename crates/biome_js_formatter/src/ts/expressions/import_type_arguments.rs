use crate::prelude::*;
use biome_formatter::write;
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

        write!(f, [l_paren_token.format(), argument.format()])?;

        if let Some(comma_token) = comma_token {
            write!(f, [comma_token.format(), space()])?;
        }

        if let Some(ts_import_type_assertion_block) = ts_import_type_assertion_block {
            write!(f, [ts_import_type_assertion_block.format()])?;
        }

        write!(f, [r_paren_token.format()])
    }
}

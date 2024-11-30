use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::{TsImportTypeAssertionBlock, TsImportTypeAssertionBlockFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsImportTypeAssertionBlock;
impl FormatNodeRule<TsImportTypeAssertionBlock> for FormatTsImportTypeAssertionBlock {
    fn fmt_fields(
        &self,
        node: &TsImportTypeAssertionBlock,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsImportTypeAssertionBlockFields {
            l_curly_token,
            type_assertion,
            r_curly_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_curly_token.format(),
                type_assertion.format(),
                r_curly_token.format(),
            ]
        )
    }
}

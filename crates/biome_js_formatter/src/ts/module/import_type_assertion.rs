use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::TsImportTypeAssertion;
use biome_js_syntax::TsImportTypeAssertionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsImportTypeAssertion;
impl FormatNodeRule<TsImportTypeAssertion> for FormatTsImportTypeAssertion {
    fn fmt_fields(&self, node: &TsImportTypeAssertion, f: &mut JsFormatter) -> FormatResult<()> {
        let TsImportTypeAssertionFields {
            assertion_kind,
            colon_token,
            l_curly_token,
            assertions,
            r_curly_token,
        } = node.as_fields();

        write!(
            f,
            [
                assertion_kind.format(),
                colon_token.format(),
                space(),
                l_curly_token.format(),
                space(),
                assertions.format(),
                space(),
                r_curly_token.format(),
            ]
        )
    }
}

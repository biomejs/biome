use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::TsIndexedAccessType;
use biome_js_syntax::TsIndexedAccessTypeFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsIndexedAccessType;

impl FormatNodeRule<TsIndexedAccessType> for FormatTsIndexedAccessType {
    fn fmt_fields(&self, node: &TsIndexedAccessType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsIndexedAccessTypeFields {
            object_type,
            l_brack_token,
            index_type,
            r_brack_token,
        } = node.as_fields();
        write![
            f,
            [
                object_type.format(),
                l_brack_token.format(),
                index_type.format(),
                r_brack_token.format()
            ]
        ]
    }
}

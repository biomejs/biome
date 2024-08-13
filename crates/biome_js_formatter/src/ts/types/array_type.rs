use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::{TsArrayType, TsArrayTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsArrayType;

impl FormatNodeRule<TsArrayType> for FormatTsArrayType {
    fn fmt_fields(&self, node: &TsArrayType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsArrayTypeFields {
            l_brack_token,
            element_type,
            r_brack_token,
        } = node.as_fields();
        write![
            f,
            [
                element_type.format(),
                l_brack_token.format(),
                r_brack_token.format(),
            ]
        ]
    }
}

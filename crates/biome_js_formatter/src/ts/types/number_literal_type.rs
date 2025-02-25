use crate::prelude::*;

use biome_formatter::token::number::{format_number_token, NumberFormatOptions};
use biome_formatter::write;
use biome_js_syntax::{TsNumberLiteralType, TsNumberLiteralTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsNumberLiteralType;

impl FormatNodeRule<TsNumberLiteralType> for FormatTsNumberLiteralType {
    fn fmt_fields(&self, node: &TsNumberLiteralType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsNumberLiteralTypeFields {
            minus_token,
            literal_token,
        } = node.as_fields();
        write![
            f,
            [
                minus_token.format(),
                format_number_token(
                    &literal_token?,
                    NumberFormatOptions::default().keep_one_trailing_decimal_zero()
                )
            ]
        ]
    }
}

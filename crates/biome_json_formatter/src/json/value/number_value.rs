use crate::prelude::*;
use biome_formatter::token::number::{format_number_token, NumberFormatOptions};
use biome_json_syntax::JsonNumberValue;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonNumberValue;

impl FormatNodeRule<JsonNumberValue> for FormatJsonNumberValue {
    fn fmt_fields(&self, node: &JsonNumberValue, f: &mut JsonFormatter) -> FormatResult<()> {
        format_number_token(
            &node.value_token()?,
            NumberFormatOptions::default().keep_one_trailing_decimal_zero(),
        )
        .fmt(f)
    }
}

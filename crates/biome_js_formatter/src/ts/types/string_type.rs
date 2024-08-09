use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::{TsStringType, TsStringTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsStringType;

impl FormatNodeRule<TsStringType> for FormatTsStringType {
    fn fmt_fields(&self, node: &TsStringType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsStringTypeFields { string_token } = node.as_fields();

        write![f, [string_token.format()]]
    }
}

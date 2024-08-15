use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::{TsUnknownType, TsUnknownTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsUnknownType;

impl FormatNodeRule<TsUnknownType> for FormatTsUnknownType {
    fn fmt_fields(&self, node: &TsUnknownType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsUnknownTypeFields { unknown_token } = node.as_fields();

        write![f, [unknown_token.format()]]
    }
}

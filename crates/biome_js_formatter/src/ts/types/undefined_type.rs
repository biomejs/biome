use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::{TsUndefinedType, TsUndefinedTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsUndefinedType;

impl FormatNodeRule<TsUndefinedType> for FormatTsUndefinedType {
    fn fmt_fields(&self, node: &TsUndefinedType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsUndefinedTypeFields { undefined_token } = node.as_fields();

        write![f, [undefined_token.format()]]
    }
}

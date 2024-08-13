use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::{TsNeverType, TsNeverTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsNeverType;

impl FormatNodeRule<TsNeverType> for FormatTsNeverType {
    fn fmt_fields(&self, node: &TsNeverType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsNeverTypeFields { never_token } = node.as_fields();
        write![f, [never_token.format()]]
    }
}

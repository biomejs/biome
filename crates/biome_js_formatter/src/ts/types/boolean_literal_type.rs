use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::{TsBooleanLiteralType, TsBooleanLiteralTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsBooleanLiteralType;

impl FormatNodeRule<TsBooleanLiteralType> for FormatTsBooleanLiteralType {
    fn fmt_fields(&self, node: &TsBooleanLiteralType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsBooleanLiteralTypeFields { literal } = node.as_fields();
        write![f, [literal.format()]]
    }
}

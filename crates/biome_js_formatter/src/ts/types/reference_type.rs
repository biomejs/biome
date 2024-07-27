use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::{TsReferenceType, TsReferenceTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsReferenceType;

impl FormatNodeRule<TsReferenceType> for FormatTsReferenceType {
    fn fmt_fields(&self, node: &TsReferenceType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsReferenceTypeFields {
            name,
            type_arguments,
        } = node.as_fields();

        write![f, [name.format(), type_arguments.format()]]
    }
}

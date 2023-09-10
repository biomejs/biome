use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::TsPredicateReturnType;
use biome_js_syntax::TsPredicateReturnTypeFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsPredicateReturnType;

impl FormatNodeRule<TsPredicateReturnType> for FormatTsPredicateReturnType {
    fn fmt_fields(&self, node: &TsPredicateReturnType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsPredicateReturnTypeFields {
            parameter_name,
            is_token,
            ty,
        } = node.as_fields();
        write![
            f,
            [
                parameter_name.format(),
                space(),
                is_token.format(),
                space(),
                ty.format()
            ]
        ]
    }
}

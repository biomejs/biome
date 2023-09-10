use crate::prelude::*;

use biome_js_syntax::{TsTypeParameterName, TsTypeParameterNameFields};
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub struct FormatTsTypeParameterName;

impl FormatNodeRule<TsTypeParameterName> for FormatTsTypeParameterName {
    fn fmt_fields(&self, node: &TsTypeParameterName, f: &mut JsFormatter) -> FormatResult<()> {
        let TsTypeParameterNameFields { ident_token } = node.as_fields();

        write![f, [ident_token.format()]]
    }
}

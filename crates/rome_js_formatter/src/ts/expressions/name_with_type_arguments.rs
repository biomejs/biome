use crate::prelude::*;

use biome_js_syntax::{TsNameWithTypeArguments, TsNameWithTypeArgumentsFields};
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub struct FormatTsNameWithTypeArguments;

impl FormatNodeRule<TsNameWithTypeArguments> for FormatTsNameWithTypeArguments {
    fn fmt_fields(&self, node: &TsNameWithTypeArguments, f: &mut JsFormatter) -> FormatResult<()> {
        let TsNameWithTypeArgumentsFields {
            name,
            type_arguments,
        } = node.as_fields();

        write![f, [name.format(), type_arguments.format()]]
    }
}

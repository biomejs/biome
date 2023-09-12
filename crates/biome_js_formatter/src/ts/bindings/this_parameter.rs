use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::{TsThisParameter, TsThisParameterFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsThisParameter;

impl FormatNodeRule<TsThisParameter> for FormatTsThisParameter {
    fn fmt_fields(&self, node: &TsThisParameter, f: &mut JsFormatter) -> FormatResult<()> {
        let TsThisParameterFields {
            this_token,
            type_annotation,
        } = node.as_fields();

        write![f, [this_token.format(), type_annotation.format()]]
    }
}

use crate::prelude::*;

use biome_js_syntax::{TsThisParameter, TsThisParameterFields};
use rome_formatter::write;

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

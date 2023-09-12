use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::{TsNamedTupleTypeElement, TsNamedTupleTypeElementFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsNamedTupleTypeElement;

impl FormatNodeRule<TsNamedTupleTypeElement> for FormatTsNamedTupleTypeElement {
    fn fmt_fields(&self, node: &TsNamedTupleTypeElement, f: &mut JsFormatter) -> FormatResult<()> {
        let TsNamedTupleTypeElementFields {
            ty,
            question_mark_token,
            colon_token,
            name,
            dotdotdot_token,
        } = node.as_fields();
        write![
            f,
            [
                dotdotdot_token.format(),
                name.format(),
                question_mark_token.format(),
                colon_token.format(),
                space(),
                ty.format(),
            ]
        ]
    }
}

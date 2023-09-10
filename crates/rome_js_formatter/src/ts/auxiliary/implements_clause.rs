use crate::prelude::*;

use biome_js_syntax::TsImplementsClause;
use biome_js_syntax::TsImplementsClauseFields;
use rome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub struct FormatTsImplementsClause;

impl FormatNodeRule<TsImplementsClause> for FormatTsImplementsClause {
    fn fmt_fields(&self, node: &TsImplementsClause, f: &mut JsFormatter) -> FormatResult<()> {
        let TsImplementsClauseFields {
            implements_token,
            types,
        } = node.as_fields();

        write!(
            f,
            [
                implements_token.format(),
                group(&indent(&format_args![
                    soft_line_break_or_space(),
                    types.format()
                ]))
            ]
        )
    }
}

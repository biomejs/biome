use crate::prelude::*;

use biome_js_syntax::{TsExtendsClause, TsExtendsClauseFields};
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub struct FormatTsExtendsClause;

impl FormatNodeRule<TsExtendsClause> for FormatTsExtendsClause {
    fn fmt_fields(&self, node: &TsExtendsClause, f: &mut JsFormatter) -> FormatResult<()> {
        let TsExtendsClauseFields {
            extends_token,
            types,
        } = node.as_fields();

        write!(f, [extends_token.format(), space()])?;

        if types.len() == 1 {
            write!(f, [types.format()])
        } else {
            write!(f, [indent(&types.format())])
        }
    }
}

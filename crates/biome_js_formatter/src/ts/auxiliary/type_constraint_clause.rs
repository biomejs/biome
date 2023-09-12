use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::{TsTypeConstraintClause, TsTypeConstraintClauseFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsTypeConstraintClause;

impl FormatNodeRule<TsTypeConstraintClause> for FormatTsTypeConstraintClause {
    fn fmt_fields(&self, node: &TsTypeConstraintClause, f: &mut JsFormatter) -> FormatResult<()> {
        let TsTypeConstraintClauseFields { extends_token, ty } = node.as_fields();

        let extends = extends_token.format();
        let ty = ty.format();
        write![f, [extends, space(), ty]]
    }
}

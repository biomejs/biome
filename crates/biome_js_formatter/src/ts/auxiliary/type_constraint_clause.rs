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
        let group_id = f.group_id("constraint");
        write![
            f,
            [
                extends,
                group(&indent(&soft_line_break_or_space())).with_group_id(Some(group_id)),
                indent_if_group_breaks(&ty, group_id)
            ]
        ]
    }
}

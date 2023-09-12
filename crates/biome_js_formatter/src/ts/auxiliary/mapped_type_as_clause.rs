use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::{TsMappedTypeAsClause, TsMappedTypeAsClauseFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsMappedTypeAsClause;

impl FormatNodeRule<TsMappedTypeAsClause> for FormatTsMappedTypeAsClause {
    fn fmt_fields(&self, node: &TsMappedTypeAsClause, f: &mut JsFormatter) -> FormatResult<()> {
        let TsMappedTypeAsClauseFields { as_token, ty } = node.as_fields();

        write![f, [as_token.format(), space(), ty.format()]]
    }
}

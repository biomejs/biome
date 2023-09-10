use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::TsMappedTypeReadonlyModifierClause;
use biome_js_syntax::TsMappedTypeReadonlyModifierClauseFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsMappedTypeReadonlyModifierClause;

impl FormatNodeRule<TsMappedTypeReadonlyModifierClause>
    for FormatTsMappedTypeReadonlyModifierClause
{
    fn fmt_fields(
        &self,
        node: &TsMappedTypeReadonlyModifierClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsMappedTypeReadonlyModifierClauseFields {
            operator_token,
            readonly_token,
        } = node.as_fields();
        write![f, [operator_token.format(), readonly_token.format()]]
    }
}

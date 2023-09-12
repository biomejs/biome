use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::{JsxAttributeInitializerClause, JsxAttributeInitializerClauseFields};

#[derive(Debug, Clone, Default)]
pub struct FormatJsxAttributeInitializerClause;

impl FormatNodeRule<JsxAttributeInitializerClause> for FormatJsxAttributeInitializerClause {
    fn fmt_fields(
        &self,
        node: &JsxAttributeInitializerClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsxAttributeInitializerClauseFields { eq_token, value } = node.as_fields();

        write![f, [eq_token.format(), value.format()]]
    }
}

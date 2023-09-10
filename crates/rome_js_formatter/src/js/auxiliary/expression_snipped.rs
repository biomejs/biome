use crate::prelude::*;

use biome_js_syntax::JsExpressionSnipped;
use biome_js_syntax::JsExpressionSnippedFields;
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsExpressionSnipped;

impl FormatNodeRule<JsExpressionSnipped> for FormatJsExpressionSnipped {
    fn fmt_fields(&self, node: &JsExpressionSnipped, f: &mut JsFormatter) -> FormatResult<()> {
        let JsExpressionSnippedFields {
            expression,
            eof_token,
        } = node.as_fields();

        write![f, [expression.format(), format_removed(&eof_token?),]]
    }
}

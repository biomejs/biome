use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::JsExpressionSnippet;
use biome_js_syntax::JsExpressionSnippetFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsExpressionSnippet;

impl FormatNodeRule<JsExpressionSnippet> for FormatJsExpressionSnippet {
    fn fmt_fields(&self, node: &JsExpressionSnippet, f: &mut JsFormatter) -> FormatResult<()> {
        let JsExpressionSnippetFields {
            expression,
            eof_token,
        } = node.as_fields();

        write![f, [expression.format(), format_removed(&eof_token?),]]
    }
}

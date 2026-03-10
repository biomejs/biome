use crate::prelude::*;
use biome_formatter::write;
use biome_js_syntax::JsExpressionTemplateRoot;
use biome_js_syntax::JsExpressionTemplateRootFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsExpressionTemplateRoot;
impl FormatNodeRule<JsExpressionTemplateRoot> for FormatJsExpressionTemplateRoot {
    fn fmt_fields(&self, node: &JsExpressionTemplateRoot, f: &mut JsFormatter) -> FormatResult<()> {
        let JsExpressionTemplateRootFields {
            expression,
            eof_token,
        } = node.as_fields();

        write!(f, [expression.format(), eof_token.format()])
    }
}

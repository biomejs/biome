use crate::prelude::*;
use biome_formatter::write;
use biome_js_syntax::{JsDecorator, JsDecoratorFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsDecorator;
impl FormatNodeRule<JsDecorator> for FormatJsDecorator {
    fn fmt_fields(&self, node: &JsDecorator, f: &mut JsFormatter) -> FormatResult<()> {
        let JsDecoratorFields {
            at_token,
            expression,
        } = node.as_fields();

        write![f, [at_token.format(), expression.format()]]
    }
}

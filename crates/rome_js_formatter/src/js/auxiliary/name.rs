use crate::prelude::*;

use biome_js_syntax::JsName;
use biome_js_syntax::JsNameFields;
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsName;

impl FormatNodeRule<JsName> for FormatJsName {
    fn fmt_fields(&self, node: &JsName, f: &mut JsFormatter) -> FormatResult<()> {
        let JsNameFields { value_token } = node.as_fields();

        write![f, [value_token.format()]]
    }
}

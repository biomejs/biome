use crate::prelude::*;

use biome_js_syntax::JsSpread;
use biome_js_syntax::JsSpreadFields;
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsSpread;

impl FormatNodeRule<JsSpread> for FormatJsSpread {
    fn fmt_fields(&self, node: &JsSpread, f: &mut JsFormatter) -> FormatResult<()> {
        let JsSpreadFields {
            dotdotdot_token,
            argument,
        } = node.as_fields();

        write![f, [dotdotdot_token.format(), argument.format()]]
    }
}

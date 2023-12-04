use crate::prelude::*;
use biome_formatter::write;
use biome_js_syntax::{JsLabel, JsLabelFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsLabel;
impl FormatNodeRule<JsLabel> for FormatJsLabel {
    fn fmt_fields(&self, node: &JsLabel, f: &mut JsFormatter) -> FormatResult<()> {
        let JsLabelFields { value_token } = node.as_fields();
        write![f, [value_token.format()]]
    }
}

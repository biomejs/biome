use crate::prelude::*;
use biome_js_syntax::{JsAccessorModifier, JsAccessorModifierFields};
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAccessorModifier;

impl FormatNodeRule<JsAccessorModifier> for FormatJsAccessorModifier {
    fn fmt_fields(&self, node: &JsAccessorModifier, f: &mut JsFormatter) -> FormatResult<()> {
        let JsAccessorModifierFields { modifier_token } = node.as_fields();

        write![f, [modifier_token.format()]]
    }
}

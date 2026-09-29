use crate::prelude::*;
use biome_formatter::write;
use biome_js_syntax::{JsVueSlotPropsRoot, JsVueSlotPropsRootFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsVueSlotPropsRoot;
impl FormatNodeRule<JsVueSlotPropsRoot> for FormatJsVueSlotPropsRoot {
    fn fmt_fields(&self, node: &JsVueSlotPropsRoot, f: &mut JsFormatter) -> FormatResult<()> {
        let JsVueSlotPropsRootFields {
            parameters,
            eof_token,
        } = node.as_fields();

        write!(f, [group(&parameters.format()), eof_token.format()])
    }
}

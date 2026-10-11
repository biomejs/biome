use crate::prelude::*;
use biome_formatter::write;
use biome_js_syntax::{JsVueGenericRoot, JsVueGenericRootFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsVueGenericRoot;
impl FormatNodeRule<JsVueGenericRoot> for FormatJsVueGenericRoot {
    fn fmt_fields(&self, node: &JsVueGenericRoot, f: &mut JsFormatter) -> FormatResult<()> {
        let JsVueGenericRootFields { items, eof_token } = node.as_fields();

        write!(f, [group(&items.format()), eof_token.format()])
    }
}

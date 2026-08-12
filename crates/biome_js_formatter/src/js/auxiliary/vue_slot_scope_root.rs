use crate::prelude::*;
use biome_formatter::write;
use biome_js_syntax::{JsVueSlotScopeRoot, JsVueSlotScopeRootFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsVueSlotScopeRoot;

impl FormatNodeRule<JsVueSlotScopeRoot> for FormatJsVueSlotScopeRoot {
    fn fmt_fields(&self, node: &JsVueSlotScopeRoot, f: &mut JsFormatter) -> FormatResult<()> {
        let JsVueSlotScopeRootFields { pattern, eof_token } = node.as_fields();

        write!(f, [pattern.format(), format_removed(&eof_token?)])
    }
}

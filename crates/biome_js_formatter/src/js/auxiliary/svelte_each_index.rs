use crate::prelude::*;
use biome_formatter::write;
use biome_js_syntax::{JsSvelteEachIndex, JsSvelteEachIndexFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsSvelteEachIndex;

impl FormatNodeRule<JsSvelteEachIndex> for FormatJsSvelteEachIndex {
    fn fmt_fields(&self, node: &JsSvelteEachIndex, f: &mut JsFormatter) -> FormatResult<()> {
        let JsSvelteEachIndexFields {
            comma_token,
            binding,
        } = node.as_fields();

        write!(f, [comma_token.format(), space(), binding.format()])
    }
}

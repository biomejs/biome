use crate::prelude::*;
use biome_formatter::write;
use biome_js_syntax::{JsSvelteGenericsRoot, JsSvelteGenericsRootFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsSvelteGenericsRoot;
impl FormatNodeRule<JsSvelteGenericsRoot> for FormatJsSvelteGenericsRoot {
    fn fmt_fields(&self, node: &JsSvelteGenericsRoot, f: &mut JsFormatter) -> FormatResult<()> {
        let JsSvelteGenericsRootFields {
            type_parameters,
            eof_token,
        } = node.as_fields();

        write!(f, [type_parameters.format(), eof_token.format()])
    }
}

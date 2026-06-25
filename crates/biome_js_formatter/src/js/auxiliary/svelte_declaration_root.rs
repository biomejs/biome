use crate::prelude::*;
use biome_formatter::write;
use biome_js_syntax::{JsSvelteDeclarationRoot, JsSvelteDeclarationRootFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsSvelteDeclarationRoot;
impl FormatNodeRule<JsSvelteDeclarationRoot> for FormatJsSvelteDeclarationRoot {
    fn fmt_fields(&self, node: &JsSvelteDeclarationRoot, f: &mut JsFormatter) -> FormatResult<()> {
        let JsSvelteDeclarationRootFields {
            declarations,
            eof_token,
        } = node.as_fields();

        write!(f, [declarations.format(), eof_token.format()])
    }
}

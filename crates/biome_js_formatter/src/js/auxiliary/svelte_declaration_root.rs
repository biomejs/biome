use crate::prelude::*;
use biome_formatter::write;
use biome_js_syntax::{JsSvelteDeclarationRoot, JsSvelteDeclarationRootFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsSvelteDeclarationRoot;
impl FormatNodeRule<JsSvelteDeclarationRoot> for FormatJsSvelteDeclarationRoot {
    fn fmt_fields(&self, node: &JsSvelteDeclarationRoot, f: &mut JsFormatter) -> FormatResult<()> {
        let JsSvelteDeclarationRootFields {
            declaration,
            semicolon_token,
            eof_token,
        } = node.as_fields();

        write!(f, [declaration.format()])?;
        if let Some(semicolon_token) = semicolon_token {
            write!(f, [format_removed(&semicolon_token)])?;
        }
        write!(f, [format_removed(&eof_token?)])
    }
}

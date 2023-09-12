use crate::prelude::*;
use biome_formatter::{format_args, write};

use biome_js_syntax::JsVariableDeclaration;
use biome_js_syntax::JsVariableDeclarationFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsVariableDeclaration;

impl FormatNodeRule<JsVariableDeclaration> for FormatJsVariableDeclaration {
    fn fmt_fields(&self, node: &JsVariableDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        let JsVariableDeclarationFields {
            await_token,
            kind,
            declarators,
        } = node.as_fields();

        if let Some(await_token) = await_token {
            write!(f, [await_token.format(), space()])?;
        }

        write![
            f,
            [group(&format_args![
                kind.format(),
                space(),
                declarators.format()
            ])]
        ]
    }
}

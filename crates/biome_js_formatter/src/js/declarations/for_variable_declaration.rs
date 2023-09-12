use crate::prelude::*;

use biome_formatter::{format_args, write};
use biome_js_syntax::JsForVariableDeclaration;
use biome_js_syntax::JsForVariableDeclarationFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsForVariableDeclaration;

impl FormatNodeRule<JsForVariableDeclaration> for FormatJsForVariableDeclaration {
    fn fmt_fields(&self, node: &JsForVariableDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        let JsForVariableDeclarationFields {
            await_token,
            kind_token,
            declarator,
        } = node.as_fields();

        if let Some(await_token) = await_token {
            write!(f, [await_token.format(), space()])?;
        }

        write![
            f,
            [group(&format_args![
                kind_token.format(),
                space(),
                declarator.format()
            ])]
        ]
    }
}

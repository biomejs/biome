use crate::prelude::*;
use biome_css_syntax::{ScssVariableDeclaration, ScssVariableDeclarationFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssVariableDeclaration;

impl FormatNodeRule<ScssVariableDeclaration> for FormatScssVariableDeclaration {
    fn fmt_fields(&self, node: &ScssVariableDeclaration, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssVariableDeclarationFields {
            name,
            colon_token,
            value,
            modifiers,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [name.format(), colon_token.format(), space(), value.format(),]
        )?;

        if !modifiers.is_empty() {
            write!(f, [modifiers.format()])?;
        }

        write!(f, [semicolon_token.format()])
    }
}

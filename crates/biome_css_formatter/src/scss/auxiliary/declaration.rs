use crate::prelude::*;
use biome_css_syntax::{ScssDeclaration, ScssDeclarationFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssDeclaration;

impl FormatNodeRule<ScssDeclaration> for FormatScssDeclaration {
    fn fmt_fields(&self, node: &ScssDeclaration, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssDeclarationFields {
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

use crate::prelude::*;
use biome_css_syntax::{ScssSupportsDeclaration, ScssSupportsDeclarationFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssSupportsDeclaration;

impl FormatNodeRule<ScssSupportsDeclaration> for FormatScssSupportsDeclaration {
    fn fmt_fields(&self, node: &ScssSupportsDeclaration, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssSupportsDeclarationFields {
            name,
            colon_token,
            value,
            important,
        } = node.as_fields();

        write!(
            f,
            [name.format(), colon_token.format(), space(), value.format()]
        )?;

        if let Some(important) = important {
            write!(f, [space(), important.format()])?;
        }

        Ok(())
    }
}

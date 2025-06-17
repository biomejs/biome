use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritLanguageDeclaration, GritLanguageDeclarationFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritLanguageDeclaration;
impl FormatNodeRule<GritLanguageDeclaration> for FormatGritLanguageDeclaration {
    fn fmt_fields(
        &self,
        node: &GritLanguageDeclaration,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        let GritLanguageDeclarationFields {
            language_token,
            name,
            flavor,
            semicolon_token,
        } = node.as_fields();
        write!(
            f,
            [
                language_token.format(),
                space(),
                name.format(),
                flavor.format(),
            ]
        )?;

        if let Some(semicolon_token) = semicolon_token {
            write!(f, [format_removed(&semicolon_token)])?;
        }

        Ok(())
    }
}

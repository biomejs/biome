use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritLanguageFlavor, GritLanguageFlavorFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritLanguageFlavor;
impl FormatNodeRule<GritLanguageFlavor> for FormatGritLanguageFlavor {
    fn fmt_fields(&self, node: &GritLanguageFlavor, f: &mut GritFormatter) -> FormatResult<()> {
        let GritLanguageFlavorFields {
            l_paren_token,
            flavors,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_paren_token.format(),
                flavors.format(),
                r_paren_token.format()
            ]
        )
    }
}

use crate::prelude::*;
use biome_css_syntax::{TwFunctionalUtilityName, TwFunctionalUtilityNameFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwFunctionalUtilityName;
impl FormatNodeRule<TwFunctionalUtilityName> for FormatTwFunctionalUtilityName {
    fn fmt_fields(&self, node: &TwFunctionalUtilityName, f: &mut CssFormatter) -> FormatResult<()> {
        let TwFunctionalUtilityNameFields {
            identifier,
            minus_token,
            star_token,
        } = node.as_fields();

        write!(
            f,
            [
                identifier.format(),
                minus_token.format(),
                star_token.format()
            ]
        )
    }
}

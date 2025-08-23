use crate::prelude::*;
use biome_css_syntax::{TwValueThemeReference, TwValueThemeReferenceFields};
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwValueThemeReference;
impl FormatNodeRule<TwValueThemeReference> for FormatTwValueThemeReference {
    fn fmt_fields(&self, node: &TwValueThemeReference, f: &mut CssFormatter) -> FormatResult<()> {
        let TwValueThemeReferenceFields {
            reference,
            minus_token,
            star_token,
        } = node.as_fields();

        write!(
            f,
            [
                reference.format(),
                minus_token.format(),
                star_token.format()
            ]
        )
    }
}

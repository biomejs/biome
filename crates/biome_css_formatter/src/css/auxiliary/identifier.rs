use crate::{prelude::*, utils::string_utils::FormatTokenAsLowercase};
use biome_css_syntax::{CssIdentifier, CssIdentifierFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIdentifier;

impl FormatNodeRule<CssIdentifier> for FormatCssIdentifier {
    fn fmt_fields(&self, node: &CssIdentifier, f: &mut CssFormatter) -> FormatResult<()> {
        let CssIdentifierFields { value_token } = node.as_fields();

        // Identifiers in CSS are used all over the place. Type selectors,
        // declaration names, value definitions, and plenty more. For the most
        // part, these identifiers are case-insensitive, meaning they can
        // safely be re-written in any casing, and for formatting we want them
        // to always be in lowercase.
        //
        // Other kinds of identifiers (custom identifiers and dashed
        // identifiers) are defined to be case-sensitive, which is why they
        // have their own types to be parsed and formatted separately, ensuring
        // that only identifiers which _can_ be re-written this way are.
        write!(f, [FormatTokenAsLowercase::from(value_token?)])
    }
}

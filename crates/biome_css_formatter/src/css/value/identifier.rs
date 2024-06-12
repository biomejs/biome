use crate::{prelude::*, utils::string_utils::FormatTokenAsLowercase};
use biome_css_syntax::{CssIdentifier, CssIdentifierFields};
use biome_formatter::{write, FormatRuleWithOptions};

#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct FormatCssIdentifier {
    prevent_lowercase: bool,
}

#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct FormatCssIdentifierOptions {
    pub(crate) prevent_lowercase: bool,
}

impl FormatCssIdentifierOptions {
    pub(crate) fn prevent_lowercase(mut self, prevent_lowercase: bool) -> Self {
        self.prevent_lowercase = prevent_lowercase;
        self
    }
}

impl FormatRuleWithOptions<CssIdentifier> for FormatCssIdentifier {
    type Options = FormatCssIdentifierOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.prevent_lowercase = options.prevent_lowercase;
        self
    }
}

impl FormatNodeRule<CssIdentifier> for FormatCssIdentifier {
    fn fmt_fields(&self, node: &CssIdentifier, f: &mut CssFormatter) -> FormatResult<()> {
        let CssIdentifierFields { value_token } = node.as_fields();

        if self.prevent_lowercase {
            return write!(f, [value_token.format()]);
        }

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

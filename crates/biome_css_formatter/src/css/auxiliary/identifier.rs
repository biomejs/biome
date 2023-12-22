use std::borrow::Cow;

use crate::prelude::*;
use biome_css_syntax::{CssIdentifier, CssIdentifierFields};
use biome_formatter::{token::string::ToAsciiLowercaseCow, write, FormatRuleWithOptions};

#[derive(Default, Debug)]
pub(crate) struct FormatCssIdentifierOptions {
    /// Whether the formatter should rewrite the identifier using lowercase
    /// letters.
    pub(crate) forced_lowercase: bool,
}

impl FormatRuleWithOptions<CssIdentifier> for FormatCssIdentifier {
    type Options = FormatCssIdentifierOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.forced_lowercase = options.forced_lowercase;
        self
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIdentifier {
    forced_lowercase: bool,
}

impl FormatNodeRule<CssIdentifier> for FormatCssIdentifier {
    fn fmt_fields(&self, node: &CssIdentifier, f: &mut CssFormatter) -> FormatResult<()> {
        let CssIdentifierFields { value_token } = node.as_fields();

        // The parser uses identifiers to represent a few different things:
        // selector names, rule names, values, and also units. For formatting,
        // we always want to write units in lowercase, but all of the others
        // we want to preserve their casing.
        if self.forced_lowercase {
            let value_token = value_token?;

            let original = value_token.text_trimmed();
            match original.to_ascii_lowercase_cow() {
                Cow::Borrowed(_) => write![f, [value_token.format()]],
                Cow::Owned(lowercase) => {
                    write!(
                        f,
                        [format_replaced(
                            &value_token,
                            &dynamic_text(&lowercase, value_token.text_trimmed_range().start())
                        )]
                    )
                }
            }
        } else {
            write!(f, [value_token.format()])
        }
    }
}

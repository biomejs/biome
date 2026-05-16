use super::interpolated_string::ScssInterpolatedStringQuotes;
use crate::prelude::*;
use biome_css_syntax::{ScssStringText, ScssStringTextFields};
use biome_formatter::{FormatRuleWithOptions, prelude::text, token::string::normalize_string};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssStringText {
    quotes: Option<ScssInterpolatedStringQuotes>,
}

impl FormatRuleWithOptions<ScssStringText> for FormatScssStringText {
    type Options = ScssInterpolatedStringQuotes;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.quotes = Some(options);
        self
    }
}

impl FormatNodeRule<ScssStringText> for FormatScssStringText {
    fn fmt_fields(&self, node: &ScssStringText, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssStringTextFields { value_token } = node.as_fields();
        let token = value_token?;
        let quotes = self.quotes.unwrap_or_else(|| {
            ScssInterpolatedStringQuotes::from_quote_style(f.options().quote_style())
        });
        // `ScssStringText` is only the text between quotes in `"foo #{$bar}"`,
        // so the parent `ScssInterpolatedString` owns the quote tokens.
        let normalized = normalize_string(
            token.text_trimmed(),
            quotes.preferred().into(),
            quotes.will_change(),
        );

        format_replaced(
            &token,
            &text(normalized.as_ref(), token.text_trimmed_range().start()),
        )
        .fmt(f)
    }
}

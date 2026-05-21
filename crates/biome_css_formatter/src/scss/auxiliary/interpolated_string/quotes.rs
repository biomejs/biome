use crate::prelude::*;
use crate::utils::string_utils::preferred_quote_style_for_contents;
use biome_css_syntax::{
    AnyScssInterpolatedStringPart, CssSyntaxToken, ScssInterpolatedStringPartList,
};
use biome_formatter::{QuoteStyle, prelude::text};

/// Quote policy for one SCSS interpolated string.
///
/// Unlike `CssString`, the quotes and text parts are separate nodes:
/// `"foo #{$bar}"` owns quotes on `ScssInterpolatedString` and `foo ` as
/// `ScssStringText`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ScssInterpolatedStringQuotes {
    opening: Option<QuoteStyle>,
    preferred: QuoteStyle,
    will_change: bool,
}

impl ScssInterpolatedStringQuotes {
    /// Chooses the quote style for the whole interpolated string.
    ///
    /// Text parts reuse the normal string quote scoring. Interpolation-only
    /// strings may keep source quotes so `'#{my-fn("x")}'` does not need to
    /// escape the raw `"` inside `#{...}`.
    pub(crate) fn new(
        opening_quote_token: &CssSyntaxToken,
        parts: &ScssInterpolatedStringPartList,
        chosen_quote: QuoteStyle,
    ) -> Self {
        let opening = opening_quote_token
            .text_trimmed()
            .bytes()
            .next()
            .and_then(QuoteStyle::from_byte);
        let preferred = preferred_quote_style(parts, opening, chosen_quote);
        let will_change = opening.is_some_and(|current_quote| current_quote != preferred);

        Self {
            opening,
            preferred,
            will_change,
        }
    }

    /// Uses the configured quote style when no owner supplied string policy.
    pub(crate) fn from_quote_style(chosen_quote: QuoteStyle) -> Self {
        Self {
            opening: None,
            preferred: chosen_quote,
            will_change: false,
        }
    }

    /// Returns the quote used by the source string.
    pub(super) fn opening(self) -> Option<QuoteStyle> {
        self.opening
    }

    /// Returns the quote chosen for this interpolated string.
    pub(crate) fn preferred(self) -> QuoteStyle {
        self.preferred
    }

    /// Returns true when content escaping must account for changed delimiters.
    pub(crate) fn will_change(self) -> bool {
        self.will_change
    }

    /// Replaces only the quote token.
    ///
    /// `ScssStringText` normalizes the content separately with `normalize_string`.
    pub(super) fn write_quote_token(
        self,
        token: &CssSyntaxToken,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let replacement = if self.preferred.is_double() {
            "\""
        } else {
            "'"
        };

        format_replaced(
            token,
            &text(replacement, token.text_trimmed_range().start()),
        )
        .fmt(f)
    }
}

/// Chooses the quote style for one interpolated string.
///
/// Text parts use the normal string quote scoring. Interpolation-only strings,
/// such as `'#{my-fn("x")}'`, can keep the outer `'` to leave raw `"` alone.
fn preferred_quote_style(
    parts: &ScssInterpolatedStringPartList,
    opening_quote: Option<QuoteStyle>,
    chosen_quote: QuoteStyle,
) -> QuoteStyle {
    let mut contents = parts
        .iter()
        .filter_map(|part| match part {
            AnyScssInterpolatedStringPart::ScssStringText(text) => text
                .value_token()
                .ok()
                .map(|token| token.token_text_trimmed()),
            AnyScssInterpolatedStringPart::ScssInterpolation(_) => None,
        })
        .peekable();

    if contents.peek().is_some() {
        return preferred_quote_style_for_contents(contents, chosen_quote);
    }

    // No text parts means there is nothing to score. Keep `'#{my-fn("x")}'`
    // single-quoted when `quoteStyle: double` would force escaping the raw `"`.
    if let Some(opening_quote) = opening_quote
        && opening_quote != chosen_quote
        && raw_interpolation_has_quote(parts, chosen_quote)
    {
        return opening_quote;
    }

    chosen_quote
}

/// Checks raw `#{...}` bodies for a quote conflict.
///
/// Example: with `quoteStyle: double`, `'#{my-fn("x")}'` keeps the outer `'`.
fn raw_interpolation_has_quote(parts: &ScssInterpolatedStringPartList, quote: QuoteStyle) -> bool {
    let quote = quote.as_byte() as char;

    parts.iter().any(|part| {
        matches!(
            part,
            AnyScssInterpolatedStringPart::ScssInterpolation(interpolation)
                if interpolation.syntax().text_trimmed().contains_char(quote)
        )
    })
}

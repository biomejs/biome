use crate::prelude::*;
use crate::utils::string_utils::preferred_quote_style_for_contents;
use biome_css_syntax::{
    AnyScssInterpolatedStringPart, CssSyntaxToken, ScssInterpolatedString,
    ScssInterpolatedStringFields, ScssStringText,
};
use biome_formatter::{QuoteStyle, token::string::normalize_string};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssInterpolatedString;

impl FormatNodeRule<ScssInterpolatedString> for FormatScssInterpolatedString {
    fn fmt_fields(&self, node: &ScssInterpolatedString, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssInterpolatedStringFields {
            opening_quote_token,
            parts,
            closing_quote_token,
        } = node.as_fields();
        let opening_quote_token = opening_quote_token?;
        let closing_quote_token = closing_quote_token?;

        let preferred_quote = preferred_quote_style(&parts, f.options().quote_style());
        let quotes_will_change = opening_quote_token
            .text_trimmed()
            .bytes()
            .next()
            .and_then(QuoteStyle::from_byte)
            .is_some_and(|current_quote| current_quote != preferred_quote);

        write_quote_token(&opening_quote_token, preferred_quote, f)?;

        for part in parts {
            match part {
                AnyScssInterpolatedStringPart::ScssInterpolation(interpolation) => {
                    interpolation.format().fmt(f)?;
                }
                AnyScssInterpolatedStringPart::ScssStringText(text) => {
                    if f.comments().is_suppressed(text.syntax()) {
                        text.format().fmt(f)?;
                    } else {
                        write_string_text(&text, preferred_quote, quotes_will_change, f)?;
                    }
                }
            }
        }

        write_quote_token(&closing_quote_token, preferred_quote, f)
    }
}

fn preferred_quote_style(
    parts: &biome_css_syntax::ScssInterpolatedStringPartList,
    chosen_quote: QuoteStyle,
) -> QuoteStyle {
    preferred_quote_style_for_contents(
        parts
            .iter()
            .filter_map(|part| match part {
                AnyScssInterpolatedStringPart::ScssStringText(text) => text.value_token().ok(),
                AnyScssInterpolatedStringPart::ScssInterpolation(_) => None,
            })
            .map(|token| token.token_text_trimmed()),
        chosen_quote,
    )
}

fn write_quote_token(
    token: &CssSyntaxToken,
    quote_style: QuoteStyle,
    f: &mut CssFormatter,
) -> FormatResult<()> {
    let replacement = if quote_style.is_double() { "\"" } else { "'" };

    format_replaced(
        token,
        &text(replacement, token.text_trimmed_range().start()),
    )
    .fmt(f)
}

fn write_string_text(
    string_text: &ScssStringText,
    preferred_quote: QuoteStyle,
    quotes_will_change: bool,
    f: &mut CssFormatter,
) -> FormatResult<()> {
    let token = string_text.value_token()?;
    let normalized = normalize_string(
        token.text_trimmed(),
        preferred_quote.into(),
        quotes_will_change,
    );

    format_replaced(
        &token,
        &text(normalized.as_ref(), token.text_trimmed_range().start()),
    )
    .fmt(f)
}

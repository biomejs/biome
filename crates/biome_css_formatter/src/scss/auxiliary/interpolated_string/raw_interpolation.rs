use super::ScssInterpolatedStringQuotes;
use crate::prelude::*;
use crate::verbatim::{CssVerbatimTokenFormat, format_css_verbatim_range};
use biome_css_syntax::{
    CssNumberScanOptions, CssSyntaxKind::CSS_STRING_LITERAL, CssSyntaxToken, ScssInterpolation,
    ScssInterpolationFields, scan_css_number,
};
use biome_formatter::{
    token::number::{NumberFormatOptions, format_trimmed_number},
    write,
};
use biome_rowan::{SyntaxNodeText, TextRange, TextSize};
use std::borrow::Cow;

/// Prints `#{...}` inside strings as mostly raw source text.
///
/// This is intentionally different from [`FormatScssInterpolation`]. Outside
/// strings, Prettier formats interpolation as a normal SCSS value:
/// `#{$x   +   $y}` becomes `#{$x + $y}`. Inside quoted strings, Prettier
/// treats the interpolation body as string content and preserves its spacing:
/// `"#{$x   +   $y}"` stays `"#{$x   +   $y}"`.
///
/// The only formatting we still apply inside the raw body is token-local string
/// normalization. For example, `#{".5"}` becomes `#{"0.5"}`.
///
/// The raw body delegates token tracking, suppression checks, and comment
/// marking to the verbatim range formatter because child [`FormatNodeRule`]s
/// do not run for preserved source text.
pub(super) struct FormatRawScssStringInterpolation<'a> {
    interpolation: &'a ScssInterpolation,
    quotes: ScssInterpolatedStringQuotes,
}

impl<'a> FormatRawScssStringInterpolation<'a> {
    pub(super) fn new(
        interpolation: &'a ScssInterpolation,
        quotes: ScssInterpolatedStringQuotes,
    ) -> Self {
        Self {
            interpolation,
            quotes,
        }
    }

    /// Returns true when quote style must be applied inside `#{...}`.
    ///
    /// With `quoteStyle: double`, `content: '#{my-fn('x')}'` formats the body
    /// normally so the inner string becomes `"x"`.
    fn should_format_body_for_quote_style(&self, raw_body: Option<&SyntaxNodeText>) -> bool {
        self.quotes
            .opening()
            .is_some_and(|quote| !quote.is_double())
            && self.quotes.preferred().is_double()
            && raw_body.is_some_and(|body| body.contains_char('\''))
    }
}

impl Format<CssFormatContext> for FormatRawScssStringInterpolation<'_> {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let interpolation = self.interpolation;
        let raw_body = raw_interpolation_body(interpolation);

        if self.should_format_body_for_quote_style(raw_body.as_ref()) {
            return interpolation.format().fmt(f);
        }

        let ScssInterpolationFields {
            hash_token,
            l_curly_token,
            r_curly_token,
            ..
        } = interpolation.as_fields();

        write!(
            f,
            [
                hash_token.format(),
                l_curly_token.format(),
                FormatRawScssStringInterpolationBody::new(interpolation),
                r_curly_token.format()
            ]
        )
    }
}

/// Returns the raw body of `#{...}` without the interpolation braces.
///
/// Example: `#{ get($map, "key") }` returns ` get($map, "key") `.
fn raw_interpolation_body(interpolation: &ScssInterpolation) -> Option<SyntaxNodeText> {
    let body_range = raw_interpolation_body_range(interpolation)?;
    let raw_start = interpolation.syntax().text_trimmed_range().start();
    let relative_range =
        TextRange::new(body_range.start() - raw_start, body_range.end() - raw_start);
    let raw = interpolation.syntax().text_trimmed();

    Some(raw.slice(relative_range))
}

/// Streams the raw body of `#{...}` inside strings.
///
/// Prettier keeps `#{.5 + .6}` raw, but changes `#{".5" + ".6"}` to
/// `#{"0.5" + "0.6"}`.
struct FormatRawScssStringInterpolationBody<'a> {
    interpolation: &'a ScssInterpolation,
}

impl<'a> FormatRawScssStringInterpolationBody<'a> {
    fn new(interpolation: &'a ScssInterpolation) -> Self {
        Self { interpolation }
    }
}

impl Format<CssFormatContext> for FormatRawScssStringInterpolationBody<'_> {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let Some(body_range) = raw_interpolation_body_range(self.interpolation) else {
            return Ok(());
        };

        format_css_verbatim_range(
            self.interpolation.syntax(),
            body_range,
            format_raw_interpolation_token,
        )
        .fmt(f)
    }
}

/// Returns the source range between interpolation braces.
///
/// Example: `#{ get($map) }` returns the range for ` get($map) `.
fn raw_interpolation_body_range(interpolation: &ScssInterpolation) -> Option<TextRange> {
    let range = interpolation.syntax().text_trimmed_range();

    (range.len() >= TextSize::from(3)).then(|| {
        TextRange::new(
            range.start() + TextSize::from(2),
            range.end() - TextSize::from(1),
        )
    })
}

fn format_raw_interpolation_token(
    token: &CssSyntaxToken,
    range: TextRange,
    f: &mut CssFormatter,
) -> FormatResult<CssVerbatimTokenFormat> {
    if token.kind() == CSS_STRING_LITERAL {
        write_raw_string_token(token, range, f)
    } else {
        Ok(CssVerbatimTokenFormat::Source)
    }
}

/// Writes one string token in a raw interpolation body.
///
/// Example: `#{".5"}` prints the string token as `"0.5"`.
fn write_raw_string_token(
    token: &CssSyntaxToken,
    range: TextRange,
    f: &mut CssFormatter,
) -> FormatResult<CssVerbatimTokenFormat> {
    let trimmed_range = token.text_trimmed_range();
    let Some(content_range) = range.intersect(trimmed_range) else {
        return Ok(CssVerbatimTokenFormat::Source);
    };

    if content_range != trimmed_range {
        return Ok(CssVerbatimTokenFormat::Source);
    }

    if range.start() < content_range.start() {
        located_token_text(token, TextRange::new(range.start(), content_range.start())).fmt(f)?;
    }

    let normalized = normalize_numbers_in_string_token(token.text_trimmed());
    syntax_token_cow_slice(normalized, token, content_range.start()).fmt(f)?;

    if content_range.end() < range.end() {
        located_token_text(token, TextRange::new(content_range.end(), range.end())).fmt(f)?;
    }

    Ok(CssVerbatimTokenFormat::Replacement)
}

/// Normalizes CSS-like numbers in one raw string token.
///
/// Example: `".5px"` becomes `"0.5px"`.
fn normalize_numbers_in_string_token(raw: &str) -> Cow<'_, str> {
    if has_string_escape(raw) {
        return Cow::Borrowed(raw);
    }

    let mut normalized = None;
    let mut last_copied = 0;
    let mut index = 0;
    let scan_options = CssNumberScanOptions::default().with_standalone_boundary(true);

    while index < raw.len() {
        let Some(end) = scan_css_number(raw, index, scan_options) else {
            let Some(next) = raw.get(index..).and_then(|tail| tail.chars().next()) else {
                break;
            };

            index += next.len_utf8();
            continue;
        };

        let Some(number) = raw.get(index..end) else {
            return Cow::Borrowed(raw);
        };

        let formatted = format_trimmed_number(number, NumberFormatOptions::default());
        let Cow::Owned(formatted) = formatted else {
            index = end;
            continue;
        };

        if index == 0 && end == raw.len() {
            return Cow::Owned(formatted);
        }

        let normalized = normalized.get_or_insert_with(|| {
            String::with_capacity(raw.len() + formatted.len().saturating_sub(number.len()))
        });

        let Some(unchanged) = raw.get(last_copied..index) else {
            return Cow::Borrowed(raw);
        };

        normalized.push_str(unchanged);
        normalized.push_str(&formatted);
        last_copied = end;
        index = end;
    }

    if let Some(mut normalized) = normalized {
        let Some(tail) = raw.get(last_copied..) else {
            return Cow::Borrowed(raw);
        };

        normalized.push_str(tail);
        Cow::Owned(normalized)
    } else {
        Cow::Borrowed(raw)
    }
}

/// Detects CSS escapes inside a raw string token.
///
/// Example: `"\\.5"` stays raw because changing `.5` to `0.5` would change the
/// escaped string value.
fn has_string_escape(raw: &str) -> bool {
    raw.as_bytes().contains(&b'\\')
}

use crate::content_lines::ContentLines;
use crate::prelude::*;
use biome_formatter::{QuoteStyle, write};
use biome_yaml_syntax::YamlSyntaxToken;
use std::borrow::Cow;

/// Formats the value token of a flow scalar: a plain, single quoted, or
/// double quoted scalar.
///
/// The line structure of a multiline scalar is preserved, but the whitespace
/// around its line breaks folds into the breaks and isn't content, so it is
/// normalized: the trailing whitespace of every line is dropped, and the
/// indentation of the continuation lines is replaced with the formatter's own
/// indentation. Blank lines are content, a line feed each, and are kept:
///
/// ```yaml
/// key: word
///       word
///
///       word
/// ```
///
/// becomes
///
/// ```yaml
/// key: word
///   word
///
///   word
/// ```
pub(crate) struct FormatFlowScalar<'a> {
    pub(crate) token: &'a YamlSyntaxToken,
}

impl Format<YamlFormatContext> for FormatFlowScalar<'_> {
    fn fmt(&self, f: &mut YamlFormatter) -> FormatResult<()> {
        let normalized = normalize_quotes(
            self.token.text_trimmed().trim_end(),
            f.options().quote_style(),
        );
        let value = normalized.as_ref();
        let position = Some(self.token.text_trimmed_range().start());

        if !value.contains(['\n', '\r']) {
            return write!(f, [format_replaced(self.token, &text(value, position))]);
        }

        let content = format_with(|f| {
            let mut lines = ContentLines::new(value);

            // Leading whitespace before the token is trivia rather than part
            // of its text, so the first line only has its end trimmed
            if let Some(first) = lines.next() {
                write!(f, [text(first.trim_end(), position)])?;
            }

            let mut prev_empty = false;
            while let Some(line) = lines.next() {
                // The trailing whitespace of the last line is kept: in a
                // quoted scalar it sits against the closing quote and is
                // content
                let line = if lines.has_remaining() {
                    line.trim()
                } else {
                    line.trim_start()
                };

                if line.is_empty() {
                    // Blank lines are written as literal `\n` text because
                    // line break elements can't produce them: the printer
                    // collapses consecutive line breaks to at most one blank
                    // line, and it prints the current indentation after each
                    // one. A blank line here must stay completely empty, and
                    // consecutive blank lines must keep their exact count,
                    // since every blank line folds into a line feed of the
                    // scalar's value
                    write!(f, [text("\n", None)])?;
                    prev_empty = true;
                } else {
                    if prev_empty {
                        // Terminates the last blank line. The output then
                        // already ends with a line break, and the printer
                        // doesn't stack another one on top, so the
                        // hard_line_break below only writes the content
                        // line's indentation
                        write!(f, [text("\n", None)])?;
                    }
                    write!(f, [hard_line_break(), text(line, None)])?;
                    prev_empty = false;
                }
            }

            Ok(())
        });

        write!(f, [format_replaced(self.token, &content)])
    }
}

/// Normalizes the quotes of a quoted scalar toward `quote_style`:
///
/// ```yaml
/// - 'no special characters'
/// - 'contains ''single'' quotes'
/// - "contains \"double\" quotes"
/// ```
///
/// becomes
///
/// ```yaml
/// - "no special characters"
/// - "contains 'single' quotes"
/// - 'contains "double" quotes'
/// ```
///
/// A quote character in the text picks the opposite quotes regardless of the
/// preferred style, so it doesn't have to be escaped. Escape sequences only
/// exist in double quoted scalars and only `\"` has a single quoted
/// equivalent, so text with any other escape keeps its double quotes; a
/// backslash in a single quoted scalar is content that double quotes would
/// misread as an escape, so such text keeps its single quotes
fn normalize_quotes(value: &str, quote_style: QuoteStyle) -> Cow<'_, str> {
    let bytes = value.as_bytes();
    let quote = match bytes.first() {
        Some(b'"') if bytes.len() >= 2 && bytes.last() == Some(&b'"') => QuoteStyle::Double,
        Some(b'\'') if bytes.len() >= 2 && bytes.last() == Some(&b'\'') => QuoteStyle::Single,
        _ => return Cow::Borrowed(value),
    };
    let raw = &value[1..value.len() - 1];

    let pinned = match quote {
        QuoteStyle::Single => raw.contains('\\'),
        QuoteStyle::Double => {
            let mut bytes = raw.bytes().enumerate();
            let mut pinned = false;
            while let Some((index, byte)) = bytes.next() {
                if byte == b'\\' {
                    if raw.as_bytes().get(index + 1) != Some(&b'"') {
                        pinned = true;
                        break;
                    }
                    bytes.next();
                }
            }
            pinned
        }
    };
    if pinned {
        return Cow::Borrowed(value);
    }

    if raw.contains('"') {
        return match quote {
            QuoteStyle::Single => Cow::Borrowed(value),
            QuoteStyle::Double => Cow::Owned(std::format!(
                "'{}'",
                raw.replace("\\\"", "\"").replace('\'', "''")
            )),
        };
    }
    if raw.contains('\'') {
        return match quote {
            QuoteStyle::Double => Cow::Borrowed(value),
            QuoteStyle::Single => Cow::Owned(std::format!("\"{}\"", raw.replace("''", "'"))),
        };
    }

    if quote == quote_style {
        Cow::Borrowed(value)
    } else {
        let preferred = quote_style.as_char();
        Cow::Owned(std::format!("{preferred}{raw}{preferred}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_double(value: &str) -> String {
        normalize_quotes(value, QuoteStyle::Double).into_owned()
    }

    fn to_single(value: &str) -> String {
        normalize_quotes(value, QuoteStyle::Single).into_owned()
    }

    #[test]
    fn scalar_without_quotes_is_left_alone() {
        assert_eq!(to_double("foo"), "foo");
        assert_eq!(to_single("foo"), "foo");
        // A quote that opens nothing
        assert_eq!(to_double("\""), "\"");
        assert_eq!(to_double("'foo"), "'foo");
    }

    #[test]
    fn quotes_become_the_preferred_style() {
        assert_eq!(to_double("'foo'"), "\"foo\"");
        assert_eq!(to_double("\"foo\""), "\"foo\"");
        assert_eq!(to_single("\"foo\""), "'foo'");
        assert_eq!(to_single("'foo'"), "'foo'");
        assert_eq!(to_double("''"), "\"\"");
    }

    #[test]
    fn a_quote_in_the_text_picks_the_other_style() {
        assert_eq!(to_double("\"a \\\"b\\\" c\""), "'a \"b\" c'");
        assert_eq!(to_double("'a \"b\" c'"), "'a \"b\" c'");
        assert_eq!(to_single("'it''s'"), "\"it's\"");
        assert_eq!(to_single("\"it's\""), "\"it's\"");
    }

    #[test]
    fn both_kinds_of_quote_in_the_text_pick_single_quotes() {
        assert_eq!(to_double("\"it's \\\"quoted\\\"\""), "'it''s \"quoted\"'");
    }

    #[test]
    fn a_backslash_pins_the_quotes() {
        // `\n` has no single quoted equivalent
        assert_eq!(to_single("\"a\\nb\""), "\"a\\nb\"");
        // The backslash is content, which double quotes would read as an
        // escape
        assert_eq!(to_double("'a\\nb'"), "'a\\nb'");
    }
}

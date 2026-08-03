use crate::prelude::*;
use biome_formatter::{FormatRuleWithOptions, format_args, write};
use biome_html_syntax::{HtmlString, HtmlStringFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlString {
    compact: bool,
}
impl FormatNodeRule<HtmlString> for FormatHtmlString {
    fn fmt_fields(&self, node: &HtmlString, f: &mut HtmlFormatter) -> FormatResult<()> {
        let HtmlStringFields { value_token } = node.as_fields();

        if self.compact {
            let value_token = value_token.clone()?;
            return format_removed(&value_token).fmt(f);
        }

        if let Ok(value) = value_token.as_ref() {
            let value_text = value.text_trimmed();

            // Rewriting the token loses the source mapping of everything
            // inside it, so it is only worth doing when the quoting really
            // does change.
            if let Some(quoted) = requote(value_text) {
                return write!(
                    f,
                    [format_replaced(
                        value,
                        &text(&quoted, Some(value.text_trimmed_range().start()))
                    )]
                );
            }

            // An unquoted value still has to gain its quotes, and that much can
            // be done without giving up the mapping.
            if !is_quoted(value_text) {
                return write!(
                    f,
                    [format_replaced(
                        value,
                        &group(&format_args![
                            token("\""),
                            located_token_text(value, value.text_trimmed_range()),
                            token("\""),
                        ])
                    )]
                );
            }
        }

        write!(f, [value_token.format()])
    }
}

/// Whether the attribute value carries its own quotes, as opposed to being
/// written bare such as `<div class=header>`.
fn is_quoted(value_text: &str) -> bool {
    let mut characters = value_text.chars();
    // A lone quote leaves `next_back` empty, so a one character value is
    // correctly not treated as quoted.
    matches!(
        (characters.next(), characters.next_back()),
        (Some('"'), Some('"')) | (Some('\''), Some('\''))
    )
}

/// Rewrites an attribute value with the quote character that costs the fewest
/// escapes, matching Prettier.
///
/// Double quotes are preferred, and give way to single quotes only when the
/// value holds more of them than it holds single quotes. Whichever character
/// ends up as the delimiter is the only one that stays escaped, so a value that
/// was written as `'&apos;&quot;'` comes back as `"'&quot;"`.
fn requote(value_text: &str) -> Option<String> {
    let inner = if is_quoted(value_text) {
        &value_text[1..value_text.len() - 1]
    } else {
        value_text
    };

    let bytes = inner.as_bytes();
    let mut double_quotes = 0;
    let mut single_quotes = 0;
    let mut literal_double_quotes = 0;
    let mut literal_single_quotes = 0;
    let mut escaped_double_quotes = 0;
    let mut escaped_single_quotes = 0;
    let mut index = 0;

    while index < bytes.len() {
        match bytes[index] {
            b'"' => {
                double_quotes += 1;
                literal_double_quotes += 1;
            }
            b'\'' => {
                single_quotes += 1;
                literal_single_quotes += 1;
            }
            b'&' if bytes[index..].starts_with(b"&quot;") => {
                double_quotes += 1;
                escaped_double_quotes += 1;
                index += "&quot;".len() - 1;
            }
            b'&' if bytes[index..].starts_with(b"&apos;") => {
                single_quotes += 1;
                escaped_single_quotes += 1;
                index += "&apos;".len() - 1;
            }
            _ => {}
        }
        index += 1;
    }

    let (
        delimiter,
        escaped_delimiter,
        unescaped_delimiter,
        literal_delimiters,
        escaped_unescaped_delimiters,
    ) = if double_quotes > single_quotes {
        (
            '\'',
            "&apos;",
            "&quot;",
            literal_single_quotes,
            escaped_double_quotes,
        )
    } else {
        (
            '"',
            "&quot;",
            "&apos;",
            literal_double_quotes,
            escaped_single_quotes,
        )
    };

    let delimiter_byte = delimiter as u8;
    if value_text.as_bytes().first() == Some(&delimiter_byte)
        && value_text.as_bytes().last() == Some(&delimiter_byte)
        && literal_delimiters == 0
        && escaped_unescaped_delimiters == 0
    {
        return None;
    }

    let capacity = inner.len() + 2 + (literal_delimiters * 5) - (escaped_unescaped_delimiters * 5);
    let mut result = String::with_capacity(capacity);
    result.push(delimiter);

    let mut copied_until = 0;
    index = 0;
    while index < bytes.len() {
        if bytes[index] == delimiter_byte {
            result.push_str(&inner[copied_until..index]);
            result.push_str(escaped_delimiter);
            index += 1;
            copied_until = index;
        } else if bytes[index..].starts_with(unescaped_delimiter.as_bytes()) {
            result.push_str(&inner[copied_until..index]);
            result.push(if delimiter == '"' { '\'' } else { '"' });
            index += unescaped_delimiter.len();
            copied_until = index;
        } else {
            index += 1;
        }
    }
    result.push_str(&inner[copied_until..]);
    result.push(delimiter);

    Some(result)
}

impl FormatRuleWithOptions<HtmlString> for FormatHtmlString {
    type Options = bool;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.compact = options;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::requote;

    #[test]
    fn returns_none_for_values_that_need_no_changes() {
        assert_eq!(requote("\"abc\""), None);
        assert_eq!(requote("'123 \" 456'"), None);
    }

    #[test]
    fn prefers_double_quotes() {
        assert_eq!(requote("\"abc\""), None);
        assert_eq!(requote("'abc'").as_deref(), Some("\"abc\""));
        assert_eq!(requote("abc").as_deref(), Some("\"abc\""));
    }

    #[test]
    fn gives_way_to_single_quotes_only_when_they_cost_less() {
        // One double quote inside, none single: single quotes cost less.
        assert_eq!(requote("'123 \" 456'"), None);
        // One single quote inside, none double: double quotes still win.
        assert_eq!(requote("\"123 ' 456\""), None);
        // A tie goes to the preferred quote.
        assert_eq!(requote("'&apos;&quot;'").as_deref(), Some("\"'&quot;\""));
    }

    #[test]
    fn unescapes_the_quote_that_is_no_longer_the_delimiter() {
        assert_eq!(requote("\"&apos;\"").as_deref(), Some("\"'\""));
        assert_eq!(requote("'&quot;&quot;'").as_deref(), Some("'\"\"'"));
    }

    #[test]
    fn escapes_the_delimiter_that_was_written_literally() {
        assert_eq!(
            requote("'a \" b ' c'").as_deref(),
            Some("\"a &quot; b ' c\"")
        );
        assert_eq!(
            requote("\"a ' b \" c \" d\"").as_deref(),
            Some("'a &apos; b \" c \" d'")
        );
    }

    #[test]
    fn leaves_other_entities_alone() {
        assert_eq!(requote("\"&amp;&lt;&#39;\""), None);
    }
}

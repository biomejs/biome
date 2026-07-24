use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::YamlSyntaxToken;

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
        let value = self.token.text_trimmed().trim_end();
        let position = Some(self.token.text_trimmed_range().start());

        if !value.contains(['\n', '\r']) {
            return write!(f, [format_replaced(self.token, &text(value, position))]);
        }

        let content = format_with(|f| {
            let mut lines = split_lines(value).peekable();

            // Leading whitespace before the token is trivia rather than part
            // of its text, so the first line only has its end trimmed
            let first = lines.next().unwrap_or("").trim_end();
            write!(f, [text(first, position)])?;

            let mut prev_empty = false;
            while let Some(line) = lines.next() {
                // The trailing whitespace of the last line is kept: in a
                // quoted scalar it sits against the closing quote and is
                // content
                let line = if lines.peek().is_some() {
                    line.trim()
                } else {
                    line.trim_start()
                };

                if line.is_empty() {
                    // Blank lines are written as literal `\n` text because
                    // line break elements can't produce them: the printer
                    // collapses a run of line breaks to at most one blank
                    // line, and it prints the current indentation after each
                    // one. A blank line here must stay completely empty, and
                    // a run must keep its exact length, since every blank
                    // line folds into a line feed of the scalar's value
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

/// Splits `text` at its line breaks: `\r\n`, `\n`, or a lone `\r`.
///
/// [str::lines] can't be used here because it doesn't split at a lone `\r`,
/// which YAML accepts as a line break. Leaving one embedded in a line would
/// print a `\r` the printer's line ending option never normalized.
fn split_lines(text: &str) -> impl Iterator<Item = &str> {
    let mut rest = Some(text);
    std::iter::from_fn(move || {
        let current = rest?;
        match current.find(['\n', '\r']) {
            Some(index) => {
                let len = if current[index..].starts_with("\r\n") {
                    2
                } else {
                    1
                };
                rest = Some(&current[index + len..]);
                Some(&current[..index])
            }
            None => {
                rest = None;
                Some(current)
            }
        }
    })
}

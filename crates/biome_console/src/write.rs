mod html;
mod string;
mod termcolor;

use std::{fmt, io};

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::fmt::MarkupElements;

pub use self::{html::HTML, string::StringBuffer, termcolor::Termcolor};

pub trait Write {
    fn write_str(&mut self, elements: &MarkupElements, content: &str) -> io::Result<()>;
    fn write_fmt(&mut self, elements: &MarkupElements, content: fmt::Arguments) -> io::Result<()>;
}

/// Writes `content` to `writer`, replacing every non-whitespace grapheme that
/// occupies no display width with U+FFFD and passing the rest through byte for
/// byte.
///
/// A terminal acts on such a grapheme instead of showing it: U+001B starts a
/// command, U+202D reorders what follows it. Whitespace is exempt because a
/// newline also occupies no width, and it is text.
///
/// Symbols keep their code point here, unlike [`Termcolor`], which maps four
/// of them to ASCII approximations.
pub fn write_verbatim<W: io::Write>(writer: &mut W, content: &str) -> io::Result<()> {
    // Grapheme segmentation is considerably more expensive than validating ASCII bytes.
    if content.is_ascii()
        && content
            .bytes()
            .all(|byte| !byte.is_ascii_control() || byte.is_ascii_whitespace())
    {
        return writer.write_all(content.as_bytes());
    }

    let mut buffer = [0; 4];
    let mut segment_start = 0;

    for (offset, grapheme) in content.grapheme_indices(true) {
        let is_whitespace = grapheme.chars().all(char::is_whitespace);

        if !is_whitespace && UnicodeWidthStr::width(grapheme) == 0 {
            writer.write_all(&content.as_bytes()[segment_start..offset])?;
            writer.write_all(
                char::REPLACEMENT_CHARACTER
                    .encode_utf8(&mut buffer)
                    .as_bytes(),
            )?;
            segment_start = offset + grapheme.len();
        }
    }

    writer.write_all(&content.as_bytes()[segment_start..])
}

#[cfg(test)]
mod tests {
    use std::str::from_utf8;

    use termcolor::NoColor;

    use super::{Termcolor, write_verbatim};
    use crate as biome_console;
    use crate::fmt::Formatter;
    use biome_markup::markup;

    fn verbatim(content: &str) -> String {
        let mut buffer = Vec::new();
        write_verbatim(&mut buffer, content).unwrap();
        String::from_utf8(buffer).unwrap()
    }

    fn as_markup(content: &str) -> String {
        let mut buffer = Vec::new();
        {
            // `NoColor` reports no colour support, which is what `EnvConsole`
            // ends up with when stdout is not a terminal, and is the branch
            // where `Termcolor` substitutes ASCII.
            let mut writer = Termcolor(NoColor::new(&mut buffer));
            Formatter::new(&mut writer)
                .write_markup(markup! {{content}})
                .unwrap();
        }
        from_utf8(&buffer).unwrap().to_string()
    }

    #[test]
    fn keeps_diagnostic_symbols_that_appear_in_source() {
        // `unicode_to_ascii` maps U+26A0 to `!` and U+2714 to U+221A, so a
        // source file holding them is where the two writers disagree.
        const SOURCE: &str = "const a = `\u{26a0}`;\nconst b = `\u{2714}`;\n";

        assert_eq!(verbatim(SOURCE), SOURCE);
        assert_eq!(as_markup(SOURCE), "const a = `!`;\nconst b = `\u{221a}`;\n");
    }

    #[test]
    fn still_replaces_characters_a_terminal_would_execute() {
        assert_eq!(verbatim("a\u{1b}[31mb"), "a\u{FFFD}[31mb");
        assert_eq!(verbatim("t\u{202D}es\u{200B}t"), "t\u{FFFD}es\u{FFFD}t");
    }

    #[test]
    fn keeps_whitespace_and_multi_codepoint_graphemes() {
        assert_eq!(verbatim("a\tb\r\nc"), "a\tb\r\nc");
        assert_eq!(
            verbatim("\u{26a0}\u{fe0f}1\u{fe0f}\u{20e3}"),
            "\u{26a0}\u{fe0f}1\u{fe0f}\u{20e3}"
        );
    }
}

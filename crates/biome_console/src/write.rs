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

/// Writes `content` to `writer` as it is, replacing only the characters a
/// terminal reads as commands instead of as text.
///
/// The markup path also rewrites symbols such as `✔` to ASCII to keep
/// diagnostics readable where the terminal may not render them. That is a
/// choice about console UI, and it corrupts content that the caller is
/// redirecting into a file, so it is not applied here.
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
            // `NoColor` is what the console resolves to when stdout is not a
            // terminal, which is when the ASCII fallback applies.
            let mut writer = Termcolor(NoColor::new(&mut buffer));
            Formatter::new(&mut writer)
                .write_markup(markup! {{content}})
                .unwrap();
        }
        from_utf8(&buffer).unwrap().to_string()
    }

    #[test]
    fn keeps_diagnostic_symbols_that_appear_in_source() {
        // `⚠` and `✔` are both in the ASCII fallback table, and source code
        // echoed to stdout goes through the console just like diagnostics do.
        const SOURCE: &str = "const a = `\u{26a0}`;\nconst b = `\u{2714}`;\n";

        assert_eq!(verbatim(SOURCE), SOURCE);
        // Control: the markup path still rewrites them, so the fallback is
        // scoped rather than removed.
        assert_eq!(as_markup(SOURCE), "const a = `!`;\nconst b = `\u{221a}`;\n");
    }

    #[test]
    fn still_replaces_characters_a_terminal_would_execute() {
        // Writing content as it is must not become a way for a source file to
        // drive the terminal.
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

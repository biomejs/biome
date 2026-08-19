use std::{
    fmt::{self, Write as _},
    io,
};

use termcolor::{Color, ColorSpec, WriteColor};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::{MarkupElement, fmt::MarkupElements};

use super::Write;

/// Adapter struct implementing [Write] over types implementing [WriteColor]
pub struct Termcolor<W>(pub W);

impl<W> Write for Termcolor<W>
where
    W: WriteColor,
{
    fn write_str(&mut self, elements: &MarkupElements, content: &str) -> io::Result<()> {
        with_format(&mut self.0, elements, |writer| {
            let mut adapter = SanitizeAdapter {
                writer,
                error: Ok(()),
            };

            match adapter.write_str(content) {
                Ok(()) => Ok(()),
                Err(..) => {
                    if adapter.error.is_err() {
                        adapter.error
                    } else {
                        // SanitizeAdapter can only fail if the underlying
                        // writer returns an error
                        unreachable!()
                    }
                }
            }
        })
    }

    fn write_fmt(&mut self, elements: &MarkupElements, content: fmt::Arguments) -> io::Result<()> {
        with_format(&mut self.0, elements, |writer| {
            let mut adapter = SanitizeAdapter {
                writer,
                error: Ok(()),
            };

            match adapter.write_fmt(content) {
                Ok(()) => Ok(()),
                Err(..) => {
                    if adapter.error.is_err() {
                        adapter.error
                    } else {
                        Err(io::Error::other("a Display formatter returned an error"))
                    }
                }
            }
        })
    }
}

/// Applies the current format in `state` to `writer`, calls `func` to
/// print a piece of text, then reset the printing format
fn with_format<W>(
    writer: &mut W,
    state: &MarkupElements,
    func: impl FnOnce(&mut W) -> io::Result<()>,
) -> io::Result<()>
where
    W: WriteColor,
{
    let mut color = ColorSpec::new();
    let mut link = None;
    let mut inverse = false;

    state.for_each(&mut |elements| {
        for element in elements {
            match element {
                MarkupElement::Inverse => {
                    inverse = !inverse;
                }
                MarkupElement::Hyperlink { href } => {
                    link = Some(href);
                }
                _ => {
                    element.update_color(&mut color);
                }
            }
        }

        Ok(())
    })?;

    if inverse {
        let fg = color.fg().map_or(Color::White, |c| *c);
        let bg = color.bg().map_or(Color::Black, |c| *c);
        color.set_bg(Some(fg));
        color.set_fg(Some(bg));
    }

    if let Err(err) = writer.set_color(&color) {
        writer.reset()?;
        return Err(err);
    }

    let mut reset_link = false;
    if let Some(href) = link {
        // `is_synchronous` is used to check if the underlying writer
        // is using the Windows Console API, that does not support ANSI
        // escape codes. Generally this would only be true when running
        // in the legacy `cmd.exe` terminal emulator, since in modern
        // clients like the Windows Terminal ANSI is used instead
        if writer.supports_color() && !writer.is_synchronous() {
            write!(writer, "\x1b]8;;{href}\x1b\\")?;
            reset_link = true;
        }
    }

    let result = func(writer);

    if reset_link {
        write!(writer, "\x1b]8;;\x1b\\")?;
    }

    writer.reset()?;
    result
}

/// Adapter [fmt::Write] calls to [io::Write] with sanitization,
/// implemented as an internal struct to avoid exposing [fmt::Write] on
/// [Termcolor]
struct SanitizeAdapter<W> {
    writer: W,
    error: io::Result<()>,
}

impl<W> SanitizeAdapter<W>
where
    W: WriteColor,
{
    fn write_verbatim(&mut self, bytes: &[u8]) -> fmt::Result {
        if bytes.is_empty() {
            return Ok(());
        }

        if let Err(err) = self.writer.write_all(bytes) {
            self.error = Err(err);
            return Err(fmt::Error);
        }

        Ok(())
    }

    fn write_character(&mut self, character: char, buffer: &mut [u8; 4]) -> fmt::Result {
        character.encode_utf8(buffer);
        self.write_verbatim(&buffer[..character.len_utf8()])
    }
}

impl<W> fmt::Write for SanitizeAdapter<W>
where
    W: WriteColor,
{
    fn write_str(&mut self, content: &str) -> fmt::Result {
        // Grapheme segmentation is considerably more expensive than validating ASCII bytes.
        if content.is_ascii()
            && content
                .bytes()
                .all(|byte| !byte.is_ascii_control() || byte.is_ascii_whitespace())
        {
            return self.write_verbatim(content.as_bytes());
        }

        let mut buffer = [0; 4];
        let supports_color = self.writer.supports_color();
        let mut segment_start = 0;

        for (offset, grapheme) in content.grapheme_indices(true) {
            let width = UnicodeWidthStr::width(grapheme);
            let is_whitespace = grapheme_is_whitespace(grapheme);

            if !is_whitespace && width == 0 {
                self.write_verbatim(&content.as_bytes()[segment_start..offset])?;
                self.write_character(char::REPLACEMENT_CHARACTER, &mut buffer)?;
                segment_start = offset + grapheme.len();
                continue;
            }

            // Unicode is currently poorly supported on most Windows
            // terminal clients, so we always strip emojis in Windows.
            // When colors are disabled on non-Windows systems, we need to balance two concerns:
            // 1. Convert diagnostic UI symbols (✔ ℹ ⚠ ✖) to ASCII for better readability
            // 2. Preserve source code fidelity for multi-codepoint graphemes
            let is_ascii = grapheme.is_ascii();

            if !is_ascii {
                if cfg!(windows) {
                    let mut characters = grapheme.chars();
                    let character = characters.next().unwrap();
                    let replacement = unicode_to_ascii(character);

                    if replacement != character || characters.next().is_some() {
                        self.write_verbatim(&content.as_bytes()[segment_start..offset])?;
                        self.write_character(replacement, &mut buffer)?;
                        segment_start = offset + grapheme.len();
                    }
                } else if !supports_color {
                    // On non-Windows with colors disabled:
                    // Only convert single-codepoint graphemes (diagnostic symbols)
                    // Multi-codepoint graphemes (like emoji with modifiers) are preserved for source code fidelity
                    let mut characters = grapheme.chars();
                    let character = characters.next().unwrap();
                    if characters.next().is_none() {
                        let replacement = unicode_to_ascii(character);

                        if replacement != character {
                            self.write_verbatim(&content.as_bytes()[segment_start..offset])?;
                            self.write_character(replacement, &mut buffer)?;
                            segment_start = offset + grapheme.len();
                        }
                    }
                }
            }
        }

        self.write_verbatim(&content.as_bytes()[segment_start..])
    }
}

/// Determines if a unicode grapheme consists only of code points
/// which are considered whitespace characters in ASCII
fn grapheme_is_whitespace(grapheme: &str) -> bool {
    grapheme.chars().all(|c| c.is_whitespace())
}

/// Replace emoji characters with similar but more widely supported ASCII
/// characters
fn unicode_to_ascii(c: char) -> char {
    match c {
        '\u{2714}' => '\u{221a}',
        '\u{2139}' => 'i',
        '\u{26a0}' => '!',
        '\u{2716}' => '\u{00d7}',
        _ => c,
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fmt::Write,
        io::{self, Write as IoWrite},
        str::from_utf8,
    };

    use biome_markup::markup;
    use termcolor::Ansi;

    use crate as biome_console;
    use crate::fmt::Formatter;

    use super::{SanitizeAdapter, Termcolor};

    #[derive(Default)]
    struct TestWriter {
        buffer: Vec<u8>,
        write_count: usize,
    }

    impl IoWrite for TestWriter {
        fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
            self.write_count += 1;
            self.buffer.write(buffer)
        }

        fn flush(&mut self) -> io::Result<()> {
            self.buffer.flush()
        }
    }

    #[test]
    fn test_sanitize() {
        // Sanitization should leave whitespace control characters (space,
        // tabs, newline, ...) and non-ASCII unicode characters as-is but
        // redact zero-width characters (RTL override, null character, bell,
        // zero-width space, ...)
        const INPUT: &str = "t\tes t\r\n\u{202D}t\0es\x07t\u{202E}\nt\u{200B}es🐛t";
        const OUTPUT: &str = "t\tes t\r\n\u{FFFD}t\u{FFFD}es\u{FFFD}t\u{FFFD}\nt\u{FFFD}es🐛t";

        let mut buffer = Vec::new();

        {
            let writer = termcolor::Ansi::new(&mut buffer);
            let mut adapter = SanitizeAdapter {
                writer,
                error: Ok(()),
            };

            adapter.write_str(INPUT).unwrap();
            adapter.error.unwrap();
        }

        assert_eq!(from_utf8(&buffer).unwrap(), OUTPUT);
    }

    #[test]
    fn test_hyperlink() {
        const OUTPUT: &str = "\x1b[0m\x1b]8;;https://biomejs.dev/\x1b\\link\x1b]8;;\x1b\\\x1b[0m";

        let mut buffer = Vec::new();
        let mut writer = Termcolor(Ansi::new(&mut buffer));
        let mut formatter = Formatter::new(&mut writer);

        formatter
            .write_markup(markup! {
                <Hyperlink href="https://biomejs.dev/">"link"</Hyperlink>
            })
            .unwrap();

        assert_eq!(from_utf8(&buffer).unwrap(), OUTPUT);
    }

    #[test]
    fn test_printing_complex_emojis() {
        const INPUT: &str = "⚠️1️⃣ℹ️";
        const OUTPUT: &str = "⚠️1️⃣ℹ️";
        const WINDOWS_OUTPUT: &str = "!1i";

        let mut buffer = Vec::new();

        {
            let writer = termcolor::Ansi::new(&mut buffer);
            let mut adapter = SanitizeAdapter {
                writer,
                error: Ok(()),
            };

            adapter.write_str(INPUT).unwrap();
            adapter.error.unwrap();
        }

        if cfg!(windows) {
            assert_eq!(from_utf8(&buffer).unwrap(), WINDOWS_OUTPUT);
        } else {
            assert_eq!(from_utf8(&buffer).unwrap(), OUTPUT);
        }
    }

    #[test]
    fn test_preserve_multi_codepoint_graphemes_without_colors() {
        // Test that multi-codepoint graphemes are preserved when colors are disabled
        // while single-codepoint diagnostic symbols are still converted for readability.
        // This is critical for source code fidelity when using --colors off.
        const INPUT: &str = "⚠️ â ｶﾞ 👨🏻‍🦱 ⚠";

        let mut buffer = Vec::new();

        {
            let writer = termcolor::NoColor::new(&mut buffer);
            let mut adapter = SanitizeAdapter {
                writer,
                error: Ok(()),
            };

            adapter.write_str(INPUT).unwrap();
            adapter.error.unwrap();
        }

        let actual = from_utf8(&buffer).unwrap();

        if cfg!(windows) {
            // On Windows, all non-ASCII are converted due to poor Unicode support
            assert_eq!(
                actual, "! â ｶ 👨 !",
                "On Windows, all emojis should be converted.\nExpected: {:?}\nActual: {:?}",
                "! â ｶ 👨 !", actual
            );
        } else {
            // On non-Windows:
            // - Multi-codepoint graphemes like ⚠️ (U+26A0 + U+FE0F) are preserved
            // - Single-codepoint symbols like ⚠ (U+26A0 only) are converted to !
            const EXPECTED: &str = "⚠️ â ｶﾞ 👨🏻‍🦱 !";
            assert_eq!(
                actual, EXPECTED,
                "Multi-codepoint graphemes should be preserved, single symbols converted.\nExpected: {:?}\nActual: {:?}",
                EXPECTED, actual
            );
        }
    }

    #[test]
    fn test_safe_ascii_is_written_once() {
        const INPUT: &str = "  1 | const answer = 42;\n\t= help: plain ASCII diagnostic\r\n";

        let mut output = TestWriter::default();
        {
            let writer = termcolor::Ansi::new(&mut output);
            let mut adapter = SanitizeAdapter {
                writer,
                error: Ok(()),
            };
            adapter.write_str(INPUT).unwrap();
            adapter.error.unwrap();
        }

        assert_eq!(from_utf8(&output.buffer).unwrap(), INPUT);
        assert_eq!(output.write_count, 1);
    }

    #[test]
    fn test_sanitized_characters_split_verbatim_segments() {
        const INPUT: &str = "first\0second\u{200B}third";
        const OUTPUT: &str = "first\u{FFFD}second\u{FFFD}third";

        let mut output = TestWriter::default();
        {
            let writer = termcolor::Ansi::new(&mut output);
            let mut adapter = SanitizeAdapter {
                writer,
                error: Ok(()),
            };
            adapter.write_str(INPUT).unwrap();
            adapter.error.unwrap();
        }

        assert_eq!(from_utf8(&output.buffer).unwrap(), OUTPUT);
        assert_eq!(output.write_count, 5);
    }

    #[test]
    fn test_unchanged_unicode_is_batched_without_colors() {
        const INPUT: &str = "  1 │ source code\n    ━━━━━━━━━━━";

        let mut output = TestWriter::default();
        {
            let writer = termcolor::NoColor::new(&mut output);
            let mut adapter = SanitizeAdapter {
                writer,
                error: Ok(()),
            };
            adapter.write_str(INPUT).unwrap();
            adapter.error.unwrap();
        }

        assert_eq!(from_utf8(&output.buffer).unwrap(), INPUT);
        assert_eq!(output.write_count, 1);
    }
}

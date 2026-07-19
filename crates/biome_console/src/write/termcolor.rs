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
    /// Writes `bytes` verbatim.
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
}

impl<W> fmt::Write for SanitizeAdapter<W>
where
    W: WriteColor,
{
    fn write_str(&mut self, content: &str) -> fmt::Result {
        let mut buffer = [0; 4];
        // Start of the current stretch of bytes that can be written verbatim
        // (i.e. need no replacement). Flushed in one `write_all` call
        // whenever a grapheme needing replacement is found, instead of
        // writing one character at a time: most content (source code,
        // punctuation, plain ASCII text) needs no sanitization at all, so
        // this keeps the common case to a single write per `write_str`
        // call rather than one per character.
        let mut segment_start = 0;

        for (offset, grapheme) in content.grapheme_indices(true) {
            let width = UnicodeWidthStr::width(grapheme);
            let is_whitespace = grapheme_is_whitespace(grapheme);

            if !is_whitespace && width == 0 {
                self.write_verbatim(&content.as_bytes()[segment_start..offset])?;

                let char_to_write = char::REPLACEMENT_CHARACTER;
                char_to_write.encode_utf8(&mut buffer);
                self.write_verbatim(&buffer[..char_to_write.len_utf8()])?;

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
                    // On Windows, always convert all non-ASCII graphemes due to poor terminal support
                    self.write_verbatim(&content.as_bytes()[segment_start..offset])?;

                    let replacement = unicode_to_ascii(grapheme.chars().nth(0).unwrap());
                    replacement.encode_utf8(&mut buffer);
                    self.write_verbatim(&buffer[..replacement.len_utf8()])?;

                    segment_start = offset + grapheme.len();
                } else if !self.writer.supports_color() {
                    // On non-Windows with colors disabled:
                    // Only convert single-codepoint graphemes (diagnostic symbols)
                    // Multi-codepoint graphemes (like emoji with modifiers) are preserved for source code fidelity
                    let mut chars = grapheme.chars();
                    let first = chars.next();
                    if let (Some(character), None) = (first, chars.next()) {
                        self.write_verbatim(&content.as_bytes()[segment_start..offset])?;

                        let replacement = unicode_to_ascii(character);
                        replacement.encode_utf8(&mut buffer);
                        self.write_verbatim(&buffer[..replacement.len_utf8()])?;

                        segment_start = offset + grapheme.len();
                    }
                    // Multi-codepoint graphemes fall through to be written as-is below
                }
            }

            // ASCII grapheme, or a non-ASCII one that isn't being replaced:
            // leave it as part of the current verbatim stretch.
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

    /// Counts how many times [io::Write::write] is called.
    #[derive(Default)]
    struct TestWriter {
        buffer: Vec<u8>,
        write_count: usize,
    }

    impl IoWrite for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.write_count += 1;
            self.buffer.write(buf)
        }

        fn flush(&mut self) -> io::Result<()> {
            self.buffer.flush()
        }
    }

    #[test]
    fn test_clean_content_is_written_in_a_single_call() {
        const INPUT: &str = "the quick brown fox jumps over the lazy dog, 42 times.";

        let mut counting = TestWriter::default();

        {
            let writer = termcolor::Ansi::new(&mut counting);
            let mut adapter = SanitizeAdapter {
                writer,
                error: Ok(()),
            };

            adapter.write_str(INPUT).unwrap();
            adapter.error.unwrap();
        }

        assert_eq!(from_utf8(&counting.buffer).unwrap(), INPUT);
        assert_eq!(
            counting.write_count, 1,
            "clean content should be flushed in a single write, not one per character"
        );
    }

    #[test]
    fn test_mixed_content_batches_clean_runs() {
        // Two zero-width characters (replaced) surrounded by three runs of
        // plain ASCII text (batched): 3 clean-run writes + 2 replacement
        // writes = 5, regardless of how long the surrounding clean runs are.
        const INPUT: &str = "some reasonably long clean run of text\0another fairly long clean run of text\0and a final clean run";

        let mut counting = TestWriter::default();

        {
            let writer = termcolor::Ansi::new(&mut counting);
            let mut adapter = SanitizeAdapter {
                writer,
                error: Ok(()),
            };

            adapter.write_str(INPUT).unwrap();
            adapter.error.unwrap();
        }

        assert_eq!(
            counting.write_count, 5,
            "expected one write per clean run plus one per replaced character"
        );
    }
}

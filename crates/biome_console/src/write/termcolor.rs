use std::{
    fmt::{self, Write as _},
    io,
};

use termcolor::{Color, ColorSpec, WriteColor};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::{fmt::MarkupElements, MarkupElement};

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
                        Err(io::Error::new(
                            io::ErrorKind::Other,
                            "a Display formatter returned an error",
                        ))
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

impl<W> fmt::Write for SanitizeAdapter<W>
where
    W: WriteColor,
{
    fn write_str(&mut self, content: &str) -> fmt::Result {
        let mut buffer = [0; 4];

        for grapheme in content.graphemes(true) {
            let width = UnicodeWidthStr::width(grapheme);
            let is_whitespace = grapheme_is_whitespace(grapheme);

            if !is_whitespace && width == 0 {
                let char_to_write = char::REPLACEMENT_CHARACTER;
                char_to_write.encode_utf8(&mut buffer);

                if let Err(err) = self.writer.write_all(&buffer[..char_to_write.len_utf8()]) {
                    self.error = Err(err);
                    return Err(fmt::Error);
                }

                continue;
            }

            // Unicode is currently poorly supported on most Windows
            // terminal clients, so we always strip emojis in Windows
            if cfg!(windows) || !self.writer.supports_color() {
                let is_ascii = grapheme.is_ascii();

                if !is_ascii {
                    let replacement = unicode_to_ascii(grapheme.chars().nth(0).unwrap());

                    replacement.encode_utf8(&mut buffer);

                    if let Err(err) = self.writer.write_all(&buffer[..replacement.len_utf8()]) {
                        self.error = Err(err);
                        return Err(fmt::Error);
                    }

                    continue;
                }
            };

            for char in grapheme.chars() {
                char.encode_utf8(&mut buffer);

                if let Err(err) = self.writer.write_all(&buffer[..char.len_utf8()]) {
                    self.error = Err(err);
                    return Err(fmt::Error);
                }
            }
        }

        Ok(())
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
    use std::{fmt::Write, str::from_utf8};

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
}

use std::{
    fmt,
    io::{self, Write as _},
};

use crate::{fmt::MarkupElements, MarkupElement};

use super::Write;

/// Adapter struct implementing [Write] over types implementing [io::Write],
/// renders markup as UTF-8 strings of HTML code
pub struct HTML<W>(pub W);

impl<W> Write for HTML<W>
where
    W: io::Write,
{
    fn write_str(&mut self, elements: &MarkupElements, content: &str) -> io::Result<()> {
        push_styles(&mut self.0, elements)?;
        EscapeAdapter(&mut self.0).write_all(content.as_bytes())?;
        pop_styles(&mut self.0, elements)
    }

    fn write_fmt(&mut self, elements: &MarkupElements, content: fmt::Arguments) -> io::Result<()> {
        push_styles(&mut self.0, elements)?;
        EscapeAdapter(&mut self.0).write_fmt(content)?;
        pop_styles(&mut self.0, elements)
    }
}

fn push_styles<W: io::Write>(fmt: &mut W, elements: &MarkupElements) -> io::Result<()> {
    elements.for_each(&mut |styles| {
        for style in styles {
            match style {
                MarkupElement::Emphasis => write!(fmt, "<strong>")?,
                MarkupElement::Dim => write!(fmt, "<span style=\"opacity: 0.8;\">")?,
                MarkupElement::Italic => write!(fmt, "<i>")?,
                MarkupElement::Underline => write!(fmt, "<u>")?,
                MarkupElement::Error => write!(fmt, "<span style=\"color: Tomato;\">")?,
                MarkupElement::Success => write!(fmt, "<span style=\"color: MediumSeaGreen;\">")?,
                MarkupElement::Warn => write!(fmt, "<span style=\"color: Orange;\">")?,
                MarkupElement::Info => write!(fmt, "<span style=\"color: rgb(38, 148, 255);\">")?,
                MarkupElement::Inverse => {
                    write!(fmt, "<span style=\"color: #000; background-color: #ddd;\">")?
                }
                MarkupElement::Hyperlink { href } => write!(fmt, "<a href=\"{href}\">")?,
            }
        }

        Ok(())
    })
}

fn pop_styles<W: io::Write>(fmt: &mut W, elements: &MarkupElements) -> io::Result<()> {
    elements.for_each_rev(&mut |styles| {
        for style in styles.iter().rev() {
            match style {
                MarkupElement::Emphasis => write!(fmt, "</strong>")?,
                MarkupElement::Italic => write!(fmt, "</i>")?,
                MarkupElement::Underline => write!(fmt, "</u>")?,
                MarkupElement::Dim
                | MarkupElement::Error
                | MarkupElement::Success
                | MarkupElement::Warn
                | MarkupElement::Info
                | MarkupElement::Inverse => write!(fmt, "</span>")?,
                MarkupElement::Hyperlink { .. } => write!(fmt, "</a>")?,
            }
        }

        Ok(())
    })
}

/// Adapter wrapping a type implementing [io::Write] and adding HTML special
/// characters escaping to the written byte sequence
struct EscapeAdapter<W>(W);

impl<W: io::Write> io::Write for EscapeAdapter<W> {
    fn write(&mut self, mut buf: &[u8]) -> io::Result<usize> {
        let mut bytes = 0;

        const HTML_ESCAPES: [u8; 4] = [b'"', b'&', b'<', b'>'];
        while let Some(idx) = buf.iter().position(|b| HTML_ESCAPES.contains(b)) {
            let (before, after) = buf.split_at(idx);

            self.0.write_all(before)?;
            bytes += before.len();

            // SAFETY: Because of the above `position` match we know the buffer
            // contains at least the matching byte
            let (byte, after) = after.split_first().unwrap();
            match *byte {
                b'"' => self.0.write_all(b"&quot;")?,
                b'&' => self.0.write_all(b"&amp;")?,
                b'<' => self.0.write_all(b"&lt;")?,
                b'>' => self.0.write_all(b"&gt;")?,
                _ => unreachable!(),
            }

            // Only 1 byte of the input was written
            bytes += 1;
            buf = after;
        }

        self.0.write_all(buf)?;
        bytes += buf.len();
        Ok(bytes)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

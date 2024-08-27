use std::{
    fmt,
    io::{self, Write as _},
};

use crate::{fmt::MarkupElements, MarkupElement};

use super::Write;

/// Adapter struct implementing [Write] over types implementing [io::Write],
/// renders markup as UTF-8 strings of HTML code
pub struct HTML<W>(pub W, bool);

impl<W> HTML<W> {
    pub fn new(writer: W) -> Self {
        Self(writer, false)
    }

    pub fn with_mdx(mut self) -> Self {
        self.1 = true;
        self
    }
}

impl<W> Write for HTML<W>
where
    W: io::Write,
{
    fn write_str(&mut self, elements: &MarkupElements, content: &str) -> io::Result<()> {
        push_styles(&mut self.0, elements)?;
        HtmlAdapter(&mut self.0, self.1).write_all(content.as_bytes())?;
        pop_styles(&mut self.0, elements)
    }

    fn write_fmt(&mut self, elements: &MarkupElements, content: fmt::Arguments) -> io::Result<()> {
        push_styles(&mut self.0, elements)?;
        HtmlAdapter(&mut self.0, self.1).write_fmt(content)?;
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
                MarkupElement::Debug => write!(fmt, "<span style=\"color: rgb(38, 148, 255);\">")?,
                MarkupElement::Info => write!(fmt, "<span style=\"color: lightgreen;\">")?,
                MarkupElement::Trace => write!(fmt, "<span style=\"color: fuchsia;\">")?,
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
                | MarkupElement::Debug
                | MarkupElement::Trace
                | MarkupElement::Info
                | MarkupElement::Inverse => write!(fmt, "</span>")?,
                MarkupElement::Hyperlink { .. } => write!(fmt, "</a>")?,
            }
        }

        Ok(())
    })
}

/// Adapter wrapping a type implementing [io::Write]. It's responsible for:
/// - and adding HTML special characters escaping to the written byte sequence
/// - and adding HTML line breaks for newline characters
struct HtmlAdapter<W>(W, bool);

impl<W: io::Write> HtmlAdapter<W> {
    fn write_escapes(&mut self, current_byte: &u8) -> io::Result<bool> {
        match *current_byte {
            b'"' => self.0.write_all(b"&quot;")?,
            b'&' => self.0.write_all(b"&amp;")?,
            b'<' => self.0.write_all(b"&lt;")?,
            b'>' => self.0.write_all(b"&gt;")?,
            _ => return Ok(false),
        };

        Ok(true)
    }

    fn write_mdx_escapes(&mut self, current_byte: &u8) -> io::Result<bool> {
        if !self.1 {
            return Ok(false);
        } else {
            match current_byte {
                b'\n' => self.0.write_all(b"<br />")?,
                b'\r' => self.0.write_all(b"<br />")?,
                b'{' => self.0.write_all(b"&#123;")?,
                b'}' => self.0.write_all(b"&#125;")?,
                b'*' => self.0.write_all(b"&#42;")?,
                b'_' => self.0.write_all(b"&#95;")?,
                b'\\' => self.0.write_all(b"&#92;")?,
                _ => return Ok(false),
            }
        }

        Ok(true)
    }
}

impl<W: io::Write> io::Write for HtmlAdapter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        for byte in buf {
            let escaped = self.write_escapes(byte)?;
            let mdx_escaped = self.write_mdx_escapes(byte)?;
            if !escaped && !mdx_escaped {
                self.0.write_all(&[*byte])?;
            }
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

#[cfg(test)]
mod test {
    use crate as biome_console;
    use crate::fmt::Formatter;
    use biome_markup::markup;

    #[test]
    fn test_mdx_new_lines() {
        let mut buf = Vec::new();
        let mut writer = super::HTML(&mut buf, true);
        let mut formatter = Formatter::new(&mut writer);

        formatter
            .write_markup(markup! {
                "Hello"
            })
            .unwrap();

        formatter
            .write_markup(markup! {
                "\n"
            })
            .unwrap();

        formatter
            .write_markup(markup! {
                "World"
            })
            .unwrap();

        assert_eq!(String::from_utf8(buf).unwrap(), "Hello<br />World");
    }

    #[test]
    fn test_escapes() {
        let mut buf = Vec::new();
        let mut writer = super::HTML(&mut buf, false);
        let mut formatter = Formatter::new(&mut writer);

        formatter
            .write_markup(markup! {
                "\""
            })
            .unwrap();
        formatter
            .write_markup(markup! {
                "\""
            })
            .unwrap();

        assert_eq!(String::from_utf8(buf).unwrap(), "&quot;&quot;");
    }

    #[test]
    fn test_escapes_and_new_lines() {
        let mut buf = Vec::new();
        let mut writer = super::HTML(&mut buf, true);
        let mut formatter = Formatter::new(&mut writer);

        formatter
            .write_markup(markup! {
                "New rules that are still under development.\n\n."
            })
            .unwrap();

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            "New rules that are still under development.<br /><br />."
        );
    }

    #[test]
    fn does_not_escape_curly_braces() {
        let mut buf = Vec::new();
        let mut writer = super::HTML(&mut buf, false);
        let mut formatter = Formatter::new(&mut writer);

        formatter
            .write_markup(markup! {
                "New rules that are {still} under development."
            })
            .unwrap();

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            "New rules that are {still} under development."
        );
    }

    #[test]
    fn escape_curly_braces() {
        let mut buf = Vec::new();
        let mut writer = super::HTML(&mut buf, false).with_mdx();
        let mut formatter = Formatter::new(&mut writer);

        formatter
            .write_markup(markup! {
                "New rules that are {still} under development.\n\n."
            })
            .unwrap();

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            "New rules that are &#123;still&#125; under development.<br /><br />."
        );
    }
    #[test]
    fn test_from_website() {
        let mut buf = Vec::new();
        let mut writer = super::HTML(&mut buf, false).with_mdx();
        let mut formatter = Formatter::new(&mut writer);

        formatter
            .write_markup(markup! {
                "Rules focused on preventing accessibility problems."
            })
            .unwrap();

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            "Rules focused on preventing accessibility problems."
        );
    }
}

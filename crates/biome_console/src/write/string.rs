use crate::fmt::{MarkupElements, Write};
use std::fmt::Arguments;
use std::io;

/// Adapter struct implementing [Write] over types implementing [io::Write],
/// renders markup as it is inside a string.
pub struct StringBuffer<W>(pub W);

impl<W> StringBuffer<W> {
    pub fn new(writer: W) -> Self {
        Self(writer)
    }
}

impl<W> Write for StringBuffer<W>
where
    W: io::Write,
{
    fn write_str(&mut self, _elements: &MarkupElements, content: &str) -> io::Result<()> {
        self.0.write_all(content.as_bytes())
    }

    fn write_fmt(&mut self, _elements: &MarkupElements, content: Arguments) -> io::Result<()> {
        self.0.write_fmt(content)
    }
}

#[cfg(test)]
mod test {
    use crate as biome_console;
    use crate::fmt::Formatter;
    use biome_markup::markup;
    use std::str::from_utf8;

    #[test]
    fn test_new_lines() {
        let mut buf = Vec::new();
        let mut writer = super::StringBuffer(&mut buf);
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

        assert_eq!(String::from_utf8(buf).unwrap(), "Hello\nWorld");
    }

    #[test]
    fn test_printing_complex_emojis() {
        const INPUT: &str = "⚠️1️⃣ℹ️";
        const OUTPUT: &str = "⚠️1️⃣ℹ️";

        let mut buffer = Vec::new();
        let mut writer = super::StringBuffer(&mut buffer);
        let mut formatter = Formatter::new(&mut writer);

        formatter
            .write_markup(markup! {
                {INPUT}
            })
            .unwrap();

        assert_eq!(from_utf8(&buffer).unwrap(), OUTPUT);
    }

    #[test]
    fn tags_are_stripped() {
        let mut buffer = Vec::new();
        let mut writer = super::StringBuffer(&mut buffer);
        let mut formatter = Formatter::new(&mut writer);

        formatter
            .write_markup(markup! {
                <Info>" info "</Info>
            })
            .unwrap();
        formatter
            .write_markup(markup! {
                <Error>" error "</Error>
            })
            .unwrap();

        assert_eq!(from_utf8(&buffer).unwrap(), " info  error ");
    }
}

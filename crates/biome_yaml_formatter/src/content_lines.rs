/// Iterator over the lines of a scalar's text, splitting at `\r\n`, `\n`,
/// and lone `\r` line breaks alike.
///
/// [str::lines] can't be used here because it doesn't split at a lone `\r`,
/// which YAML accepts as a line break. Leaving one embedded in a line would
/// print a `\r` the printer's line ending option never normalized.
///
/// Text that ends with a line break yields a final empty line, which is how
/// it is distinguished from text that ends mid-line
#[derive(Debug, Clone)]
pub(crate) struct ContentLines<'a> {
    rest: Option<&'a str>,
}

impl<'a> ContentLines<'a> {
    pub(crate) fn new(text: &'a str) -> Self {
        Self { rest: Some(text) }
    }

    /// Whether the remaining text ends with a line break, i.e. whether the
    /// last line the iterator yields is an empty one split off by that break
    pub(crate) fn ends_with_break(&self) -> bool {
        self.rest.is_some_and(|text| text.ends_with(['\n', '\r']))
    }

    /// Whether the iterator will yield another line
    pub(crate) fn has_remaining(&self) -> bool {
        self.rest.is_some()
    }
}

impl<'a> Iterator for ContentLines<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let text = self.rest.take()?;
        // A `\r\n` pair is always entered at the `\r`, never in the middle,
        // since the search matches whichever of the two bytes comes first
        match text.find(['\n', '\r']) {
            Some(index) => {
                let bytes = text.as_bytes();
                let break_len = match bytes[index] {
                    b'\r' if bytes.get(index + 1) == Some(&b'\n') => 2,
                    _ => 1,
                };
                self.rest = Some(&text[index + break_len..]);
                Some(&text[..index])
            }
            None => Some(text),
        }
    }
}

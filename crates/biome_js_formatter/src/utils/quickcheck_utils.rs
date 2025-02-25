/// Generates a string with only ascii chars
#[derive(Debug, Clone)]
pub struct AsciiString(String);

impl std::ops::Deref for AsciiString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

impl PartialEq<AsciiString> for &str {
    fn eq(&self, other: &AsciiString) -> bool {
        self == &other.0.as_str()
    }
}

impl std::fmt::Display for AsciiString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl quickcheck::Arbitrary for AsciiString {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let s = String::arbitrary(g);

        let mut ascii = String::new();
        for chr in s.chars() {
            if chr.is_ascii() {
                ascii.push(chr);
            } else {
                const WIDTH: u8 = b'~' - b' ';
                let chr = b' ' + (u8::arbitrary(g) % WIDTH);
                ascii.push(chr as char);
            }
        }

        assert!(ascii.is_ascii());

        Self(ascii)
    }
}

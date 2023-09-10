use crate::prelude::*;

pub fn invalid_digits_after_unicode_escape_sequence(start: usize, end: usize) -> ParseDiagnostic {
    ParseDiagnostic::new("invalid digits after unicode escape sequence", start..end)
        .hint("expected valid unicode escape sequence")
}

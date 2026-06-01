/// Options for scanning a CSS number from source text.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct CssNumberScanOptions {
    stop_before_scss_ellipsis: bool,
    require_standalone_boundary: bool,
}

impl CssNumberScanOptions {
    /// Stops `10...` before the SCSS ellipsis token.
    pub const fn with_scss_ellipsis_boundary(mut self, enabled: bool) -> Self {
        self.stop_before_scss_ellipsis = enabled;
        self
    }

    /// Requires the number start to be outside an identifier-like word.
    ///
    /// Example: this rejects the `5` in `$size-5`.
    pub const fn with_standalone_boundary(mut self, enabled: bool) -> Self {
        self.require_standalone_boundary = enabled;
        self
    }
}

/// Scans one CSS number and returns its end byte offset.
///
/// Example: `.5px` returns the offset after `.5`, leaving `px` to the caller.
/// The caller still classifies dimensions and `%`.
pub fn scan_css_number(source: &str, start: usize, options: CssNumberScanOptions) -> Option<usize> {
    if !is_css_number_start(source, start, options) {
        return None;
    }

    let bytes = source.as_bytes();
    let mut index = start;

    if matches!(bytes.get(index), Some(b'+' | b'-')) {
        index += 1;
    }

    while bytes.get(index).is_some_and(u8::is_ascii_digit) {
        index += 1;
    }

    if bytes.get(index) == Some(&b'.') {
        let is_scss_ellipsis = options.stop_before_scss_ellipsis
            && bytes.get(index + 1) == Some(&b'.')
            && bytes.get(index + 2) == Some(&b'.');

        if !is_scss_ellipsis {
            index += 1;

            while bytes.get(index).is_some_and(u8::is_ascii_digit) {
                index += 1;
            }
        }
    }

    if matches!(bytes.get(index), Some(b'e' | b'E')) {
        let exponent = index;
        index += 1;

        if matches!(bytes.get(index), Some(b'+' | b'-')) {
            index += 1;
        }

        let digits_start = index;

        while bytes.get(index).is_some_and(u8::is_ascii_digit) {
            index += 1;
        }

        if digits_start == index {
            index = exponent;
        }
    }

    Some(index)
}

fn is_css_number_start(source: &str, start: usize, options: CssNumberScanOptions) -> bool {
    let bytes = source.as_bytes();
    let Some(&current) = bytes.get(start) else {
        return false;
    };

    if options.require_standalone_boundary
        && previous_byte(source, start).is_some_and(|byte| is_identifier_byte(*byte))
    {
        return false;
    }

    match current {
        b'0'..=b'9' => true,
        b'.' => bytes.get(start + 1).is_some_and(u8::is_ascii_digit),
        b'+' | b'-' => bytes.get(start + 1).is_some_and(|next| {
            next.is_ascii_digit()
                || (*next == b'.' && bytes.get(start + 2).is_some_and(u8::is_ascii_digit))
        }),
        _ => false,
    }
}

fn previous_byte(source: &str, start: usize) -> Option<&u8> {
    start
        .checked_sub(1)
        .and_then(|index| source.as_bytes().get(index))
}

fn is_identifier_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric()
        || matches!(byte, b'\\' | b'_' | b'-' | b'$' | b'@')
        || byte >= 0x80
}

#[cfg(test)]
mod tests {
    use super::{CssNumberScanOptions, scan_css_number};

    #[test]
    fn scans_css_number_span() {
        let options = CssNumberScanOptions::default();

        assert_eq!(scan_css_number(".5px", 0, options), Some(2));
        assert_eq!(scan_css_number("+.5", 0, options), Some(3));
        assert_eq!(scan_css_number("1.e10", 0, options), Some(5));
        assert_eq!(scan_css_number("1e", 0, options), Some(1));
        assert_eq!(scan_css_number("1-2", 1, options), Some(3));
    }

    #[test]
    fn keeps_scss_ellipsis_out_of_number_span() {
        assert_eq!(
            scan_css_number("10...", 0, CssNumberScanOptions::default()),
            Some(3)
        );
        assert_eq!(
            scan_css_number(
                "10...",
                0,
                CssNumberScanOptions::default().with_scss_ellipsis_boundary(true),
            ),
            Some(2)
        );
    }

    #[test]
    fn can_require_standalone_number_span() {
        let options = CssNumberScanOptions::default().with_standalone_boundary(true);

        assert_eq!(scan_css_number("$size-5", 6, options), None);
        assert_eq!(scan_css_number("\\35 foo", 1, options), None);
        assert_eq!(scan_css_number("2px/.5", 4, options), Some(6));
    }
}

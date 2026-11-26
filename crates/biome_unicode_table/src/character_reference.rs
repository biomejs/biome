//! HTML character references that stand for quote characters.
//!
//! An HTML attribute value is decoded before another language reads it, so
//! `style="font-family: &quot;Rotis Sans&quot;"` gives CSS a quoted string.
//! Lexers of code embedded in attribute values use these references as string
//! delimiters, while the syntax tree keeps the source text.

/// Returns the length of the HTML character reference at the start of `bytes`
/// and the quote byte it stands for, when it stands for `"` or `'`.
///
/// Recognizes the named references `&quot;`, `&QUOT;` and `&apos;`, and decimal
/// or hexadecimal numeric references such as `&#34;` and `&#x27;`. The
/// reference must end with `;`. Returns `None` for any other text.
///
/// ```
/// use biome_unicode_table::html_quote_reference;
///
/// assert_eq!(html_quote_reference(b"&quot;Rotis"), Some((6, b'"')));
/// assert_eq!(html_quote_reference(b"&#x27;"), Some((6, b'\'')));
/// assert_eq!(html_quote_reference(b"&amp;"), None);
/// ```
pub fn html_quote_reference(bytes: &[u8]) -> Option<(usize, u8)> {
    const NAMED: [(&[u8], u8); 3] = [(b"&quot;", b'"'), (b"&QUOT;", b'"'), (b"&apos;", b'\'')];

    if let Some((name, quote)) = NAMED.iter().find(|(name, _)| bytes.starts_with(name)) {
        return Some((name.len(), *quote));
    }

    let (prefix_len, radix) = match bytes {
        [b'&', b'#', b'x' | b'X', ..] => (3, 16),
        [b'&', b'#', ..] => (2, 10),
        _ => return None,
    };
    let digits = &bytes[prefix_len..];
    let digits_len = digits
        .iter()
        .take_while(|byte| char::from(**byte).is_digit(radix))
        .count();
    if digits_len == 0 || digits.get(digits_len) != Some(&b';') {
        return None;
    }

    let value = digits[..digits_len].iter().try_fold(0_u32, |value, byte| {
        value
            .checked_mul(radix)?
            .checked_add(char::from(*byte).to_digit(radix)?)
    })?;
    let quote = match value {
        0x22 => b'"',
        0x27 => b'\'',
        _ => return None,
    };

    Some((prefix_len + digits_len + 1, quote))
}

#[cfg(test)]
mod tests {
    use super::html_quote_reference;

    #[test]
    fn recognizes_named_references() {
        assert_eq!(html_quote_reference(b"&quot;"), Some((6, b'"')));
        assert_eq!(html_quote_reference(b"&QUOT;"), Some((6, b'"')));
        assert_eq!(html_quote_reference(b"&apos;"), Some((6, b'\'')));
        assert_eq!(html_quote_reference(b"&APOS;"), None);
        assert_eq!(html_quote_reference(b"&quot"), None);
    }

    #[test]
    fn recognizes_numeric_references() {
        assert_eq!(html_quote_reference(b"&#34;"), Some((5, b'"')));
        assert_eq!(html_quote_reference(b"&#0039;"), Some((7, b'\'')));
        assert_eq!(html_quote_reference(b"&#x22;"), Some((6, b'"')));
        assert_eq!(html_quote_reference(b"&#X0027;"), Some((8, b'\'')));
        assert_eq!(html_quote_reference(b"&#35;"), None);
        assert_eq!(html_quote_reference(b"&#;"), None);
        assert_eq!(html_quote_reference(b"&#x;"), None);
        assert_eq!(html_quote_reference(b"&#34"), None);
        assert_eq!(html_quote_reference(b"&#99999999999;"), None);
    }
}

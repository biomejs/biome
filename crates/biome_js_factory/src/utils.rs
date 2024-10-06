/// Utility function to escape strings.
///
/// This function iterates over characters of strings and adds an escape character if needed.
/// If there are already odd number of escape characters before the character needs to be escaped,
/// the escape character is not added.
///
/// **sample case**
///
/// | needs_escaping | unescaped | escaped |
/// |:----:|:---------:|:----------:|
/// | `${` | `${abc`   | `\${abc`   |
/// | `${` | `\${abc`  | `\${abc`   |
/// | `${` | `\\${abc` | `\\\${abc` |
///
pub fn escape<'a>(
    unescaped_string: &'a str,
    needs_escaping: &[&str],
    escaping_char: u8,
) -> std::borrow::Cow<'a, str> {
    debug_assert!(!needs_escaping.is_empty());
    debug_assert!(
        escaping_char.is_ascii(),
        "escaping_char must be a valid ASCII character."
    );
    let mut escaped = String::new();
    let mut iter = unescaped_string.bytes().enumerate();
    let mut last_copied_idx = 0;
    while let Some((idx, byte)) = iter.next() {
        if byte == escaping_char {
            // The next character is escaped
            iter.next();
        } else {
            for candidate in needs_escaping {
                if unescaped_string.as_bytes()[idx..].starts_with(candidate.as_bytes()) {
                    if escaped.is_empty() {
                        escaped = String::with_capacity(unescaped_string.len() * 2 - idx);
                    }
                    escaped.push_str(&unescaped_string[last_copied_idx..idx]);
                    escaped.push(escaping_char as char);
                    escaped.push_str(candidate);
                    for _ in candidate.bytes().skip(1) {
                        iter.next();
                    }
                    last_copied_idx = idx + candidate.len();
                    break;
                }
            }
        }
    }
    if escaped.is_empty() {
        std::borrow::Cow::Borrowed(unescaped_string)
    } else {
        escaped.push_str(&unescaped_string[last_copied_idx..unescaped_string.len()]);
        std::borrow::Cow::Owned(escaped)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_escape_dollar_signs_and_backticks() {
        assert_eq!(escape("abc", &["${"], b'\\'), "abc");
        assert_eq!(escape("abc", &["`"], b'\\'), "abc");
        assert_eq!(escape(r"abc\", &["`"], b'\\'), r"abc\");
        assert_eq!(escape("abc $ bca", &["${"], b'\\'), "abc $ bca");
        assert_eq!(escape("abc ${a} bca", &["${"], b'\\'), r"abc \${a} bca");
        assert_eq!(
            escape("abc ${} ${} bca", &["${"], b'\\'),
            r"abc \${} \${} bca"
        );
        assert_eq!(escape("€", &["'"], b'\\'), "€");

        assert_eq!(escape(r"\`", &["`"], b'\\'), r"\`");
        assert_eq!(escape(r"\${}", &["${"], b'\\'), r"\${}");
        assert_eq!(escape(r"\\`", &["`"], b'\\'), r"\\\`");
        assert_eq!(escape(r"\\${}", &["${"], b'\\'), r"\\\${}");
        assert_eq!(escape(r"\\\`", &["`"], b'\\'), r"\\\`");
        assert_eq!(escape(r"\\\${}", &["${"], b'\\'), r"\\\${}");
        assert_eq!(escape("€", &["€"], b'\\'), r"\€");
        assert_eq!(escape("😀€", &["€"], b'\\'), r"😀\€");

        assert_eq!(escape("abc", &["${", "`"], b'\\'), "abc");
        assert_eq!(escape("${} `", &["${", "`"], b'\\'), r"\${} \`");
        assert_eq!(
            escape(r"abc \${a} \`bca", &["${", "`"], b'\\'),
            r"abc \${a} \`bca"
        );
        assert_eq!(escape(r"abc \${bca}", &["${", "`"], b'\\'), r"abc \${bca}");
        assert_eq!(escape(r"abc \`bca", &["${", "`"], b'\\'), r"abc \`bca");

        assert_eq!(escape(r"\n`", &["`"], b'\\'), r"\n\`");
    }
}

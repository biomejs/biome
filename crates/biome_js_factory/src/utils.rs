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
    escaping_char: char,
) -> std::borrow::Cow<'a, str> {
    debug_assert!(!needs_escaping.is_empty());
    let mut escaped = String::new();
    let mut iter = unescaped_string.char_indices();
    let mut last_copied_idx = 0;
    while let Some((idx, chr)) = iter.next() {
        if chr == escaping_char {
            // The next character is esacaped
            iter.next();
        } else {
            for candidate in needs_escaping {
                if unescaped_string[idx..].starts_with(candidate) {
                    if escaped.is_empty() {
                        escaped = String::with_capacity(unescaped_string.len() * 2 - idx);
                    }
                    escaped.push_str(&unescaped_string[last_copied_idx..idx]);
                    escaped.push(escaping_char);
                    escaped.push_str(candidate);
                    for _ in candidate.chars().skip(1) {
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
        assert_eq!(escape("abc", &["${"], '\\'), "abc");
        assert_eq!(escape("abc", &["`"], '\\'), "abc");
        assert_eq!(escape(r"abc\", &["`"], '\\'), r"abc\");
        assert_eq!(escape("abc $ bca", &["${"], '\\'), "abc $ bca");
        assert_eq!(escape("abc ${a} bca", &["${"], '\\'), r"abc \${a} bca");
        assert_eq!(
            escape("abc ${} ${} bca", &["${"], '\\'),
            r"abc \${} \${} bca"
        );

        assert_eq!(escape(r"\`", &["`"], '\\'), r"\`");
        assert_eq!(escape(r"\${}", &["${"], '\\'), r"\${}");
        assert_eq!(escape(r"\\`", &["`"], '\\'), r"\\\`");
        assert_eq!(escape(r"\\${}", &["${"], '\\'), r"\\\${}");
        assert_eq!(escape(r"\\\`", &["`"], '\\'), r"\\\`");
        assert_eq!(escape(r"\\\${}", &["${"], '\\'), r"\\\${}");

        assert_eq!(escape("abc", &["${", "`"], '\\'), "abc");
        assert_eq!(escape("${} `", &["${", "`"], '\\'), r"\${} \`");
        assert_eq!(
            escape(r"abc \${a} \`bca", &["${", "`"], '\\'),
            r"abc \${a} \`bca"
        );
        assert_eq!(escape(r"abc \${bca}", &["${", "`"], '\\'), r"abc \${bca}");
        assert_eq!(escape(r"abc \`bca", &["${", "`"], '\\'), r"abc \`bca");

        assert_eq!(escape(r"\n`", &["`"], '\\'), r"\n\`");
    }
}

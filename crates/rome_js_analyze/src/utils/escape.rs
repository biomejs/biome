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
/// # Examples
///
/// ```
/// use rome_js_analyze::utils::escape::escape;
///
/// let escaped_str_1 = escape("${abc", &["${"], '\\');
/// assert_eq!(escaped_str_1, r#"\${abc"#);
///
/// let escaped_str_2 = escape(r#"\${abc"#, &["${"], '\\');
/// assert_eq!(escaped_str_2, r#"\${abc"#);
///
/// let escaped_str_3 = escape("${ `", &["${", "`"], '\\');
/// assert_eq!(escaped_str_3, r#"\${ \`"#);
/// ```
pub fn escape<'a>(
    unescaped_string: &'a str,
    needs_escaping: &[&str],
    escaping_char: char,
) -> std::borrow::Cow<'a, str> {
    let mut escaped = String::new();
    let mut iter = unescaped_string.char_indices();
    let mut is_escaped = false;
    'unescaped: while let Some((idx, chr)) = iter.next() {
        if chr == escaping_char {
            is_escaped = !is_escaped;
        } else {
            for candidate in needs_escaping {
                if unescaped_string[idx..].starts_with(candidate) {
                    if !is_escaped {
                        if escaped.is_empty() {
                            escaped = String::with_capacity(unescaped_string.len() * 2);
                            escaped.push_str(&unescaped_string[0..idx]);
                        }
                        escaped.push(escaping_char);
                        escaped.push_str(candidate);
                        for _ in candidate.chars().skip(1) {
                            iter.next();
                        }
                        continue 'unescaped;
                    } else {
                        is_escaped = false;
                    }
                }
            }
        }

        if !escaped.is_empty() {
            escaped.push(chr);
        }
    }

    if escaped.is_empty() {
        std::borrow::Cow::Borrowed(unescaped_string)
    } else {
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
    }
}

//! How many letters each token of the tree spans.
//!
//! The lexer fixes the letters and their kinds before parsing. Where a token of the tree spans
//! several letters, such as the value `red-500` or the selector `&>div`, the grammar asks one of
//! these functions how many letters to merge, starting `n` letters after the current one, and
//! bumps them as one token. They only read the letters.

use crate::lexer::css_identifier_kind;
use crate::token_source::TailwindTokenSource;
use biome_parser::token_source::TokenSource;
use biome_tailwind_syntax::T;
use biome_tailwind_syntax::TailwindSyntaxKind::{self, *};

/// Whether `byte` is whitespace.
fn is_whitespace(byte: u8) -> bool {
    matches!(byte, b'\t' | b'\n' | 0x0B | 0x0C | b'\r' | b' ')
}

/// Returns how many letters from the current one come before the first one whose first byte
/// `stop` matches, or the end.
fn letters_until(source: &TailwindTokenSource, n: usize, stop: impl Fn(u8) -> bool) -> usize {
    let mut count = 0;
    while let Some(byte) = source.nth_byte(n + count) {
        if stop(byte) {
            break;
        }
        count += 1;
    }
    count
}

/// Whether the current letter starts a basename: an ASCII letter, digit or `_`, or an `@`
/// unless the basename follows the `-` of a negative candidate.
pub(crate) fn is_at_base(source: &TailwindTokenSource, negative: bool) -> bool {
    match source.nth_byte(0) {
        Some(b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_') => true,
        Some(b'@') => !negative,
        _ => false,
    }
}

/// Returns the length in letters of the basename at the current letter: up to the next `-`,
/// whitespace, `!`, `:` or `/`, continued over each `-` that a dashed basename continues over.
pub(crate) fn base(source: &TailwindTokenSource) -> usize {
    let count = letters_until(source, 1, |byte| {
        is_whitespace(byte) || matches!(byte, b'-' | b'!' | b':' | b'/')
    }) + 1;
    if source.nth_byte(count) != Some(b'-') {
        return count;
    }
    let end = source.dashed_base_end(source.nth_start(0), source.nth_start(count));
    source.letters_before(end)
}

/// Where a value ends: after the `-` of a candidate, or after the `/` of a modifier.
#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum ValueEnd {
    /// Whitespace, `:`, `/`, `!`, `]`, `)` or `%`.
    Candidate,
    /// Whitespace, `!`, `%` or `:`.
    Modifier,
}

/// Returns the kind and length in letters of the value at the current letter, which may be
/// none: a `TW_NUMBER` if it holds only digits and `.`, and a `TW_VALUE` otherwise.
pub(crate) fn value(source: &TailwindTokenSource, end: ValueEnd) -> (TailwindSyntaxKind, usize) {
    let count = letters_until(source, 0, |byte| {
        is_whitespace(byte)
            || match end {
                ValueEnd::Candidate => matches!(byte, b':' | b'/' | b'!' | b']' | b')' | b'%'),
                ValueEnd::Modifier => matches!(byte, b'!' | b'%' | b':'),
            }
    });
    let numeric = (0..count).all(|n| {
        source
            .nth_byte(n)
            .is_some_and(|byte| byte.is_ascii_digit() || byte == b'.')
    });
    (if numeric { TW_NUMBER } else { TW_VALUE }, count)
}

/// Whether the current letter starts a selector, rather than being whitespace or a bracket of
/// its own.
pub(crate) fn is_at_selector(source: &TailwindTokenSource) -> bool {
    source
        .nth_byte(0)
        .is_some_and(|byte| !is_whitespace(byte) && !matches!(byte, b'(' | b')' | b']'))
}

/// Returns the length in letters of the selector at the current letter: up to the next
/// whitespace or the `]` that closes the brackets it opens.
pub(crate) fn selector(source: &TailwindTokenSource) -> usize {
    let mut depth = 0u32;
    let mut count = 0;
    while let Some(byte) = source.nth_byte(count) {
        match byte {
            b'[' => depth += 1,
            b']' if depth > 0 => depth -= 1,
            b']' => break,
            _ if is_whitespace(byte) => break,
            _ => {}
        }
        count += 1;
    }
    count
}

/// Whether the current letter starts the property of an arbitrary candidate.
pub(crate) fn is_at_property(source: &TailwindTokenSource) -> bool {
    source.nth_byte(0).is_some_and(|byte| {
        !is_whitespace(byte) && !matches!(byte, b'(' | b')' | b'[' | b']' | b':')
    })
}

/// Returns the length in letters of the property at the current letter: up to the next `:` or
/// whitespace.
pub(crate) fn property(source: &TailwindTokenSource) -> usize {
    letters_until(source, 0, |byte| is_whitespace(byte) || byte == b':')
}

/// Whether the current letter starts a variant segment name.
pub(crate) fn is_at_segment(source: &TailwindTokenSource) -> bool {
    matches!(
        source.nth_byte(0),
        Some(b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'@' | b'*')
    )
}

/// Returns the length in letters of the variant segment name at the current letter: up to the
/// next whitespace, `:`, `-`, `/`, `!`, `[`, `]` or `)`.
pub(crate) fn segment(source: &TailwindTokenSource) -> usize {
    letters_until(source, 1, |byte| {
        is_whitespace(byte) || matches!(byte, b':' | b'-' | b'/' | b'!' | b'[' | b']' | b')')
    }) + 1
}

/// Returns the length in letters of the raw URL at the current letter: up to the next `)` or
/// whitespace.
pub(crate) fn url_raw_value(source: &TailwindTokenSource) -> usize {
    letters_until(source, 0, |byte| is_whitespace(byte) || byte == b')')
}

/// Returns the kind and length in letters of the token of a CSS value that starts `n` letters
/// after the current one.
pub(crate) fn css_token(source: &TailwindTokenSource, n: usize) -> (TailwindSyntaxKind, usize) {
    let Some(byte) = source.nth_byte(n) else {
        return (EOF, 0);
    };
    let second = source.nth_byte(n + 1);
    match byte {
        _ if is_whitespace(byte) => (WHITESPACE, 1),
        // In math functions, Tailwind adds whitespace around a `-` that directly follows a value
        // (`calc(100svh-var(--header))` becomes `calc(100svh - var(--header))`), so it is a
        // binary operator rather than the start of a negative number or an identifier. This
        // doesn't track the enclosing function, so it also applies outside math functions.
        //
        // This deviates from the CSS Syntax spec and `biome_css_parser`, which require
        // whitespace around `-` in math functions and lex `100svh-var` as one dimension with the
        // unit `svh-var`.
        b'-' if source
            .nth_start(n)
            .checked_sub(1)
            .map(|last| source.text().as_bytes()[last])
            .is_some_and(|last| last.is_ascii_alphanumeric() || matches!(last, b'%' | b')')) =>
        {
            (T![-], 1)
        }
        b'-' if second.is_some_and(|second| second.is_ascii_alphabetic() || second == b'-') => {
            css_identifier(source, n)
        }
        b'-' if second.is_some_and(|second| second.is_ascii_digit() || second == b'.') => {
            css_number(source, n)
        }
        b'0'..=b'9' => css_number(source, n),
        b'.' if second.is_some_and(|second| second.is_ascii_digit()) => css_number(source, n),
        b'a'..=b'z' | b'A'..=b'Z' => css_identifier(source, n),
        b'"' | b'\'' => (CSS_STRING_LITERAL, css_string(source, n)),
        b'#' => (CSS_COLOR_LITERAL, css_color(source, n)),
        _ => match source.nth_kind(n) {
            kind @ (T!['(']
            | T![')']
            | T!['[']
            | T![']']
            | T![:]
            | T![-]
            | T![%]
            | T![+]
            | T![*]
            | T![,]
            | T![.]
            | T![=]
            | T![/]
            | CSS_WHITESPACE) => (kind, 1),
            _ => (ERROR_TOKEN, 1),
        },
    }
}

/// Returns the kind and length in letters of the CSS identifier that starts `n` letters after
/// the current one: ASCII letters, digits and `-`, and also `_` and non-ASCII chars in a custom
/// property, which starts with `--`. Its kind is the keyword it spells, such as a unit, or
/// `IDENT`.
fn css_identifier(source: &TailwindTokenSource, n: usize) -> (TailwindSyntaxKind, usize) {
    let start = source.nth_start(n);
    let bytes = source.text().as_bytes();
    let is_custom_property = bytes[start..].starts_with(b"--");
    let mut count = 0;
    while let Some(byte) = source.nth_byte(n + count) {
        let continues = byte.is_ascii_alphanumeric()
            || byte == b'-'
            || (is_custom_property && (byte == b'_' || !byte.is_ascii()));
        if !continues {
            break;
        }
        count += 1;
    }
    let end = source.nth_start(n + count);
    (css_identifier_kind(&bytes[start..end]), count)
}

/// Returns the kind and length in letters of the CSS number that starts `n` letters after the
/// current one: an optional sign, digits, a fraction and an exponent. Its kind is a percentage if
/// `%` follows, a dimension if a letter follows, and a number otherwise.
fn css_number(source: &TailwindTokenSource, n: usize) -> (TailwindSyntaxKind, usize) {
    let is_digits = |at: usize| {
        source
            .nth_byte(at)
            .is_some_and(|byte| byte.is_ascii_digit())
    };
    let mut at = n;
    if matches!(source.nth_byte(at), Some(b'+' | b'-')) {
        at += 1;
    }
    if is_digits(at) {
        at += 1;
    }
    if source.nth_byte(at) == Some(b'.') && is_digits(at + 1) {
        at += 2;
    }
    if matches!(source.nth_text(at), "e" | "E") {
        let mut exponent = at + 1;
        if matches!(source.nth_byte(exponent), Some(b'+' | b'-')) {
            exponent += 1;
        }
        if is_digits(exponent) {
            at = exponent + 1;
        }
    }
    let kind = match source.nth_byte(at) {
        Some(b'%') => CSS_PERCENTAGE_VALUE,
        Some(byte) if byte.is_ascii_alphabetic() => CSS_DIMENSION_VALUE,
        _ => CSS_NUMBER_LITERAL,
    };
    (kind, at - n)
}

/// Returns the length in letters of the CSS string that starts `n` letters after the current
/// one, which runs to its closing quote or to the next whitespace.
fn css_string(source: &TailwindTokenSource, n: usize) -> usize {
    let quote = source.nth_byte(n);
    let mut at = n + 1;
    while let Some(byte) = source.nth_byte(at) {
        if is_whitespace(byte) {
            break;
        }
        at += 1;
        if Some(byte) == quote {
            break;
        }
        if byte == b'\\'
            && source
                .nth_byte(at)
                .is_some_and(|escaped| !is_whitespace(escaped))
        {
            at += 1;
        }
    }
    at - n
}

/// Returns the length in letters of the hex color that starts `n` letters after the current
/// one: a `#` and the hex digits after it. The lexer splits the color from the letters after it.
fn css_color(source: &TailwindTokenSource, n: usize) -> usize {
    let mut at = n + 1;
    while source
        .nth_byte(at)
        .is_some_and(|byte| byte.is_ascii_alphanumeric())
        && source
            .nth_text(at)
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
    {
        at += 1;
    }
    at - n
}

#[cfg(test)]
mod tests {
    use super::segment;
    use crate::lexer::BASENAME_STORE;
    use crate::token_source::TailwindTokenSource;

    #[test]
    fn segments_end_at_a_closing_parenthesis() {
        let source = TailwindTokenSource::from_str("a)b", &BASENAME_STORE);
        assert_eq!(segment(&source), 1);
    }
}

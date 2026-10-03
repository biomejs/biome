//! The content of [flow scalars](https://yaml.org/spec/1.2.2/#73-flow-scalar-styles): plain,
//! single-quoted, and double-quoted.
use biome_rowan::{SyntaxKind, TokenText};
use biome_yaml_syntax::AnyYamlFlowScalar;
use std::iter::{Peekable, repeat_n};
use std::str::CharIndices;

/// Returns the content of a flow scalar: without quotes, with escape sequences processed, and
/// with lines folded. Returns `None` if the scalar has no token.
///
/// If the scalar has a single line without escape sequences, its text is returned without any
/// allocation.
pub(super) fn flow_scalar_text(scalar: &AnyYamlFlowScalar) -> Option<TokenText> {
    let text = scalar.inner_string_text().ok()?;
    let needs_processing = text.contains(['\n', '\r'])
        || match scalar {
            AnyYamlFlowScalar::YamlPlainScalar(_) => false,
            AnyYamlFlowScalar::YamlSingleQuotedScalar(_) => text.contains("''"),
            AnyYamlFlowScalar::YamlDoubleQuotedScalar(_) => text.contains('\\'),
        };
    if !needs_processing {
        return Some(text);
    }
    let kind = scalar.value_token().ok()?.kind();
    Some(TokenText::new_raw(
        kind.to_raw(),
        &fold_flow_scalar(&text, scalar),
    ))
}

/// Folds the lines of a flow scalar, and processes its escape sequences.
///
/// A line break between two lines becomes a space, unless empty lines follow it, which become
/// line feeds. The blanks around line breaks aren't part of the content.
/// See <https://yaml.org/spec/1.2.2/#73-flow-scalar-styles>.
fn fold_flow_scalar(text: &str, scalar: &AnyYamlFlowScalar) -> String {
    let mut result = String::with_capacity(text.len());
    let mut chars = text.char_indices().peekable();
    // The start of the last blanks, which are only part of the content when a non-blank
    // character follows them on the same line
    let mut blanks_start = None;
    while let Some((index, c)) = chars.next() {
        match c {
            ' ' | '\t' => {
                blanks_start.get_or_insert(index);
                continue;
            }
            '\n' | '\r' => {
                blanks_start = None;
                if c == '\r' {
                    chars.next_if(|&(_, c)| c == '\n');
                }
                match skip_empty_lines(&mut chars) {
                    0 => result.push(' '),
                    empty_lines => result.extend(repeat_n('\n', empty_lines)),
                }
                continue;
            }
            _ => {}
        }
        if let Some(start) = blanks_start.take() {
            result.push_str(&text[start..index]);
        }
        match c {
            '\\' if matches!(scalar, AnyYamlFlowScalar::YamlDoubleQuotedScalar(_)) => {
                unescape(&mut chars, &mut result);
            }
            '\'' if matches!(scalar, AnyYamlFlowScalar::YamlSingleQuotedScalar(_)) => {
                // `''` is an escaped quote
                chars.next_if(|&(_, c)| c == '\'');
                result.push('\'');
            }
            c => result.push(c),
        }
    }
    // The blanks that precede the closing quote are part of the content
    if !matches!(scalar, AnyYamlFlowScalar::YamlPlainScalar(_))
        && let Some(start) = blanks_start
    {
        result.push_str(&text[start..]);
    }
    result
}

/// Skips the empty lines that follow a line break, and the indentation of the next line.
///
/// Returns the number of empty lines.
fn skip_empty_lines(chars: &mut Peekable<CharIndices>) -> usize {
    let mut empty_lines = 0;
    loop {
        while chars.next_if(|&(_, c)| matches!(c, ' ' | '\t')).is_some() {}
        match chars.next_if(|&(_, c)| matches!(c, '\n' | '\r')) {
            Some((_, c)) => {
                if c == '\r' {
                    chars.next_if(|&(_, c)| c == '\n');
                }
                empty_lines += 1;
            }
            None => return empty_lines,
        }
    }
}

/// Processes the escape sequence that follows a `\` in a double-quoted scalar.
///
/// See <https://yaml.org/spec/1.2.2/#57-escaped-characters>.
fn unescape(chars: &mut Peekable<CharIndices>, result: &mut String) {
    let Some((_, c)) = chars.next() else {
        // The lexer reports the incomplete escape sequence
        result.push('\\');
        return;
    };
    let unescaped = match c {
        // An escaped line break joins the lines without a space
        '\n' | '\r' => {
            if c == '\r' {
                chars.next_if(|&(_, c)| c == '\n');
            }
            let empty_lines = skip_empty_lines(chars);
            result.extend(repeat_n('\n', empty_lines));
            return;
        }
        '0' => '\0',
        'a' => '\u{7}',
        'b' => '\u{8}',
        't' => '\t',
        'n' => '\n',
        'v' => '\u{b}',
        'f' => '\u{c}',
        'r' => '\r',
        'e' => '\u{1b}',
        'N' => '\u{85}',
        '_' => '\u{a0}',
        'L' => '\u{2028}',
        'P' => '\u{2029}',
        'x' | 'u' | 'U' => {
            let digits = match c {
                'x' => 2,
                'u' => 4,
                _ => 8,
            };
            let code_point = (0..digits)
                .map_while(|_| {
                    chars
                        .next_if(|(_, c)| c.is_ascii_hexdigit())?
                        .1
                        .to_digit(16)
                })
                .fold(0, |code_point, digit| code_point * 16 + digit);
            // The lexer reports invalid code points
            let Some(unescaped) = char::from_u32(code_point) else {
                return;
            };
            unescaped
        }
        // A tab, a space, `"`, `/`, and `\` escape themselves.
        // The lexer reports unknown escape sequences.
        c => c,
    };
    result.push(unescaped);
}

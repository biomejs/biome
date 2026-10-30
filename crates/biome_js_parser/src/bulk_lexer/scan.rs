//! Scanners for single tokens and the tables they use.

use super::REGULAR;
use biome_js_syntax::JsSyntaxKind::{self, *};
use biome_js_syntax::T;

/// The length of the UTF-8 char whose first byte is `byte`.
pub(super) fn utf8_len(byte: u8) -> usize {
    match byte {
        0xF0.. => 4,
        0xE0.. => 3,
        _ => 2,
    }
}

/// The code of a keyword is one plus its index in [KEYWORDS].
pub(super) const KEYWORD_CODES: u8 = 1;

/// The kind and flags of the tokens in the regular lexing context that their first byte
/// determines, or zero for the bytes that start other tokens.
pub(super) static BYTE_TOKENS: [u16; 256] = {
    let mut table = [0; 256];
    let mut byte = 0;
    while byte < 128 {
        let kind = match byte as u8 {
            b'\t' | 0x0B | 0x0C | b' ' => WHITESPACE,
            b'\n' | b'\r' => NEWLINE,
            b'a'..=b'z' | b'A'..=b'Z' | b'_' | b'$' => IDENT,
            b'(' => T!['('],
            b')' => T![')'],
            b'[' => T!['['],
            b']' => T![']'],
            b'{' => T!['{'],
            b'}' => T!['}'],
            b',' => T![,],
            b';' => T![;],
            b':' => T![:],
            b'~' => T![~],
            b'@' => T![@],
            b'#' => T![#],
            b'`' => BACKTICK,
            b'>' => T![>],
            b'<' => T![<],
            b'=' => T![=],
            b'!' => T![!],
            b'?' => T![?],
            b'%' => T![%],
            b'&' => T![&],
            b'*' => T![*],
            b'+' => T![+],
            b'-' => T![-],
            b'.' => T![.],
            b'/' => T![/],
            b'^' => T![^],
            b'|' => T![|],
            _ => TOMBSTONE,
        };
        if !matches!(kind, TOMBSTONE) {
            table[byte] = kind as u16 | REGULAR;
        }
        byte += 1;
    }
    table
};

/// Returns the kind and length of the operator longer than one byte that starts with `first`,
/// as `JsLexer` lexes it in the regular context.
#[inline(always)]
pub(super) fn operator(first: u8, second: u8, third: u8) -> Option<(JsSyntaxKind, usize)> {
    let with_eq = |two: JsSyntaxKind, three: JsSyntaxKind| {
        if third == b'=' { (three, 3) } else { (two, 2) }
    };
    Some(match (first, second) {
        (b'!', b'=') => with_eq(T![!=], T![!==]),
        (b'%', b'=') => (T![%=], 2),
        (b'&', b'&') => with_eq(T![&&], T![&&=]),
        (b'&', b'=') => (T![&=], 2),
        (b'*', b'*') => with_eq(T![**], T![**=]),
        (b'*', b'=') => (T![*=], 2),
        (b'+', b'+') => (T![++], 2),
        (b'+', b'=') => (T![+=], 2),
        (b'-', b'-') => (T![--], 2),
        (b'-', b'=') => (T![-=], 2),
        (b'.', b'.') if third == b'.' => (T![...], 3),
        (b'<', b'<') => with_eq(T![<<], T![<<=]),
        (b'<', b'=') => (T![<=], 2),
        (b'=', b'=') => with_eq(T![==], T![===]),
        (b'=', b'>') => (T![=>], 2),
        (b'?', b'?') => with_eq(T![??], T![??=]),
        (b'?', b'.') if !third.is_ascii_digit() => (T![?.], 2),
        (b'^', b'=') => (T![^=], 2),
        (b'|', b'|') => with_eq(T![||], T![||=]),
        (b'|', b'=') => (T![|=], 2),
        _ => return None,
    })
}

/// Returns the end of the number starting at `start`, a digit or a `.` followed by a digit, as
/// `JsLexer` lexes it. Returns `None` for numbers the lexer reports diagnostics for, and for
/// numeric separators.
pub(super) fn scan_number(source: &[u8], start: usize) -> Option<usize> {
    let byte = |offset: usize| source.get(offset).copied().unwrap_or(0);
    let end = match (byte(start), byte(start + 1)) {
        (b'.', _) => scan_fraction(source, start)?,
        (b'0', b'x' | b'X') => scan_radix(source, start + 1, |byte| byte.is_ascii_hexdigit())?,
        (b'0', b'b' | b'B') => scan_radix(source, start + 1, |byte| matches!(byte, b'0' | b'1'))?,
        (b'0', b'o' | b'O') => scan_radix(source, start + 1, |byte| matches!(byte, b'0'..=b'7'))?,
        (b'0', b'n') => start + 2,
        (b'0', b'.') => scan_fraction(source, start + 1)?,
        (b'0', b'e' | b'E') => scan_exponent(source, start + 1)?,
        (b'0', _) => scan_integer(source, start, true)?,
        _ => scan_integer(source, start, false)?,
    };
    // A number followed by the start of an identifier is an error.
    match byte(end) {
        byte if byte.is_ascii_alphabetic() || matches!(byte, b'_' | b'$' | b'\\') => None,
        byte if byte >= 0x80 => None,
        _ => Some(end),
    }
}

/// Scans the digits after the integer digit at `start`.
fn scan_integer(source: &[u8], start: usize, leading_zero: bool) -> Option<usize> {
    let byte = |offset: usize| source.get(offset).copied().unwrap_or(0);
    let mut offset = start + 1;
    loop {
        match byte(offset) {
            b'0'..=b'9' => offset += 1,
            b'.' if !leading_zero => return scan_fraction(source, offset),
            b'e' | b'E' => return scan_exponent(source, offset).or(Some(offset)),
            b'n' if !leading_zero => return Some(offset + 1),
            b'_' | b'.' | b'n' => return None,
            _ => return Some(offset),
        }
    }
}

/// Scans the digits and the exponent after the `.` at `dot`.
fn scan_fraction(source: &[u8], dot: usize) -> Option<usize> {
    let byte = |offset: usize| source.get(offset).copied().unwrap_or(0);
    let mut offset = dot + 1;
    loop {
        match byte(offset) {
            b'0'..=b'9' => offset += 1,
            b'e' | b'E' => return scan_exponent(source, offset).or(Some(offset)),
            b'_' => return None,
            _ => return Some(offset),
        }
    }
}

/// Scans the exponent starting with the `e` at `e`. Returns `None` if no digits follow.
fn scan_exponent(source: &[u8], e: usize) -> Option<usize> {
    let byte = |offset: usize| source.get(offset).copied().unwrap_or(0);
    let mut offset = e + 1;
    if matches!(byte(offset), b'+' | b'-') {
        offset += 1;
    }
    if !byte(offset).is_ascii_digit() {
        return None;
    }
    loop {
        match byte(offset) {
            b'0'..=b'9' => offset += 1,
            b'_' => return None,
            _ => return Some(offset),
        }
    }
}

/// Scans the digits after the radix prefix letter at `prefix`, and the `n` of a bigint.
fn scan_radix(source: &[u8], prefix: usize, is_digit: impl Fn(u8) -> bool) -> Option<usize> {
    let byte = |offset: usize| source.get(offset).copied().unwrap_or(0);
    if !is_digit(byte(prefix + 1)) {
        return None;
    }
    let mut offset = prefix + 1;
    loop {
        match byte(offset) {
            byte if is_digit(byte) => offset += 1,
            b'_' => return None,
            b'n' => return Some(offset + 1),
            _ => return Some(offset),
        }
    }
}

/// Keywords, which [KEYWORD_SLOTS] refers to by their index plus one.
pub(super) static KEYWORDS: [(&[u8], JsSyntaxKind); 84] = [
    (b"break", BREAK_KW),
    (b"case", CASE_KW),
    (b"catch", CATCH_KW),
    (b"class", CLASS_KW),
    (b"const", CONST_KW),
    (b"continue", CONTINUE_KW),
    (b"debugger", DEBUGGER_KW),
    (b"default", DEFAULT_KW),
    (b"defer", DEFER_KW),
    (b"delete", DELETE_KW),
    (b"do", DO_KW),
    (b"else", ELSE_KW),
    (b"enum", ENUM_KW),
    (b"export", EXPORT_KW),
    (b"extends", EXTENDS_KW),
    (b"false", FALSE_KW),
    (b"finally", FINALLY_KW),
    (b"for", FOR_KW),
    (b"function", FUNCTION_KW),
    (b"if", IF_KW),
    (b"in", IN_KW),
    (b"import", IMPORT_KW),
    (b"instanceof", INSTANCEOF_KW),
    (b"new", NEW_KW),
    (b"null", NULL_KW),
    (b"return", RETURN_KW),
    (b"source", SOURCE_KW),
    (b"super", SUPER_KW),
    (b"switch", SWITCH_KW),
    (b"this", THIS_KW),
    (b"throw", THROW_KW),
    (b"try", TRY_KW),
    (b"true", TRUE_KW),
    (b"typeof", TYPEOF_KW),
    (b"var", VAR_KW),
    (b"void", VOID_KW),
    (b"while", WHILE_KW),
    (b"with", WITH_KW),
    (b"implements", IMPLEMENTS_KW),
    (b"interface", INTERFACE_KW),
    (b"let", LET_KW),
    (b"package", PACKAGE_KW),
    (b"private", PRIVATE_KW),
    (b"protected", PROTECTED_KW),
    (b"public", PUBLIC_KW),
    (b"static", STATIC_KW),
    (b"yield", YIELD_KW),
    (b"abstract", ABSTRACT_KW),
    (b"accessor", ACCESSOR_KW),
    (b"as", AS_KW),
    (b"asserts", ASSERTS_KW),
    (b"assert", ASSERT_KW),
    (b"any", ANY_KW),
    (b"async", ASYNC_KW),
    (b"await", AWAIT_KW),
    (b"boolean", BOOLEAN_KW),
    (b"constructor", CONSTRUCTOR_KW),
    (b"declare", DECLARE_KW),
    (b"get", GET_KW),
    (b"infer", INFER_KW),
    (b"is", IS_KW),
    (b"keyof", KEYOF_KW),
    (b"module", MODULE_KW),
    (b"namespace", NAMESPACE_KW),
    (b"never", NEVER_KW),
    (b"readonly", READONLY_KW),
    (b"require", REQUIRE_KW),
    (b"number", NUMBER_KW),
    (b"object", OBJECT_KW),
    (b"satisfies", SATISFIES_KW),
    (b"set", SET_KW),
    (b"string", STRING_KW),
    (b"symbol", SYMBOL_KW),
    (b"type", TYPE_KW),
    (b"undefined", UNDEFINED_KW),
    (b"unique", UNIQUE_KW),
    (b"unknown", UNKNOWN_KW),
    (b"from", FROM_KW),
    (b"global", GLOBAL_KW),
    (b"bigint", BIGINT_KW),
    (b"override", OVERRIDE_KW),
    (b"of", OF_KW),
    (b"out", OUT_KW),
    (b"using", USING_KW),
];

/// Hashes a word from its first two bytes, its last byte and its length. No two keywords share
/// a slot.
#[inline(always)]
const fn keyword_slot(first: u8, second: u8, last: u8, len: usize) -> usize {
    let key = first as u32 | (second as u32) << 8 | (last as u32) << 16 | (len as u32) << 24;
    (key.wrapping_mul(0x9d88_b9e3) >> 23) as usize
}

/// The code of the keyword in each [keyword_slot], or zero.
static KEYWORD_SLOTS: [u8; 512] = {
    let mut slots = [0; 512];
    let mut index = 0;
    while index < KEYWORDS.len() {
        let text = KEYWORDS[index].0;
        let slot = keyword_slot(text[0], text[1], text[text.len() - 1], text.len());
        assert!(slots[slot] == 0, "two keywords share a slot");
        slots[slot] = index as u8 + 1;
        index += 1;
    }
    slots
};

/// A keyword's text as two little-endian words padded with zeros, for comparing it with eight
/// bytes at a time.
#[derive(Clone, Copy)]
struct KeywordText {
    low: u64,
    high: u64,
    len: usize,
}

/// The text of each keyword by its code, with an empty text that no word matches at code zero.
static KEYWORD_TEXTS: [KeywordText; KEYWORDS.len() + 1] = {
    let mut texts = [KeywordText {
        low: 0,
        high: 0,
        len: usize::MAX,
    }; KEYWORDS.len() + 1];
    let mut index = 0;
    while index < KEYWORDS.len() {
        let text = KEYWORDS[index].0;
        let mut keyword = KeywordText {
            low: 0,
            high: 0,
            len: text.len(),
        };
        let mut byte = 0;
        while byte < text.len() {
            if byte < 8 {
                keyword.low |= (text[byte] as u64) << (8 * byte);
            } else {
                keyword.high |= (text[byte] as u64) << (8 * (byte - 8));
            }
            byte += 1;
        }
        texts[KEYWORD_CODES as usize + index] = keyword;
        index += 1;
    }
    texts
};

/// The masks of the first `len` bytes of 16, by `len`, in the low and the high word.
static LOW_MASKS: [u64; 12] = {
    let mut masks = [u64::MAX; 12];
    let mut len = 0;
    while len < 8 {
        masks[len] = (1 << (8 * len)) - 1;
        len += 1;
    }
    masks
};
static HIGH_MASKS: [u64; 12] = {
    let mut masks = [0; 12];
    let mut len = 9;
    while len < 12 {
        masks[len] = (1 << (8 * (len - 8))) - 1;
        len += 1;
    }
    masks
};

/// Returns the code of the keyword spelled by `source[start..end]`, or zero.
#[inline(always)]
pub(super) fn keyword(source: &[u8], start: usize, end: usize) -> u8 {
    let len = end - start;
    // The first 16 bytes from `start`, padded with zeros past the end of the source.
    let (low, high) = match source.get(start..start + 16) {
        Some(bytes) => (
            u64::from_le_bytes(bytes[..8].try_into().expect("8 bytes")),
            u64::from_le_bytes(bytes[8..].try_into().expect("8 bytes")),
        ),
        None => {
            let mut bytes = [0; 16];
            let available = &source[start..];
            bytes[..available.len()].copy_from_slice(available);
            (
                u64::from_le_bytes(bytes[..8].try_into().expect("8 bytes")),
                u64::from_le_bytes(bytes[8..].try_into().expect("8 bytes")),
            )
        }
    };
    // Words longer than 16 bytes hash their 16th byte, rather than their last, and match no
    // keyword either way.
    let last = (len - 1).min(15);
    let last_byte = (if last < 8 { low } else { high } >> (8 * (last % 8))) as u8;
    let code = KEYWORD_SLOTS[keyword_slot(low as u8, (low >> 8) as u8, last_byte, len)];
    let keyword = &KEYWORD_TEXTS[code as usize];
    // One comparison of all the differences, rather than one branch on the data for each.
    let mask = len.min(LOW_MASKS.len() - 1);
    let difference = (keyword.len ^ len) as u64
        | ((low & LOW_MASKS[mask]) ^ keyword.low)
        | ((high & HIGH_MASKS[mask]) ^ keyword.high);
    code * u8::from(difference == 0)
}

/// The kind of an identifier by the code of its keyword, or zero.
pub(super) static KEYWORD_KINDS: [u16; KEYWORDS.len() + 1] = {
    let mut kinds = [IDENT as u16 | REGULAR; KEYWORDS.len() + 1];
    let mut index = 0;
    while index < KEYWORDS.len() {
        kinds[KEYWORD_CODES as usize + index] = KEYWORDS[index].1 as u16 | REGULAR;
        index += 1;
    }
    kinds
};

/// Whether an expression, rather than an operator, follows `keyword`, so that a `/` after it
/// starts a regex and a `<` may start a JSX element.
pub(super) fn is_operand_keyword(keyword: JsSyntaxKind) -> bool {
    matches!(
        keyword,
        T![return]
            | T![typeof]
            | T![instanceof]
            | T![in]
            | T![of]
            | T![new]
            | T![delete]
            | T![void]
            | T![throw]
            | T![case]
            | T![do]
            | T![else]
            | T![yield]
            | T![await]
    )
}

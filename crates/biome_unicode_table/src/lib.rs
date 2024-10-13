use crate::bytes::DISPATCHER;
use crate::tables::derived_property::{ID_Continue, ID_Start};

mod bytes;
mod tables;

pub use crate::bytes::Dispatch;

/// Tests if `c` is a valid start of a CSS identifier
#[inline]
pub fn is_html_id_start(c: char) -> bool {
    ID_Start(c)
}

/// Is `c` a CSS non-ascii character.
/// See <https://drafts.csswg.org/css-syntax-3/#ident-token-diagram>
/// See <https://drafts.csswg.org/css-syntax-3/#non-ascii-ident-code-point>
///
/// In contrast to the standard we also accept all characters from:
/// - the Miscellaneous Symbols Unicode block
/// - the Dingbats Unicode block
///
/// We also accept some characters of the Miscellaneous Technical Unicode block.
#[inline]
pub fn is_css_non_ascii(c: char) -> bool {
    matches!(
        c as u32,
        0xB7
        | 0xc0..=0xd6
        | 0xd8..=0xf6
        | 0xf8..=0x37D
        | 0x37F..=0x1FFF
        | 0x200C
        | 0x200D
        | 0x203F
        | 0x2040
        | 0x2070..=0x218F
        // https://en.wikipedia.org/wiki/List_of_Unicode_characters#Miscellaneous_Technical
        | 0x2318 | 0x231A | 0x231B | 0x2328 | 0x2399
        | 0x23E9..=0x23F3
        | 0x23F9..=0x23FE
        // https://en.wikipedia.org/wiki/List_of_Unicode_characters#Miscellaneous_Symbols
        // https://en.wikipedia.org/wiki/Dingbats_(Unicode_block)
        | 0x2600..=0x27BF
        | 0x2C00..=0x2FEF
        | 0x3001..=0xD7FF
        | 0xF900..=0xFDCF
        | 0xFDF0..=0xFFFD
        | 0x10000..
    )
}

/// Tests if `c` is a valid start of a js identifier
#[inline]
pub fn is_js_id_start(c: char) -> bool {
    c == '_' || c == '$' || ID_Start(c)
}

/// Tests if `c` is a valid continuation of a js identifier.
#[inline]
pub fn is_js_id_continue(c: char) -> bool {
    c == '$' || c == '\u{200d}' || c == '\u{200c}' || ID_Continue(c)
}

/// Check if `s` is a valid _JavaScript_ identifier.
/// Currently, it doesn't check escaped unicode chars.
///
/// ```
/// use biome_unicode_table::is_js_ident;
///
/// assert!(is_js_ident("id0"));
/// assert!(is_js_ident("$id$"));
/// assert!(is_js_ident("_id_"));
/// assert!(is_js_ident("𐊧"));
///
/// assert!(!is_js_ident(""));
/// assert!(!is_js_ident("@"));
/// assert!(!is_js_ident("custom-id"));
/// assert!(!is_js_ident("0"));
/// ```
pub fn is_js_ident(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    s.chars().enumerate().all(|(index, c)| {
        if index == 0 {
            is_js_id_start(c)
        } else {
            is_js_id_continue(c)
        }
    })
}

/// Looks up a byte in the lookup table.
#[inline]
pub fn lookup_byte(byte: u8) -> Dispatch {
    // Safety: the lookup table maps all values of u8, so it's impossible for a u8 to be out of bounds
    unsafe { *DISPATCHER.get_unchecked(byte as usize) }
}

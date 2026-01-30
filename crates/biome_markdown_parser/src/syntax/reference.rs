use std::borrow::Cow;

use biome_string_case::StrOnlyExtension;

/// Normalize a reference label per CommonMark spec.
///
/// Per CommonMark, label normalization involves:
/// 1. Collapsing consecutive whitespace into a single space
/// 2. Case-folding (case-insensitive matching)
///
/// IMPORTANT: Backslash escapes are NOT stripped during normalization.
/// This means `[foo\!]` does NOT match `[foo!]` - the backslash is preserved.
/// This matches cmark's reference implementation behavior.
pub(crate) fn normalize_reference_label(text: &str) -> String {
    let mut out = String::new();
    let mut saw_whitespace = false;

    for c in text.chars() {
        if c.is_whitespace() {
            saw_whitespace = true;
            continue;
        }

        push_normalized_char(&mut out, c, &mut saw_whitespace);
    }

    // CommonMark uses Unicode case folding for case-insensitive matching (utf8proc).
    let folded = out.as_str().to_casefold_cow();
    match folded {
        Cow::Borrowed(_) => out,
        Cow::Owned(folded) => folded,
    }
}

fn push_normalized_char(out: &mut String, c: char, saw_whitespace: &mut bool) {
    if *saw_whitespace && !out.is_empty() {
        out.push(' ');
    }
    *saw_whitespace = false;
    out.push(c);
}

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
pub(crate) fn normalize_reference_label(text: &str) -> Cow<'_, str> {
    if is_whitespace_normalized(text) {
        // Apply Unicode case folding for case-insensitive matching.
        return text.to_casefold_cow();
    }

    let mut out = String::new();
    let mut saw_whitespace = false;

    for c in text.chars() {
        if c.is_whitespace() {
            saw_whitespace = true;
        } else {
            if saw_whitespace && !out.is_empty() {
                out.push(' ');
            }
            saw_whitespace = false;
            out.push(c);
        }
    }

    let folded = out.as_str().to_casefold_cow();
    match folded {
        Cow::Borrowed(_) => Cow::Owned(out),
        Cow::Owned(folded) => Cow::Owned(folded),
    }
}

fn is_whitespace_normalized(text: &str) -> bool {
    let mut saw_non_whitespace = false;
    let mut last_was_space = false;

    for c in text.chars() {
        if c.is_whitespace() {
            if c != ' ' {
                return false;
            }
            if !saw_non_whitespace || last_was_space {
                return false;
            }
            last_was_space = true;
        } else {
            saw_non_whitespace = true;
            last_was_space = false;
        }
    }

    !last_was_space
}

#[cfg(test)]
mod tests {
    use crate::syntax::reference::normalize_reference_label;

    #[test]
    fn normalizes_whitespace_and_case() {
        assert_eq!(normalize_reference_label("  Foo\tBar  "), "foo bar");
        assert_eq!(normalize_reference_label("Foo   Bar Baz"), "foo bar baz");
    }

    #[test]
    fn preserves_backslash_escapes() {
        assert_eq!(normalize_reference_label(r"foo\!"), r"foo\!");
        assert_eq!(normalize_reference_label(r"Foo\! Bar"), r"foo\! bar");
    }

    #[test]
    fn avoids_allocation_for_normalized_labels() {
        let normalized = normalize_reference_label("foo bar");
        assert!(matches!(normalized, std::borrow::Cow::Borrowed(_)));
    }
}

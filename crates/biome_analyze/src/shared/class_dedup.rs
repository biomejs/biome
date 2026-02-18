use biome_console::markup;
use biome_diagnostics::Category;
use biome_diagnostics::location::AsSpan;
use rustc_hash::FxHashSet;

use crate::RuleDiagnostic;

/// Result of analyzing a class string for duplicates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassDedupResult<'a> {
    /// The deduplicated class string, preserving original whitespace.
    pub deduplicated: String,
    /// Duplicate class names as borrowed slices of the original input string,
    /// ordered by when each duplicate was first detected.
    ///
    /// For example, in `"a b b a"`, the result is `["b", "a"]` because the second "b"
    /// is encountered before the second "a".
    pub duplicates: Vec<&'a str>,
}

/// Creates a [`RuleDiagnostic`] for duplicate CSS classes.
///
/// Handles singular/plural messaging. For a single duplicate,
/// the class name is wrapped in `Emphasis` markup.
pub fn duplicate_classes_diagnostic(
    category: &'static Category,
    span: impl AsSpan,
    duplicates: &[Box<str>],
) -> RuleDiagnostic {
    let diagnostic = if duplicates.len() == 1 {
        RuleDiagnostic::new(
            category,
            span,
            markup! {
                "This class string contains a duplicate class."
            },
        )
        .note(markup! {
            "The class "<Emphasis>{&*duplicates[0]}</Emphasis>" appears multiple times."
        })
    } else {
        let duplicates_list = duplicates
            .iter()
            .map(|s| s.as_ref())
            .collect::<Vec<&str>>()
            .join(", ");

        RuleDiagnostic::new(
            category,
            span,
            markup! {
                "This class string contains duplicate classes."
            },
        )
        .note(markup! {
            "The classes "{duplicates_list}" appear multiple times."
        })
    };

    diagnostic.note(markup! {
        "Duplicate classes are redundant and can indicate copy-paste errors or merge conflicts."
    })
}

/// Analyzes a class string and returns deduplication info if duplicates exist.
///
/// Preserves original whitespace around kept classes.
/// Returns `None` if no duplicates are found.
///
/// # Examples
///
/// ```
/// use biome_analyze::shared::class_dedup::find_duplicate_classes;
///
/// // No duplicates returns None
/// assert!(find_duplicate_classes("foo bar baz").is_none());
///
/// // Duplicates are detected and removed
/// let result = find_duplicate_classes("foo bar foo").unwrap();
/// assert_eq!(result.deduplicated, "foo bar");
/// assert_eq!(result.duplicates, vec!["foo"]);
///
/// // Original whitespace is preserved
/// let result = find_duplicate_classes("  foo  bar  foo  ").unwrap();
/// assert_eq!(result.deduplicated, "  foo  bar  ");
/// ```
pub fn find_duplicate_classes(value_str: &str) -> Option<ClassDedupResult<'_>> {
    // Parse the class string into tokens, preserving whitespace positions.
    // Each token tracks: where its preceding whitespace starts, where the class ends,
    // and the class name itself.
    struct Token<'a> {
        prefix_start: usize,
        text_end: usize,
        class: &'a str,
    }

    let mut tokens: Vec<Token<'_>> = Vec::new();
    let mut pos = 0;

    while pos < value_str.len() {
        let prefix_start = pos;

        // Skip whitespace
        for c in value_str[pos..].chars() {
            if !c.is_whitespace() {
                break;
            }
            pos += c.len_utf8();
        }

        if pos >= value_str.len() {
            break;
        }

        let class_start = pos;

        // Read class name
        for c in value_str[pos..].chars() {
            if c.is_whitespace() {
                break;
            }
            pos += c.len_utf8();
        }

        tokens.push(Token {
            prefix_start,
            text_end: pos,
            class: &value_str[class_start..pos],
        });
    }

    // Identify duplicates and track which tokens to keep.
    // Use a Vec to track duplicates in order of detection for deterministic output,
    // plus a HashSet for O(1) dedup checking.
    let mut seen: FxHashSet<&str> = FxHashSet::default();
    let mut duplicate_set: FxHashSet<&str> = FxHashSet::default();
    let mut duplicates_in_order: Vec<&str> = Vec::new();
    let mut kept_indices: Vec<usize> = Vec::new();

    for (idx, token) in tokens.iter().enumerate() {
        if seen.contains(token.class) {
            // Only add to the ordered list if this is the first time we see it as a duplicate
            if duplicate_set.insert(token.class) {
                duplicates_in_order.push(token.class);
            }
        } else {
            seen.insert(token.class);
            kept_indices.push(idx);
        }
    }

    if duplicates_in_order.is_empty() {
        return None;
    }

    // Reconstruct the string, preserving original whitespace around kept classes
    let mut deduplicated = String::new();
    for &idx in &kept_indices {
        let token = &tokens[idx];
        deduplicated.push_str(&value_str[token.prefix_start..token.text_end]);
    }

    // Preserve trailing whitespace from the original string
    if let Some(last) = tokens.last() {
        deduplicated.push_str(&value_str[last.text_end..]);
    }

    Some(ClassDedupResult {
        deduplicated,
        duplicates: duplicates_in_order,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_duplicates() {
        assert!(find_duplicate_classes("foo bar baz").is_none());
    }

    #[test]
    fn simple_duplicate() {
        let result = find_duplicate_classes("foo bar foo").unwrap();
        assert_eq!(result.deduplicated, "foo bar");
        assert_eq!(result.duplicates, vec!["foo"]);
    }

    #[test]
    fn multiple_occurrences_of_same_class() {
        let result = find_duplicate_classes("foo foo foo").unwrap();
        assert_eq!(result.deduplicated, "foo");
        assert_eq!(result.duplicates, vec!["foo"]);
    }

    #[test]
    fn multiple_different_duplicates() {
        let result = find_duplicate_classes("a b a c b").unwrap();
        assert_eq!(result.deduplicated, "a b c");
        assert_eq!(result.duplicates, vec!["a", "b"]);
    }

    #[test]
    fn preserves_leading_whitespace() {
        let result = find_duplicate_classes("  foo bar foo").unwrap();
        assert_eq!(result.deduplicated, "  foo bar");
        assert_eq!(result.duplicates, vec!["foo"]);
    }

    #[test]
    fn preserves_trailing_whitespace() {
        let result = find_duplicate_classes("foo bar foo  ").unwrap();
        assert_eq!(result.deduplicated, "foo bar  ");
        assert_eq!(result.duplicates, vec!["foo"]);
    }

    #[test]
    fn preserves_internal_whitespace() {
        let result = find_duplicate_classes("foo  bar  foo").unwrap();
        assert_eq!(result.deduplicated, "foo  bar");
        assert_eq!(result.duplicates, vec!["foo"]);
    }

    #[test]
    fn preserves_tabs() {
        let result = find_duplicate_classes("foo\tbar\tfoo").unwrap();
        assert_eq!(result.deduplicated, "foo\tbar");
        assert_eq!(result.duplicates, vec!["foo"]);
    }

    #[test]
    fn preserves_newlines() {
        let result = find_duplicate_classes("foo\nbar\nfoo").unwrap();
        assert_eq!(result.deduplicated, "foo\nbar");
        assert_eq!(result.duplicates, vec!["foo"]);
    }

    #[test]
    fn preserves_mixed_whitespace() {
        let result = find_duplicate_classes("  foo\n\tbar  foo  ").unwrap();
        assert_eq!(result.deduplicated, "  foo\n\tbar  ");
        assert_eq!(result.duplicates, vec!["foo"]);
    }

    #[test]
    fn empty_string() {
        assert!(find_duplicate_classes("").is_none());
    }

    #[test]
    fn whitespace_only() {
        assert!(find_duplicate_classes("   \n\t  ").is_none());
    }

    #[test]
    fn single_class() {
        assert!(find_duplicate_classes("foo").is_none());
    }

    #[test]
    fn duplicate_at_start() {
        let result = find_duplicate_classes("foo foo bar").unwrap();
        assert_eq!(result.deduplicated, "foo bar");
        assert_eq!(result.duplicates, vec!["foo"]);
    }

    #[test]
    fn duplicate_at_end() {
        let result = find_duplicate_classes("foo bar bar").unwrap();
        assert_eq!(result.deduplicated, "foo bar");
        assert_eq!(result.duplicates, vec!["bar"]);
    }

    #[test]
    fn duplicates_ordered_by_detection() {
        // Duplicates are ordered by when they are first detected (second occurrence),
        // not by their first occurrence in the string.
        // Here "b" is detected as duplicate at position 2, "a" at position 3.
        let result = find_duplicate_classes("a b b a").unwrap();
        assert_eq!(result.deduplicated, "a b");
        assert_eq!(result.duplicates, vec!["b", "a"]);
    }

    #[test]
    fn realistic_tailwind_classes() {
        let result = find_duplicate_classes("flex items-center p-4 flex bg-white p-4").unwrap();
        assert_eq!(result.deduplicated, "flex items-center p-4 bg-white");
        assert_eq!(result.duplicates, vec!["flex", "p-4"]);
    }

    #[test]
    fn case_sensitive() {
        // "Foo" and "foo" are different classes
        assert!(find_duplicate_classes("Foo foo").is_none());
    }

    #[test]
    fn unicode_class_names() {
        // Test multi-byte UTF-8 characters to verify byte-index handling is correct
        let result = find_duplicate_classes("日本語 クラス 日本語").unwrap();
        assert_eq!(result.deduplicated, "日本語 クラス");
        assert_eq!(result.duplicates, vec!["日本語"]);
    }

    #[test]
    fn unicode_whitespace() {
        // Unicode whitespace (non-breaking space U+00A0, ideographic space U+3000)
        // is treated as a separator
        let result = find_duplicate_classes("foo\u{00A0}bar\u{3000}foo").unwrap();
        assert_eq!(result.deduplicated, "foo\u{00A0}bar");
        assert_eq!(result.duplicates, vec!["foo"]);
    }

    #[test]
    fn mixed_ascii_unicode() {
        let result = find_duplicate_classes("flex 日本語 p-4 日本語 flex").unwrap();
        assert_eq!(result.deduplicated, "flex 日本語 p-4");
        assert_eq!(result.duplicates, vec!["日本語", "flex"]);
    }

    #[test]
    fn classes_with_special_chars() {
        // Class names can contain hyphens, underscores, colons (Tailwind modifiers)
        let result = find_duplicate_classes("hover:bg-red-500 text-sm hover:bg-red-500").unwrap();
        assert_eq!(result.deduplicated, "hover:bg-red-500 text-sm");
        assert_eq!(result.duplicates, vec!["hover:bg-red-500"]);
    }
}

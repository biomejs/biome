use std::cmp::Ordering;

use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, SyntaxNodeText};
use biome_tailwind_syntax::{AnyTwCandidate, AnyTwFullCandidate, AnyTwValue, TwRoot};

use super::tailwind_preset_v4::{
    Branch, FUNCTIONAL_UTILITIES, KEYWORD_POOL, Negative, STATIC_UTILITIES,
};

/// Sort the candidates of a parsed Tailwind class list and return the joined,
/// space-separated result.
pub fn sort_class_list(root: &TwRoot) -> String {
    let candidates = root.candidates();
    let mut keyed: Vec<(SortKey, SyntaxNodeText)> = Vec::with_capacity(candidates.len());
    for candidate in candidates {
        let text = candidate.syntax().text_trimmed();
        let key = SortKey::from_candidate(&candidate);
        keyed.push((key, text));
    }

    // `Vec::sort_by` is stable, so Unknown-vs-Unknown comparisons returning
    // `Equal` keep input order, and Known entries with identical triples
    // also keep input order.
    keyed.sort_by(|a, b| compare(&a.0, &b.0));

    // Sort is in-place; total text length is unchanged. Pre-size the output
    // so chunked emission never re-allocates.
    let text_bytes: usize = keyed.iter().map(|(_, t)| usize::from(t.len())).sum();
    let separators = keyed.len().saturating_sub(1);
    let mut result = String::with_capacity(text_bytes + separators);
    for (_, text) in &keyed {
        if !result.is_empty() {
            result.push(' ');
        }
        text.for_each_chunk(|chunk| result.push_str(chunk));
    }
    result
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum SortKey {
    Unknown,
    Known {
        property_idx: u16,
        property_count: u8,
        registration_idx: u16,
    },
}

impl SortKey {
    /// Build a sort key from a parsed candidate. Returns `Unknown` for
    /// shapes we cannot yet place; each `// TODO:` below tags an input
    /// class awaiting follow-up implementation.
    fn from_candidate(candidate: &AnyTwFullCandidate) -> Self {
        let AnyTwFullCandidate::TwFullCandidate(node) = candidate else {
            return Self::Unknown;
        };

        // TODO: variant weight (`hover:`, `sm:`, `[&:hover]:`).
        if !node.variants().is_empty() {
            return Self::Unknown;
        }
        // TODO: important suffix (`flex!`).
        if node.excl_token().is_some() {
            return Self::Unknown;
        }

        let is_negative = node.negative_token().is_some();

        let Ok(inner) = node.candidate() else {
            return Self::Unknown;
        };
        match inner {
            // TODO: arbitrary CSS `[mask:none]` — needs to read property_token.
            AnyTwCandidate::TwArbitraryCandidate(_) => Self::Unknown,
            AnyTwCandidate::TwBogusCandidate(_) => Self::Unknown,

            AnyTwCandidate::TwStaticCandidate(s) => {
                let Ok(name) = s.base_token() else {
                    return Self::Unknown;
                };
                let Some(entry) = STATIC_UTILITIES.get(name.text_trimmed()) else {
                    return Self::Unknown;
                };
                let registration_idx = if is_negative {
                    let Some(neg) = entry.negative_registration_idx else {
                        return Self::Unknown;
                    };
                    neg
                } else {
                    entry.registration_idx
                };
                Self::Known {
                    property_idx: entry.property_idx,
                    property_count: entry.property_count,
                    registration_idx,
                }
            }

            AnyTwCandidate::TwFunctionalCandidate(f) => {
                // TODO: modifier (`bg-red-500/50`) — opacity/etc.
                if f.modifier().is_some() {
                    return Self::Unknown;
                }

                let Ok(base) = f.base_token() else {
                    return Self::Unknown;
                };
                let Some(entry) = FUNCTIONAL_UTILITIES.get(base.text_trimmed()) else {
                    return Self::Unknown;
                };

                let (registration_idx, branches) = if is_negative {
                    match entry.negative {
                        None => return Self::Unknown,
                        Some(Negative::SameBranches { registration_idx }) => {
                            (registration_idx, entry.branches)
                        }
                        Some(Negative::Distinct {
                            registration_idx,
                            branches,
                        }) => (registration_idx, branches),
                    }
                } else {
                    (entry.registration_idx, entry.branches)
                };

                let Ok(value) = f.value() else {
                    return Self::Unknown;
                };

                match value {
                    // TODO: ArbitraryTyped / Arbitrary branches (`bg-[#abc]`, `p-[10px]`).
                    AnyTwValue::TwArbitraryValue(_) => Self::Unknown,
                    // TODO: CSS variable values (`bg-(--my-color)`).
                    AnyTwValue::TwCssVariableValue(_) => Self::Unknown,
                    // TODO: data-attribute values inside utility (rare).
                    AnyTwValue::TwDataAttribute(_) => Self::Unknown,
                    AnyTwValue::TwBogusValue(_) => Self::Unknown,

                    AnyTwValue::TwNamedValue(named) => {
                        let Ok(value_token) = named.value_token() else {
                            return Self::Unknown;
                        };
                        resolve_branch(branches, value_token.text_trimmed(), registration_idx)
                            .unwrap_or(Self::Unknown)
                    }
                }
            }
        }
    }
}

fn compare(a: &SortKey, b: &SortKey) -> Ordering {
    match (*a, *b) {
        // Unknowns float to the front; relative order between unknowns is
        // preserved by the stable sort.
        (SortKey::Unknown, SortKey::Unknown) => Ordering::Equal,
        (SortKey::Unknown, SortKey::Known { .. }) => Ordering::Less,
        (SortKey::Known { .. }, SortKey::Unknown) => Ordering::Greater,
        (
            SortKey::Known {
                property_idx: p1,
                property_count: c1,
                registration_idx: r1,
            },
            SortKey::Known {
                property_idx: p2,
                property_count: c2,
                registration_idx: r2,
            },
        ) => p1
            .cmp(&p2)
            // Wider utilities (e.g. `sr-only` setting 9 properties) win
            // their property bucket so they sort before single-property
            // utilities in the same bucket.
            .then_with(|| c2.cmp(&c1))
            .then_with(|| r1.cmp(&r2)),
    }
}

/// Walk a basename's branch list and return the first matching branch as
/// a complete `SortKey::Known`. Branch order in the preset already
/// reflects the resolution precedence we want
/// (NamedKeyword → Named → NamedTyped → ArbitraryTyped → Arbitrary).
fn resolve_branch(branches: &[Branch], value: &str, registration_idx: u16) -> Option<SortKey> {
    for &branch in branches {
        let (property_idx, property_count) = match branch {
            Branch::Named(namespace, p, c) if namespace.keys().contains(value) => (p, c),
            Branch::NamedKeyword(pool_idx, p, c)
                if KEYWORD_POOL[usize::from(pool_idx)].contains(&value) =>
            {
                (p, c)
            }
            Branch::NamedTyped(value_type, p, c) if value_type.matches(value) => (p, c),
            // ArbitraryTyped / Arbitrary only fire for bracketed arbitrary values,
            // which the caller filters out before reaching this loop.
            _ => continue,
        };
        return Some(SortKey::Known {
            property_idx,
            property_count,
            registration_idx,
        });
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_tailwind_parser::parse_tailwind;

    use super::super::tailwind_preset_v4::ValueType;

    fn known(property_idx: u16, property_count: u8, registration_idx: u16) -> SortKey {
        SortKey::Known {
            property_idx,
            property_count,
            registration_idx,
        }
    }

    fn sort(input: &str) -> String {
        sort_class_list(&parse_tailwind(input).tree())
    }

    // region: compare

    #[test]
    fn compare_unknown_is_less_than_known() {
        assert_eq!(
            compare(&SortKey::Unknown, &known(5, 1, 0)),
            Ordering::Less
        );
        assert_eq!(
            compare(&known(5, 1, 0), &SortKey::Unknown),
            Ordering::Greater
        );
    }

    #[test]
    fn compare_unknown_pair_is_equal_so_stable_sort_keeps_input_order() {
        assert_eq!(
            compare(&SortKey::Unknown, &SortKey::Unknown),
            Ordering::Equal
        );
    }

    #[test]
    fn compare_orders_by_property_idx_ascending() {
        assert_eq!(compare(&known(3, 1, 0), &known(5, 1, 0)), Ordering::Less);
    }

    #[test]
    fn compare_breaks_property_idx_tie_by_property_count_descending() {
        // sr-only-shape utility (count=9) wins over a single-property one.
        let wider = known(5, 9, 100);
        let narrow = known(5, 1, 0);
        assert_eq!(compare(&wider, &narrow), Ordering::Less);
    }

    #[test]
    fn compare_breaks_full_tie_by_registration_idx_ascending() {
        let early = known(5, 1, 1);
        let late = known(5, 1, 9);
        assert_eq!(compare(&early, &late), Ordering::Less);
    }

    #[test]
    fn compare_returns_equal_for_identical_known_keys() {
        assert_eq!(compare(&known(5, 1, 0), &known(5, 1, 0)), Ordering::Equal);
    }

    // endregion: compare

    // region: resolve_branch

    #[test]
    fn resolve_branch_returns_first_matching_branch() {
        // Two NamedTyped(Number) branches with different property_idx;
        // first one to match wins.
        let branches = &[
            Branch::NamedTyped(ValueType::Number, 10, 1),
            Branch::NamedTyped(ValueType::Number, 20, 1),
        ];
        assert_eq!(
            resolve_branch(branches, "5", 99),
            Some(known(10, 1, 99))
        );
    }

    #[test]
    fn resolve_branch_skips_arbitrary_typed_for_bare_value() {
        // Only the bracketed code-path activates ArbitraryTyped / Arbitrary;
        // bare values fall through.
        let branches = &[
            Branch::ArbitraryTyped(ValueType::Number, 10, 1),
            Branch::Arbitrary(20, 1),
        ];
        assert_eq!(resolve_branch(branches, "5", 99), None);
    }

    #[test]
    fn resolve_branch_passes_registration_idx_through() {
        let branches = &[Branch::NamedTyped(ValueType::Number, 1, 1)];
        assert_eq!(
            resolve_branch(branches, "0", 42),
            Some(known(1, 1, 42))
        );
    }

    #[test]
    fn resolve_branch_returns_none_when_value_matches_no_branch() {
        let branches = &[Branch::NamedTyped(ValueType::Number, 1, 1)];
        // "abc" is not a number → no branch matches.
        assert_eq!(resolve_branch(branches, "abc", 0), None);
    }

    // endregion: resolve_branch

    // region: sort_class_list edge cases

    #[test]
    fn sort_returns_empty_for_empty_input() {
        assert_eq!(sort(""), "");
    }

    #[test]
    fn sort_returns_empty_for_whitespace_only_input() {
        assert_eq!(sort("   "), "");
    }

    // endregion: sort_class_list edge cases
}


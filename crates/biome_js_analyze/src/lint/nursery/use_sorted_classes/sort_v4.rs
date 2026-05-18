use std::cmp::Ordering;

use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, SyntaxNodeText};
use biome_tailwind_syntax::{
    AnyTwCandidate, AnyTwFullCandidate, AnyTwModifier, AnyTwValue, CssGenericComponentValueList,
    TwRoot,
};

use super::tailwind_preset_v4::{
    FUNCTIONAL_UTILITIES, KEYWORD_POOL, PROPERTY_INDEX, STATIC_UTILITIES,
};
use super::tailwind_preset_v4_types::{Branch, Negative, ValueType};
use super::value_match::value_matches_type;

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
            AnyTwCandidate::TwArbitraryCandidate(a) => {
                let Ok(property_token) = a.property_token() else {
                    return Self::Unknown;
                };
                let property_text = property_token.text_trimmed();
                let Some(&property_idx) = PROPERTY_INDEX.get(property_text) else {
                    return Self::Unknown;
                };
                Self::Known {
                    property_idx,
                    property_count: 1,
                    registration_idx: 0,
                }
            }
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

                resolve_functional_value(&value, f.modifier(), branches, registration_idx)
                    .unwrap_or(Self::Unknown)
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

/// Classified value for `resolve_branch`. Dispatching by parser node
/// kind keeps `NamedTyped(Number)` from matching `red-500` and
/// `NamedTyped(Ratio)` from firing without an `n/m` modifier.
enum ValueKind<'a> {
    Named(&'a str),
    Number(&'a str),
    Percentage,
    Ratio,
    Arbitrary(CssGenericComponentValueList),
}

/// Does this utility's branch list declare a `NamedTyped(Ratio)` slot?
/// Used as the gate for fraction-aliased modifiers (`w-1/2` ↔ `w-[50%]`) —
/// mirrors Tailwind's `supportsFractions` flag
fn entry_has_ratio_branch(branches: &[Branch]) -> bool {
    branches
        .iter()
        .any(|b| matches!(b, Branch::NamedTyped(ValueType::Ratio, _, _)))
}

/// `n/m` Tailwind fraction shorthand: the value is a bare number, the
/// modifier is a bare number, and the utility actually accepts fractions.
fn is_fraction_modifier(value: &AnyTwValue, modifier: &AnyTwModifier, branches: &[Branch]) -> bool {
    let AnyTwModifier::TwModifier(m) = modifier else {
        return false;
    };
    matches!(value, AnyTwValue::TwNumberValue(_))
        && matches!(m.value(), Ok(AnyTwValue::TwNumberValue(_)))
        && entry_has_ratio_branch(branches)
}

fn resolve_functional_value(
    value: &AnyTwValue,
    modifier: Option<AnyTwModifier>,
    branches: &[Branch],
    registration_idx: u16,
) -> Option<SortKey> {
    if let AnyTwValue::TwArbitraryValue(arb) = value {
        if modifier.is_some() {
            return None;
        }
        let kind = ValueKind::Arbitrary(arb.value());
        return resolve_branch(branches, &kind, registration_idx);
    }

    // Only `n/m` fraction modifiers resolve here; see `is_fraction_modifier`.
    // Color opacity, line-height, gradient interpolation, ... need modifier
    // metadata and stay out of this PR.
    let has_fraction_modifier = match modifier.as_ref() {
        None => false,
        Some(m) if is_fraction_modifier(value, m, branches) => true,
        Some(_) => return None,
    };

    let value_token = match value {
        AnyTwValue::TwNamedValue(n) => n.value_token(),
        AnyTwValue::TwNumberValue(n) => n.value_token(),
        AnyTwValue::TwPercentageValue(p) => p.value_token(),
        AnyTwValue::TwArbitraryValue(_) => return None,
        // TODO: CSS variable values (`bg-(--my-color)`).
        AnyTwValue::TwCssVariableValue(_) => return None,
        // TODO: data-attribute values inside utility (rare).
        AnyTwValue::TwDataAttribute(_) => return None,
        AnyTwValue::TwBogusValue(_) => return None,
    };
    let Ok(value_token) = value_token else {
        return None;
    };
    let value_text = value_token.text_trimmed();

    let kind = match (value, has_fraction_modifier) {
        (AnyTwValue::TwNamedValue(_), false) => ValueKind::Named(value_text),
        (AnyTwValue::TwNumberValue(_), false) => ValueKind::Number(value_text),
        (AnyTwValue::TwNumberValue(_), true) => ValueKind::Ratio,
        (AnyTwValue::TwPercentageValue(_), false) => ValueKind::Percentage,
        _ => return None,
    };

    resolve_branch(branches, &kind, registration_idx)
}

/// Walk a basename's branch list and return the first matching branch as
/// a complete `SortKey::Known`. Branch order in the preset already
/// reflects the resolution precedence we want
/// (NamedKeyword → Named → NamedTyped → ArbitraryTyped → Arbitrary).
fn resolve_branch(
    branches: &[Branch],
    kind: &ValueKind<'_>,
    registration_idx: u16,
) -> Option<SortKey> {
    for &branch in branches {
        let (property_idx, property_count) = match branch {
            // Theme-namespace lookup (`text-lg` ↔ `--text-lg`). Both Named
            // and Number kinds query it — users can register numeric
            // theme keys like `--spacing-12`.
            Branch::Named(namespace, p, c) => {
                let text = match kind {
                    ValueKind::Named(v) | ValueKind::Number(v) => *v,
                    ValueKind::Percentage | ValueKind::Ratio | ValueKind::Arbitrary(_) => continue,
                };
                if !namespace.keys().contains(text) {
                    continue;
                }
                (p, c)
            }
            // Hard-coded keyword pool (`origin-top`, `accent-current`).
            Branch::NamedKeyword(pool_idx, p, c) => {
                let ValueKind::Named(text) = kind else {
                    continue;
                };
                if !KEYWORD_POOL[usize::from(pool_idx)].contains(text) {
                    continue;
                }
                (p, c)
            }
            Branch::NamedTyped(value_type, p, c) => {
                let matched = matches!(
                    (value_type, kind),
                    (ValueType::Number, ValueKind::Number(_))
                        | (ValueType::Percentage, ValueKind::Percentage)
                        | (ValueType::Ratio, ValueKind::Ratio)
                );
                if !matched {
                    continue;
                }
                (p, c)
            }
            Branch::ArbitraryTyped(b_vt, p, c) => {
                let ValueKind::Arbitrary(list) = kind else {
                    continue;
                };
                if !value_matches_type(list, b_vt) {
                    continue;
                }
                (p, c)
            }
            Branch::Arbitrary(p, c) => {
                let ValueKind::Arbitrary(_) = kind else {
                    continue;
                };
                (p, c)
            }
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
        assert_eq!(compare(&SortKey::Unknown, &known(5, 1, 0)), Ordering::Less);
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
            resolve_branch(branches, &ValueKind::Number("5"), 99),
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
        assert_eq!(resolve_branch(branches, &ValueKind::Number("5"), 99), None);
    }

    #[test]
    fn resolve_branch_skips_arbitrary_typed_when_matcher_returns_false_then_falls_back() {
        let branches = &[
            Branch::ArbitraryTyped(ValueType::Number, 10, 1),
            Branch::Arbitrary(20, 1),
        ];
        let full = parse_tailwind("p-[10px]").tree().candidates().iter().next().unwrap();
        let full = full.as_tw_full_candidate().unwrap();
        let candidate = full.candidate().unwrap();
        let AnyTwCandidate::TwFunctionalCandidate(functional) = candidate else {
            panic!("expected functional candidate")
        };
        let AnyTwValue::TwArbitraryValue(arbitrary) = functional.value().unwrap() else {
            panic!("expected arbitrary value")
        };
        let kind = ValueKind::Arbitrary(arbitrary.value());
        assert_eq!(resolve_branch(branches, &kind, 99), Some(known(20, 1, 99)));
    }

    #[test]
    fn resolve_branch_passes_registration_idx_through() {
        let branches = &[Branch::NamedTyped(ValueType::Number, 1, 1)];
        assert_eq!(
            resolve_branch(branches, &ValueKind::Number("0"), 42),
            Some(known(1, 1, 42))
        );
    }

    #[test]
    fn resolve_branch_returns_none_when_kind_does_not_match_value_type() {
        // A NamedValue text like "abc" never satisfies NamedTyped(Number)
        // because dispatch is by parser node kind, not text scanning.
        let branches = &[Branch::NamedTyped(ValueType::Number, 1, 1)];
        assert_eq!(resolve_branch(branches, &ValueKind::Named("abc"), 0), None);
    }

    #[test]
    fn resolve_branch_ratio_matches_ratio_typed_branch() {
        let branches = &[Branch::NamedTyped(ValueType::Ratio, 7, 1)];
        assert_eq!(
            resolve_branch(branches, &ValueKind::Ratio, 11),
            Some(known(7, 1, 11))
        );
    }

    #[test]
    fn resolve_branch_percentage_only_matches_percentage_typed_branch() {
        let branches = &[
            Branch::NamedTyped(ValueType::Number, 1, 1),
            Branch::NamedTyped(ValueType::Percentage, 2, 1),
        ];
        assert_eq!(
            resolve_branch(branches, &ValueKind::Percentage, 0),
            Some(known(2, 1, 0))
        );
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

    #[test]
    fn sort_routes_arbitrary_values_to_functional_arbitrary_fallback() {
        assert_eq!(sort("p-[10px] flex some-unknown"), "some-unknown flex p-[10px]");
    }

    #[test]
    fn arbitrary_candidate_sorts_by_inner_property() {
        assert_eq!(sort("flex [color:red]"), "flex [color:red]");
    }

    #[test]
    fn arbitrary_candidate_with_unknown_property_is_unknown() {
        assert_eq!(sort("flex [--my-var:1]"), "[--my-var:1] flex");
    }

    #[test]
    fn arbitrary_candidate_with_modifier_sorts_by_inner_property() {
        assert_eq!(sort("flex [color:red]/50"), "flex [color:red]/50");
    }

    #[test]
    fn arbitrary_candidate_uses_registration_idx_zero() {
        let parsed = parse_tailwind("[display:block]");
        let full = parsed.tree().candidates().iter().next().unwrap();
        let display_idx = *PROPERTY_INDEX.get("display").unwrap();
        assert_eq!(SortKey::from_candidate(&full), known(display_idx, 1, 0));
    }

    // endregion: sort_class_list edge cases
}

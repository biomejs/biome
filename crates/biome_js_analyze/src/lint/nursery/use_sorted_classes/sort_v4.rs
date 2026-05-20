use std::cmp::Ordering;

use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, SyntaxNodeText, TokenText};
use biome_tailwind_syntax::{
    AnyTwCandidate, AnyTwFullCandidate, AnyTwModifier, AnyTwValue, CssGenericComponentValueList,
    TwRoot,
};

use super::tailwind_preset_v4::{
    FUNCTIONAL_UTILITIES, KEYWORD_POOL, PROPERTY_INDEX, STATIC_UTILITIES,
};
use super::tailwind_preset_v4_types::{
    ArbitraryBranch, NamedBranch, NamedValueType, Negative,
};
use super::arbitrary_value_match::value_matches_type;

#[cfg(test)]
use super::tailwind_preset_v4_types::CssDataType;

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

                let (registration_idx, named_branches, arbitrary_branches) = if is_negative {
                    match entry.negative {
                        None => return Self::Unknown,
                        Some(Negative::SameBranches { registration_idx }) => (
                            registration_idx,
                            entry.named_branches,
                            entry.arbitrary_branches,
                        ),
                        Some(Negative::Distinct {
                            registration_idx,
                            named_branches,
                            arbitrary_branches,
                        }) => (registration_idx, named_branches, arbitrary_branches),
                    }
                } else {
                    (
                        entry.registration_idx,
                        entry.named_branches,
                        entry.arbitrary_branches,
                    )
                };

                let Ok(value) = f.value() else {
                    return Self::Unknown;
                };

                if let AnyTwValue::TwArbitraryValue(arb) = &value {
                    return resolve_arbitrary_branch(arbitrary_branches, &arb.value(), registration_idx)
                        .unwrap_or(Self::Unknown);
                }

                let modifier = f.modifier();
                resolve_named_branch(
                    named_branches,
                    &value,
                    modifier.as_ref(),
                    registration_idx,
                )
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

/// Does this utility's branch list declare a `NamedBranch::Typed(Ratio)` slot?
/// Used as the gate for fraction-aliased modifiers (`w-1/2` ↔ `w-[50%]`) —
/// mirrors Tailwind's `supportsFractions` flag
fn entry_has_ratio_branch(branches: &[NamedBranch]) -> bool {
    branches
        .iter()
        .any(|b| matches!(b, NamedBranch::Typed(NamedValueType::Ratio, _, _)))
}

/// `n/m` Tailwind fraction shorthand: the value is a bare number, the
/// modifier is a bare number, and the utility actually accepts fractions.
fn is_fraction_modifier(
    value: &AnyTwValue,
    modifier: &AnyTwModifier,
    branches: &[NamedBranch],
) -> bool {
    let AnyTwModifier::TwModifier(m) = modifier else {
        return false;
    };
    matches!(value, AnyTwValue::TwNumberValue(_))
        && matches!(m.value(), Ok(AnyTwValue::TwNumberValue(_)))
        && entry_has_ratio_branch(branches)
}

fn named_text(value: &AnyTwValue) -> Option<TokenText> {
    let AnyTwValue::TwNamedValue(named) = value else {
        return None;
    };
    named
        .value_token()
        .ok()
        .map(|token| token.token_text_trimmed())
}

fn named_or_number_text(value: &AnyTwValue) -> Option<TokenText> {
    match value {
        AnyTwValue::TwNamedValue(named) => named
            .value_token()
            .ok()
            .map(|token| token.token_text_trimmed()),
        AnyTwValue::TwNumberValue(number) => number
            .value_token()
            .ok()
            .map(|token| token.token_text_trimmed()),
        _ => None,
    }
}

fn named_value_type_matches(
    value_type: NamedValueType,
    value: &AnyTwValue,
    has_fraction_modifier: bool,
) -> bool {
    matches!(
        (value_type, value, has_fraction_modifier),
        (NamedValueType::Number, AnyTwValue::TwNumberValue(_), false)
            | (
                NamedValueType::Percentage,
                AnyTwValue::TwPercentageValue(_),
                false
            )
            | (NamedValueType::Ratio, AnyTwValue::TwNumberValue(_), true)
    )
}

/// Walk a basename's named branch list and return the first matching
/// branch as a complete `SortKey::Known`. Branch order in the preset
/// already reflects the resolution precedence we want
/// (Keyword → Theme → Typed).
fn resolve_named_branch(
    branches: &[NamedBranch],
    value: &AnyTwValue,
    modifier: Option<&AnyTwModifier>,
    registration_idx: u16,
) -> Option<SortKey> {
    let has_fraction_modifier = match modifier {
        None => false,
        Some(m) if is_fraction_modifier(value, m, branches) => true,
        Some(_) => return None,
    };

    for &branch in branches {
        let (property_idx, property_count) = match branch {
            // Theme-namespace lookup (`text-lg` ↔ `--text-lg`). Both Named
            // and Number kinds query it — users can register numeric
            // theme keys like `--spacing-12`.
            NamedBranch::Theme(namespace, p, c) => {
                if has_fraction_modifier {
                    continue;
                }
                let Some(text) = named_or_number_text(value) else {
                    continue;
                };
                if !namespace.keys().contains(text.text()) {
                    continue;
                }
                (p, c)
            }
            // Hard-coded keyword pool (`origin-top`, `accent-current`).
            NamedBranch::Keyword(pool_idx, p, c) => {
                let Some(text) = named_text(value) else {
                    continue;
                };
                if !KEYWORD_POOL[usize::from(pool_idx)].contains(&text.text()) {
                    continue;
                }
                (p, c)
            }
            NamedBranch::Typed(value_type, p, c) => {
                if !named_value_type_matches(value_type, value, has_fraction_modifier) {
                    continue;
                }
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

/// Walk a basename's arbitrary branch list and return the first matching
/// branch as a complete `SortKey::Known`. Typed branches precede the
/// type-blind fallback in generated preset order.
fn resolve_arbitrary_branch(
    branches: &[ArbitraryBranch],
    list: &CssGenericComponentValueList,
    registration_idx: u16,
) -> Option<SortKey> {
    for &branch in branches {
        let (property_idx, property_count) = match branch {
            ArbitraryBranch::Typed(value_type, p, c) => {
                if !value_matches_type(list, value_type) {
                    continue;
                }
                (p, c)
            }
            ArbitraryBranch::Fallback(p, c) => (p, c),
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

    fn functional_parts(input: &str) -> (AnyTwValue, Option<AnyTwModifier>) {
        let parsed = parse_tailwind(input);
        let full = parsed.tree().candidates().iter().next().unwrap();
        let full = full.as_tw_full_candidate().unwrap();
        let candidate = full.candidate().unwrap();
        let AnyTwCandidate::TwFunctionalCandidate(functional) = candidate else {
            panic!("expected functional candidate")
        };
        (functional.value().unwrap(), functional.modifier())
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

    // region: branch resolution

    #[test]
    fn resolve_named_branch_returns_first_matching_branch() {
        // Two NamedBranch::Typed(Number) branches with different property_idx;
        // first one to match wins.
        let (value, modifier) = functional_parts("p-5");
        let branches = &[
            NamedBranch::Typed(NamedValueType::Number, 10, 1),
            NamedBranch::Typed(NamedValueType::Number, 20, 1),
        ];
        assert_eq!(
            resolve_named_branch(branches, &value, modifier.as_ref(), 99),
            Some(known(10, 1, 99))
        );
    }

    #[test]
    fn resolve_named_branch_classifies_value_internally() {
        let (value, modifier) = functional_parts("p-5");
        let branches = &[NamedBranch::Typed(NamedValueType::Number, 10, 1)];

        assert_eq!(
            resolve_named_branch(branches, &value, modifier.as_ref(), 99),
            Some(known(10, 1, 99))
        );
    }

    #[test]
    fn resolve_arbitrary_branch_skips_typed_when_matcher_returns_false_then_falls_back() {
        let branches = &[
            ArbitraryBranch::Typed(CssDataType::Number, 10, 1),
            ArbitraryBranch::Fallback(20, 1),
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
        assert_eq!(
            resolve_arbitrary_branch(branches, &arbitrary.value(), 99),
            Some(known(20, 1, 99))
        );
    }

    #[test]
    fn resolve_arbitrary_branch_skips_named_branches_by_construction() {
        let full = parse_tailwind("p-[10px]")
            .tree()
            .candidates()
            .iter()
            .next()
            .unwrap();
        let full = full.as_tw_full_candidate().unwrap();
        let candidate = full.candidate().unwrap();
        let AnyTwCandidate::TwFunctionalCandidate(functional) = candidate else {
            panic!("expected functional candidate")
        };
        let AnyTwValue::TwArbitraryValue(arbitrary) = functional.value().unwrap() else {
            panic!("expected arbitrary value")
        };
        let branches = &[ArbitraryBranch::Fallback(20, 1)];
        assert_eq!(
            resolve_arbitrary_branch(branches, &arbitrary.value(), 99),
            Some(known(20, 1, 99))
        );
    }

    #[test]
    fn resolve_named_branch_passes_registration_idx_through() {
        let (value, modifier) = functional_parts("p-0");
        let branches = &[NamedBranch::Typed(NamedValueType::Number, 1, 1)];
        assert_eq!(
            resolve_named_branch(branches, &value, modifier.as_ref(), 42),
            Some(known(1, 1, 42))
        );
    }

    #[test]
    fn resolve_named_branch_returns_none_when_kind_does_not_match_value_type() {
        // A named value like "abc" never satisfies NamedBranch::Typed(Number)
        // because dispatch is by parser node kind, not text scanning.
        let (value, modifier) = functional_parts("p-abc");
        let branches = &[NamedBranch::Typed(NamedValueType::Number, 1, 1)];
        assert_eq!(
            resolve_named_branch(branches, &value, modifier.as_ref(), 0),
            None
        );
    }

    #[test]
    fn resolve_named_branch_ratio_matches_ratio_typed_branch() {
        let (value, modifier) = functional_parts("w-1/2");
        let branches = &[NamedBranch::Typed(NamedValueType::Ratio, 7, 1)];
        assert_eq!(
            resolve_named_branch(branches, &value, modifier.as_ref(), 11),
            Some(known(7, 1, 11))
        );
    }

    #[test]
    fn resolve_named_branch_percentage_only_matches_percentage_typed_branch() {
        let (value, modifier) = functional_parts("from-25%");
        let branches = &[
            NamedBranch::Typed(NamedValueType::Number, 1, 1),
            NamedBranch::Typed(NamedValueType::Percentage, 2, 1),
        ];
        assert_eq!(
            resolve_named_branch(branches, &value, modifier.as_ref(), 0),
            Some(known(2, 1, 0))
        );
    }

    // endregion: branch resolution

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
    fn functional_arbitrary_modifier_does_not_change_sort_key() {
        assert_eq!(
            sort("ring-[#000]/50 text-[20px]/8 border-[#f00]/50 flex bg-[#fff]/50"),
            "flex border-[#f00]/50 bg-[#fff]/50 text-[20px]/8 ring-[#000]/50"
        );
    }

    #[test]
    fn arbitrary_candidate_uses_registration_idx_zero() {
        let parsed = parse_tailwind("[display:block]");
        let full = parsed.tree().candidates().iter().next().unwrap();
        let display_idx = *PROPERTY_INDEX.get("display").unwrap();
        assert_eq!(SortKey::from_candidate(&full), known(display_idx, 1, 0));
    }

    #[test]
    fn sort_routes_arbitrary_typed_background_values_to_their_css_properties() {
        assert_eq!(
            sort("bg-[50%] bg-[url('/a.png')] bg-[cover] bg-[#fff]"),
            "bg-[#fff] bg-[url('/a.png')] bg-[cover] bg-[50%]"
        );
    }

    #[test]
    fn sort_routes_arbitrary_typed_border_width_before_color() {
        assert_eq!(sort("border-[#f00] border-[2px]"), "border-[2px] border-[#f00]");
    }

    #[test]
    fn sort_routes_arbitrary_typed_text_size_before_color() {
        assert_eq!(
            sort("text-[#fff] text-[larger] text-[20px]"),
            "text-[larger] text-[20px] text-[#fff]"
        );
    }

    #[test]
    fn sort_handles_realistic_arbitrary_value_mix() {
        assert_eq!(
            sort("ring-[4px] p-[calc(100%-1rem)] bg-[cover] shadow-[#000] [mask-type:luminance] text-[20px] from-[20%] bg-[url('/hero.png')] border-[2px] flex"),
            "flex border-[2px] bg-[url('/hero.png')] from-[20%] bg-[cover] [mask-type:luminance] p-[calc(100%-1rem)] text-[20px] shadow-[#000] ring-[4px]"
        );
    }

    // endregion: sort_class_list edge cases
}

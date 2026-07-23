use std::cmp::Ordering;

use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, SyntaxNodeText, TokenText};
use biome_string_case::Collator;
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
use super::tailwind_preset_v4_types::{CssDataType, ThemeNamespace};

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

#[derive(Clone, Debug, Eq, PartialEq)]
enum SortKey {
    Unknown,
    Known {
        property_idx: u16,
        property_count: u8,
        registration_idx: u16,
        value: ValueKey,
        important: bool,
    },
}

/// The value and modifier text of a candidate — `red-500` + `50` for
/// `bg-red-500/50`, `[13px]` + nothing for `w-[13px]`, `1` + `2` for the
/// fraction `w-1/2` — held as allocation-free views into the syntax
/// tree. Candidates with an identical (property, registration) placement
/// order by natural comparison of these texts, mirroring how Tailwind
/// orders same-utility candidates in generated CSS.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct ValueKey {
    value: Option<SyntaxNodeText>,
    modifier: Option<SyntaxNodeText>,
}

impl ValueKey {
    fn from_candidate(candidate: &AnyTwCandidate) -> Self {
        match candidate {
            AnyTwCandidate::TwArbitraryCandidate(arbitrary) => Self {
                value: Some(arbitrary.value().syntax().text_trimmed()),
                modifier: modifier_text(arbitrary.modifier().as_ref()),
            },
            AnyTwCandidate::TwFunctionalCandidate(functional) => Self {
                value: functional
                    .value()
                    .ok()
                    .map(|value| value.syntax().text_trimmed()),
                modifier: modifier_text(functional.modifier().as_ref()),
            },
            // Static candidates have no value; ties between distinct
            // static utilities are broken by `registration_idx`.
            AnyTwCandidate::TwStaticCandidate(_) | AnyTwCandidate::TwBogusCandidate(_) => {
                Self::default()
            }
        }
    }

    fn compare(&self, other: &Self) -> Ordering {
        natural_cmp(self.value.as_ref(), other.value.as_ref())
            .then_with(|| natural_cmp(self.modifier.as_ref(), other.modifier.as_ref()))
    }
}

fn modifier_text(modifier: Option<&AnyTwModifier>) -> Option<SyntaxNodeText> {
    match modifier {
        Some(AnyTwModifier::TwModifier(modifier)) => modifier
            .value()
            .ok()
            .map(|value| value.syntax().text_trimmed()),
        Some(AnyTwModifier::TwBogusModifier(bogus)) => Some(bogus.syntax().text_trimmed()),
        None => None,
    }
}

/// Orders candidate value text the way Tailwind's own `compare()` does:
/// plain code-point order, except that digit sequences compare as
/// integers (`75` < `700`, `red-50` < `red-100`). Code-point order
/// places digits before `[` and `[` before letters
/// (`4` < `[1px]` < `auto`), matching the order Tailwind emits
/// same-utility candidates in — which is why this does not reuse
/// [biome_string_case::CldrAsciiCollator]: CLDR collation places
/// punctuation before digits and interleaves letter case.
struct TwValueCollator;

impl Collator for TwValueCollator {
    type Char = char;

    fn weight(&self, c: &char) -> impl Ord {
        *c
    }

    fn as_digit(&self, c: &char) -> Option<impl Ord> {
        c.is_ascii_digit().then_some(*c)
    }
}

/// Natural comparison of two optional text views without materializing
/// either side; a missing text compares as empty, so a bare value
/// precedes any longer one (`in` < `in-out`).
fn natural_cmp(a: Option<&SyntaxNodeText>, b: Option<&SyntaxNodeText>) -> Ordering {
    let chars = |text: Option<&SyntaxNodeText>| text.map(|text| text.chars()).into_iter().flatten();
    TwValueCollator.cmp(chars(a), chars(b))
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

        let is_negative = node.negative_token().is_some();
        // An important candidate (`flex!`) sorts exactly where its plain
        // twin does; `compare` breaks exact-key ties plain-first.
        let is_important = node.excl_token().is_some();

        let Ok(inner) = node.candidate() else {
            return Self::Unknown;
        };
        let value = ValueKey::from_candidate(&inner);
        let base = match inner {
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
                    value: ValueKey::default(),
                    important: false,
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
                    value: ValueKey::default(),
                    important: false,
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
                    resolve_arbitrary_branch(arbitrary_branches, &arb.value(), registration_idx)
                        .unwrap_or(Self::Unknown)
                } else {
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
        };

        match base {
            Self::Unknown => Self::Unknown,
            Self::Known {
                property_idx,
                property_count,
                registration_idx,
                ..
            } => Self::Known {
                property_idx,
                property_count,
                registration_idx,
                value,
                important: is_important,
            },
        }
    }
}

fn compare(a: &SortKey, b: &SortKey) -> Ordering {
    match (a, b) {
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
                value: v1,
                important: i1,
            },
            SortKey::Known {
                property_idx: p2,
                property_count: c2,
                registration_idx: r2,
                value: v2,
                important: i2,
            },
        ) => p1
            .cmp(p2)
            // Wider utilities (e.g. `sr-only` setting 9 properties) win
            // their property bucket so they sort before single-property
            // utilities in the same bucket.
            .then_with(|| c2.cmp(c1))
            .then_with(|| r1.cmp(r2))
            // Same-utility candidates order by their value (`p-2 p-4
            // p-10`, `bg-red-50 bg-red-100`), then modifier
            // (`bg-red-500/25 bg-red-500/50`).
            .then_with(|| v1.compare(v2))
            // A plain utility precedes its important twin (`flex flex!`).
            .then_with(|| i1.cmp(i2)),
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
/// branch as a `SortKey::Known` (value and importance are stamped by
/// `SortKey::from_candidate`). Branch order in the preset
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
        Some(AnyTwModifier::TwModifier(_)) => false,
        Some(AnyTwModifier::TwBogusModifier(_)) => return None,
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
            value: ValueKey::default(),
            important: false,
        });
    }
    None
}

/// Walk a basename's arbitrary branch list and return the first matching
/// branch as a `SortKey::Known` (value and importance are stamped by
/// `SortKey::from_candidate`). Typed branches precede the
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
            value: ValueKey::default(),
            important: false,
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
            value: ValueKey::default(),
            important: false,
        }
    }

    /// The value and modifier text of a known key, materialized for
    /// assertion messages; a missing text reads as empty.
    fn value_texts(key: &SortKey) -> (String, String) {
        let SortKey::Known { value, .. } = key else {
            panic!("expected a known key");
        };
        let text = |text: Option<&SyntaxNodeText>| {
            text.map(ToString::to_string).unwrap_or_default()
        };
        (text(value.value.as_ref()), text(value.modifier.as_ref()))
    }

    fn nat_cmp(a: &str, b: &str) -> Ordering {
        TwValueCollator.cmp(a.chars(), b.chars())
    }

    fn classify(input: &str) -> SortKey {
        let parsed = parse_tailwind(input);
        let full = parsed.tree().candidates().iter().next().unwrap();
        SortKey::from_candidate(&full)
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

    #[test]
    fn compare_breaks_exact_key_tie_plain_before_important() {
        let plain = known(5, 1, 0);
        let important = SortKey::Known {
            property_idx: 5,
            property_count: 1,
            registration_idx: 0,
            value: ValueKey::default(),
            important: true,
        };
        assert_eq!(compare(&plain, &important), Ordering::Less);
        assert_eq!(compare(&important, &plain), Ordering::Greater);
    }

    #[test]
    fn compare_breaks_registration_tie_by_value_before_importance() {
        // `p-2! p-4`: the value decides, the important suffix does not
        // pull `p-4` ahead of `p-2!`.
        let important_two = classify("p-2!");
        let plain_four = classify("p-4");
        assert_eq!(compare(&important_two, &plain_four), Ordering::Less);
    }

    #[test]
    fn compare_breaks_value_tie_by_modifier() {
        let twenty_five = classify("bg-red-500/25");
        let fifty = classify("bg-red-500/50");
        assert_eq!(compare(&twenty_five, &fifty), Ordering::Less);
        assert_eq!(compare(&fifty, &fifty), Ordering::Equal);
    }

    // endregion: compare

    // region: natural comparison

    #[test]
    fn collator_compares_digit_sequences_numerically() {
        assert_eq!(nat_cmp("2", "10"), Ordering::Less);
        assert_eq!(nat_cmp("red-50", "red-100"), Ordering::Less);
        // The digit sequence is compared as a whole number, not from the
        // point of divergence: `75` < `700` even though `5` > `0`.
        assert_eq!(nat_cmp("75", "700"), Ordering::Less);
        assert_eq!(nat_cmp("[2px]", "[10rem]"), Ordering::Less);
    }

    #[test]
    fn collator_uses_code_point_order_outside_digit_sequences() {
        // Digits precede `[`, and `[` precedes letters.
        assert_eq!(nat_cmp("4", "[1px]"), Ordering::Less);
        assert_eq!(nat_cmp("[13px]", "auto"), Ordering::Less);
        assert_eq!(nat_cmp("2xl", "base"), Ordering::Less);
        assert_eq!(nat_cmp("bold", "light"), Ordering::Less);
    }

    #[test]
    fn collator_puts_prefixes_first() {
        assert_eq!(nat_cmp("", "sm"), Ordering::Less);
        assert_eq!(nat_cmp("in", "in-out"), Ordering::Less);
        assert_eq!(nat_cmp("1", "1.5"), Ordering::Less);
        assert_eq!(nat_cmp("sm", "sm"), Ordering::Equal);
    }

    // endregion: natural comparison

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

    #[test]
    fn resolve_named_branch_ignores_non_fraction_modifier() {
        let (value, modifier) = functional_parts("bg-red-500/50");
        let branches = &[NamedBranch::Theme(ThemeNamespace::Color, 10, 1)];

        assert_eq!(
            resolve_named_branch(branches, &value, modifier.as_ref(), 99),
            Some(known(10, 1, 99))
        );
    }

    // endregion: branch resolution

    // region: sort key classification

    #[test]
    fn arbitrary_candidate_uses_registration_idx_zero() {
        let parsed = parse_tailwind("[display:block]");
        let full = parsed.tree().candidates().iter().next().unwrap();
        let display_idx = *PROPERTY_INDEX.get("display").unwrap();
        let key = SortKey::from_candidate(&full);
        let SortKey::Known {
            property_idx,
            property_count,
            registration_idx,
            important: false,
            ..
        } = &key
        else {
            panic!("expected a plain known key");
        };
        assert_eq!(*property_idx, display_idx);
        assert_eq!(*property_count, 1);
        assert_eq!(*registration_idx, 0);
        assert_eq!(value_texts(&key), ("block".to_string(), String::new()));
    }

    #[test]
    fn important_suffix_is_position_neutral_in_the_key() {
        let SortKey::Known {
            property_idx,
            property_count,
            registration_idx,
            value,
            important: false,
        } = classify("flex")
        else {
            panic!("expected `flex` to classify as a plain known key");
        };
        assert_eq!(
            classify("flex!"),
            SortKey::Known {
                property_idx,
                property_count,
                registration_idx,
                value,
                important: true,
            }
        );
    }

    #[test]
    fn important_suffix_classifies_functional_and_arbitrary_candidates() {
        assert!(matches!(
            classify("p-4!"),
            SortKey::Known {
                important: true,
                ..
            }
        ));
        assert!(matches!(
            classify("[display:block]!"),
            SortKey::Known {
                important: true,
                ..
            }
        ));
    }

    #[test]
    fn important_with_variants_is_still_unknown() {
        // Variant weight is the remaining TODO; `!` must not bypass it.
        assert_eq!(classify("hover:flex!"), SortKey::Unknown);
    }

    #[test]
    fn classification_captures_value_and_modifier_text() {
        let texts_of = |input: &str| value_texts(&classify(input));
        let pair = |value: &str, modifier: &str| (value.to_string(), modifier.to_string());
        assert_eq!(texts_of("p-4"), pair("4", ""));
        // The parser splits a fraction into value and modifier.
        assert_eq!(texts_of("w-1/2"), pair("1", "2"));
        assert_eq!(texts_of("bg-red-500/50"), pair("red-500", "50"));
        // Arbitrary values keep their brackets; arbitrary properties
        // contribute their value text.
        assert_eq!(texts_of("w-[13px]"), pair("[13px]", ""));
        assert_eq!(texts_of("[color:red]/50"), pair("red", "50"));
        // Static utilities have no value.
        assert_eq!(texts_of("flex"), pair("", ""));
    }

    // endregion: sort key classification
}

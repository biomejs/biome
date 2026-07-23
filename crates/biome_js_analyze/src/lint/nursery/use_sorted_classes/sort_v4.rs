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
    ArbitraryBranch, NamedBranch, NamedValueType, Negative, UtilityEntry,
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
    // `Equal` keep input order, and Known entries with identical keys
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
        name: NameKey,
        important: bool,
    },
}

/// The candidate's name — the negative sign plus the `base-value/modifier`
/// text, without variants or the important suffix — held as an
/// allocation-free view into the syntax tree. Candidates inside one
/// (property, count) bucket order by natural comparison of this name,
/// which is the order Tailwind emits utilities in: `getClassOrder` ranks
/// `m-2 m-4 m-auto m-px` and `block flow-root inline-block` by name, not
/// by utility registration order.
// TODO: the derived equality compares text chunk-wise and cannot
// short-circuit on syntax kind mismatches; switch to a structural node
// comparison once a generic `is_node_equal` is available.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct NameKey {
    negative: bool,
    text: Option<SyntaxNodeText>,
}

impl NameKey {
    fn compare(&self, other: &Self) -> Ordering {
        fn chars(key: &NameKey) -> impl Iterator<Item = char> + '_ {
            let sign = key.negative.then_some('-');
            let text = key.text.as_ref().map(|text| text.chars()).into_iter().flatten();
            sign.into_iter().chain(text)
        }
        TwNameCollator.cmp(chars(self), chars(other))
    }
}

/// Orders candidate names the way Tailwind's own `compare()` does:
/// plain code-point order, except that digit sequences compare as
/// integers (`p-75` < `p-700`, `red-50` < `red-100`). Code-point order
/// places `-` before digits, digits before `[`, and `[` before letters
/// (`-m-4` < `m-4`, `w-4` < `w-[1px]` < `w-auto`), matching the order
/// Tailwind emits candidates in — which is why this does not reuse
/// [biome_string_case::CldrAsciiCollator]: CLDR collation places
/// punctuation before digits and interleaves letter case.
struct TwNameCollator;

impl Collator for TwNameCollator {
    type Char = char;

    fn weight(&self, c: &char) -> impl Ord {
        *c
    }

    fn as_digit(&self, c: &char) -> Option<impl Ord> {
        c.is_ascii_digit().then_some(*c)
    }
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
        let placement = match &inner {
            AnyTwCandidate::TwArbitraryCandidate(a) => {
                let Ok(property_token) = a.property_token() else {
                    return Self::Unknown;
                };
                PROPERTY_INDEX
                    .get(property_token.text_trimmed())
                    .map(|&property_idx| (property_idx, 1))
            }
            AnyTwCandidate::TwBogusCandidate(_) => None,

            AnyTwCandidate::TwStaticCandidate(s) => {
                let Ok(name) = s.base_token() else {
                    return Self::Unknown;
                };
                STATIC_UTILITIES
                    .get(name.text_trimmed())
                    // Tailwind registers negative statics individually
                    // (`-m-px` exists, `-flex` does not).
                    .filter(|entry| !is_negative || entry.has_negative)
                    .map(|entry| (entry.property_idx, entry.property_count))
            }

            AnyTwCandidate::TwFunctionalCandidate(f) => {
                let Ok(base) = f.base_token() else {
                    return Self::Unknown;
                };

                let Ok(value) = f.value() else {
                    return Self::Unknown;
                };

                // Tailwind resolves a candidate's full name as a static
                // utility before trying functional roots: `w-full`,
                // `m-auto`, and `justify-center` are static
                // registrations even though the grammar splits them
                // into base and value. Statics take no modifier, so a
                // modifier skips the lookup.
                if f.modifier().is_none()
                    && let Some(text) = named_text(&value)
                    && let Some(entry) = joined_static_entry(base.text_trimmed(), text.text())
                    && (!is_negative || entry.has_negative)
                {
                    Some((entry.property_idx, entry.property_count))
                } else {
                    let Some(entry) = FUNCTIONAL_UTILITIES.get(base.text_trimmed()) else {
                        return Self::Unknown;
                    };

                    let (named_branches, arbitrary_branches) = if is_negative {
                        match entry.negative {
                            None => return Self::Unknown,
                            Some(Negative::SameBranches) => {
                                (entry.named_branches, entry.arbitrary_branches)
                            }
                            Some(Negative::Distinct {
                                named_branches,
                                arbitrary_branches,
                            }) => (named_branches, arbitrary_branches),
                        }
                    } else {
                        (entry.named_branches, entry.arbitrary_branches)
                    };

                    if let AnyTwValue::TwArbitraryValue(arb) = &value {
                        resolve_arbitrary_branch(arbitrary_branches, &arb.value())
                    } else {
                        let modifier = f.modifier();
                        resolve_named_branch(named_branches, &value, modifier.as_ref())
                    }
                }
            }
        };

        match placement {
            None => Self::Unknown,
            Some((property_idx, property_count)) => Self::Known {
                property_idx,
                property_count,
                name: NameKey {
                    negative: is_negative,
                    text: Some(inner.syntax().text_trimmed()),
                },
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
                name: n1,
                important: i1,
            },
            SortKey::Known {
                property_idx: p2,
                property_count: c2,
                name: n2,
                important: i2,
            },
        ) => p1
            .cmp(p2)
            // Wider utilities (e.g. `sr-only` setting 9 properties) win
            // their property bucket so they sort before single-property
            // utilities in the same bucket.
            .then_with(|| c2.cmp(c1))
            // Candidates inside one bucket order by name, the way
            // Tailwind emits them (`m-2 m-4 m-auto m-px`,
            // `collapse invisible visible`).
            .then_with(|| n1.compare(n2))
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

/// Look up `base-value` in `STATIC_UTILITIES` without allocating,
/// reassembling the name in a stack buffer sized for the longest static
/// utility name (`font-stretch-ultra-condensed`, 28 bytes).
fn joined_static_entry(base: &str, value: &str) -> Option<&'static UtilityEntry> {
    const LONGEST_STATIC_NAME: usize = 32;
    let len = base.len() + 1 + value.len();
    if len > LONGEST_STATIC_NAME {
        return None;
    }
    let mut buf = [0u8; LONGEST_STATIC_NAME];
    buf[..base.len()].copy_from_slice(base.as_bytes());
    buf[base.len()] = b'-';
    buf[base.len() + 1..len].copy_from_slice(value.as_bytes());
    // Joining two `str`s with an ASCII byte always forms valid UTF-8.
    let name = str::from_utf8(&buf[..len]).ok()?;
    STATIC_UTILITIES.get(name)
}

/// Walk a basename's named branch list and return the first matching
/// branch's `(property_idx, property_count)` placement. Branch order in
/// the preset already reflects the resolution precedence we want
/// (Keyword → Theme → Typed).
fn resolve_named_branch(
    branches: &[NamedBranch],
    value: &AnyTwValue,
    modifier: Option<&AnyTwModifier>,
) -> Option<(u16, u8)> {
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
        return Some((property_idx, property_count));
    }
    None
}

/// Walk a basename's arbitrary branch list and return the first matching
/// branch's `(property_idx, property_count)` placement. Typed branches
/// precede the type-blind fallback in generated preset order.
fn resolve_arbitrary_branch(
    branches: &[ArbitraryBranch],
    list: &CssGenericComponentValueList,
) -> Option<(u16, u8)> {
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
        return Some((property_idx, property_count));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_tailwind_parser::parse_tailwind;

    fn known(property_idx: u16, property_count: u8) -> SortKey {
        SortKey::Known {
            property_idx,
            property_count,
            name: NameKey::default(),
            important: false,
        }
    }

    /// The name of a known key — sign plus candidate text — materialized
    /// for assertion messages.
    fn name_text(key: &SortKey) -> String {
        let SortKey::Known { name, .. } = key else {
            panic!("expected a known key");
        };
        let mut out = String::new();
        if name.negative {
            out.push('-');
        }
        if let Some(text) = &name.text {
            out.push_str(&text.to_string());
        }
        out
    }

    fn nat_cmp(a: &str, b: &str) -> Ordering {
        TwNameCollator.cmp(a.chars(), b.chars())
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
        assert_eq!(compare(&SortKey::Unknown, &known(5, 1)), Ordering::Less);
        assert_eq!(compare(&known(5, 1), &SortKey::Unknown), Ordering::Greater);
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
        assert_eq!(compare(&known(3, 1), &known(5, 1)), Ordering::Less);
    }

    #[test]
    fn compare_breaks_property_idx_tie_by_property_count_descending() {
        // sr-only-shape utility (count=9) wins over a single-property one.
        let wider = known(5, 9);
        let narrow = known(5, 1);
        assert_eq!(compare(&wider, &narrow), Ordering::Less);
    }

    #[test]
    fn compare_breaks_bucket_tie_by_name() {
        // Distinct static utilities in one visibility bucket order by
        // name, the way Tailwind emits them — not by registration order,
        // which would put `visible` first.
        let collapse = classify("collapse");
        let visible = classify("visible");
        assert_eq!(compare(&collapse, &visible), Ordering::Less);
        assert_eq!(compare(&visible, &collapse), Ordering::Greater);
    }

    #[test]
    fn compare_returns_equal_for_identical_known_keys() {
        assert_eq!(compare(&known(5, 1), &known(5, 1)), Ordering::Equal);
    }

    #[test]
    fn compare_breaks_exact_key_tie_plain_before_important() {
        let plain = known(5, 1);
        let important = SortKey::Known {
            property_idx: 5,
            property_count: 1,
            name: NameKey::default(),
            important: true,
        };
        assert_eq!(compare(&plain, &important), Ordering::Less);
        assert_eq!(compare(&important, &plain), Ordering::Greater);
    }

    #[test]
    fn compare_breaks_bucket_tie_by_name_before_importance() {
        // `p-2! p-4`: the name decides, the important suffix does not
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

    #[test]
    fn compare_puts_negatives_before_positives_in_one_bucket() {
        // `-m-4 m-2`: the sign participates in the name, and `-`
        // precedes every letter in code-point order.
        let negative = classify("-m-4");
        let positive = classify("m-2");
        assert_eq!(compare(&negative, &positive), Ordering::Less);
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
            resolve_named_branch(branches, &value, modifier.as_ref()),
            Some((10, 1))
        );
    }

    #[test]
    fn resolve_named_branch_classifies_value_internally() {
        let (value, modifier) = functional_parts("p-5");
        let branches = &[NamedBranch::Typed(NamedValueType::Number, 10, 1)];

        assert_eq!(
            resolve_named_branch(branches, &value, modifier.as_ref()),
            Some((10, 1))
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
            resolve_arbitrary_branch(branches, &arbitrary.value()),
            Some((20, 1))
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
            resolve_arbitrary_branch(branches, &arbitrary.value()),
            Some((20, 1))
        );
    }

    #[test]
    fn resolve_named_branch_returns_none_when_kind_does_not_match_value_type() {
        // A named value like "abc" never satisfies NamedBranch::Typed(Number)
        // because dispatch is by parser node kind, not text scanning.
        let (value, modifier) = functional_parts("p-abc");
        let branches = &[NamedBranch::Typed(NamedValueType::Number, 1, 1)];
        assert_eq!(resolve_named_branch(branches, &value, modifier.as_ref()), None);
    }

    #[test]
    fn resolve_named_branch_ratio_matches_ratio_typed_branch() {
        let (value, modifier) = functional_parts("w-1/2");
        let branches = &[NamedBranch::Typed(NamedValueType::Ratio, 7, 1)];
        assert_eq!(
            resolve_named_branch(branches, &value, modifier.as_ref()),
            Some((7, 1))
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
            resolve_named_branch(branches, &value, modifier.as_ref()),
            Some((2, 1))
        );
    }

    #[test]
    fn resolve_named_branch_ignores_non_fraction_modifier() {
        let (value, modifier) = functional_parts("bg-red-500/50");
        let branches = &[NamedBranch::Theme(ThemeNamespace::Color, 10, 1)];

        assert_eq!(
            resolve_named_branch(branches, &value, modifier.as_ref()),
            Some((10, 1))
        );
    }

    // endregion: branch resolution

    // region: sort key classification

    #[test]
    fn arbitrary_candidate_takes_bucket_from_property_index() {
        let parsed = parse_tailwind("[display:block]");
        let full = parsed.tree().candidates().iter().next().unwrap();
        let display_idx = *PROPERTY_INDEX.get("display").unwrap();
        let key = SortKey::from_candidate(&full);
        let SortKey::Known {
            property_idx,
            property_count,
            important: false,
            ..
        } = &key
        else {
            panic!("expected a plain known key");
        };
        assert_eq!(*property_idx, display_idx);
        assert_eq!(*property_count, 1);
        assert_eq!(name_text(&key), "[display:block]");
    }

    #[test]
    fn important_suffix_is_position_neutral_in_the_key() {
        let SortKey::Known {
            property_idx,
            property_count,
            name,
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
                name,
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
    fn functional_shaped_static_names_resolve_through_the_static_table() {
        // `w-full` parses as functional `w` + `full`; the whole name is
        // a static registration, and the join must land in the same
        // width bucket as functional `w` candidates.
        let SortKey::Known { property_idx, .. } = classify("w-full") else {
            panic!("expected `w-full` to classify as known");
        };
        let SortKey::Known {
            property_idx: functional_idx,
            ..
        } = classify("w-10")
        else {
            panic!("expected `w-10` to classify as known");
        };
        assert_eq!(property_idx, functional_idx);
        // Static names whose parsed base is no functional utility at
        // all resolve too.
        assert!(matches!(classify("justify-center"), SortKey::Known { .. }));
        assert!(matches!(classify("inline-block"), SortKey::Known { .. }));
    }

    #[test]
    fn joined_static_negatives_require_a_registered_negative_form() {
        // Tailwind registers `-m-px` but no `-w-full`.
        assert!(matches!(classify("-m-px"), SortKey::Known { .. }));
        assert_eq!(classify("-w-full"), SortKey::Unknown);
    }

    #[test]
    fn joined_static_lookup_skips_modified_candidates() {
        // Statics take no modifier; `w-full/50` is not a valid
        // candidate and resolves through no branch either.
        assert_eq!(classify("w-full/50"), SortKey::Unknown);
    }

    #[test]
    fn classification_captures_name_text() {
        let text_of = |input: &str| name_text(&classify(input));
        assert_eq!(text_of("p-4"), "p-4");
        // Fractions, modifiers, and arbitrary values all ride along in
        // the candidate text.
        assert_eq!(text_of("w-1/2"), "w-1/2");
        assert_eq!(text_of("bg-red-500/50"), "bg-red-500/50");
        assert_eq!(text_of("w-[13px]"), "w-[13px]");
        assert_eq!(text_of("[color:red]/50"), "[color:red]/50");
        assert_eq!(text_of("flex"), "flex");
        // The sign comes from the full candidate, outside the inner
        // candidate node.
        assert_eq!(text_of("-mt-2"), "-mt-2");
        // The important suffix is not part of the name; it is a separate
        // final tiebreak.
        assert_eq!(text_of("p-4!"), "p-4");
    }

    // endregion: sort key classification
}

//! Variant ordering for the Tailwind v4 class sorter.
//!
//! A candidate's variants (`hover:`, `sm:`, `group-has-[…]:`) form the
//! *outermost* part of its sort key: a plain utility sorts before any
//! variant of it, so `flex hover:flex sm:flex` keeps that order. This
//! module turns each candidate's variants into a [VariantWeight] that
//! [`sort_v4`](super::sort_v4) compares ahead of the utility signature.
//!
//! # Weighting takes two passes
//!
//! A variant's weight depends on the *whole* class list, not the
//! candidate alone, so the list is classified first and weighted second
//! (see [`sort_class_list`](super::sort_v4::sort_class_list)):
//!
//! 1. Parse each candidate's variants into [VariantKey]s
//!    ([variant_keys_from_candidate]).
//! 2. Collect every distinct key across the list into [VariantGroups]:
//!    sort them by Tailwind's variant order ([compare_variant_keys]) and
//!    give each an ascending group index. Keys Tailwind treats as
//!    order-equivalent share a group.
//! 3. Each candidate's [VariantWeight] is the set of group indices its
//!    variants land in — a bitset compared as one big number. No
//!    variants means zero, which sorts first; a higher-ordered variant
//!    sets a higher bit, so the weight is larger and sorts later. This
//!    mirrors how Tailwind ranks variant combinations.
//!
//! # Where the order comes from
//!
//! [VARIANTS] is generated from Tailwind's own design system, so each
//! variant's `order` is Tailwind's. Breakpoints and containers instead
//! compare by resolved length — ascending for `min-*`/`sm`/`md`,
//! descending for `max-*` ([compare_variant_values]). An arbitrary value
//! that does not parse (`min-[15xyz]`) ranks after every parseable one
//! rather than comparing equal to all of them, which would make the
//! comparator non-transitive and can panic the sort.

use std::{cmp::Ordering, collections::HashMap};

use biome_rowan::{AstNode, SyntaxNodeText, TokenText};
use biome_tailwind_syntax::{
    AnyTwVariant, AnyTwVariantSegment, TwFullCandidate, TwVariantSegmentList,
};

use super::tailwind_preset_v4::{BREAKPOINT_VALUES, CONTAINER_VALUES, VARIANTS};
use super::tailwind_preset_v4_types::{VariantCompare, VariantEntry, VariantKind};

/// The variants a candidate carries, as the set of [VariantGroups]
/// indices they occupy — a bitset compared as one big number.
///
/// This is the outermost field of the utility sort key. Empty (a plain
/// utility) is zero and sorts first; a higher-ordered variant sets a
/// higher bit, so the weight is larger and sorts later — the way
/// Tailwind ranks variant combinations. Backed by `Vec<u64>` rather
/// than a fixed-width integer because a class list's distinct-variant
/// count is unbounded.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(super) struct VariantWeight(Vec<u64>);

impl VariantWeight {
    fn set(&mut self, index: usize) {
        let word = index / 64;
        if self.0.len() <= word {
            self.0.resize(word + 1, 0);
        }
        self.0[word] |= 1u64 << (index % 64);
    }
}

impl Ord for VariantWeight {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare as a big-endian integer: longer weights first, then
        // most-significant word first. `set` only grows the vec to hold
        // a new bit and never clears one, so the top word is always
        // non-zero — equal weights have equal vecs, which keeps this
        // `Ord` consistent with the derived `Eq`.
        let self_len = trimmed_word_len(&self.0);
        let other_len = trimmed_word_len(&other.0);
        match self_len.cmp(&other_len) {
            Ordering::Equal => {}
            ordering => return ordering,
        }
        for index in (0..self_len).rev() {
            match self.0[index].cmp(&other.0[index]) {
                Ordering::Equal => {}
                ordering => return ordering,
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for VariantWeight {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn trimmed_word_len(words: &[u64]) -> usize {
    words
        .iter()
        .rposition(|word| *word != 0)
        .map_or(0, |index| index + 1)
}

/// A parsed variant. Registered roots borrow their name straight from
/// the [VARIANTS] registry (`&'static str`); source-derived text is a
/// [TokenText] (a cheap ref-counted slice, no heap copy). Only
/// `Arbitrary` owns a `Box<str>`: its selector is a
/// `CssGenericComponentValueList` that can span several tokens, so no
/// single slice covers it.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(super) enum VariantKey {
    Static(&'static str),
    Functional {
        root: &'static str,
        value: Option<VariantValue>,
    },
    Compound {
        root: &'static str,
        variant: Box<Self>,
    },
    Arbitrary(Box<str>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(super) enum VariantValue {
    Named(TokenText),
    Arbitrary(Box<str>),
}

#[derive(Clone, Debug)]
enum VariantSegment {
    Named(TokenText),
    Arbitrary(Box<str>),
    CssVariable,
}

pub(super) struct VariantGroups {
    groups: HashMap<VariantKey, usize>,
}

impl VariantGroups {
    pub(super) fn new<'a>(variants: impl IntoIterator<Item = &'a VariantKey>) -> Self {
        let mut variants: Vec<&VariantKey> = variants.into_iter().collect();
        variants.sort_by(|left, right| compare_variant_keys(left, right));

        let mut groups = HashMap::new();
        let mut previous: Option<&VariantKey> = None;
        let mut group = 0usize;

        for variant in variants {
            if groups.contains_key(variant) {
                continue;
            }
            if previous
                .is_some_and(|previous| compare_variant_keys(previous, variant) != Ordering::Equal)
            {
                group += 1;
            }
            groups.insert((*variant).clone(), group);
            previous = Some(variant);
        }

        Self { groups }
    }

    /// Returns `None` if a variant was not part of the list the groups
    /// were built from. Unreachable when the groups come from these same
    /// candidates, so callers fold it into `Unknown`.
    pub(super) fn weight_for(&self, variants: &[VariantKey]) -> Option<VariantWeight> {
        let mut weight = VariantWeight::default();
        for variant in variants {
            weight.set(*self.groups.get(variant)?);
        }
        Some(weight)
    }
}

pub(super) fn variant_keys_from_candidate(candidate: &TwFullCandidate) -> Option<Vec<VariantKey>> {
    let mut variants = Vec::new();
    for variant in candidate.variants() {
        variants.push(variant_key_from_variant(&variant.ok()?)?);
    }
    Some(variants)
}

fn variant_key_from_variant(variant: &AnyTwVariant) -> Option<VariantKey> {
    match variant {
        AnyTwVariant::TwArbitraryVariant(variant) => Some(VariantKey::Arbitrary(
            variant.selector_token().ok()?.text_trimmed().into(),
        )),
        AnyTwVariant::TwVariantExpression(expression) => {
            // `raw` is only a lookup buffer; the key stores the registry's
            // own `&'static str`, never this string. (The whole-expression
            // match is also reachable through the segment path below, so
            // this early check could likely be dropped — kept for now to
            // preserve behavior.)
            let raw = syntax_text_to_box(&expression.syntax().text_trimmed());
            if let Some((&name, entry)) = VARIANTS.get_entry(raw.as_ref())
                && entry.kind == VariantKind::Static
            {
                return Some(VariantKey::Static(name));
            }

            let segments = variant_segments(expression.segments())?;
            variant_key_from_segments(&segments)
        }
        AnyTwVariant::TwBogusVariant(_) => None,
    }
}

fn variant_segments(segments: TwVariantSegmentList) -> Option<Vec<VariantSegment>> {
    let mut result = Vec::new();
    for segment in segments {
        let segment = segment.ok()?;
        result.push(match segment {
            AnyTwVariantSegment::TwNamedVariantSegment(segment) => {
                VariantSegment::Named(segment.value_token().ok()?.token_text_trimmed())
            }
            AnyTwVariantSegment::TwArbitraryVariantSegment(segment) => VariantSegment::Arbitrary(
                syntax_text_to_box(&segment.value().syntax().text_trimmed()),
            ),
            AnyTwVariantSegment::TwCssVariableVariantSegment(_) => VariantSegment::CssVariable,
            AnyTwVariantSegment::TwBogusVariantSegment(_) => return None,
        });
    }
    Some(result)
}

fn variant_key_from_segments(segments: &[VariantSegment]) -> Option<VariantKey> {
    match segments.first()? {
        VariantSegment::Arbitrary(selector) if segments.len() == 1 => {
            Some(VariantKey::Arbitrary(selector.clone()))
        }
        VariantSegment::Named(_) => {
            let (root, entry, value_segments) = variant_root_from_segments(segments)?;
            match entry.kind {
                VariantKind::Static if value_segments.is_empty() => Some(VariantKey::Static(root)),
                VariantKind::Functional => Some(VariantKey::Functional {
                    root,
                    value: Some(variant_value_from_segments(value_segments)?),
                }),
                VariantKind::Compound => Some(VariantKey::Compound {
                    root,
                    variant: Box::new(variant_key_from_segments(value_segments)?),
                }),
                VariantKind::Static => None,
            }
        }
        VariantSegment::Arbitrary(_) | VariantSegment::CssVariable => None,
    }
}

fn variant_root_from_segments(
    segments: &[VariantSegment],
) -> Option<(&'static str, &'static VariantEntry, &[VariantSegment])> {
    // `root` is a scratch buffer that grows one segment at a time to probe
    // the registry for the longest matching prefix; the matched key is
    // stored, not this string.
    let mut root = String::new();
    let mut best = None;

    for (index, segment) in segments.iter().enumerate() {
        let VariantSegment::Named(segment) = segment else {
            break;
        };
        if !root.is_empty() {
            root.push('-');
        }
        root.push_str(segment);

        if let Some((&name, entry)) = VARIANTS.get_entry(root.as_str()) {
            best = Some((name, entry, index + 1));
        }
    }

    let (name, entry, rest_index) = best?;
    Some((name, entry, &segments[rest_index..]))
}

fn variant_value_from_segments(segments: &[VariantSegment]) -> Option<VariantValue> {
    match segments {
        [VariantSegment::Named(value)] => Some(VariantValue::Named(value.clone())),
        [VariantSegment::Arbitrary(value)] => Some(VariantValue::Arbitrary(value.clone())),
        _ => None,
    }
}

/// Orders variant keys the way Tailwind orders variants, so a list of
/// keys can be bucketed into [VariantGroups].
///
/// Deliberately a free function rather than an `Ord` impl on
/// [VariantKey]: this is a *grouping* order in which distinct keys can
/// compare `Equal` (e.g. two arbitrary breakpoints that resolve to the
/// same length share a bucket). [VariantKey] derives `Eq`/`Hash` and is
/// used as a `HashMap` key, and `Ord` must agree with `Eq` — an `Ord`
/// returning `Equal` for `a != b` would break that contract. Keeping the
/// grouping order out of the trait keeps them from disagreeing.
fn compare_variant_keys(left: &VariantKey, right: &VariantKey) -> Ordering {
    match (left, right) {
        (VariantKey::Arbitrary(left), VariantKey::Arbitrary(right)) => left.cmp(right),
        (VariantKey::Arbitrary(_), _) => Ordering::Greater,
        (_, VariantKey::Arbitrary(_)) => Ordering::Less,
        _ => compare_registered_variant_keys(left, right),
    }
}

fn compare_registered_variant_keys(left: &VariantKey, right: &VariantKey) -> Ordering {
    let Some(left_entry) = variant_entry(left) else {
        return Ordering::Greater;
    };
    let Some(right_entry) = variant_entry(right) else {
        return Ordering::Less;
    };

    left_entry
        .order
        .cmp(&right_entry.order)
        .then_with(|| compare_same_order_variant_keys(left, left_entry, right, right_entry))
}

fn compare_same_order_variant_keys(
    left: &VariantKey,
    left_entry: &VariantEntry,
    right: &VariantKey,
    right_entry: &VariantEntry,
) -> Ordering {
    if let (
        VariantKey::Compound {
            root: left_root,
            variant: left_variant,
        },
        VariantKey::Compound {
            root: right_root,
            variant: right_variant,
        },
    ) = (left, right)
        && left_root == right_root
    {
        return compare_variant_keys(left_variant, right_variant);
    }

    compare_variant_values(left, left_entry.compare, right, right_entry.compare)
        .then_with(|| variant_root(left).cmp(variant_root(right)))
        .then_with(|| compare_functional_values(left, right))
}

fn compare_variant_values(
    left: &VariantKey,
    left_compare: VariantCompare,
    right: &VariantKey,
    right_compare: VariantCompare,
) -> Ordering {
    if left_compare != right_compare || left_compare == VariantCompare::Default {
        return Ordering::Equal;
    }

    let Some(left_value) = variant_compare_value(left, left_compare) else {
        return Ordering::Equal;
    };
    let Some(right_value) = variant_compare_value(right, right_compare) else {
        return Ordering::Equal;
    };

    let ordering = left_value
        .partial_cmp(&right_value)
        .unwrap_or(Ordering::Equal);
    match left_compare {
        VariantCompare::BreakpointAsc | VariantCompare::ContainerAsc => ordering,
        VariantCompare::BreakpointDesc | VariantCompare::ContainerDesc => ordering.reverse(),
        VariantCompare::Default => Ordering::Equal,
    }
}

fn variant_compare_value(key: &VariantKey, compare: VariantCompare) -> Option<f64> {
    let value = match key {
        VariantKey::Static(root) => *root,
        VariantKey::Functional {
            value: Some(value), ..
        } => value.text(),
        _ => return None,
    };

    let resolved = match compare {
        VariantCompare::BreakpointAsc | VariantCompare::BreakpointDesc => {
            BREAKPOINT_VALUES.get(value).copied().unwrap_or(value)
        }
        VariantCompare::ContainerAsc | VariantCompare::ContainerDesc => {
            CONTAINER_VALUES.get(value).copied().unwrap_or(value)
        }
        VariantCompare::Default => return None,
    };

    // An arbitrary value that does not parse (e.g. `min-[foo]`) ranks
    // after every parseable one rather than comparing equal to all of
    // them, which would make the comparator non-transitive.
    Some(parse_length_value(resolved).unwrap_or(f64::INFINITY))
}

fn parse_length_value(value: &str) -> Option<f64> {
    if let Some(value) = value.strip_suffix("rem") {
        return value.parse::<f64>().ok().map(|value| value * 16.0);
    }
    if let Some(value) = value.strip_suffix("px") {
        return value.parse::<f64>().ok();
    }
    value.parse::<f64>().ok()
}

fn compare_functional_values(left: &VariantKey, right: &VariantKey) -> Ordering {
    match (variant_value(left), variant_value(right)) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(left), Some(right)) => left.cmp(right),
    }
}

fn variant_entry(key: &VariantKey) -> Option<&'static VariantEntry> {
    VARIANTS.get(variant_root(key))
}

fn variant_root(key: &VariantKey) -> &str {
    match key {
        VariantKey::Static(root)
        | VariantKey::Functional { root, .. }
        | VariantKey::Compound { root, .. } => root,
        VariantKey::Arbitrary(_) => "",
    }
}

fn variant_value(key: &VariantKey) -> Option<&VariantValue> {
    match key {
        VariantKey::Functional { value, .. } => value.as_ref(),
        _ => None,
    }
}

impl VariantValue {
    fn text(&self) -> &str {
        match self {
            Self::Named(value) => value,
            Self::Arbitrary(value) => value,
        }
    }
}

impl Ord for VariantValue {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Named(left), Self::Named(right)) => left.cmp(right),
            (Self::Arbitrary(left), Self::Arbitrary(right)) => left.cmp(right),
            (Self::Named(_), Self::Arbitrary(_)) => Ordering::Less,
            (Self::Arbitrary(_), Self::Named(_)) => Ordering::Greater,
        }
    }
}

impl PartialOrd for VariantValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn syntax_text_to_box(text: &SyntaxNodeText) -> Box<str> {
    let mut result = String::with_capacity(usize::from(text.len()));
    text.for_each_chunk(|chunk| result.push_str(chunk));
    result.into_boxed_str()
}

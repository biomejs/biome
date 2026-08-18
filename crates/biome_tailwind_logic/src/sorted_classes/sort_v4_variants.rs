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
//!    sort them by [VariantKey]'s `Ord` (Tailwind's variant order) and
//!    give each an ascending group index.
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
//! descending for `max-*` ([compare_lengths]) — and by length ONLY:
//! variants that resolve the same length share one rank, the way
//! Tailwind's `getVariantOrder` collapses compare-equal variants, and
//! the full candidate text breaks the tie in
//! [`sort_class_list`](super::sort_v4::sort_class_list).

use std::cmp::Ordering;

use biome_rowan::{AstNode, SyntaxNodeText, Text, TextRange, TextSize, TokenText};
use biome_tailwind_syntax::{
    AnyTwModifier, AnyTwValue, AnyTwVariant, AnyTwVariantSegment, TwFullCandidate,
    TwVariantSegmentList,
};
use smallvec::SmallVec;

use super::tailwind_preset_v4_types::{VariantCompare, VariantEntry, VariantKind};
use super::tailwind_registry::TailwindRegistry;

/// The variants a candidate carries, as the set of [VariantGroups]
/// indices they occupy — a bitset compared as one big number.
///
/// This is the outermost field of the utility sort key. Empty (a plain
/// utility) is zero and sorts first; a higher-ordered variant sets a
/// higher bit, so the weight is larger and sorts later — the way
/// Tailwind ranks variant combinations. Backed by a `SmallVec<[u64; 1]>`:
/// inline (no heap) for the ≤64 distinct-variant case that covers
/// essentially every class list, spilling to the heap only in the
/// unbounded pathological case.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(super) struct VariantWeight(SmallVec<[u64; 1]>);

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
/// the [VARIANTS] registry (`&'static str`); named values borrow their
/// source token as a [TokenText] (a cheap ref-counted slice). Arbitrary
/// selectors hold a [Text], which likewise borrows single-token text
/// and copies only when the selector spans several tokens
/// (`data-[foo=bar]`'s bracketed CSS value list).
///
/// Only group/peer compounds and container queries accept a modifier;
/// everywhere else it makes the candidate invalid, so no key is built at
/// all ([attach_modifier]). [Self::Compound] carries its modifier because
/// group/peer scopes order by it; [Self::Functional] does not, because
/// container modifiers never participate in ordering — `@sm/main:` ties
/// `@sm:` on length and the candidate text breaks the tie.
///
/// Keys carry no `Eq`/`Ord`: two keys are "equal" when
/// [compare_variant_keys] ties them, which needs the registry (a custom
/// variant's order lives there), and [VariantGroups] relies on that
/// comparator to collapse exactly the keys Tailwind gives one shared
/// rank (`@sm/main:` and `@sm:` resolve the same container width).
///
/// Roots borrow from the registry — a `&'static str` out of the preset's
/// `phf` tables or a `&str` into the registry's own map keys — so
/// building a key allocates nothing.
#[derive(Clone, Debug)]
pub(super) enum VariantKey<'r> {
    Static(&'r str),
    Functional {
        root: &'r str,
        value: Option<VariantValue>,
    },
    Compound {
        root: &'r str,
        variant: Box<Self>,
        /// The `/scope` name, as an allocation-free view of the modifier's
        /// value node (its inner value for bracketed forms, so
        /// `group-hover/[.5]:` compares as `.5`, before `/menu`).
        modifier: Option<SyntaxNodeText>,
    },
    Arbitrary(Text),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum VariantValue {
    Named(TokenText),
    Arbitrary(Text),
}

#[derive(Clone, Debug)]
enum VariantSegment {
    Named(TokenText),
    Arbitrary(Text),
    CssVariable,
}

pub(super) struct VariantGroups<'r> {
    /// Distinct variant keys in ascending order; a key's rank — its
    /// weight-bit index — is its position here.
    ranked: Vec<VariantKey<'r>>,
    registry: &'r TailwindRegistry,
}

impl<'r> VariantGroups<'r> {
    pub(super) fn new<'a>(
        variants: impl IntoIterator<Item = &'a VariantKey<'r>>,
        registry: &'r TailwindRegistry,
    ) -> Self
    where
        'r: 'a,
    {
        // Each distinct key's index is its rank. Keys the comparator ties
        // collapse into one, so `dedup_by` keeps exactly the keys Tailwind
        // gives one shared rank (`@sm/main:` with `@sm:`).
        let mut ranked: Vec<VariantKey<'r>> = variants.into_iter().cloned().collect();
        ranked.sort_by(|a, b| compare_variant_keys(a, b, registry));
        ranked.dedup_by(|a, b| compare_variant_keys(a, b, registry) == Ordering::Equal);
        Self { ranked, registry }
    }

    /// Returns `None` if a variant was not part of the list the ranks
    /// were built from. Unreachable when the ranks come from these same
    /// candidates, so callers fold it into `Unknown`.
    pub(super) fn weight_for(&self, variants: &[VariantKey<'r>]) -> Option<VariantWeight> {
        let mut weight = VariantWeight::default();
        for variant in variants {
            weight.set(
                self.ranked
                    .binary_search_by(|probe| compare_variant_keys(probe, variant, self.registry))
                    .ok()?,
            );
        }
        Some(weight)
    }
}

pub(super) fn variant_keys_from_candidate<'r>(
    candidate: &TwFullCandidate,
    registry: &'r TailwindRegistry,
) -> Option<Vec<VariantKey<'r>>> {
    let mut variants = Vec::new();
    for variant in candidate.variants() {
        variants.push(variant_key_from_variant(&variant.ok()?, registry)?);
    }
    Some(variants)
}

fn variant_key_from_variant<'r>(
    variant: &AnyTwVariant,
    registry: &'r TailwindRegistry,
) -> Option<VariantKey<'r>> {
    match variant {
        AnyTwVariant::TwArbitraryVariant(variant) => Some(VariantKey::Arbitrary(
            variant.selector_token().ok()?.token_text_trimmed().into(),
        )),
        AnyTwVariant::TwVariantExpression(expression) => {
            let mut segments = variant_segments(expression.segments())?;
            if let Some(glued) = expression.glued_value() {
                // `@[400px]:` glues an arbitrary size to the root with no
                // `-`; treat it like a dashed arbitrary segment.
                segments.push(VariantSegment::Arbitrary(
                    glued.value_token().ok()?.token_text_trimmed().into(),
                ));
            }
            let key = variant_key_from_segments(&segments, registry)?;
            match expression.modifier() {
                None => Some(key),
                Some(modifier) => attach_modifier(key, &modifier),
            }
        }
        AnyTwVariant::TwBogusVariant(_) => None,
    }
}

/// Attach a variant's `/modifier` to its key, or reject the candidate.
/// Tailwind only accepts a modifier as the scope name of a group/peer
/// compound (`group-hover/menu:`) or of a container query (`@sm/main:`,
/// `@max-[959px]/name:`); on any other variant it invalidates the whole
/// candidate (`hover/foo:flex` sorts as unknown).
fn attach_modifier<'r>(key: VariantKey<'r>, modifier: &AnyTwModifier) -> Option<VariantKey<'r>> {
    let value = modifier_value(modifier)?;
    match key {
        VariantKey::Compound { root, variant, .. } if matches!(root, "group" | "peer") => {
            Some(VariantKey::Compound {
                root,
                variant,
                modifier: Some(value),
            })
        }
        // A container modifier names the container without affecting the
        // ordering, so the key does not keep it.
        VariantKey::Functional { root, value } if root.starts_with('@') => {
            Some(VariantKey::Functional { root, value })
        }
        _ => None,
    }
}

/// The comparable text of a modifier, as a tree view with no copy:
/// bracketed values compare by their inner value list the way Tailwind
/// compares the decoded modifier value, every other value by its own node
/// text.
fn modifier_value(modifier: &AnyTwModifier) -> Option<SyntaxNodeText> {
    let AnyTwModifier::TwModifier(modifier) = modifier else {
        return None;
    };
    match modifier.value().ok()? {
        AnyTwValue::TwArbitraryValue(value) => Some(value.value().syntax().text_trimmed()),
        AnyTwValue::TwPercentageValue(_) | AnyTwValue::TwBogusValue(_) => None,
        value => Some(value.syntax().text_trimmed()),
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
            AnyTwVariantSegment::TwArbitraryVariantSegment(segment) => {
                VariantSegment::Arbitrary(segment.value_token().ok()?.token_text_trimmed().into())
            }
            AnyTwVariantSegment::TwCssVariableVariantSegment(_) => VariantSegment::CssVariable,
            AnyTwVariantSegment::TwBogusVariantSegment(_) => return None,
        });
    }
    Some(result)
}

fn variant_key_from_segments<'r>(
    segments: &[VariantSegment],
    registry: &'r TailwindRegistry,
) -> Option<VariantKey<'r>> {
    match segments.first()? {
        VariantSegment::Arbitrary(selector) if segments.len() == 1 => {
            Some(VariantKey::Arbitrary(selector.clone()))
        }
        VariantSegment::Named(name) => {
            let Some((root, entry, value_segments)) =
                variant_root_from_segments(segments, registry)
            else {
                // `@sm` glues the container root to its size with no `-` for
                // the segment splitter to see (unlike `@max-lg` / `@min-[…]`,
                // whose root is a registered dashed prefix).
                return glued_container_variant(name, registry);
            };
            match entry.kind {
                VariantKind::Static if value_segments.is_empty() => Some(VariantKey::Static(root)),
                VariantKind::Functional => {
                    let value = variant_value_from_segments(value_segments)?;
                    // A breakpoint or container size that cannot resolve
                    // to a length (`min-abc:`, `@max-[var(--w)]:`) is not
                    // a valid variant, so the candidate sorts as unknown.
                    if !length_value_resolves(entry.compare, &value, registry) {
                        return None;
                    }
                    Some(VariantKey::Functional {
                        root,
                        value: Some(value),
                    })
                }
                VariantKind::Compound => Some(VariantKey::Compound {
                    root,
                    variant: Box::new(variant_key_from_segments(value_segments, registry)?),
                    modifier: None,
                }),
                VariantKind::Static => None,
            }
        }
        VariantSegment::Arbitrary(_) | VariantSegment::CssVariable => None,
    }
}

/// Upper bound on registered variant name length, so root names can be
/// reassembled on the stack. The longest today is `any-pointer-coarse`
/// at 18 bytes; a test asserts the registry stays within the bound.
const LONGEST_VARIANT_NAME: usize = 24;

/// Longest variant name the root probe reassembles on the stack; a custom
/// `@custom-variant` name past this is not resolved.
const VARIANT_JOIN_BUFFER: usize = 64;

/// Match the longest run of leading named segments against a registered
/// variant root and return its entry plus the remaining value segments.
///
/// The grammar splits a variant on every `-` without consulting the
/// registry, so which dashed prefix is the root (`group-has-[…]:` is
/// root `group-has`, `group-hover:` is all root) is only decidable
/// here. The name is reassembled without allocating in a stack buffer,
/// the way [`sort_v4`](super::sort_v4) joins static utility names.
fn variant_root_from_segments<'r, 's>(
    segments: &'s [VariantSegment],
    registry: &'r TailwindRegistry,
) -> Option<(&'r str, &'r VariantEntry, &'s [VariantSegment])> {
    let max_len = LONGEST_VARIANT_NAME.max(registry.longest_variant_name);
    let mut buf = [0u8; VARIANT_JOIN_BUFFER];
    let mut len = 0;
    let mut best = None;

    for (index, segment) in segments.iter().enumerate() {
        let VariantSegment::Named(segment) = segment else {
            break;
        };
        let segment = segment.text();
        let start = if index == 0 { 0 } else { len + 1 };
        let end = start + segment.len();
        if end > max_len || end > buf.len() {
            // No registered name is this long, and prefixes only grow.
            break;
        }
        if index != 0 {
            buf[len] = b'-';
        }
        buf[start..end].copy_from_slice(segment.as_bytes());
        len = end;
        // Joining `str`s with an ASCII byte always forms valid UTF-8.
        let probe = str::from_utf8(&buf[..len]).ok()?;
        if let Some((name, entry)) = registry.get_variant(probe) {
            best = Some((name, entry, index + 1));
        }
    }

    let (name, entry, rest_index) = best?;
    Some((name, entry, &segments[rest_index..]))
}

/// Resolve a glued container-query variant (`@sm`, `@3xl`) to the `@`
/// container root plus its size value. The `@` root borrows from the
/// registry and the size is a no-copy [TokenText] slice past the `@`. A
/// remainder that is not a known container size (`@container`) is not a
/// sortable variant, matching Tailwind.
fn glued_container_variant<'r>(
    name: &TokenText,
    registry: &'r TailwindRegistry,
) -> Option<VariantKey<'r>> {
    let value = name.text().strip_prefix('@')?;
    if !registry.container_contains(value) {
        return None;
    }
    let value = name
        .clone()
        .slice(TextRange::new(TextSize::from(1), name.len()));
    Some(VariantKey::Functional {
        root: "@",
        value: Some(VariantValue::Named(value)),
    })
}

/// Whether a breakpoint or container size value resolves to a length the
/// way Tailwind's own variant compare resolves it: a named size must be a
/// theme value (`min-sm` resolves, `min-abc` does not) and an arbitrary
/// size must not depend on a CSS variable. An unresolvable size makes the
/// whole candidate invalid rather than merely unsorted.
fn length_value_resolves(
    compare: VariantCompare,
    value: &VariantValue,
    registry: &TailwindRegistry,
) -> bool {
    match compare {
        VariantCompare::Default => true,
        VariantCompare::BreakpointAsc | VariantCompare::BreakpointDesc => match value {
            VariantValue::Named(name) => registry.breakpoint_contains(name.text()),
            VariantValue::Arbitrary(text) => !text.text().contains("var("),
        },
        VariantCompare::ContainerAsc | VariantCompare::ContainerDesc => match value {
            VariantValue::Named(name) => registry.container_contains(name.text()),
            VariantValue::Arbitrary(text) => !text.text().contains("var("),
        },
    }
}

fn variant_value_from_segments(segments: &[VariantSegment]) -> Option<VariantValue> {
    match segments {
        [VariantSegment::Named(value)] => Some(VariantValue::Named(value.clone())),
        [VariantSegment::Arbitrary(value)] => Some(VariantValue::Arbitrary(value.clone())),
        _ => None,
    }
}

/// Ranks variants the way Tailwind's own `Variants.compare` does, so
/// [VariantGroups] can position each distinct variant of a class list.
/// Arbitrary selectors sort after every registered variant and among
/// themselves by decoded selector text; registered variants sort by
/// Tailwind's `order` (a custom variant's from the registry), then per
/// shape (see [compare_same_order_variant_keys]). This is a total order,
/// and `Equal` between structurally different keys is meaningful:
/// Tailwind gives such variants one shared rank, and the candidate text
/// breaks the tie downstream.
pub(super) fn compare_variant_keys(
    left: &VariantKey<'_>,
    right: &VariantKey<'_>,
    registry: &TailwindRegistry,
) -> Ordering {
    match (left, right) {
        (VariantKey::Arbitrary(left), VariantKey::Arbitrary(right)) => {
            decoded_chars(left.text().chars()).cmp(decoded_chars(right.text().chars()))
        }
        (VariantKey::Arbitrary(_), _) => Ordering::Greater,
        (_, VariantKey::Arbitrary(_)) => Ordering::Less,
        _ => compare_registered_variant_keys(left, right, registry),
    }
}

/// The characters of a bracketed value the way Tailwind decodes them
/// before comparing: `_` is a space (`[&_p]` compares as `& p`, before
/// `[&>p]`) and `\_` a literal underscore.
fn decoded_chars<I: Iterator<Item = char>>(chars: I) -> impl Iterator<Item = char> {
    let mut chars = chars.peekable();
    std::iter::from_fn(move || {
        Some(match chars.next()? {
            '\\' if chars.peek() == Some(&'_') => {
                chars.next();
                '_'
            }
            '_' => ' ',
            c => c,
        })
    })
}

fn compare_registered_variant_keys(
    left: &VariantKey<'_>,
    right: &VariantKey<'_>,
    registry: &TailwindRegistry,
) -> Ordering {
    let Some(left_entry) = variant_entry(left, registry) else {
        return Ordering::Greater;
    };
    let Some(right_entry) = variant_entry(right, registry) else {
        return Ordering::Less;
    };

    left_entry.order.cmp(&right_entry.order).then_with(|| {
        compare_same_order_variant_keys(left, left_entry, right, right_entry, registry)
    })
}

/// Order two registered variants that share Tailwind's `order`, the way
/// `Variants.compare` does past its order check: compound variants
/// compare by their nested variant and then bare-before-modifier
/// (`group-hover:` < `group-hover/a:` < `group-hover/b:`); breakpoint and
/// container groups compare by resolved length ONLY, so equal lengths tie
/// into one shared rank (`@sm/main:` ties `@sm:`, `min-[40rem]:` ties
/// `sm:`) with no root or value fallback; everything else compares by
/// root text, then functional value.
fn compare_same_order_variant_keys(
    left: &VariantKey<'_>,
    left_entry: &VariantEntry,
    right: &VariantKey<'_>,
    right_entry: &VariantEntry,
    registry: &TailwindRegistry,
) -> Ordering {
    if let (
        VariantKey::Compound {
            variant: left_variant,
            modifier: left_modifier,
            ..
        },
        VariantKey::Compound {
            variant: right_variant,
            modifier: right_modifier,
            ..
        },
    ) = (left, right)
    {
        return compare_variant_keys(left_variant, right_variant, registry)
            .then_with(|| compare_modifiers(left_modifier.as_ref(), right_modifier.as_ref()));
    }

    if left_entry.compare != VariantCompare::Default && left_entry.compare == right_entry.compare {
        return compare_lengths(left, right, left_entry.compare, registry);
    }

    variant_root(left)
        .cmp(variant_root(right))
        .then_with(|| compare_functional_values(left, right))
}

/// A bare variant sorts before its modified forms, which order by
/// modifier text; container modifiers never get here because container
/// roots compare by length alone.
fn compare_modifiers(left: Option<&SyntaxNodeText>, right: Option<&SyntaxNodeText>) -> Ordering {
    match (left, right) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(left), Some(right)) => decoded_chars(left.chars()).cmp(decoded_chars(right.chars())),
    }
}

/// Tailwind's breakpoint/container comparator: resolve each variant to
/// its length (`sm` → `40rem`, `min-[600px]` → `600px`) and compare with
/// [compare_length_values]. An unresolvable side sorts first ascending
/// and last descending; unresolvable named or `var(`-dependent sizes
/// never get here because [length_value_resolves] rejects the candidate.
fn compare_lengths(
    left: &VariantKey<'_>,
    right: &VariantKey<'_>,
    compare: VariantCompare,
    registry: &TailwindRegistry,
) -> Ordering {
    let ascending = matches!(
        compare,
        VariantCompare::BreakpointAsc | VariantCompare::ContainerAsc
    );
    let container = matches!(
        compare,
        VariantCompare::ContainerAsc | VariantCompare::ContainerDesc
    );
    match (
        resolved_length(left, container, registry),
        resolved_length(right, container, registry),
    ) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) if ascending => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (Some(_), None) if ascending => Ordering::Greater,
        (Some(_), None) => Ordering::Less,
        (Some(left), Some(right)) => compare_length_values(left, right, ascending),
    }
}

fn resolved_length<'a>(
    key: &'a VariantKey<'_>,
    container: bool,
    registry: &'a TailwindRegistry,
) -> Option<&'a str> {
    match key {
        VariantKey::Static(root) => {
            if container {
                registry.get_container_value(root)
            } else {
                registry.get_breakpoint_value(root)
            }
        }
        VariantKey::Functional {
            value: Some(VariantValue::Named(name)),
            ..
        } => {
            if container {
                registry.get_container_value(name.text())
            } else {
                registry.get_breakpoint_value(name.text())
            }
        }
        VariantKey::Functional {
            value: Some(VariantValue::Arbitrary(text)),
            ..
        } => Some(text.text()),
        _ => None,
    }
}

/// Tailwind's length comparator (`compareBreakpoints`): the unit strings
/// compare first — the text minus digit and dot runs, or the prefix up to
/// a `(` — so `px` values group before `rem` values regardless of
/// magnitude, which is why arbitrary pixel sizes sort before every named
/// (rem) breakpoint. Equal units compare by leading integer, reversed for
/// descending groups (the unit comparison is not reversed, matching
/// Tailwind). A side with no leading integer falls back to plain text.
/// Equal integers tie even when the text differs (`24rem` vs `24.5rem`),
/// which shares a rank.
fn compare_length_values(left: &str, right: &str, ascending: bool) -> Ordering {
    if left == right {
        return Ordering::Equal;
    }
    let unit_ordering = unit_chars(left).cmp(unit_chars(right));
    if unit_ordering != Ordering::Equal {
        return unit_ordering;
    }
    match (leading_integer(left), leading_integer(right)) {
        (Some(left), Some(right)) if ascending => left.cmp(&right),
        (Some(left), Some(right)) => right.cmp(&left),
        _ => left.cmp(right),
    }
}

/// The "unit" of a length value: everything up to a `(` for functional
/// values (`calc(…)` → `calc`), otherwise the text with digit and dot
/// characters removed (`40rem` → `rem`).
fn unit_chars(value: &str) -> impl Iterator<Item = char> + '_ {
    let (prefix, strip_digits) = match value.find('(') {
        Some(index) => (&value[..index], false),
        None => (value, true),
    };
    prefix
        .chars()
        .filter(move |c| !strip_digits || !matches!(c, '0'..='9' | '.'))
}

/// `parseInt` on a length value: an optional sign and leading digit run,
/// `None` when the text does not start with one.
fn leading_integer(value: &str) -> Option<i64> {
    let value = value.trim_start();
    let (negative, digits) = match value.as_bytes().first() {
        Some(b'-') => (true, &value[1..]),
        Some(b'+') => (false, &value[1..]),
        _ => (false, value),
    };
    let end = digits
        .bytes()
        .take_while(|byte| byte.is_ascii_digit())
        .count();
    let magnitude: i64 = digits[..end].parse().ok()?;
    Some(if negative { -magnitude } else { magnitude })
}

fn compare_functional_values(left: &VariantKey<'_>, right: &VariantKey<'_>) -> Ordering {
    match (variant_value(left), variant_value(right)) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(left), Some(right)) => left.cmp(right),
    }
}

fn variant_entry<'r>(
    key: &VariantKey<'_>,
    registry: &'r TailwindRegistry,
) -> Option<&'r VariantEntry> {
    registry.get_variant_entry(variant_root(key))
}

fn variant_root<'a>(key: &'a VariantKey<'_>) -> &'a str {
    match key {
        VariantKey::Static(root)
        | VariantKey::Functional { root, .. }
        | VariantKey::Compound { root, .. } => root,
        VariantKey::Arbitrary(_) => "",
    }
}

fn variant_value<'a>(key: &'a VariantKey<'_>) -> Option<&'a VariantValue> {
    match key {
        VariantKey::Functional { value, .. } => value.as_ref(),
        _ => None,
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

#[cfg(test)]
mod tests {
    use super::super::tailwind_preset_v4::VARIANTS;
    use super::*;

    #[test]
    fn every_registered_variant_name_fits_the_root_probe_buffer() {
        let longest = VARIANTS.keys().map(|name| name.len()).max().unwrap();
        assert!(
            longest <= LONGEST_VARIANT_NAME,
            "a registered variant name is {longest} bytes; grow LONGEST_VARIANT_NAME"
        );
    }

    #[test]
    fn leading_integer_matches_parse_int() {
        assert_eq!(leading_integer("40rem"), Some(40));
        assert_eq!(leading_integer("600px"), Some(600));
        assert_eq!(leading_integer("+5"), Some(5));
        assert_eq!(leading_integer("-12px"), Some(-12));
        assert_eq!(leading_integer(" 600px"), Some(600));
        // `parseInt` stops at the dot, so `24.5rem` compares as 24.
        assert_eq!(leading_integer("24.5rem"), Some(24));
        // ...and a leading dot has no integer part at all.
        assert_eq!(leading_integer(".5rem"), None);
        assert_eq!(leading_integer("px"), None);
        assert_eq!(leading_integer(""), None);
    }

    fn unit_of(value: &str) -> String {
        unit_chars(value).collect()
    }

    #[test]
    fn unit_chars_strip_digits_or_stop_at_a_paren() {
        assert_eq!(unit_of("40rem"), "rem");
        assert_eq!(unit_of("1.5rem"), "rem");
        assert_eq!(unit_of("15xyz"), "xyz");
        assert_eq!(unit_of("100%"), "%");
        // A functional value keeps its digits and cuts at the paren.
        assert_eq!(unit_of("calc(100px+1px)"), "calc");
    }

    #[test]
    fn length_values_compare_by_unit_text_then_magnitude() {
        // `px` < `rem` textually, so pixel sizes group before rem sizes
        // regardless of magnitude...
        assert_eq!(
            compare_length_values("600px", "40rem", true),
            Ordering::Less
        );
        // ...and descending only reverses the magnitude, never the unit.
        assert_eq!(
            compare_length_values("900px", "24rem", false),
            Ordering::Less
        );
        assert_eq!(
            compare_length_values("900px", "100px", false),
            Ordering::Less
        );
        // Equal leading integers tie even when the text differs; the tie
        // shares a rank.
        assert_eq!(
            compare_length_values("24rem", "24.5rem", true),
            Ordering::Equal
        );
        // A side with no leading integer falls back to plain text.
        assert_eq!(
            compare_length_values(".5rem", "24rem", true),
            Ordering::Less
        );
    }

    fn decoded(text: &str) -> String {
        decoded_chars(text.chars()).collect()
    }

    #[test]
    fn decoded_chars_unescape_underscores() {
        assert_eq!(decoded("&_p"), "& p");
        assert_eq!(decoded(r"&\_p"), "&_p");
        assert_eq!(decoded(r"a\__b"), "a_ b");
        // A space compares below `.` and `>`, giving Tailwind's
        // `[&_p]` < `[&.a]` < `[&>p]` selector order.
        assert!(decoded("&_p") < decoded("&.a"));
        assert!(decoded("&.a") < decoded("&>p"));
    }
}

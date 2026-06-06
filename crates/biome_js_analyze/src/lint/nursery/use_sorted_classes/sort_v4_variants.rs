use std::{cmp::Ordering, collections::HashMap};

use biome_rowan::{AstNode, SyntaxNodeText};
use biome_tailwind_syntax::{
    AnyTwVariant, AnyTwVariantSegment, TwFullCandidate, TwVariantSegmentList,
};

use super::tailwind_preset_v4::{BREAKPOINT_VALUES, CONTAINER_VALUES, VARIANTS};
use super::tailwind_preset_v4_types::{VariantCompare, VariantEntry, VariantKind};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(super) struct VariantBits(Vec<u64>);

impl VariantBits {
    fn set(&mut self, index: usize) {
        let word = index / 64;
        if self.0.len() <= word {
            self.0.resize(word + 1, 0);
        }
        self.0[word] |= 1u64 << (index % 64);
    }

    pub(super) fn cmp_numeric(&self, other: &Self) -> Ordering {
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

fn trimmed_word_len(words: &[u64]) -> usize {
    words
        .iter()
        .rposition(|word| *word != 0)
        .map_or(0, |index| index + 1)
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(super) enum VariantKey {
    Static(Box<str>),
    Functional {
        root: Box<str>,
        value: Option<VariantValue>,
    },
    Compound {
        root: Box<str>,
        variant: Box<Self>,
    },
    Arbitrary(Box<str>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(super) enum VariantValue {
    Named(Box<str>),
    Arbitrary(Box<str>),
}

#[derive(Clone, Debug)]
enum VariantSegment {
    Named(Box<str>),
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

    pub(super) fn bits_for(&self, variants: &[VariantKey]) -> Option<VariantBits> {
        let mut bits = VariantBits::default();
        for variant in variants {
            bits.set(*self.groups.get(variant)?);
        }
        Some(bits)
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
            let raw = syntax_text_to_box(&expression.syntax().text_trimmed());
            if VARIANTS
                .get(raw.as_ref())
                .is_some_and(|entry| entry.kind == VariantKind::Static)
            {
                return Some(VariantKey::Static(raw));
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
                VariantSegment::Named(segment.value_token().ok()?.text_trimmed().into())
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
                    value: variant_value_from_segments(value_segments)?,
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
) -> Option<(Box<str>, &'static VariantEntry, &[VariantSegment])> {
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

        if let Some(entry) = VARIANTS.get(root.as_str()) {
            best = Some((root.len(), entry, index + 1));
        }
    }

    let (root_len, entry, rest_index) = best?;
    root.truncate(root_len);
    Some((root.into_boxed_str(), entry, &segments[rest_index..]))
}

fn variant_value_from_segments(segments: &[VariantSegment]) -> Option<Option<VariantValue>> {
    match segments {
        [] => None,
        [VariantSegment::Named(value)] => Some(Some(VariantValue::Named(value.clone()))),
        [VariantSegment::Arbitrary(value)] => Some(Some(VariantValue::Arbitrary(value.clone()))),
        [VariantSegment::CssVariable] => None,
        _ => None,
    }
}

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
        VariantKey::Static(root) => root.as_ref(),
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

    parse_length_value(resolved)
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
            Self::Named(value) | Self::Arbitrary(value) => value,
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Named(left), Self::Named(right))
            | (Self::Arbitrary(left), Self::Arbitrary(right)) => left.cmp(right),
            (Self::Named(_), Self::Arbitrary(_)) => Ordering::Less,
            (Self::Arbitrary(_), Self::Named(_)) => Ordering::Greater,
        }
    }
}

fn syntax_text_to_box(text: &SyntaxNodeText) -> Box<str> {
    let mut result = String::with_capacity(usize::from(text.len()));
    text.for_each_chunk(|chunk| result.push_str(chunk));
    result.into_boxed_str()
}

use biome_rowan::{AstNode, SyntaxNodeText, TextSize};
use biome_tailwind_syntax::{
    AnyTwCandidate, AnyTwFullCandidate, AnyTwVariant, TwFullCandidate, TwRoot,
};
use std::cmp::Ordering;
use std::sync::LazyLock;

use super::presets::{ConfigPreset, UseSortedClassesPreset, get_config_preset};

static PRESET: LazyLock<ConfigPreset> =
    LazyLock::new(|| get_config_preset(&UseSortedClassesPreset::default()));

/// Sort the candidates of a parsed Tailwind class list and return the joined,
/// space-separated result.
pub fn sort_class_list(root: &TwRoot) -> String {
    let mut unknown: Vec<SyntaxNodeText> = Vec::new();
    let mut known: Vec<SortKey> = Vec::new();

    for candidate in root.candidates() {
        match candidate {
            AnyTwFullCandidate::TwBogusCandidate(node) => {
                unknown.push(node.syntax().text_trimmed());
            }
            AnyTwFullCandidate::TwFullCandidate(node) => {
                let text = node.syntax().text_trimmed();
                match SortKey::from_candidate(text.clone(), &node) {
                    Some(key) => known.push(key),
                    None => unknown.push(text),
                }
            }
        }
    }

    // Pre-sort lexicographically so that classes with identical sort keys keep
    // a deterministic alphabetical order.
    known.sort_unstable_by(|a, b| a.text.chars().cmp(b.text.chars()));
    // Stable 6-step compare.
    known.sort_by(SortKey::compare);

    // Concatenate directly into the output buffer so that each candidate's
    // source bytes are copied exactly once.
    let mut result = String::new();
    for text in unknown.iter().chain(known.iter().map(|k| &k.text)) {
        if !result.is_empty() {
            result.push(' ');
        }
        text.for_each_chunk(|chunk| result.push_str(chunk));
    }
    result
}

/// Sort information extracted from a parsed `TwFullCandidate`.
struct SortKey {
    /// Source-backed handle to the original class text. Used as the output
    /// payload and as the lexicographic fallback for ties.
    text: SyntaxNodeText,
    /// Source-backed texts of arbitrary `[...]:` variants, in source order.
    arbitrary_variants: Vec<SyntaxNodeText>,
    /// Sorted, deduplicated indices of recognized variants in
    /// [`PRESET::variants`].
    variant_indices: Vec<usize>,
    /// Index of the layer this utility belongs to inside [`PRESET::utilities`].
    /// `utilities.len()` is reserved for arbitrary `[property:value]` utilities.
    layer_index: usize,
    /// Index of the matched utility entry inside its layer. Arbitrary utilities
    /// always use `0`.
    utility_index: usize,
}

impl SortKey {
    fn from_candidate(text: SyntaxNodeText, node: &TwFullCandidate) -> Option<Self> {
        let mut arbitrary_variants: Vec<SyntaxNodeText> = Vec::new();
        let mut variant_indices: Vec<usize> = Vec::new();

        for variant in node.variants() {
            let Ok(variant) = variant else { continue };
            match variant {
                AnyTwVariant::TwArbitraryVariant(v) => {
                    arbitrary_variants.push(v.syntax().text_trimmed());
                }
                AnyTwVariant::TwStaticVariant(v) => {
                    let name = v.syntax().text_trimmed();
                    if let Some(idx) = locate_variant(&name) {
                        variant_indices.push(idx);
                    }
                }
                AnyTwVariant::TwFunctionalVariant(v) => {
                    let name = v.syntax().text_trimmed();
                    if let Some(idx) = locate_variant(&name) {
                        variant_indices.push(idx);
                    }
                }
                AnyTwVariant::TwDataAttribute(v) => {
                    let name = v.syntax().text_trimmed();
                    if let Some(idx) = locate_variant(&name) {
                        variant_indices.push(idx);
                    }
                }
                AnyTwVariant::TwBogusVariant(_) => {}
            }
        }

        // Dedupe + sort so the comparator can walk indices in order.
        variant_indices.sort_unstable();
        variant_indices.dedup();

        let candidate = node.candidate().ok()?;
        let (layer_index, utility_index) = locate_utility(&candidate)?;

        Some(Self {
            text,
            arbitrary_variants,
            variant_indices,
            layer_index,
            utility_index,
        })
    }

    fn compare(a: &Self, b: &Self) -> Ordering {
        // Step 1: classes with arbitrary variants go last.
        match (
            a.arbitrary_variants.is_empty(),
            b.arbitrary_variants.is_empty(),
        ) {
            (false, true) => return Ordering::Greater,
            (true, false) => return Ordering::Less,
            _ => {}
        }

        // Step 2: compare arbitrary variants by count, then lexicographically.
        if !a.arbitrary_variants.is_empty() {
            let len_cmp = a
                .arbitrary_variants
                .len()
                .cmp(&b.arbitrary_variants.len());
            if len_cmp != Ordering::Equal {
                return len_cmp;
            }
            for (av, bv) in a.arbitrary_variants.iter().zip(&b.arbitrary_variants) {
                let lex_cmp = av.chars().cmp(bv.chars());
                if lex_cmp != Ordering::Equal {
                    return lex_cmp;
                }
            }
        }

        // Step 3: classes with no variants go before classes with variants.
        match (a.variant_indices.is_empty(), b.variant_indices.is_empty()) {
            (false, true) => return Ordering::Greater,
            (true, false) => return Ordering::Less,
            _ => {}
        }

        // Step 4: compare layer indices.
        let layer_cmp = a.layer_index.cmp(&b.layer_index);
        if layer_cmp != Ordering::Equal {
            return layer_cmp;
        }

        // Step 5: compare variant weight.
        if !a.variant_indices.is_empty() {
            let weight_cmp = compare_variant_indices(&a.variant_indices, &b.variant_indices);
            if weight_cmp != Ordering::Equal {
                return weight_cmp;
            }
        }

        // Step 6: compare utility indices within the same layer.
        a.utility_index.cmp(&b.utility_index)
    }
}

/// Compares two sorted, unique `variant_indices` arrays as if they were the
/// bit-positions of a `Lsb0` bitvector.
///
/// First by length (= max index + 1), then by walking positions: the smallest
/// index that appears in only one of the two sets wins (the side that has it
/// goes later, mirroring the `T > F` element-wise comparison of the original
/// `BitVec`).
fn compare_variant_indices(a: &[usize], b: &[usize]) -> Ordering {
    let a_max = a.last().copied().unwrap_or(0);
    let b_max = b.last().copied().unwrap_or(0);
    let max_cmp = a_max.cmp(&b_max);
    if max_cmp != Ordering::Equal {
        return max_cmp;
    }

    let mut i = 0;
    let mut j = 0;
    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            // a has a bit set at a smaller position than b → a > b.
            Ordering::Less => return Ordering::Greater,
            Ordering::Greater => return Ordering::Less,
            Ordering::Equal => {
                i += 1;
                j += 1;
            }
        }
    }
    if i < a.len() {
        return Ordering::Greater;
    }
    if j < b.len() {
        return Ordering::Less;
    }
    Ordering::Equal
}

/// Resolves the layer/utility position of a candidate against the preset.
///
/// Returns `None` if the utility is not recognized (the caller treats it as a
/// custom class and preserves source order).
fn locate_utility(candidate: &AnyTwCandidate) -> Option<(usize, usize)> {
    if matches!(candidate, AnyTwCandidate::TwArbitraryCandidate(_)) {
        // Arbitrary `[property:value]` utilities live in a synthetic layer that
        // sorts after every named layer.
        return Some((PRESET.utilities.len(), 0));
    }

    let utility_text = candidate.syntax().text_trimmed();

    let mut best: Option<(usize, usize)> = None;
    let mut best_target_len = 0usize;

    for (layer_idx, layer) in PRESET.utilities.iter().enumerate() {
        for (idx, &target) in layer.classes.iter().enumerate() {
            match match_utility(target, &utility_text) {
                UtilityHit::Exact => return Some((layer_idx, idx)),
                UtilityHit::Partial(target_len) => {
                    if target_len > best_target_len {
                        best = Some((layer_idx, idx));
                        best_target_len = target_len;
                    }
                }
                UtilityHit::None => {}
            }
        }
    }
    best
}

enum UtilityHit {
    Exact,
    Partial(usize),
    None,
}

/// Mirrors the preset format:
/// - `name$` → exact match against `name`.
/// - `name`  → partial match: the utility text must start with `name` and be
///   strictly longer than it.
///
/// The parser splits `negative_token` from the candidate, so `utility_text` is
/// never prefixed with `-`. The preset itself has no negative entries either,
/// so both sides are clean.
fn match_utility(target: &str, utility_text: &SyntaxNodeText) -> UtilityHit {
    if let Some(exact) = target.strip_suffix('$') {
        if utility_text == exact {
            return UtilityHit::Exact;
        }
        return UtilityHit::None;
    }

    if utility_text.len() > TextSize::of(target) && utility_text.starts_with(target) {
        return UtilityHit::Partial(target.len());
    }

    UtilityHit::None
}

/// Resolves the position of a variant name inside the preset's variant list.
///
/// Three matching paths:
/// - Exact equality (`hover` == `hover`) → return immediately.
/// - `target` followed by `-[...]` (e.g. `group-[:visited]` matches `group`)
///   → also exact, return immediately.
/// - `target` is a prefix of `variant_text` with any other suffix (e.g. `peer`
///   matches `peer-has-[:checked]`) → tracked as a partial match. The loop
///   keeps going to prefer the *longest* matching target, so a more specific
///   entry like `peer-has` wins over the bare `peer` when both are present.
fn locate_variant(variant_text: &SyntaxNodeText) -> Option<usize> {
    let mut best: Option<usize> = None;
    let mut best_target_len = 0usize;

    for (idx, &target) in PRESET.variants.iter().enumerate() {
        if variant_text == target {
            return Some(idx);
        }
        let target_len = TextSize::of(target);
        if variant_text.len() > target_len && variant_text.starts_with(target) {
            if variant_text.slice(target_len..).starts_with("-[") {
                return Some(idx);
            }
            if target.len() > best_target_len {
                best = Some(idx);
                best_target_len = target.len();
            }
        }
    }

    best
}


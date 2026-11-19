//! Variant weighting for the Tailwind v4 class sorter.
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
//! 1. Resolve each candidate's variants into [VariantKey]s.
//! 2. Collect every distinct key across the list into [VariantGroups]:
//!    sort them by [VariantKey]'s `Ord` (Tailwind's variant order) and
//!    give each an ascending group index.
//! 3. Each candidate's [VariantWeight] is the set of group indices its
//!    variants land in — a bitset compared as one big number. No
//!    variants means zero, which sorts first; a higher-ordered variant
//!    sets a higher bit, so the weight is larger and sorts later. This
//!    mirrors how Tailwind ranks variant combinations.

use std::cmp::Ordering;

use smallvec::SmallVec;

use crate::tailwind_v4::VariantKey;

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

pub(super) struct VariantGroups {
    /// Distinct variant keys in ascending order; a key's rank — its
    /// weight-bit index — is its position here.
    ranked: Vec<VariantKey>,
}

impl VariantGroups {
    pub(super) fn new<'a>(variants: impl IntoIterator<Item = &'a VariantKey>) -> Self {
        // Each distinct key's index is its rank. Equality is defined as
        // comparing equal, so `dedup` collapses exactly the keys Tailwind
        // gives one shared rank (`@sm/main:` with `@sm:`).
        let mut ranked: Vec<VariantKey> = variants.into_iter().cloned().collect();
        ranked.sort();
        ranked.dedup();
        Self { ranked }
    }

    /// Returns `None` if a variant was not part of the list the ranks
    /// were built from. Unreachable when the ranks come from these same
    /// candidates, so callers fold it into `Unknown`.
    pub(super) fn weight_for(&self, variants: &[VariantKey]) -> Option<VariantWeight> {
        let mut weight = VariantWeight::default();
        for variant in variants {
            weight.set(self.ranked.binary_search(variant).ok()?);
        }
        Some(weight)
    }
}

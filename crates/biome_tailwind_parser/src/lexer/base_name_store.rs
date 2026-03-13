use std::sync::LazyLock;

use biome_tailwind_syntax::metadata::BASENAMES_WITH_DASHES;
use biome_unicode_table::{Dispatch, Dispatch::*, lookup_byte};

/// A global store of dashed basenames for efficient reuse across lexing.
///
/// Ideally, this will be built by parsing a project's tailwind config, since it can add custom utilities and variants.
pub static BASENAME_STORE: LazyLock<BaseNameStore> =
    LazyLock::new(|| BaseNameStore::new(BASENAMES_WITH_DASHES));

/// A compact trie storing all dashed basenames to efficiently match the longest valid basename
/// at the beginning of a byte slice.
///
/// Build it once from `BASENAMES_WITH_DASHES` and reuse it across lexing.
pub(crate) struct BaseNameStore {
    nodes: Vec<Node>,
}

#[derive(Default)]
struct Node {
    terminal: bool,
    // Children are stored as (byte, child_index)
    children: Vec<(u8, usize)>,
}

impl BaseNameStore {
    /// Creates a store from a list of ASCII basenames.
    pub(crate) fn new(list: &[&str]) -> Self {
        let mut store = Self {
            nodes: vec![Node::default()], // root
        };

        for name in list {
            store.insert(name.as_bytes());
        }

        // Optional: sort children to enable binary search (currently linear search is fine).
        for i in 0..store.nodes.len() {
            let children = &mut store.nodes[i].children;
            children.sort_unstable_by_key(|(b, _)| *b);
            // Deduplicate in case of construction anomalies (shouldn't happen, but cheap insurance)
            children.dedup_by_key(|(b, _)| *b);
        }

        store
    }

    fn insert(&mut self, word: &[u8]) {
        let mut node_idx = 0usize;

        for &b in word {
            node_idx = match self.find_child(node_idx, b) {
                Some(next) => next,
                None => {
                    let next = self.new_node();
                    self.nodes[node_idx].children.push((b, next));
                    next
                }
            };
        }

        self.nodes[node_idx].terminal = true;
    }

    #[inline]
    fn new_node(&mut self) -> usize {
        let next_index = self.nodes.len();
        self.nodes.push(Node::default());
        next_index
    }

    #[inline]
    fn find_child(&self, node: usize, byte: u8) -> Option<usize> {
        // Linear search is fine because children fan-out is small in practice.
        self.nodes[node]
            .children
            .iter()
            .find_map(|(b, idx)| (*b == byte).then_some(*idx))
    }

    /// Creates a matcher for the provided text slice, starting at offset 0.
    #[inline]
    pub(crate) fn matcher<'s, 't>(&'s self, text: &'t [u8]) -> BaseNameMatcher<'s, 't> {
        BaseNameMatcher { store: self, text }
    }
}

/// A streaming helper that scans the provided text slice from the beginning
/// and returns the longest dashed basename prefix end if any.
///
/// It stops scanning when:
/// - It encounters a byte that isn't a path in the trie,
/// - Or it hits a delimiter (`whitespace`, `!`, `:`).
///
/// The returned `usize` is the number of bytes consumed for the matched basename.
/// The caller is responsible for verifying the boundary (e.g. next byte is `-`, `:`, whitespace, or end-of-input).
pub(crate) struct BaseNameMatcher<'s, 't> {
    store: &'s BaseNameStore,
    text: &'t [u8],
}

impl<'s, 't> BaseNameMatcher<'s, 't> {
    pub(crate) fn base_end(&self) -> usize {
        let mut node_idx = 0usize;
        let mut best_end: Option<usize> = None;

        // Scan for dashed basename using the trie until a delimiter or mismatch
        let mut i = 0usize;
        while i < self.text.len() {
            let b = self.text[i];
            let dispatched = lookup_byte(b);
            if is_delimiter(dispatched) {
                break;
            }

            match self.store.find_child(node_idx, b) {
                Some(next) => {
                    node_idx = next;
                    i += 1;
                    // SAFETY: This is safe because `next` will always be a valid index into `self.store.nodes`.
                    // If it isn't, the trie is malformed.
                    if self.store.nodes[node_idx].terminal {
                        best_end = Some(i);
                    }
                }
                None => {
                    break;
                }
            }
        }

        // If we found a dashed match, accept it only if the next byte is a valid boundary or end-of-input
        if let Some(end) = best_end
            && self
                .text
                .get(end)
                .is_none_or(|b| is_boundary_byte(lookup_byte(*b)))
        {
            return end;
        }

        // Fallback: naive basename ends at the first '-', whitespace, '!' or ':'
        let mut j = 0usize;
        while j < self.text.len() {
            let b = self.text[j];
            let dispatched = lookup_byte(b);
            if dispatched == MIN || is_delimiter(dispatched) {
                break;
            }
            j += 1;
        }
        j
    }
}

#[inline]
pub(crate) const fn is_delimiter(b: Dispatch) -> bool {
    // Delimiters that cannot be part of a basename (excluding '-' which may be inside dashed basenames):
    // - whitespace
    // - '!' important modifier
    // - ':' variant separator
    matches!(b, WHS | EXL | COL)
}

#[inline]
const fn is_boundary_byte(b: Dispatch) -> bool {
    // Valid boundary after a dashed basename:
    // - '-' indicates a value follows
    // - ':' indicates a variant boundary
    // - whitespace
    matches!(b, WHS | MIN | COL)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_longest_prefix() {
        let store = BaseNameStore::new(&["border-t", "border", "mask-radial-at"]);
        assert_eq!(
            store.matcher(b"border-t-red-300").base_end(),
            "border-t".len()
        );
        assert_eq!(
            store.matcher(b"mask-radial-at-top").base_end(),
            "mask-radial-at".len()
        );
        // Does not match undashed names that aren't present
        assert_eq!(store.matcher(b"bg-red-500").base_end(), "bg".len());
    }

    #[test]
    fn respects_delimiters() {
        let store = BaseNameStore::new(&["nth-last", "rounded-bl"]);
        assert_eq!(store.matcher(b"nth-last:odd").base_end(), "nth-last".len());
        assert_eq!(
            store.matcher(b"rounded-bl-lg").base_end(),
            "rounded-bl".len()
        );
    }

    #[test]
    fn stops_on_non_edge() {
        let store = BaseNameStore::new(&["border-t"]);
        // No edge for 'f' after "border-t", so it shouldn't match "border-tf"
        assert_eq!(store.matcher(b"border-tfoo").base_end(), "border".len());
        // If the very first char doesn't match, None
        assert_eq!(store.matcher(b"bg-red-500").base_end(), "bg".len());
    }
}

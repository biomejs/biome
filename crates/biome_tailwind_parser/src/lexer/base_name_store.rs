use std::sync::LazyLock;

use biome_tailwind_syntax::metadata::BASENAMES_WITH_DASHES;
use biome_unicode_table::{Dispatch, Dispatch::*, lookup_byte};

/// A global store of dashed basenames for efficient reuse across lexing.
///
/// It holds Tailwind's built-in basenames. A project's stylesheet can add utilities, so a store
/// that also holds those can be passed through [TailwindParserOptions](crate::TailwindParserOptions).
pub static BASENAME_STORE: LazyLock<BaseNameStore> =
    LazyLock::new(|| BaseNameStore::new(BASENAMES_WITH_DASHES));

/// A compact trie storing all dashed basenames to efficiently match the longest valid basename
/// at the beginning of a byte slice.
///
/// Build it once from `BASENAMES_WITH_DASHES` and reuse it across lexing.
#[derive(Debug, Eq, PartialEq)]
pub struct BaseNameStore {
    /// The nodes of the trie, the root first.
    nodes: Vec<Node>,
    /// The bytes of the edges to the children of all nodes. The edges of each node are
    /// contiguous and sorted.
    child_bytes: Vec<u8>,
    /// The node that each edge in `child_bytes` leads to.
    child_nodes: Vec<u32>,
    /// The nodes of the paths of up to [MAX_FIRST_SEGMENT] bytes that end at their first '-',
    /// by [first_segment_key], in an open-addressing table whose length is a power of two.
    first_segments: Vec<(u128, u32)>,
}

/// The longest path that [BaseNameStore::first_segments] holds.
const MAX_FIRST_SEGMENT: usize = 15;

/// Returns the key of `path` in [BaseNameStore::first_segments]: its bytes and its length.
#[inline(always)]
fn first_segment_key(path: &[u8]) -> u128 {
    debug_assert!(path.len() <= MAX_FIRST_SEGMENT);
    let mut bytes = [0; 16];
    bytes[..path.len()].copy_from_slice(path);
    bytes[15] = path.len() as u8;
    u128::from_le_bytes(bytes)
}

/// Returns the slot of `key` in a [BaseNameStore::first_segments] table of `len` slots.
#[inline(always)]
fn first_segment_slot(key: u128, len: usize) -> usize {
    let hash = ((key as u64) ^ ((key >> 64) as u64)).wrapping_mul(0x9E37_79B9_7F4A_7C15);
    (hash >> 32) as usize & (len - 1)
}

#[derive(Debug, Eq, PartialEq)]
struct Node {
    terminal: bool,
    /// The index of the node's first edge in [BaseNameStore::child_bytes].
    first_child: u32,
    child_count: u32,
}

impl BaseNameStore {
    /// Creates a store from a list of ASCII basenames.
    pub fn new(list: &[&str]) -> Self {
        // Build the trie with a list of children per node, then lay out the children of all
        // nodes in one array.
        let mut terminal = vec![false];
        let mut children: Vec<Vec<(u8, u32)>> = vec![Vec::new()];
        for name in list {
            let mut node = 0;
            for &byte in name.as_bytes() {
                node = match children[node].iter().find(|(edge, _)| *edge == byte) {
                    Some(&(_, child)) => child as usize,
                    None => {
                        let child = terminal.len();
                        terminal.push(false);
                        children.push(Vec::new());
                        children[node].push((byte, child as u32));
                        child
                    }
                };
            }
            terminal[node] = true;
        }

        let mut store = Self {
            nodes: Vec::with_capacity(terminal.len()),
            child_bytes: Vec::with_capacity(terminal.len()),
            child_nodes: Vec::with_capacity(terminal.len()),
            first_segments: Vec::new(),
        };
        for (terminal, mut children) in terminal.into_iter().zip(children) {
            children.sort_unstable_by_key(|(byte, _)| *byte);
            store.nodes.push(Node {
                terminal,
                first_child: store.child_bytes.len() as u32,
                child_count: children.len() as u32,
            });
            store
                .child_bytes
                .extend(children.iter().map(|(byte, _)| *byte));
            store
                .child_nodes
                .extend(children.iter().map(|(_, child)| *child));
        }
        store.index_first_segments();
        store
    }

    /// Fills [Self::first_segments] with the paths that end at their first '-'.
    fn index_first_segments(&mut self) {
        let mut paths = Vec::new();
        let mut stack = vec![(0usize, Vec::new())];
        while let Some((node, path)) = stack.pop() {
            if path.len() > MAX_FIRST_SEGMENT {
                continue;
            }
            if path.last() == Some(&b'-') {
                paths.push((first_segment_key(&path), node as u32));
                continue;
            }
            let first = self.nodes[node].first_child as usize;
            for edge in first..first + self.nodes[node].child_count as usize {
                let mut child_path = path.clone();
                child_path.push(self.child_bytes[edge]);
                stack.push((self.child_nodes[edge] as usize, child_path));
            }
        }
        let len = (paths.len() * 2).next_power_of_two().max(1);
        self.first_segments = vec![(0, 0); len];
        for (key, node) in paths {
            let mut slot = first_segment_slot(key, len);
            while self.first_segments[slot].0 != 0 {
                slot = (slot + 1) & (len - 1);
            }
            self.first_segments[slot] = (key, node);
        }
    }

    /// Returns the node of `path`, which ends at its first '-'.
    #[inline]
    fn first_segment_node(&self, path: &[u8]) -> Option<usize> {
        if path.len() > MAX_FIRST_SEGMENT {
            return path
                .iter()
                .try_fold(0, |node, &byte| self.find_child(node, byte));
        }
        let key = first_segment_key(path);
        let len = self.first_segments.len();
        let mut slot = first_segment_slot(key, len);
        loop {
            let (slot_key, node) = self.first_segments[slot];
            if slot_key == key {
                return Some(node as usize);
            }
            if slot_key == 0 {
                return None;
            }
            slot = (slot + 1) & (len - 1);
        }
    }

    #[inline]
    fn find_child(&self, node: usize, byte: u8) -> Option<usize> {
        let node = &self.nodes[node];
        let first = node.first_child as usize;
        let bytes = &self.child_bytes[first..first + node.child_count as usize];
        // Linear search is fine because children fan-out is small in practice.
        let index = bytes.iter().position(|&edge| edge == byte)?;
        Some(self.child_nodes[first + index] as usize)
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
    #[cfg(test)]
    pub(crate) fn base_end(&self) -> usize {
        // The naive basename ends at the first '-', whitespace, '!', ':' or '/'.
        let naive = self
            .text
            .iter()
            .position(|&byte| {
                let dispatched = lookup_byte(byte);
                dispatched == MIN || is_delimiter(dispatched)
            })
            .unwrap_or(self.text.len());
        self.dashed_end(naive)
    }

    /// Returns [Self::base_end], given `naive`, the offset of the first '-' or delimiter in the
    /// text, or its length if it has none.
    #[inline]
    pub(crate) fn dashed_end(&self, naive: usize) -> usize {
        // The longest dashed basename that is a prefix of the text decides the basename. If it
        // ends before `naive`, it isn't followed by a boundary byte, and if it ends at `naive`,
        // it is the naive basename. Either way, the basename ends at `naive`, so only the
        // dashed basenames that continue past the first '-' matter.
        if self.text.get(naive) != Some(&b'-') {
            return naive;
        }
        let Some(mut node_idx) = self.store.first_segment_node(&self.text[..=naive]) else {
            return naive;
        };

        let mut best_end: Option<usize> = None;

        // Scan for dashed basename using the trie until a delimiter or mismatch
        let mut i = naive + 1;
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
        match best_end {
            Some(end)
                if self
                    .text
                    .get(end)
                    .is_none_or(|b| is_boundary_byte(lookup_byte(*b))) =>
            {
                end
            }
            _ => naive,
        }
    }
}

#[inline]
pub(crate) const fn is_delimiter(b: Dispatch) -> bool {
    // Delimiters that cannot be part of a basename (excluding '-' which may be inside dashed basenames):
    // - whitespace
    // - '!' important modifier
    // - ':' variant separator
    // - '/' modifier separator (`@container/sidebar`, `flex/50`)
    matches!(b, WHS | EXL | COL | SLH)
}

#[inline]
const fn is_boundary_byte(b: Dispatch) -> bool {
    // Valid boundary after a dashed basename:
    // - '-' indicates a value follows
    // - ':' indicates a variant boundary
    // - '/' indicates a modifier follows (`drop-shadow/50`)
    // - '!' indicates the important flag follows (`drop-shadow!`)
    // - whitespace
    matches!(b, WHS | MIN | COL | SLH | EXL)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::BASENAME_STORE;

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

    /// The matching that [BaseNameMatcher::base_end] must equal: walk the trie from the start,
    /// keep the longest basename, and fall back to the first '-' or delimiter unless a boundary
    /// byte follows it.
    fn reference_base_end(store: &BaseNameStore, text: &[u8]) -> usize {
        let mut node_idx = 0;
        let mut best_end = None;
        let mut i = 0;
        while i < text.len() && !is_delimiter(lookup_byte(text[i])) {
            match store.find_child(node_idx, text[i]) {
                Some(next) => {
                    node_idx = next;
                    i += 1;
                    if store.nodes[node_idx].terminal {
                        best_end = Some(i);
                    }
                }
                None => break,
            }
        }
        if let Some(end) = best_end
            && text
                .get(end)
                .is_none_or(|b| is_boundary_byte(lookup_byte(*b)))
        {
            return end;
        }
        text.iter()
            .position(|&b| lookup_byte(b) == MIN || is_delimiter(lookup_byte(b)))
            .unwrap_or(text.len())
    }

    #[test]
    fn matches_the_reference_at_every_offset() {
        let custom = BaseNameStore::new(&[
            "a-b",
            "a-b-c",
            "-x",
            "x",
            "border-t",
            "border-tf",
            "abcdefghijklmnop-q",
            "abcdefghijklmno-q",
            "\u{e9}-a",
        ]);
        let text = "a-b-cx a-b-c a-b a-bc -x-y x-y border-tfoo border-t-red bg-red-500 \
            mask-radial-at-top inset-x-0 max-w-7xl gap-x-6 text-shadow-lg/50 nth-last:odd \
            translate-x-1/2 data-[a]:x \u{e9}-a group-hover/menu:flex abcdefghijklmnop-q-r \
            abcdefghijklmno-q-r abcdefghijklmn-q";
        for store in [&*BASENAME_STORE, &custom] {
            for start in 0..text.len() {
                for end in start..=text.len() {
                    let slice = &text.as_bytes()[start..end];
                    assert_eq!(
                        store.matcher(slice).base_end(),
                        reference_base_end(store, slice),
                        "{:?}",
                        String::from_utf8_lossy(slice)
                    );
                }
            }
        }
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

use biome_text_size::TextSize;
use hashbrown::hash_map::{RawEntryMut, RawOccupiedEntryMut, RawVacantEntryMut};
use rustc_hash::FxHasher;
use std::fmt::{self, Debug, Formatter};
use std::hash::{BuildHasherDefault, Hash, Hasher};
use std::marker::PhantomData;
use std::ops::Not;
use std::ptr::NonNull;

use crate::green::Slot;
use crate::syntax::{TriviaPiece, TriviaPieceKind};
use crate::{
    green::GreenElementRef, GreenNode, GreenNodeData, GreenToken, GreenTokenData, NodeOrToken,
    RawSyntaxKind,
};

use super::element::GreenElement;
use super::trivia::{GreenTrivia, GreenTriviaData};

type HashMap<K, V> = hashbrown::HashMap<K, V, BuildHasherDefault<FxHasher>>;

/// Internal representation for a green pointer and a generation index in the
/// cache, packed into a single `usize`. This relies on the fact that "green
/// elements" (tokens, nodes and trivia) have memory alignment constraints that
/// exceed a single byte (and thus the lower bits of the pointer will always be
/// zero), while the generation index only needs a single bit of storage.
struct GenerationalPointer<T: IntoRawPointer> {
    data: usize,
    _ty: PhantomData<T>,
}

impl<T: IntoRawPointer> GenerationalPointer<T> {
    fn new(value: T, generation: Generation) -> Self {
        let ptr = value.into_raw();
        let mut data = ptr as usize;
        debug_assert!(data & 1 == 0);
        data |= generation as usize;
        Self {
            data,
            _ty: PhantomData,
        }
    }

    fn value(&self) -> &T::Pointee {
        // SAFETY: This clears the least significant bit from `data`. This bit
        // should have been set to zero in the original pointer due to the
        // alignment requirements of the underlying data (this is checked by an
        // assertion on debug builds), so this essentially extracts the pointer
        // value from the bit field. Said point is safe to dereference at this
        // point since we're holding a valid reference to `self` which
        // guarantees `Drop` has not been called and the memory associated with
        // the pointer has not been released yet.
        let data = self.data & !1;
        let ptr = data as *const T::Pointee;
        unsafe { &*ptr }
    }

    fn generation(&self) -> Generation {
        match self.data & 1 {
            0 => Generation::A,
            1 => Generation::B,
            // SAFETY: The `& 1` operation above ensures only the least
            // significant bit can be set
            _ => unreachable!(),
        }
    }

    fn set_generation(&mut self, generation: Generation) {
        let data = self.data & !1;
        self.data = data | generation as usize;
    }
}

impl<T: IntoRawPointer> Debug for GenerationalPointer<T>
where
    T::Pointee: Debug,
{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("GenerationalPointer")
            .field("value", self.value())
            .field("generation", &self.generation())
            .finish()
    }
}

impl<T: IntoRawPointer> Drop for GenerationalPointer<T> {
    fn drop(&mut self) {
        let ptr = self.value() as *const _ as *mut _;
        let value = unsafe { T::from_raw(ptr) };
        drop(value);
    }
}

/// Trait implemented for types that can be turned into a raw pointer, and
/// reconstructed back from it. Used by [GenerationalPointer] internally.
trait IntoRawPointer {
    type Pointee;
    fn into_raw(self) -> *mut Self::Pointee;
    unsafe fn from_raw(ptr: *mut Self::Pointee) -> Self;
}

/// A token stored in the `NodeCache`.
/// Does intentionally not implement `Hash` to have compile-time guarantees that the `NodeCache`
/// uses the correct hash.
#[derive(Debug)]
struct CachedToken(GenerationalPointer<GreenToken>);

impl IntoRawPointer for GreenToken {
    type Pointee = GreenTokenData;

    fn into_raw(self) -> *mut Self::Pointee {
        GreenToken::into_raw(self).as_ptr()
    }

    unsafe fn from_raw(ptr: *mut Self::Pointee) -> Self {
        GreenToken::from_raw(NonNull::new(ptr).unwrap())
    }
}

/// A node stored in the `NodeCache`. It stores a pre-computed hash
/// because re-computing the hash requires traversing the whole sub-tree.
/// The hash also differs from the `GreenNode` hash implementation as it
/// only hashes occupied slots and excludes empty slots.
///
/// Does intentionally not implement `Hash` to have compile-time guarantees that the `NodeCache`
/// uses the correct hash.
#[derive(Debug)]
struct CachedNode {
    node: GenerationalPointer<GreenNode>,
    // Store the hash as it's expensive to re-compute
    // involves re-computing the hash of the whole sub-tree
    hash: u64,
}

impl IntoRawPointer for GreenNode {
    type Pointee = GreenNodeData;

    fn into_raw(self) -> *mut Self::Pointee {
        GreenNode::into_raw(self).as_ptr()
    }

    unsafe fn from_raw(ptr: *mut Self::Pointee) -> Self {
        GreenNode::from_raw(NonNull::new(ptr).unwrap())
    }
}

/// Interner for GreenTokens and GreenNodes
// XXX: the impl is a bit tricky. As usual when writing interners, we want to
// store all values in one HashSet.
//
// However, hashing trees is fun: hash of the tree is recursively defined. We
// maintain an invariant -- if the tree is interned, then all of its children
// are interned as well.
//
// That means that computing the hash naively is wasteful -- we just *know*
// hashes of children, and we can re-use those.
//
// So here we use *raw* API of hashbrown and provide the hashes manually,
// instead of going via a `Hash` impl. Our manual `Hash` and the
// `#[derive(Hash)]` are actually different! At some point we had a fun bug,
// where we accidentally mixed the two hashes, which made the cache much less
// efficient.
//
// To fix that, we additionally wrap the data in `Cached*` wrappers, to make sure
// we don't accidentally use the wrong hash!
#[derive(Default, Debug)]
pub struct NodeCache {
    nodes: HashMap<CachedNode, ()>,
    tokens: HashMap<CachedToken, ()>,
    trivia: TriviaCache,
    generation: Generation,
}

/// Represents a "generation" in the garbage collection scheme of the node
/// cache. For our purpose we only need to track two generations at most (the
/// previous and next generation) so this is represented as an enum with two
/// variants.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
enum Generation {
    #[default]
    A = 0,
    B = 1,
}

impl Not for Generation {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Generation::A => Generation::B,
            Generation::B => Generation::A,
        }
    }
}

fn token_hash_of(kind: RawSyntaxKind, text: &str) -> u64 {
    let mut h = FxHasher::default();
    kind.hash(&mut h);
    text.hash(&mut h);
    h.finish()
}

fn token_hash(token: &GreenTokenData) -> u64 {
    token_hash_of(token.kind(), token.text())
}

fn element_id(elem: GreenElementRef<'_>) -> *const () {
    match elem {
        NodeOrToken::Node(it) => it as *const GreenNodeData as *const (),
        NodeOrToken::Token(it) => it as *const GreenTokenData as *const (),
    }
}

impl NodeCache {
    /// Hash used for nodes that haven't been cached because it has too many slots or
    /// one of its children wasn't cached.
    const UNCACHED_NODE_HASH: u64 = 0;

    /// Tries to retrieve a node with the given `kind` and `children` from the cache.
    ///
    /// Returns an entry that allows the caller to:
    /// * Retrieve the cached node if it is present in the cache
    /// * Insert a node if it isn't present in the cache
    pub(crate) fn node(
        &mut self,
        kind: RawSyntaxKind,
        children: &[(u64, GreenElement)],
    ) -> NodeCacheNodeEntryMut {
        if children.len() > 3 {
            return NodeCacheNodeEntryMut::NoCache(Self::UNCACHED_NODE_HASH);
        }

        let hash = {
            let mut h = FxHasher::default();
            kind.hash(&mut h);
            for &(hash, _) in children {
                if hash == Self::UNCACHED_NODE_HASH {
                    return NodeCacheNodeEntryMut::NoCache(Self::UNCACHED_NODE_HASH);
                }
                hash.hash(&mut h);
            }
            h.finish()
        };

        // Green nodes are fully immutable, so it's ok to deduplicate them.
        // This is the same optimization that Roslyn does
        // https://github.com/KirillOsenkov/Bliki/wiki/Roslyn-Immutable-Trees
        //
        // For example, all `#[inline]` in this file share the same green node!
        // For `libsyntax/parse/parser.rs`, measurements show that deduping saves
        // 17% of the memory for green nodes!
        let entry = self.nodes.raw_entry_mut().from_hash(hash, |no_hash| {
            no_hash.node.value().kind() == kind && {
                let lhs = no_hash.node.value().slots().filter_map(|slot| match slot {
                    // Ignore empty slots. The queried node only has the present children
                    Slot::Empty { .. } => None,
                    Slot::Node { node, .. } => Some(element_id(NodeOrToken::Node(node))),
                    Slot::Token { token, .. } => Some(element_id(NodeOrToken::Token(token))),
                });

                let rhs = children
                    .iter()
                    .map(|(_, element)| element_id(element.as_deref()));

                lhs.eq(rhs)
            }
        });

        match entry {
            RawEntryMut::Occupied(mut entry) => {
                entry.key_mut().node.set_generation(self.generation);
                NodeCacheNodeEntryMut::Cached(CachedNodeEntry {
                    hash,
                    raw_entry: entry,
                })
            }
            RawEntryMut::Vacant(entry) => NodeCacheNodeEntryMut::Vacant(VacantNodeEntry {
                raw_entry: entry,
                original_kind: kind,
                hash,
                generation: self.generation,
            }),
        }
    }

    pub(crate) fn token(&mut self, kind: RawSyntaxKind, text: &str) -> (u64, GreenToken) {
        self.token_with_trivia(kind, text, &[], &[])
    }

    pub(crate) fn token_with_trivia(
        &mut self,
        kind: RawSyntaxKind,
        text: &str,
        leading: &[TriviaPiece],
        trailing: &[TriviaPiece],
    ) -> (u64, GreenToken) {
        let hash = token_hash_of(kind, text);

        let entry = self.tokens.raw_entry_mut().from_hash(hash, |token| {
            token.0.value().kind() == kind && token.0.value().text() == text
        });

        let token = match entry {
            RawEntryMut::Occupied(mut entry) => {
                entry.key_mut().0.set_generation(self.generation);
                entry.key().0.value().to_owned()
            }
            RawEntryMut::Vacant(entry) => {
                let leading = self.trivia.get(self.generation, leading);
                let trailing = self.trivia.get(self.generation, trailing);

                let token = GreenToken::with_trivia(kind, text, leading, trailing);
                let key = CachedToken(GenerationalPointer::new(token.clone(), self.generation));
                entry.insert_with_hasher(hash, key, (), |t| token_hash(t.0.value()));
                token
            }
        };

        (hash, token)
    }

    /// Increment the generation counter of the cache, all cache access from
    /// this point onward will update the generation of the corresponding entry
    /// to this new value
    pub(crate) fn increment_generation(&mut self) {
        debug_assert!(
            self.nodes
                .keys()
                .all(|entry| entry.node.generation() == self.generation)
                && self
                    .tokens
                    .keys()
                    .all(|token| token.0.generation() == self.generation)
                && self
                    .trivia
                    .cache
                    .keys()
                    .all(|trivia| trivia.0.generation() == self.generation)
        );

        self.generation = !self.generation;
    }

    /// Removes nodes, tokens and trivia entries from the cache when their
    /// generation doesn't match the current generation of the whole cache
    pub(crate) fn retain_cache(&mut self) {
        self.nodes
            .retain(|node, _| node.node.generation() == self.generation);

        self.tokens
            .retain(|token, _| token.0.generation() == self.generation);

        self.trivia
            .cache
            .retain(|trivia, _| trivia.0.generation() == self.generation);
    }
}

pub(crate) enum NodeCacheNodeEntryMut<'a> {
    Cached(CachedNodeEntry<'a>),

    /// A node that should not be cached
    NoCache(u64),
    Vacant(VacantNodeEntry<'a>),
}

/// Represents a vacant entry, a node that hasn't been cached yet.
/// The `insert` method allows to place a node inside of the vacant entry. The inserted node
/// may have a different representation (kind or children) than the originally queried node.
/// For example, a node may change its kind to bogus or add empty slots. The only importance is
/// that these changes apply for all nodes that have the same shape as the originally queried node.
pub(crate) struct VacantNodeEntry<'a> {
    hash: u64,
    original_kind: RawSyntaxKind,
    raw_entry: RawVacantEntryMut<'a, CachedNode, (), BuildHasherDefault<FxHasher>>,
    generation: Generation,
}

/// Represents an entry of a cached node.
pub(crate) struct CachedNodeEntry<'a> {
    hash: u64,
    raw_entry: RawOccupiedEntryMut<'a, CachedNode, (), BuildHasherDefault<FxHasher>>,
}

impl CachedNodeEntry<'_> {
    pub fn node(&self) -> &GreenNodeData {
        self.raw_entry.key().node.value()
    }

    pub fn hash(&self) -> u64 {
        self.hash
    }
}

impl VacantNodeEntry<'_> {
    /// Inserts the `node` into the cache so that future queries for the same kind and children resolve to the passed `node`.
    ///
    /// Returns the hash of the node.
    ///
    /// The cache does not cache the `node` if the kind doesn't match the `kind` of the queried node because
    /// cache lookups wouldn't be successful because the hash collision prevention check compares the kinds of the
    /// cached and queried node.
    pub fn cache(self, node: GreenNode) -> u64 {
        if self.original_kind != node.kind() {
            // The kind has changed since it has been queried. For example, the node has been converted to an
            // unknown node. Never cache these nodes because cache lookups will never match.
            NodeCache::UNCACHED_NODE_HASH
        } else {
            self.raw_entry.insert_with_hasher(
                self.hash,
                CachedNode {
                    node: GenerationalPointer::new(node, self.generation),
                    hash: self.hash,
                },
                (),
                |n| n.hash,
            );
            self.hash
        }
    }
}

/// A cached [GreenTrivia].
/// Deliberately doesn't implement `Hash` to make sure all
/// usages go through the custom `FxHasher`.
#[derive(Debug)]
struct CachedTrivia(GenerationalPointer<GreenTrivia>);

impl IntoRawPointer for GreenTrivia {
    type Pointee = GreenTriviaData;

    fn into_raw(self) -> *mut Self::Pointee {
        GreenTrivia::into_raw(self)
    }

    unsafe fn from_raw(ptr: *mut Self::Pointee) -> Self {
        GreenTrivia::from_raw(ptr)
    }
}

#[derive(Debug)]
struct TriviaCache {
    /// Generic cache for trivia
    cache: HashMap<CachedTrivia, ()>,

    /// Cached single whitespace trivia.
    whitespace: GreenTrivia,
}

impl Default for TriviaCache {
    fn default() -> Self {
        Self {
            cache: Default::default(),
            whitespace: GreenTrivia::new([TriviaPiece::whitespace(1)]),
        }
    }
}

impl TriviaCache {
    /// Tries to retrieve a [GreenTrivia] with the given pieces from the cache or creates a new one and caches
    /// it for further calls.
    fn get(&mut self, generation: Generation, pieces: &[TriviaPiece]) -> GreenTrivia {
        match pieces {
            [] => GreenTrivia::empty(),
            [TriviaPiece {
                kind: TriviaPieceKind::Whitespace,
                length,
            }] if *length == TextSize::from(1) => self.whitespace.clone(),

            _ => {
                let hash = Self::trivia_hash_of(pieces);

                let entry = self
                    .cache
                    .raw_entry_mut()
                    .from_hash(hash, |trivia| trivia.0.value().pieces() == pieces);

                match entry {
                    RawEntryMut::Occupied(mut entry) => {
                        entry.key_mut().0.set_generation(generation);
                        entry.key().0.value().to_owned()
                    }
                    RawEntryMut::Vacant(entry) => {
                        let trivia = GreenTrivia::new(pieces.iter().copied());
                        entry.insert_with_hasher(
                            hash,
                            CachedTrivia(GenerationalPointer::new(trivia.clone(), generation)),
                            (),
                            |cached| Self::trivia_hash_of(cached.0.value().pieces()),
                        );
                        trivia
                    }
                }
            }
        }
    }

    fn trivia_hash_of(pieces: &[TriviaPiece]) -> u64 {
        let mut h = FxHasher::default();

        pieces.len().hash(&mut h);

        for piece in pieces {
            piece.hash(&mut h);
        }

        h.finish()
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use crate::green::node_cache::{token_hash, CachedNode, CachedToken, CachedTrivia};
    use crate::green::trivia::GreenTrivia;
    use crate::{GreenToken, RawSyntaxKind};
    use biome_text_size::TextSize;

    #[test]
    fn green_token_hash() {
        let kind = RawSyntaxKind(0);
        let text = " let ";
        let t1 = GreenToken::with_trivia(
            kind,
            text,
            GreenTrivia::whitespace(TextSize::from(1)),
            GreenTrivia::whitespace(TextSize::from(1)),
        );
        let t2 = GreenToken::with_trivia(
            kind,
            text,
            GreenTrivia::whitespace(1),
            GreenTrivia::whitespace(1),
        );

        assert_eq!(token_hash(&t1), token_hash(&t2));

        let t3 = GreenToken::new(kind, "let");
        assert_ne!(token_hash(&t1), token_hash(&t3));

        let t4 = GreenToken::with_trivia(
            kind,
            "\tlet ",
            GreenTrivia::whitespace(1),
            GreenTrivia::whitespace(1),
        );
        assert_ne!(token_hash(&t1), token_hash(&t4));
    }

    #[test]
    fn cache_entry_size() {
        assert_eq!(size_of::<CachedNode>(), 16);
        assert_eq!(size_of::<CachedToken>(), 8);
        assert_eq!(size_of::<CachedTrivia>(), 8);
    }
}

use biome_text_size::TextSize;
use hashbrown::hash_map::{RawEntryMut, RawOccupiedEntryMut, RawVacantEntryMut};
use rustc_hash::FxHasher;
use std::fmt::{self, Debug, Formatter};
use std::hash::{BuildHasherDefault, Hash, Hasher};
use std::ptr::NonNull;

use crate::green::Slot;
use crate::syntax::{TriviaPiece, TriviaPieceKind};
use crate::{
    GreenNode, GreenNodeData, GreenToken, GreenTokenData, NodeOrToken, RawSyntaxKind,
    green::{ArenaRef, GreenArena, GreenElementFlags, GreenElementRef},
};

use super::element::GreenElement;
use super::trivia::{GreenTrivia, GreenTriviaData};

type HashMap<K, V> = hashbrown::HashMap<K, V, BuildHasherDefault<FxHasher>>;

/// A token stored in the `NodeCache`, in the arena of the cache's build.
/// Does intentionally not implement `Hash` to have compile-time guarantees that the `NodeCache`
/// uses the correct hash.
#[derive(Debug)]
struct CachedToken(NonNull<GreenTokenData>);

impl CachedToken {
    fn value(&self) -> &GreenTokenData {
        // SAFETY: the cache only holds elements of the arena of its build, which it keeps alive.
        unsafe { self.0.as_ref() }
    }
}

/// A node stored in the `NodeCache`, in the arena of the cache's build. It stores a pre-computed
/// hash because re-computing the hash requires traversing the whole sub-tree.
/// The hash also differs from the `GreenNode` hash implementation as it
/// only hashes occupied slots and excludes empty slots.
///
/// Does intentionally not implement `Hash` to have compile-time guarantees that the `NodeCache`
/// uses the correct hash.
#[derive(Debug)]
struct CachedNode {
    node: NonNull<GreenNodeData>,
    // Store the hash as it's expensive to re-compute
    // involves re-computing the hash of the whole sub-tree
    hash: u64,
}

impl CachedNode {
    fn value(&self) -> &GreenNodeData {
        // SAFETY: the cache only holds elements of the arena of its build, which it keeps alive.
        unsafe { self.node.as_ref() }
    }
}

/// A child of a node under construction, as stored by the [crate::TreeBuilder].
///
/// The element is in the arena of the [NodeCache], and the child doesn't count a reference to
/// it: the arena keeps it alive, so dropping a child does nothing. A child with a hash other than
/// [NodeCache::UNCACHED_NODE_HASH] is also stored in the cache; one with that hash isn't, and
/// only the node under construction will refer to it. [CacheChild::into_element] makes an
/// uncounted handle of the element when the child moves into a newly built node.
///
/// A child must not outlive the arena of the [NodeCache] it was retrieved from.
///
/// The element is stored as a tagged pointer to keep the type at two words, which
/// lets functions return it in registers.
pub(crate) struct CacheChild {
    hash: u64,
    /// Pointer to the [GreenNodeData] or the [GreenTokenData]. The lowest bit is
    /// [CacheChild::TOKEN_TAG] for tokens; it's always free because both types are
    /// at least pointer-aligned.
    element: NonNull<()>,
}

impl CacheChild {
    const TOKEN_TAG: usize = 1;

    /// Creates a child of `element`, an uncounted handle to a new element in the arena of the
    /// cache. Its hash is [NodeCache::UNCACHED_NODE_HASH].
    #[inline]
    pub(crate) fn owned(element: GreenElement) -> Self {
        let element = match element {
            NodeOrToken::Node(node) => node.into_raw().cast(),
            NodeOrToken::Token(token) => token
                .into_raw()
                .cast::<()>()
                .map_addr(|addr| addr | Self::TOKEN_TAG),
        };

        Self {
            hash: NodeCache::UNCACHED_NODE_HASH,
            element,
        }
    }

    /// Creates a child for an element stored in the cache with `hash`.
    ///
    /// # Safety
    /// `element` must be stored in the cache, see [CacheChild].
    #[inline]
    unsafe fn cached(hash: u64, element: GreenElementRef<'_>) -> Self {
        if hash == NodeCache::UNCACHED_NODE_HASH {
            // An element whose hash collides with the uncached hash: treat it as uncached so
            // that the node it moves into isn't looked up in the cache.
            // SAFETY: the handle doesn't count a reference, as the child it becomes.
            return Self::owned(unsafe { uncounted(element) });
        }

        let element = match element {
            NodeOrToken::Node(node) => NonNull::from(node).cast(),
            NodeOrToken::Token(token) => NonNull::from(token)
                .cast::<()>()
                .map_addr(|addr| addr | Self::TOKEN_TAG),
        };

        Self { hash, element }
    }

    #[inline]
    pub(crate) fn as_element_ref(&self) -> GreenElementRef<'_> {
        let addr = self.element.addr().get();
        if addr & Self::TOKEN_TAG == 0 {
            // SAFETY: The pointer is an untagged pointer to a live `GreenNodeData`.
            NodeOrToken::Node(unsafe { self.element.cast::<GreenNodeData>().as_ref() })
        } else {
            let token = self.element.map_addr(|addr| {
                // SAFETY: Clearing the tag of a non-null pointer to an aligned value
                // can't produce zero.
                unsafe { std::num::NonZeroUsize::new_unchecked(addr.get() & !Self::TOKEN_TAG) }
            });
            // SAFETY: The pointer is a pointer to a live `GreenTokenData` once untagged.
            NodeOrToken::Token(unsafe { token.cast::<GreenTokenData>().as_ref() })
        }
    }

    /// Returns an uncounted handle to the element, see [crate::green::arena].
    #[inline]
    pub(crate) fn into_element(self) -> GreenElement {
        // SAFETY: the element is in the arena of the running build, and the handle must move
        // into a node of it or be discarded, as every handle of a child.
        unsafe { uncounted(self.as_element_ref()) }
    }
}

/// Makes a handle to `element` that doesn't count a reference.
///
/// # Safety
/// The element must be in the arena of the running build, and the handle must not be dropped:
/// it must move into an element of the arena, be discarded, or be counted, see
/// [crate::green::arena].
#[inline]
unsafe fn uncounted(element: GreenElementRef<'_>) -> GreenElement {
    // SAFETY: the caller upholds the requirements of the handle.
    unsafe {
        match element {
            NodeOrToken::Node(node) => NodeOrToken::Node(GreenNode::from_raw(NonNull::from(node))),
            NodeOrToken::Token(token) => {
                NodeOrToken::Token(GreenToken::from_raw(NonNull::from(token)))
            }
        }
    }
}

// SAFETY: A child is an owned or a borrowed handle to an immutable green element, which is
// `Send` and `Sync` like `GreenElement`.
unsafe impl Send for CacheChild {}
// SAFETY: See `Send`.
unsafe impl Sync for CacheChild {}

impl Debug for CacheChild {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("CacheChild")
            .field("hash", &self.hash)
            .field("element", &self.as_element_ref())
            .finish()
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
//
// The builds with a cache allocate their elements in its arena, so a build can share the elements
// of earlier ones. The entries are pointers into that arena, which the cache keeps alive. When the
// arena has grown well past what a build needs, the cache starts a new one and empties itself:
// the trees of earlier builds keep the old arena alive until they're dropped.
#[derive(Default, Debug)]
pub struct NodeCache {
    nodes: HashMap<CachedNode, ()>,
    tokens: HashMap<CachedToken, ()>,
    trivia: TriviaCache,
    /// The arena of the builds, which the entries point into.
    arena: Option<ArenaRef>,
    /// The bytes the arena held after its first build.
    first_build_bytes: usize,
    /// Whether a build is running.
    building: bool,
}

// SAFETY: the entries are pointers to immutable elements in the arena, which the cache keeps
// alive, and only a build of the cache allocates in the arena.
unsafe impl Send for NodeCache {}
// SAFETY: see `Send`.
unsafe impl Sync for NodeCache {}

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
    pub(crate) const UNCACHED_NODE_HASH: u64 = 0;

    /// The cache starts a new arena when the current one holds this many times the bytes of its
    /// first build. The rest are elements that earlier builds made and later ones mostly don't
    /// share, which the arena can't free one by one.
    const REPLACE_FACTOR: usize = 2;

    /// The arena of a small build is kept until it holds this many bytes, so that small builds,
    /// like class strings, keep sharing their elements.
    const REPLACE_MIN_BYTES: usize = 1024 * 1024;

    /// Starts a build, in a new arena if the current one has grown past [Self::REPLACE_FACTOR]
    /// times the bytes of its first build.
    pub(crate) fn begin_build(&mut self) {
        assert!(!self.building, "a build is already running");
        if let Some(arena) = &self.arena {
            // SAFETY: no build is running.
            let allocated = unsafe { arena.arena().allocated() };
            if allocated > Self::REPLACE_MIN_BYTES
                && allocated > Self::REPLACE_FACTOR * self.first_build_bytes
            {
                self.nodes.clear();
                self.tokens.clear();
                self.trivia.cache.clear();
                self.trivia.whitespace = None;
                self.arena = None;
            }
        }
        let arena = self.arena.get_or_insert_with(|| {
            self.first_build_bytes = 0;
            ArenaRef::new()
        });
        if self.trivia.whitespace.is_none() {
            // SAFETY: the build of the arena's cache is starting, and the arena holds the trivia
            // without counting it.
            let whitespace =
                unsafe { GreenTrivia::new_in([TriviaPiece::whitespace(1)], Some(arena.arena())) };
            self.trivia.whitespace = NonNull::new(whitespace.into_raw());
        }
        self.building = true;
    }

    /// Ends the build, if one is running, and counts a reference for `root`, the uncounted
    /// handle to the root of the build. The other uncounted handles of the build must be gone,
    /// see [crate::green::arena].
    pub(crate) fn end_build(&mut self, root: Option<&GreenNode>) {
        if !std::mem::take(&mut self.building) {
            return;
        }
        let arena = self.arena.as_ref().expect("a build has an arena").arena();
        if self.first_build_bytes == 0 {
            // SAFETY: the build has ended, and no other build of the cache is running.
            self.first_build_bytes = unsafe { arena.allocated() };
        }
        if let Some(root) = root
            && root.count_field() == arena.tag()
        {
            arena.retain();
        }
    }

    /// Tries to retrieve a node with the given `kind` and `children` from the cache.
    ///
    /// Returns an entry that allows the caller to:
    /// * Retrieve the cached node if it is present in the cache
    /// * Insert a node if it isn't present in the cache
    ///
    /// Also returns the arena of the running build, to create the node in.
    pub(crate) fn node(
        &mut self,
        kind: RawSyntaxKind,
        children: &[CacheChild],
    ) -> (NodeCacheNodeEntryMut<'_>, &GreenArena) {
        assert!(self.building, "the node cache is used outside a build");
        let arena = self.arena.as_ref().expect("a build has an arena").arena();
        (Self::node_entry(&mut self.nodes, kind, children), arena)
    }

    fn node_entry<'a>(
        nodes: &'a mut HashMap<CachedNode, ()>,
        kind: RawSyntaxKind,
        children: &[CacheChild],
    ) -> NodeCacheNodeEntryMut<'a> {
        if children.len() > 3 {
            return NodeCacheNodeEntryMut::NoCache;
        }

        let hash = {
            let mut h = FxHasher::default();
            kind.hash(&mut h);
            for child in children {
                if child.hash == Self::UNCACHED_NODE_HASH {
                    return NodeCacheNodeEntryMut::NoCache;
                }
                child.hash.hash(&mut h);
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
        let entry = nodes.raw_entry_mut().from_hash(hash, |no_hash| {
            no_hash.value().kind() == kind && {
                let lhs = no_hash.value().slots().filter_map(|slot| match slot {
                    // Ignore empty slots. The queried node only has the present children
                    Slot::Empty { .. } => None,
                    Slot::Node { node, .. } => Some(element_id(NodeOrToken::Node(node))),
                    Slot::Token { token, .. } => Some(element_id(NodeOrToken::Token(token))),
                });

                let rhs = children
                    .iter()
                    .map(|child| element_id(child.as_element_ref()));

                lhs.eq(rhs)
            }
        });

        match entry {
            RawEntryMut::Occupied(entry) => NodeCacheNodeEntryMut::Cached(CachedNodeEntry {
                hash,
                raw_entry: entry,
            }),
            RawEntryMut::Vacant(entry) => NodeCacheNodeEntryMut::Vacant(VacantNodeEntry {
                raw_entry: entry,
                original_kind: kind,
                hash,
            }),
        }
    }

    pub(crate) fn token(&mut self, kind: RawSyntaxKind, text: &str) -> CacheChild {
        self.token_with_trivia(kind, text, &[], &[])
    }

    pub(crate) fn token_with_trivia(
        &mut self,
        kind: RawSyntaxKind,
        text: &str,
        leading: &[TriviaPiece],
        trailing: &[TriviaPiece],
    ) -> CacheChild {
        let hash = token_hash_of(kind, text);

        assert!(self.building, "the node cache is used outside a build");
        let arena = self.arena.as_ref().expect("a build has an arena").arena();
        let entry = self.tokens.raw_entry_mut().from_hash(hash, |token| {
            token.value().kind() == kind && token.value().text() == text
        });

        let token = match entry {
            RawEntryMut::Occupied(entry) => entry.into_key().value(),
            RawEntryMut::Vacant(entry) => {
                let (leading, leading_flags) = self.trivia.get(arena, leading);
                let (trailing, trailing_flags) = self.trivia.get(arena, trailing);

                let mut flags = leading_flags;
                flags.insert(trailing_flags);
                // SAFETY: the arena of the running build is open, and this thread runs it.
                let token = unsafe {
                    GreenToken::with_trivia_and_flags_in(
                        kind,
                        text,
                        leading,
                        trailing,
                        flags,
                        Some(arena),
                    )
                };
                let key = CachedToken(token.into_raw());
                let (key, _) = entry.insert_with_hasher(hash, key, (), |t| token_hash(t.value()));
                key.value()
            }
        };

        // SAFETY: The token is stored in the cache.
        unsafe { CacheChild::cached(hash, NodeOrToken::Token(token)) }
    }
}

pub(crate) enum NodeCacheNodeEntryMut<'a> {
    Cached(CachedNodeEntry<'a>),

    /// A node that should not be cached
    NoCache,
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
}

/// Represents an entry of a cached node.
pub(crate) struct CachedNodeEntry<'a> {
    hash: u64,
    raw_entry: RawOccupiedEntryMut<'a, CachedNode, (), BuildHasherDefault<FxHasher>>,
}

impl CachedNodeEntry<'_> {
    /// Returns the cached node as a child of the node under construction.
    #[inline]
    pub fn into_child(self) -> CacheChild {
        let node = self.raw_entry.into_key().value();
        // SAFETY: The node is stored in the cache.
        unsafe { CacheChild::cached(self.hash, NodeOrToken::Node(node)) }
    }
}

impl VacantNodeEntry<'_> {
    /// Inserts the `node` into the cache so that future queries for the same kind and children resolve to the passed `node`.
    ///
    /// Returns the node as a child of the node under construction.
    ///
    /// The cache does not cache the `node` if the kind doesn't match the `kind` of the queried node because
    /// cache lookups wouldn't be successful because the hash collision prevention check compares the kinds of the
    /// cached and queried node.
    pub fn cache(self, node: GreenNode) -> CacheChild {
        if self.original_kind != node.kind() {
            // The kind has changed since it has been queried. For example, the node has been converted to an
            // unknown node. Never cache these nodes because cache lookups will never match.
            CacheChild::owned(NodeOrToken::Node(node))
        } else {
            let (key, _) = self.raw_entry.insert_with_hasher(
                self.hash,
                CachedNode {
                    node: node.into_raw(),
                    hash: self.hash,
                },
                (),
                |n| n.hash,
            );
            let node = key.value();
            // SAFETY: The node is stored in the cache.
            unsafe { CacheChild::cached(self.hash, NodeOrToken::Node(node)) }
        }
    }
}

/// A cached [GreenTrivia], in the arena of the cache's build.
/// Deliberately doesn't implement `Hash` to make sure all
/// usages go through the custom `FxHasher`.
#[derive(Debug)]
struct CachedTrivia(NonNull<GreenTriviaData>);

impl CachedTrivia {
    fn value(&self) -> &GreenTriviaData {
        // SAFETY: the cache only holds elements of the arena of its build, which it keeps alive.
        unsafe { self.0.as_ref() }
    }
}

#[derive(Debug, Default)]
struct TriviaCache {
    /// Generic cache for trivia
    cache: HashMap<CachedTrivia, ()>,

    /// Cached single whitespace trivia, in the arena of the cache.
    whitespace: Option<NonNull<GreenTriviaData>>,
}

impl TriviaCache {
    /// Tries to retrieve a [GreenTrivia] with the given pieces from the cache or creates a new one
    /// in `arena`, the arena of the cache's build, and caches it for further calls.
    fn get(
        &mut self,
        arena: &GreenArena,
        pieces: &[TriviaPiece],
    ) -> (GreenTrivia, GreenElementFlags) {
        match pieces {
            [] => (GreenTrivia::empty(), GreenElementFlags::default()),
            [
                TriviaPiece {
                    kind: TriviaPieceKind::Whitespace,
                    length,
                },
            ] if *length == TextSize::from(1) => {
                let whitespace = self.whitespace.expect("a build has the whitespace trivia");
                // SAFETY: the trivia is in the arena of the running build, and the handle moves
                // into a token of it.
                let whitespace = unsafe { GreenTrivia::from_raw(whitespace.as_ptr()) };
                (whitespace, GreenElementFlags::default())
            }

            _ => {
                let (hash, flags) = Self::trivia_hash::<true>(pieces);

                let entry = self
                    .cache
                    .raw_entry_mut()
                    .from_hash(hash, |trivia| trivia.value().pieces() == pieces);

                match entry {
                    RawEntryMut::Occupied(entry) => {
                        let trivia = NonNull::from(entry.key().value());
                        // SAFETY: as for the whitespace above.
                        (unsafe { GreenTrivia::from_raw(trivia.as_ptr()) }, flags)
                    }
                    RawEntryMut::Vacant(entry) => {
                        // SAFETY: the arena is the one of the running build, and the handle moves
                        // into a token of it.
                        let trivia =
                            unsafe { GreenTrivia::new_in(pieces.iter().copied(), Some(arena)) };
                        // SAFETY: the copy is never dropped.
                        let cached = unsafe { super::arena::duplicate(&trivia) }.into_raw();
                        entry.insert_with_hasher(
                            hash,
                            CachedTrivia(NonNull::new(cached).expect("the trivia isn't empty")),
                            (),
                            |cached| Self::trivia_hash::<false>(cached.value().pieces()).0,
                        );
                        (trivia, flags)
                    }
                }
            }
        }
    }

    fn trivia_hash<const COMPUTE_FLAGS: bool>(pieces: &[TriviaPiece]) -> (u64, GreenElementFlags) {
        let mut h = FxHasher::default();
        let mut flags = GreenElementFlags::default();

        pieces.len().hash(&mut h);

        for piece in pieces {
            piece.hash(&mut h);
            if COMPUTE_FLAGS && !flags.contains_all(GreenElementFlags::HAS_COMMENTS_AND_SKIPPED) {
                flags.insert(GreenElementFlags::from_trivia_kind(piece.kind()));
            }
        }

        (h.finish(), flags)
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use crate::green::GreenElementFlags;
    use crate::green::node_cache::{
        CacheChild, CachedNode, CachedToken, CachedTrivia, NodeCache, token_hash,
    };
    use crate::green::trivia::GreenTrivia;
    use crate::{GreenToken, GreenTokenData, RawSyntaxKind, TriviaPiece, TriviaPieceKind};
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

        let t3 = GreenToken::new_raw(kind, "let");
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
    fn cached_token_preserves_trivia_flags() {
        let mut cache = NodeCache::default();
        cache.begin_build();
        let kind = RawSyntaxKind(0);
        let leading = [TriviaPiece::new(TriviaPieceKind::Skipped, 1)];
        let trailing = [TriviaPiece::single_line_comment(2)];

        let token = cache
            .token_with_trivia(kind, "?value//", &leading, &trailing)
            .into_element()
            .into_token()
            .unwrap();
        let cached = cache
            .token_with_trivia(kind, "?value//", &leading, &trailing)
            .into_element()
            .into_token()
            .unwrap();

        assert!(std::ptr::eq::<GreenTokenData>(&*token, &*cached));
        assert!(cached.flags().contains(GreenElementFlags::HAS_COMMENTS));
        assert!(cached.flags().contains(GreenElementFlags::HAS_SKIPPED));

        // The handles of a build don't count references.
        std::mem::forget((token, cached));
        cache.end_build(None);
    }

    #[test]
    fn cache_entry_size() {
        assert_eq!(size_of::<CachedNode>(), 16);
        assert_eq!(size_of::<CachedToken>(), 8);
        assert_eq!(size_of::<CachedTrivia>(), 8);
        assert_eq!(size_of::<CacheChild>(), 16);
    }
}

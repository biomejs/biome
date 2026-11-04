//! Arena allocation of the green elements that a [TreeBuilder](crate::TreeBuilder) builds.
//!
//! Each build allocates its nodes, tokens and trivia in a [GreenArena] instead of allocating each
//! element on its own. Elements built by edits and factories outside a build keep their own
//! allocation and reference count, and a tree can mix both kinds.
//!
//! The reference count field of an element in an arena holds a tagged pointer to its arena
//! instead of a count ([ARENA_TAG]). Cloning or dropping a handle to such an element counts a
//! reference to the whole arena, which frees all of its elements at once when the last reference
//! goes away. Elements refer to the children in their own arena without counting them, and
//! the arena never runs the destructors of its elements.
//!
//! While its build is open, handles to an arena's elements aren't counted at all: the builder,
//! its node cache and the elements themselves are the only places such handles can be, and the
//! builder discards them all before it closes the arena. When the build finishes, the root of the
//! tree becomes a counted handle, and from then on every clone counts. That's why
//! [GreenArena::is_open] is checked before counting.
//!
//! An element can still have a child from outside its arena, an element that counts references
//! itself or one in another arena. The arena keeps the handles of those children and drops them
//! when it frees its elements.

use crate::green::{GreenElement, GreenTrivia};
use std::{
    alloc::{self, Layout},
    cell::{Cell, UnsafeCell},
    mem,
    ptr::{self, NonNull},
    sync::atomic::{self, AtomicBool, AtomicUsize, Ordering},
};

/// The bit of the reference count field that tells the element is in an arena. The other bits
/// are the address of the arena, shifted right by one: an arena is aligned to at least 4 bytes,
/// so its address loses nothing, and the top bit stays free even where addresses use it.
///
/// Reference counts never reach this bit, as they abort above `isize::MAX`.
pub(crate) const ARENA_TAG: usize = 1 << (usize::BITS - 1);

/// The size of the first chunk of an arena. Each next chunk doubles, up to [MAX_CHUNK], so that
/// the arena of a small tree stays small.
const FIRST_CHUNK: usize = 4 * 1024;

/// Chunks larger than this doubled the page faults of a parse under jemalloc, which maps
/// allocations that large afresh each time; smaller ones come from memory it reuses.
const MAX_CHUNK: usize = 1024 * 1024;

/// The alignment of every allocation in an arena, which suits every green element.
const ALIGN: usize = 8;

/// The memory of the green elements of one build.
pub(crate) struct GreenArena {
    /// The references to the arena: the counted handles to its elements, and the [ArenaRef]s.
    refs: AtomicUsize,
    /// Whether the arena's build is open, during which handles to its elements aren't counted.
    open: AtomicBool,
    /// The current chunk: the next free byte and the end.
    next: Cell<*mut u8>,
    end: Cell<*mut u8>,
    /// Every chunk, to free them.
    chunks: UnsafeCell<Vec<(NonNull<u8>, Layout)>>,
    /// The children of the arena's elements that aren't in the arena, which the arena drops.
    foreign: UnsafeCell<Vec<ForeignChild>>,
    #[cfg(feature = "countme")]
    counts: UnsafeCell<ElementCounts>,
}

// SAFETY: Only the thread that runs the arena's build allocates from it and mutates `next`,
// `end`, `chunks` and `foreign`, while the build is open. Other threads only read the elements,
// count references with atomics, and free the arena once nothing refers to it.
unsafe impl Send for GreenArena {}
// SAFETY: See `Send`.
unsafe impl Sync for GreenArena {}

/// A child that an element in an arena owns, but that isn't in the arena.
enum ForeignChild {
    // The variants hold handles only to drop them.
    #[expect(dead_code, reason = "held to be dropped")]
    Element(GreenElement),
    #[expect(dead_code, reason = "held to be dropped")]
    Trivia(GreenTrivia),
}

/// The `countme` counts of the elements in an arena, which the arena drops when it frees them.
#[cfg(feature = "countme")]
#[derive(Default)]
pub(crate) struct ElementCounts {
    nodes: Vec<countme::Count<crate::GreenNode>>,
    tokens: Vec<countme::Count<crate::GreenToken>>,
    trivia: Vec<countme::Count<GreenTrivia>>,
}

/// An element type whose `countme` counts an arena can take over.
#[cfg(feature = "countme")]
pub(crate) trait ArenaCounted: Sized + 'static {
    fn push(counts: &mut ElementCounts, count: countme::Count<Self>);
}

#[cfg(feature = "countme")]
impl ArenaCounted for crate::GreenNode {
    fn push(counts: &mut ElementCounts, count: countme::Count<Self>) {
        counts.nodes.push(count);
    }
}

#[cfg(feature = "countme")]
impl ArenaCounted for crate::GreenToken {
    fn push(counts: &mut ElementCounts, count: countme::Count<Self>) {
        counts.tokens.push(count);
    }
}

#[cfg(feature = "countme")]
impl ArenaCounted for GreenTrivia {
    fn push(counts: &mut ElementCounts, count: countme::Count<Self>) {
        counts.trivia.push(count);
    }
}

impl GreenArena {
    /// Creates an open arena, and returns the only reference to it.
    pub(crate) fn new_open() -> ArenaRef {
        let arena = Box::new(Self {
            refs: AtomicUsize::new(1),
            open: AtomicBool::new(true),
            next: Cell::new(ptr::null_mut()),
            end: Cell::new(ptr::null_mut()),
            chunks: UnsafeCell::new(Vec::new()),
            foreign: UnsafeCell::new(Vec::new()),
            #[cfg(feature = "countme")]
            counts: UnsafeCell::new(ElementCounts::default()),
        });
        ArenaRef(NonNull::from(Box::leak(arena)))
    }

    /// The value of the reference count field of an element in this arena.
    #[inline]
    pub(crate) fn tag(&self) -> usize {
        // Exposed, so that `arena_of` can make a pointer of the address again.
        let addr = ptr::from_ref(self).expose_provenance();
        debug_assert_eq!(addr & 1, 0);
        (addr >> 1) | ARENA_TAG
    }

    #[inline]
    pub(crate) fn is_open(&self) -> bool {
        self.open.load(Ordering::Relaxed)
    }

    /// Allocates `layout`, whose alignment must be at most [ALIGN].
    ///
    /// # Safety
    /// Only the thread that runs the arena's build may allocate, while the build is open.
    #[inline]
    pub(crate) unsafe fn alloc(&self, layout: Layout) -> NonNull<u8> {
        debug_assert!(layout.align() <= ALIGN);
        debug_assert!(self.is_open());
        let size = layout.size().next_multiple_of(ALIGN);
        let next = self.next.get();
        if self.end.get().addr().wrapping_sub(next.addr()) >= size && !next.is_null() {
            self.next.set(next.wrapping_add(size));
            // SAFETY: `next` points into a chunk, so it isn't null.
            return unsafe { NonNull::new_unchecked(next) };
        }
        // SAFETY: the caller upholds the same requirements.
        unsafe { self.alloc_chunk(size) }
    }

    /// Allocates `size` bytes in a new chunk.
    ///
    /// # Safety
    /// As [Self::alloc].
    #[cold]
    unsafe fn alloc_chunk(&self, size: usize) -> NonNull<u8> {
        // SAFETY: only the building thread allocates, so nothing else borrows the chunks.
        let chunks = unsafe { &mut *self.chunks.get() };
        let chunk_size = FIRST_CHUNK
            .checked_shl(chunks.len().min(16) as u32)
            .map_or(MAX_CHUNK, |size| size.min(MAX_CHUNK));
        // An element that doesn't fit a chunk gets one of its own, so that the current chunk
        // keeps its free space.
        let dedicated = size > chunk_size / 2;
        let layout = Layout::from_size_align(if dedicated { size } else { chunk_size }, ALIGN)
            .expect("arena chunk size overflows");
        // SAFETY: `layout` isn't empty.
        let chunk = unsafe { alloc::alloc(layout) };
        let Some(chunk) = NonNull::new(chunk) else {
            alloc::handle_alloc_error(layout)
        };
        chunks.push((chunk, layout));
        if !dedicated {
            self.next.set(chunk.as_ptr().wrapping_add(size));
            self.end.set(chunk.as_ptr().wrapping_add(chunk_size));
        }
        chunk
    }

    /// Takes over a handle to a child of an element in this arena that isn't in the arena, so
    /// that the arena drops it with its elements.
    ///
    /// # Safety
    /// As [Self::alloc], and the handle must be a bitwise copy of the one the element holds,
    /// which is never dropped.
    #[cold]
    pub(crate) unsafe fn adopt_foreign_element(&self, child: GreenElement) {
        // SAFETY: only the building thread adds foreign children, while the build is open.
        unsafe { (*self.foreign.get()).push(ForeignChild::Element(child)) }
    }

    /// As [Self::adopt_foreign_element], for trivia.
    ///
    /// # Safety
    /// As [Self::adopt_foreign_element].
    #[cold]
    pub(crate) unsafe fn adopt_foreign_trivia(&self, trivia: GreenTrivia) {
        // SAFETY: only the building thread adds foreign children, while the build is open.
        unsafe { (*self.foreign.get()).push(ForeignChild::Trivia(trivia)) }
    }

    /// Takes over the `countme` count of an element allocated in this arena.
    ///
    /// # Safety
    /// As [Self::alloc], and the count must be a bitwise copy of the one the element holds,
    /// which is never dropped.
    #[cfg(feature = "countme")]
    pub(crate) unsafe fn adopt_count<T: ArenaCounted>(&self, count: countme::Count<T>) {
        // SAFETY: only the building thread allocates elements, while the build is open.
        T::push(unsafe { &mut *self.counts.get() }, count);
    }

    /// Counts a reference to the arena.
    #[inline]
    pub(crate) fn retain(&self) {
        let old = self.refs.fetch_add(1, Ordering::Relaxed);
        // As `Arc`, abort rather than overflow the count when references are leaked.
        if old > isize::MAX as usize {
            std::process::abort();
        }
    }

    /// Releases a reference to the arena, and frees it if it was the last one.
    ///
    /// # Safety
    /// The caller must own a reference to `arena`, and not use it afterwards.
    #[inline]
    pub(crate) unsafe fn release(arena: NonNull<Self>) {
        // SAFETY: the reference keeps the arena alive.
        if unsafe { arena.as_ref() }
            .refs
            .fetch_sub(1, Ordering::Release)
            != 1
        {
            return;
        }
        // Order the uses of the arena through other references before its release.
        atomic::fence(Ordering::Acquire);
        // SAFETY: that was the last reference, and the arena was allocated as a `Box`.
        drop(unsafe { Box::from_raw(arena.as_ptr()) });
    }
}

impl Drop for GreenArena {
    fn drop(&mut self) {
        // The foreign children may be in other arenas, which they release.
        drop(mem::take(self.foreign.get_mut()));
        for (chunk, layout) in self.chunks.get_mut().drain(..) {
            // SAFETY: the chunk was allocated with `layout`, and the elements in it are never
            // dropped.
            unsafe { alloc::dealloc(chunk.as_ptr(), layout) };
        }
    }
}

/// Counts a reference to the arena of `tag`, the reference count field of an element in it,
/// unless its build is open.
///
/// # Safety
/// `tag` must be the field of an element that is alive.
#[inline]
pub(crate) unsafe fn retain_tagged(tag: usize) {
    // SAFETY: an element keeps its arena alive.
    let arena = unsafe { &*arena_of(tag).as_ptr() };
    if !arena.is_open() {
        arena.retain();
    }
}

/// Releases a reference to the arena of `tag` that a handle counted, unless its build is open.
///
/// # Safety
/// `tag` must be the field of an element that is alive, and the handle that counted the
/// reference must not be used afterwards.
#[inline]
pub(crate) unsafe fn release_tagged(tag: usize) {
    let arena = arena_of(tag);
    // SAFETY: the handle keeps the arena alive.
    if !unsafe { arena.as_ref() }.is_open() {
        // SAFETY: the handle owned the reference.
        unsafe { GreenArena::release(arena) };
    }
}

#[inline]
fn arena_of(tag: usize) -> NonNull<GreenArena> {
    debug_assert!(tag & ARENA_TAG != 0);
    let arena = ptr::with_exposed_provenance_mut::<GreenArena>((tag & !ARENA_TAG) << 1);
    // SAFETY: the address of a live arena isn't null.
    unsafe { NonNull::new_unchecked(arena) }
}

/// An owned reference to an arena.
#[derive(Debug)]
pub(crate) struct ArenaRef(NonNull<GreenArena>);

// SAFETY: see `GreenArena`.
unsafe impl Send for ArenaRef {}
// SAFETY: see `GreenArena`.
unsafe impl Sync for ArenaRef {}

impl ArenaRef {
    #[inline]
    pub(crate) fn arena(&self) -> &GreenArena {
        // SAFETY: the reference keeps the arena alive.
        unsafe { self.0.as_ref() }
    }

    /// Closes the arena's build, after which handles to its elements count references.
    pub(crate) fn close(&self) {
        self.arena().open.store(false, Ordering::Release);
    }
}

impl Drop for ArenaRef {
    fn drop(&mut self) {
        // SAFETY: this owns the reference.
        unsafe { GreenArena::release(self.0) }
    }
}

impl std::fmt::Debug for GreenArena {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GreenArena")
            .field("refs", &self.refs.load(Ordering::Relaxed))
            .field("open", &self.is_open())
            .finish_non_exhaustive()
    }
}

thread_local! {
    /// The arena that [RawSyntaxNode::new](crate::RawSyntaxNode::new) allocates nodes in.
    static CURRENT: Cell<*const GreenArena> = const { Cell::new(ptr::null()) };
}

/// Calls `f` with the arena that nodes created on this thread go into, if any.
#[inline]
fn with_current<R>(f: impl FnOnce(Option<&GreenArena>) -> R) -> R {
    // SAFETY: a `CurrentArena` keeps the arena alive and its build open while it's current.
    CURRENT.with(|current| f(unsafe { current.get().as_ref() }))
}

/// Makes an open arena the one that [with_current] returns on this thread, until dropped.
///
/// The builder sets it only while a factory makes a node, so that no other code allocates in the
/// arena. The guard borrows the arena, which keeps it alive, and its build can't end while it's
/// borrowed.
pub(crate) struct CurrentArena<'a> {
    previous: *const GreenArena,
    _arena: std::marker::PhantomData<&'a GreenArena>,
}

impl<'a> CurrentArena<'a> {
    /// Panics if the arena's build isn't open.
    #[inline]
    pub(crate) fn enter(arena: &'a GreenArena) -> Self {
        assert!(arena.is_open(), "the build of the arena has ended");
        let previous = CURRENT.with(|current| current.replace(arena));
        Self {
            previous,
            _arena: std::marker::PhantomData,
        }
    }
}

impl Drop for CurrentArena<'_> {
    #[inline]
    fn drop(&mut self) {
        CURRENT.with(|current| current.set(self.previous));
    }
}

/// Creates a node in the current arena of this thread, if any, see [CurrentArena].
#[inline]
pub(crate) fn new_node_in_current<I>(kind: crate::RawSyntaxKind, slots: I) -> crate::GreenNode
where
    I: IntoIterator<Item = Option<GreenElement>>,
    I::IntoIter: ExactSizeIterator,
{
    with_current(|arena| {
        // SAFETY: a current arena is open, and the thread that runs its build made it current.
        unsafe { crate::GreenNode::new_in(kind, slots, arena) }
    })
}

/// Bitwise copies `handle`, for an arena to take over the copy an element holds.
///
/// # Safety
/// Exactly one of the copies may be dropped.
#[inline]
pub(crate) unsafe fn duplicate<T>(handle: &T) -> T {
    // SAFETY: the caller drops only one of the copies.
    unsafe { ptr::read(handle) }
}

#[cfg(test)]
mod tests {
    use crate::green::{GreenNode, GreenToken, NodeCache};
    use crate::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    use crate::{NodeOrToken, RawSyntaxKind, SyntaxNode, SyntaxToken};

    fn build(builder: &mut RawSyntaxTreeBuilder, words: &[&str]) {
        builder.start_node(RawLanguageKind::ROOT);
        builder.start_node(RawLanguageKind::EXPRESSION_LIST);
        for word in words {
            builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
            builder.token(RawLanguageKind::STRING_TOKEN, word);
            builder.finish_node();
        }
        builder.finish_node();
        builder.finish_node();
    }

    fn tree(words: &[&str]) -> SyntaxNode<RawLanguage> {
        let mut builder = RawSyntaxTreeBuilder::new();
        build(&mut builder, words);
        builder.finish()
    }

    fn first_literal(root: &SyntaxNode<RawLanguage>) -> SyntaxNode<RawLanguage> {
        root.descendants()
            .find(|node| node.kind() == RawLanguageKind::LITERAL_EXPRESSION)
            .unwrap()
    }

    #[test]
    fn tree_outlives_its_builder_and_cache() {
        let mut cache = NodeCache::default();
        let mut builder = RawSyntaxTreeBuilder::with_cache(&mut cache);
        build(&mut builder, &["a", "b", "a"]);
        let root = builder.finish();
        drop(cache);
        assert_eq!(root.to_string(), "aba");
    }

    #[test]
    fn token_text_outlives_its_tree() {
        let root = tree(&["hello", "world"]);
        let text = root.first_token().unwrap().token_text();
        drop(root);
        assert_eq!(text.text(), "hello");
        let copy = text.clone();
        drop(text);
        assert_eq!(copy.text(), "hello");
    }

    #[test]
    fn detached_node_outlives_its_tree() {
        let root = tree(&["a", "b"]);
        let detached = first_literal(&root).detach();
        drop(root);
        assert_eq!(detached.to_string(), "a");
    }

    #[test]
    fn edited_tree_outlives_the_original() {
        let root = tree(&["a", "b"]);
        let old = root.first_token().unwrap();
        let new =
            SyntaxToken::<RawLanguage>::new_detached(RawLanguageKind::STRING_TOKEN, "z", [], []);
        let edited = root.clone().replace_child(old.into(), new.into()).unwrap();
        drop(root);
        assert_eq!(edited.to_string(), "zb");
    }

    #[test]
    fn original_outlives_the_edited_tree() {
        let root = tree(&["a", "b"]);
        let old = root.first_token().unwrap();
        let new =
            SyntaxToken::<RawLanguage>::new_detached(RawLanguageKind::STRING_TOKEN, "z", [], []);
        let edited = root.clone().replace_child(old.into(), new.into()).unwrap();
        drop(edited);
        assert_eq!(root.to_string(), "ab");
    }

    #[test]
    fn tree_moves_to_another_thread() {
        let root = tree(&["a", "b"]);
        let send = root.as_send().unwrap();
        drop(root);
        let text = std::thread::spawn(move || send.into_node::<RawLanguage>().unwrap().to_string())
            .join()
            .unwrap();
        assert_eq!(text, "ab");
    }

    #[test]
    fn subtree_of_one_build_moves_into_another_by_an_edit() {
        let first = tree(&["a", "b"]);
        let second = tree(&["c"]);
        let edited = second
            .clone()
            .replace_child(
                first_literal(&second).into(),
                first_literal(&first).detach().into(),
            )
            .unwrap();
        drop((first, second));
        assert_eq!(edited.to_string(), "a");
    }

    #[test]
    fn abandoned_build_leaves_the_cache_usable() {
        let mut cache = NodeCache::default();
        let mut builder = RawSyntaxTreeBuilder::with_cache(&mut cache);
        builder.start_node(RawLanguageKind::ROOT);
        builder.token(RawLanguageKind::STRING_TOKEN, "a");
        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.token(RawLanguageKind::STRING_TOKEN, "b");
        builder.finish_node();
        drop(builder);

        let mut builder = RawSyntaxTreeBuilder::with_cache(&mut cache);
        build(&mut builder, &["a", "b"]);
        assert_eq!(builder.finish().to_string(), "ab");
    }

    #[test]
    fn builds_with_a_reused_cache_share_nothing() {
        let mut cache = NodeCache::default();
        let mut builder = RawSyntaxTreeBuilder::with_cache(&mut cache);
        build(&mut builder, &["a", "b"]);
        let first = builder.finish();
        let mut builder = RawSyntaxTreeBuilder::with_cache(&mut cache);
        build(&mut builder, &["a", "b"]);
        let second = builder.finish();

        let token = |root: &SyntaxNode<RawLanguage>| root.first_token().unwrap().key();
        assert!(token(&first) != token(&second));
        drop((first, cache));
        assert_eq!(second.to_string(), "ab");
    }

    #[test]
    fn nested_builds_on_one_thread() {
        let mut outer = RawSyntaxTreeBuilder::new();
        outer.start_node(RawLanguageKind::ROOT);
        outer.token(RawLanguageKind::STRING_TOKEN, "a");
        let inner = tree(&["b"]);
        outer.token(RawLanguageKind::STRING_TOKEN, "c");
        outer.finish_node();
        let outer = outer.finish();
        assert_eq!(
            (outer.to_string(), inner.to_string()),
            ("ac".into(), "b".into())
        );
    }

    #[test]
    fn arena_drops_children_from_outside_it() {
        let kind = RawSyntaxKind(0);
        let foreign = GreenToken::new_raw(kind, "x");
        let other_build = tree(&["y"]);
        let other_node: crate::cursor::SyntaxNode = first_literal(&other_build).into();
        let other_green = other_node.green().to_owned();
        drop((other_node, other_build));

        let mut cache = NodeCache::default();
        cache.begin_build();
        let (_, arena) = cache.node(kind, &[]);
        // SAFETY: the build is open, and this thread runs it.
        let node = unsafe {
            GreenNode::new_in(
                kind,
                [
                    Some(NodeOrToken::Token(foreign.clone())),
                    Some(NodeOrToken::Node(other_green.clone())),
                ],
                Some(arena),
            )
        };
        assert_eq!(foreign.count_field(), 2);
        drop(other_green);
        drop(node);
        drop(cache.end_build());
        assert_eq!(foreign.count_field(), 1);
    }
}

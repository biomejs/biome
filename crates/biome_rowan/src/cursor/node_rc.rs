//! Reference-counted handles to red nodes, allocated from a per-thread pool.
//!
//! Traversals create and drop red nodes ([NodeData]) at a high rate, nearly
//! always in last-in, first-out order: a walk allocates a child, inspects it,
//! and drops it before moving to the next one. Going through the global
//! allocator for each of them costs more than the bookkeeping the node needs,
//! so the block of a freed node goes on a stack owned by the current thread,
//! and the next node allocated on that thread takes it back.
//!
//! The pool holds at most [MAX_POOLED] blocks per thread; blocks freed beyond
//! that go back to the global allocator, so the memory a thread retains is
//! bounded no matter how many nodes it frees at once. Every block, pooled or
//! not, is allocated with the global allocator and the layout of [NodeBox], so
//! a block can always be returned to the global allocator directly. That is
//! what happens when the pool is not available: during and after the
//! destruction of the thread's locals, which also frees the pooled blocks.
//!
//! Red nodes are `!Send`, so a block is normally freed on the thread that
//! allocated it. Nothing relies on that: a block freed on another thread joins
//! that thread's pool.

use super::NodeData;
use std::alloc::{Layout, alloc, dealloc, handle_alloc_error};
use std::cell::Cell;
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::panic::{RefUnwindSafe, UnwindSafe};
use std::ptr::NonNull;
use std::{fmt, process};

/// Maximum number of free blocks a thread keeps for reuse.
///
/// The cap trades reuse against retained memory. Most frees come one at a
/// time as a walk moves on, but some come in bursts of thousands, such as
/// dropping a collection of nodes or unwinding a long ancestor chain. Blocks
/// freed beyond the cap go back to the global allocator, and the allocations
/// that follow miss the pool until it refills.
///
/// Share of red-node allocations served from the pool when linting the VS
/// Code repository (`src`, pinned to 4 CPUs):
///
/// | Cap    | Default rules | 50 rules |
/// | -----: | ------------: | -------: |
/// | 64     | 92.2%         | 81.3%    |
/// | 1024   | 96.3%         | 89.2%    |
/// | 8192   | 99.1%         | 97.4%    |
/// | 65536  | 99.9%         | 99.6%    |
///
/// 8192 is where the gain flattens. A full pool holds 320 KiB of blocks plus a
/// 64 KiB slot array per thread. Larger caps are not free: peak memory in the
/// same runs grew with the cap, by about 25 MiB at 8192 and 37 MiB at 65536,
/// more than the pooled blocks themselves account for.
const MAX_POOLED: u32 = 8192;

/// The heap allocation behind a [NodeRc]: the reference count and the node.
struct NodeBox {
    count: Cell<u32>,
    data: NodeData,
}

/// A strong reference to a heap-allocated [NodeData], like `Rc<NodeData>`
/// without weak references.
///
/// The node is dropped and its block released when the last handle is
/// dropped. Like `Rc`, the handle is neither `Send` nor `Sync`.
pub(super) struct NodeRc {
    ptr: NonNull<NodeBox>,
}

impl NodeRc {
    #[inline]
    pub(super) fn new(data: NodeData) -> Self {
        let ptr = allocate();
        // SAFETY: `allocate` returns a block with the size and alignment of
        // `NodeBox` that nothing else references.
        unsafe {
            ptr.write(NodeBox {
                count: Cell::new(1),
                data,
            });
        }
        Self { ptr }
    }

    #[inline]
    fn count(&self) -> &Cell<u32> {
        // SAFETY: the box stays alive while a handle to it exists.
        unsafe { &self.ptr.as_ref().count }
    }

    /// Returns the node if this is its only handle, and the handle otherwise.
    pub(super) fn try_unwrap(this: Self) -> Result<NodeData, Self> {
        if this.count().get() != 1 {
            return Err(this);
        }

        let this = ManuallyDrop::new(this);
        // SAFETY: this is the last handle, so the node can be moved out, and
        // the block is released without dropping the node again.
        unsafe {
            let data = (&raw const (*this.ptr.as_ptr()).data).read();
            release(this.ptr);
            Ok(data)
        }
    }

    /// Returns a mutable reference to the node if this is its only handle.
    #[inline]
    pub(super) fn get_mut(&mut self) -> Option<&mut NodeData> {
        if self.count().get() == 1 {
            // SAFETY: no other handle can observe the node while `self` is
            // borrowed mutably.
            Some(unsafe { &mut (*self.ptr.as_ptr()).data })
        } else {
            None
        }
    }

    /// Drops the node behind `ptr` after its count reached zero, then does the
    /// same for every ancestor whose count reaches zero as a result.
    ///
    /// Releasing ancestors in a loop instead of through nested `Drop` calls
    /// keeps the stack depth constant for any tree depth.
    ///
    /// # Safety
    ///
    /// The count of the box behind `ptr` must be zero, and no handle to it may
    /// be used afterwards.
    #[inline(never)]
    unsafe fn drop_slow(mut ptr: NonNull<NodeBox>) {
        loop {
            // SAFETY: guaranteed by the caller for the first iteration, and
            // by the count check below for the following ones.
            let parent = unsafe {
                let data = (&raw const (*ptr.as_ptr()).data).read();
                let parent = data.into_parent();
                release(ptr);
                parent
            };

            let Some(parent) = parent else {
                return;
            };
            let parent = ManuallyDrop::new(parent);
            let count = parent.count().get() - 1;
            parent.count().set(count);
            if count != 0 {
                return;
            }
            ptr = parent.ptr;
        }
    }
}

impl Clone for NodeRc {
    #[inline]
    fn clone(&self) -> Self {
        let count = self.count().get().wrapping_add(1);
        // Like `Rc`, abort rather than let the count wrap around and free a
        // node that still has handles.
        if count == 0 {
            process::abort();
        }
        self.count().set(count);
        Self { ptr: self.ptr }
    }
}

impl Drop for NodeRc {
    #[inline]
    fn drop(&mut self) {
        let count = self.count().get() - 1;
        self.count().set(count);
        if count == 0 {
            // SAFETY: this was the last handle, and it is being dropped.
            unsafe { Self::drop_slow(self.ptr) }
        }
    }
}

// Like `Rc`: the count is never left inconsistent by a panic, so a handle
// observed after an unwind behaves like any other.
impl UnwindSafe for NodeRc {}
impl RefUnwindSafe for NodeRc {}

impl Deref for NodeRc {
    type Target = NodeData;

    #[inline]
    fn deref(&self) -> &NodeData {
        // SAFETY: the box stays alive while a handle to it exists.
        unsafe { &self.ptr.as_ref().data }
    }
}

impl fmt::Debug for NodeRc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

/// A thread's free blocks, as a stack of pointers.
///
/// The stack lives in its own allocation, made on the first push, so taking a
/// block or returning one touches only the stack and not the block itself.
struct Pool {
    /// `MAX_POOLED` slots, of which the first `len` hold free blocks; null
    /// until the first push.
    slots: Cell<*mut NonNull<NodeBox>>,
    len: Cell<u32>,
}

impl Pool {
    const SLOTS_LAYOUT: Layout = match Layout::array::<NonNull<NodeBox>>(MAX_POOLED as usize) {
        Ok(layout) => layout,
        Err(_) => panic!("pool too large"),
    };

    #[inline]
    fn pop(&self) -> Option<NonNull<NodeBox>> {
        let len = self.len.get().checked_sub(1)?;
        self.len.set(len);
        // SAFETY: the first `len + 1` slots hold blocks, so the slots exist.
        Some(unsafe { self.slots.get().add(len as usize).read() })
    }

    /// Adds `block` to the pool, or returns `false` if the pool is full.
    ///
    /// # Safety
    ///
    /// `block` must be a block allocated by [allocate] that nothing references.
    #[inline]
    unsafe fn push(&self, block: NonNull<NodeBox>) -> bool {
        let len = self.len.get();
        if len >= MAX_POOLED {
            return false;
        }
        let mut slots = self.slots.get();
        if slots.is_null() {
            slots = Self::allocate_slots();
            self.slots.set(slots);
        }
        // SAFETY: `len` is less than the number of slots.
        unsafe { slots.add(len as usize).write(block) };
        self.len.set(len + 1);
        true
    }

    #[cold]
    fn allocate_slots() -> *mut NonNull<NodeBox> {
        // SAFETY: the layout is not zero-sized.
        let slots = unsafe { alloc(Self::SLOTS_LAYOUT) };
        if slots.is_null() {
            handle_alloc_error(Self::SLOTS_LAYOUT);
        }
        slots.cast()
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        while let Some(block) = self.pop() {
            // SAFETY: pooled blocks are unused and were allocated with this
            // layout.
            unsafe { dealloc(block.as_ptr().cast(), Layout::new::<NodeBox>()) };
        }
        let slots = self.slots.get();
        if !slots.is_null() {
            // SAFETY: the slots were allocated by `allocate_slots`.
            unsafe { dealloc(slots.cast(), Self::SLOTS_LAYOUT) };
        }
    }
}

thread_local! {
    static POOL: Pool = const {
        Pool {
            slots: Cell::new(std::ptr::null_mut()),
            len: Cell::new(0),
        }
    };
}

/// Returns an uninitialized block for a [NodeBox], from the current thread's
/// pool if it has one.
#[inline]
fn allocate() -> NonNull<NodeBox> {
    match POOL.try_with(Pool::pop) {
        Ok(Some(block)) => block,
        _ => allocate_global(),
    }
}

fn allocate_global() -> NonNull<NodeBox> {
    let layout = Layout::new::<NodeBox>();
    // SAFETY: `NodeBox` is not zero-sized.
    let ptr = unsafe { alloc(layout) };
    match NonNull::new(ptr) {
        Some(ptr) => ptr.cast(),
        None => handle_alloc_error(layout),
    }
}

/// Puts a block back on the current thread's pool, or frees it if the pool is
/// full or no longer available.
///
/// # Safety
///
/// `block` must have been returned by [allocate], must not be referenced
/// anymore, and must not hold a value that still needs dropping.
#[inline]
unsafe fn release(block: NonNull<NodeBox>) {
    // SAFETY: guaranteed by the caller.
    let pooled = POOL
        .try_with(|pool| unsafe { pool.push(block) })
        .unwrap_or(false);
    if !pooled {
        // SAFETY: every block is allocated by the global allocator with this
        // layout.
        unsafe { dealloc(block.as_ptr().cast(), Layout::new::<NodeBox>()) };
    }
}

#[cfg(test)]
mod tests {
    use super::{MAX_POOLED, NodeBox, POOL};
    use crate::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    use crate::{Direction, SyntaxNode};
    use std::alloc::{Layout, dealloc};
    use std::cell::RefCell;

    fn pooled() -> u32 {
        POOL.with(|pool| pool.len.get())
    }

    fn clear_pool() {
        POOL.with(|pool| {
            while let Some(block) = pool.pop() {
                // SAFETY: pooled blocks are unused and were allocated with
                // this layout.
                unsafe { dealloc(block.as_ptr().cast(), Layout::new::<NodeBox>()) };
            }
        });
    }

    /// A root with a list of `len` literals, each holding one number token.
    fn list_tree(len: usize) -> SyntaxNode<RawLanguage> {
        let mut builder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::ROOT);
        builder.start_node(RawLanguageKind::EXPRESSION_LIST);
        for _ in 0..len {
            builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
            builder.token(RawLanguageKind::NUMBER_TOKEN, "1");
            builder.finish_node();
        }
        builder.finish_node();
        builder.finish_node();
        builder.finish()
    }

    #[test]
    fn freed_block_is_reused() {
        let root = list_tree(1);
        clear_pool();

        let list = root.first_child().unwrap();
        drop(list);
        assert_eq!(pooled(), 1);

        let list = root.first_child().unwrap();
        assert_eq!(list.kind(), RawLanguageKind::EXPRESSION_LIST);
        assert_eq!(pooled(), 0);
    }

    #[test]
    fn pool_is_bounded() {
        let len = MAX_POOLED as usize * 2;
        let root = list_tree(len);
        let tokens: Vec<_> = root.descendants_tokens(Direction::Next).collect();
        assert_eq!(tokens.len(), len);

        drop(tokens);
        assert_eq!(pooled(), MAX_POOLED);
        drop(root);
        assert_eq!(pooled(), MAX_POOLED);
    }

    #[test]
    fn last_handle_releases_its_ancestors() {
        let depth = 100;
        let mut builder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::ROOT);
        for _ in 0..depth {
            builder.start_node(RawLanguageKind::BOGUS);
        }
        builder.token(RawLanguageKind::NUMBER_TOKEN, "1");
        for _ in 0..depth {
            builder.finish_node();
        }
        builder.finish_node();
        let root = builder.finish();

        let token = root.first_token().unwrap();
        clear_pool();

        // The token keeps the root and every node between them alive.
        drop(root);
        assert_eq!(pooled(), 0);
        assert_eq!(token.text(), "1");
        assert_eq!(token.ancestors().count(), depth + 1);

        drop(token);
        assert_eq!(pooled(), depth as u32 + 2);
    }

    #[test]
    fn nodes_can_be_used_after_the_pool_is_destroyed() {
        struct UseOnDrop(Option<SyntaxNode<RawLanguage>>);

        impl Drop for UseOnDrop {
            fn drop(&mut self) {
                if let Some(root) = self.0.take() {
                    // Elsewhere the order may differ, and the test then only
                    // covers nodes used while the pool is still alive.
                    #[cfg(any(target_os = "linux", target_os = "macos"))]
                    assert!(POOL.try_with(|_| ()).is_err());
                    let list = root.first_child().unwrap();
                    assert_eq!(list.kind(), RawLanguageKind::EXPRESSION_LIST);
                }
            }
        }

        thread_local! {
            static LATE: RefCell<UseOnDrop> = const { RefCell::new(UseOnDrop(None)) };
        }

        std::thread::spawn(|| {
            // Thread locals are destroyed in reverse order of first use on
            // Linux and macOS, so `LATE` outlives `POOL` and its destructor
            // allocates and frees nodes without the pool.
            LATE.with(|_| {});
            let root = list_tree(2);
            drop(root.first_child());
            LATE.with(|late| late.borrow_mut().0 = Some(root));
        })
        .join()
        .unwrap();
    }
}

//! Vendored and stripped down version of triomphe
use std::{
    alloc::{self, Layout},
    cmp::Ordering,
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem::{self, ManuallyDrop, offset_of},
    ops::Deref,
    ptr::{self, NonNull},
    sync::atomic::{
        self,
        Ordering::{Acquire, Relaxed, Release},
    },
};

/// A soft limit on the amount of references that may be made to an `Arc`.
///
/// Going above this limit will abort your program (although not
/// necessarily) at _exactly_ `MAX_REFCOUNT + 1` references.
const MAX_REFCOUNT: usize = (isize::MAX) as usize;

/// The object allocated by an Arc<T>
#[repr(C)]
pub(crate) struct ArcInner<T: ?Sized> {
    pub(crate) count: atomic::AtomicUsize,
    pub(crate) data: T,
}

unsafe impl<T: ?Sized + Sync + Send> Send for ArcInner<T> {}
unsafe impl<T: ?Sized + Sync + Send> Sync for ArcInner<T> {}

/// An atomically reference counted shared pointer
///
/// See the documentation for [`Arc`] in the standard library. Unlike the
/// standard library `Arc`, this `Arc` does not support weak reference counting.
///
/// [`Arc`]: https://doc.rust-lang.org/stable/std/sync/struct.Arc.html
#[repr(transparent)]
pub(crate) struct Arc<T: ?Sized> {
    pub(crate) p: ptr::NonNull<ArcInner<T>>,
    pub(crate) phantom: PhantomData<T>,
}

unsafe impl<T: ?Sized + Sync + Send> Send for Arc<T> {}
unsafe impl<T: ?Sized + Sync + Send> Sync for Arc<T> {}

impl<T> Arc<T> {
    /// Reconstruct the Arc<T> from a raw pointer obtained from into_raw()
    ///
    /// Note: This raw pointer will be offset in the allocation and must be preceded
    /// by the atomic count.
    ///
    /// It is recommended to use OffsetArc for this
    #[inline]
    pub(crate) unsafe fn from_raw(ptr: *const T) -> Self {
        let ptr = unsafe {
            // To find the corresponding pointer to the `ArcInner` we need
            // to subtract the offset of the `data` field from the pointer.
            let ptr = (ptr as *const u8).sub(offset_of!(ArcInner<T>, data));
            NonNull::new_unchecked(ptr as *mut ArcInner<T>)
        };
        Self {
            p: ptr,
            phantom: PhantomData,
        }
    }
}

impl<T: ?Sized> Arc<T> {
    #[inline]
    fn inner(&self) -> &ArcInner<T> {
        // This unsafety is ok because while this arc is alive we're guaranteed
        // that the inner pointer is valid. Furthermore, we know that the
        // `ArcInner` structure itself is `Sync` because the inner data is
        // `Sync` as well, so we're ok loaning out an immutable pointer to these
        // contents.
        unsafe { &*self.ptr() }
    }

    // Non-inlined part of `drop`. Just invokes the destructor.
    #[inline(never)]
    unsafe fn drop_slow(&mut self) {
        let _ = unsafe { Box::from_raw(self.ptr()) };
    }

    /// Test pointer equality between the two Arcs, i.e. they must be the _same_
    /// allocation
    #[inline]
    pub(crate) fn ptr_eq(this: &Self, other: &Self) -> bool {
        std::ptr::addr_eq(this.ptr(), other.ptr())
    }

    pub(crate) fn ptr(&self) -> *mut ArcInner<T> {
        self.p.as_ptr()
    }

    #[inline]
    pub(crate) fn into_raw(self) -> NonNull<T> {
        let ptr = NonNull::from(&self.inner().data);
        mem::forget(self);
        ptr
    }
}

impl<T: ?Sized> Clone for Arc<T> {
    #[inline]
    fn clone(&self) -> Self {
        // Using a relaxed ordering is alright here, as knowledge of the
        // original reference prevents other threads from erroneously deleting
        // the object.
        //
        // As explained in the [Boost documentation][1], Increasing the
        // reference counter can always be done with memory_order_relaxed: New
        // references to an object can only be formed from an existing
        // reference, and passing an existing reference from one thread to
        // another must already provide any required synchronization.
        //
        // [1]: (www.boost.org/doc/libs/1_55_0/doc/html/atomic/usage_examples.html)
        let old_size = self.inner().count.fetch_add(1, Relaxed);

        // However we need to guard against massive refcounts in case someone
        // is `mem::forget`ing Arcs. If we don't do this the count can overflow
        // and users will use-after free. We racily saturate to `isize::MAX` on
        // the assumption that there aren't ~2 billion threads incrementing
        // the reference count at once. This branch will never be taken in
        // any realistic program.
        //
        // We abort because such a program is incredibly degenerate, and we
        // don't care to support it.
        if old_size > MAX_REFCOUNT {
            std::process::abort();
        }

        unsafe {
            Self {
                p: ptr::NonNull::new_unchecked(self.ptr()),
                phantom: PhantomData,
            }
        }
    }
}

impl<T: ?Sized> Deref for Arc<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.inner().data
    }
}

#[expect(dead_code)]
impl<T: ?Sized> Arc<T> {
    /// Provides mutable access to the contents _if_ the `Arc` is uniquely owned.
    #[inline]
    pub(crate) fn get_mut(this: &mut Self) -> Option<&mut T> {
        if this.is_unique() {
            unsafe {
                // See make_mut() for documentation of the threadsafety here.
                Some(&mut (*this.ptr()).data)
            }
        } else {
            None
        }
    }

    /// Whether or not the `Arc` is uniquely owned (is the refcount 1?).
    pub(crate) fn is_unique(&self) -> bool {
        // See the extensive discussion in [1] for why this needs to be Acquire.
        //
        // [1] https://github.com/servo/servo/issues/21186
        self.inner().count.load(Acquire) == 1
    }
}

impl<T: ?Sized> Drop for Arc<T> {
    #[inline]
    fn drop(&mut self) {
        // Because `fetch_sub` is already atomic, we do not need to synchronize
        // with other threads unless we are going to delete the object.
        if self.inner().count.fetch_sub(1, Release) != 1 {
            return;
        }

        // FIXME(bholley): Use the updated comment when [2] is merged.
        //
        // This load is needed to prevent reordering of use of the data and
        // deletion of the data.  Because it is marked `Release`, the decreasing
        // of the reference count synchronizes with this `Acquire` load. This
        // means that use of the data happens before decreasing the reference
        // count, which happens before this load, which happens before the
        // deletion of the data.
        //
        // As explained in the [Boost documentation][1],
        //
        // > It is important to enforce any possible access to the object in one
        // > thread (through an existing reference) to *happen before* deleting
        // > the object in a different thread. This is achieved by a "release"
        // > operation after dropping a reference (any access to the object
        // > through this reference must obviously happened before), and an
        // > "acquire" operation before deleting the object.
        //
        // [1]: (www.boost.org/doc/libs/1_55_0/doc/html/atomic/usage_examples.html)
        // [2]: https://github.com/rust-lang/rust/pull/41714
        self.inner().count.load(Acquire);

        unsafe {
            self.drop_slow();
        }
    }
}

impl<T: ?Sized + PartialEq> PartialEq for Arc<T> {
    fn eq(&self, other: &Self) -> bool {
        Self::ptr_eq(self, other) || *(*self) == *(*other)
    }
}

impl<T: ?Sized + PartialOrd> PartialOrd for Arc<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (**self).partial_cmp(&**other)
    }

    fn lt(&self, other: &Self) -> bool {
        *(*self) < *(*other)
    }

    fn le(&self, other: &Self) -> bool {
        *(*self) <= *(*other)
    }

    fn gt(&self, other: &Self) -> bool {
        *(*self) > *(*other)
    }

    fn ge(&self, other: &Self) -> bool {
        *(*self) >= *(*other)
    }
}

impl<T: ?Sized + Ord> Ord for Arc<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        (**self).cmp(&**other)
    }
}

impl<T: ?Sized + Eq> Eq for Arc<T> {}

impl<T: ?Sized + Hash> Hash for Arc<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (**self).hash(state)
    }
}

#[derive(Debug, Eq, PartialEq, Hash, PartialOrd)]
#[repr(C)]
pub(crate) struct HeaderSlice<H, T: ?Sized> {
    pub(crate) header: H,
    length: usize,
    slice: T,
}

impl<H, T> HeaderSlice<H, [T]> {
    pub(crate) fn slice(&self) -> &[T] {
        &self.slice
    }

    /// Returns the number of items
    pub(crate) fn len(&self) -> usize {
        self.length
    }
}

impl<H, T> Deref for HeaderSlice<H, [T; 0]> {
    type Target = HeaderSlice<H, [T]>;

    fn deref(&self) -> &Self::Target {
        unsafe {
            let len = self.length;
            let fake_slice: *const [T] =
                ptr::slice_from_raw_parts(self as *const _ as *const T, len);
            &*(fake_slice as *const HeaderSlice<H, [T]>)
        }
    }
}

/// A "thin" `Arc` containing dynamically sized data
///
/// This is functionally equivalent to `Arc<(H, [T])>`
///
/// When you create an `Arc` containing a dynamically sized type
/// like `HeaderSlice<H, [T]>`, the `Arc` is represented on the stack
/// as a "fat pointer", where the length of the slice is stored
/// alongside the `Arc`'s pointer. In some situations you may wish to
/// have a thin pointer instead, perhaps for FFI compatibility
/// or space efficiency.
///
/// Note that we use `[T; 0]` in order to have the right alignment for `T`.
///
/// `ThinArc` solves this by storing the length in the allocation itself,
/// via `HeaderSlice`.
#[repr(transparent)]
pub(crate) struct ThinArc<H, T> {
    ptr: ptr::NonNull<ArcInner<HeaderSlice<H, [T; 0]>>>,
    phantom: PhantomData<(H, T)>,
}

unsafe impl<H: Sync + Send, T: Sync + Send> Send for ThinArc<H, T> {}
unsafe impl<H: Sync + Send, T: Sync + Send> Sync for ThinArc<H, T> {}

// Synthesize a fat pointer from a thin pointer.
fn thin_to_thick<H, T>(
    thin: *mut ArcInner<HeaderSlice<H, [T; 0]>>,
) -> *mut ArcInner<HeaderSlice<H, [T]>> {
    let len = unsafe { (*thin).data.length };
    let fake_slice: *mut [T] = ptr::slice_from_raw_parts_mut(thin as *mut T, len);
    // Transplants metadata.
    fake_slice as *mut ArcInner<HeaderSlice<H, [T]>>
}

impl<H, T> ThinArc<H, T> {
    /// Temporarily converts |self| into a bonafide Arc and exposes it to the
    /// provided callback. The refcount is not modified.
    #[inline]
    pub(crate) fn with_arc<F, U>(&self, f: F) -> U
    where
        F: FnOnce(&Arc<HeaderSlice<H, [T]>>) -> U,
    {
        // Synthesize transient Arc, which never touches the refcount of the ArcInner.
        let transient = unsafe {
            ManuallyDrop::new(Arc {
                p: ptr::NonNull::new_unchecked(thin_to_thick(self.ptr.as_ptr())),
                phantom: PhantomData,
            })
        };

        // Expose the transient Arc to the callback, which may clone it if it wants.
        // Forward the result.
        f(&transient)
    }

    /// Allocates a `ThinArc` for `num_items` items and writes its reference
    /// count and length. The header and the items are left uninitialized.
    #[inline]
    fn allocate(num_items: usize) -> ptr::NonNull<ArcInner<HeaderSlice<H, [T; 0]>>> {
        assert_ne!(mem::size_of::<T>(), 0, "Need to think about ZST");

        // Offset of the start of the slice in the allocation.
        let inner_to_data_offset = offset_of!(ArcInner<HeaderSlice<H, [T; 0]>>, data);
        let data_to_slice_offset = offset_of!(HeaderSlice<H, [T; 0]>, slice);
        let slice_offset = inner_to_data_offset + data_to_slice_offset;

        // Compute the size of the real payload.
        let slice_size = mem::size_of::<T>()
            .checked_mul(num_items)
            .expect("size overflows");
        let usable_size = slice_offset
            .checked_add(slice_size)
            .expect("size overflows");

        // Round up size to alignment.
        let align = mem::align_of::<ArcInner<HeaderSlice<H, [T; 0]>>>();
        let size = usable_size.wrapping_add(align - 1) & !(align - 1);
        assert!(size >= usable_size, "size overflows");
        let layout = Layout::from_size_align(size, align).expect("invalid layout");

        unsafe {
            let buffer = alloc::alloc(layout);

            if buffer.is_null() {
                alloc::handle_alloc_error(layout);
            }

            let ptr = buffer as *mut ArcInner<HeaderSlice<H, [T; 0]>>;
            debug_assert_eq!(
                ptr::addr_of_mut!((*ptr).data.slice) as usize - buffer as usize,
                slice_offset
            );
            ptr::write(ptr::addr_of_mut!((*ptr).count), atomic::AtomicUsize::new(1));
            ptr::write(ptr::addr_of_mut!((*ptr).data.length), num_items);
            ptr::NonNull::new_unchecked(ptr)
        }
    }

    /// Creates a `ThinArc` for a HeaderSlice using the given header struct and
    /// iterator to generate the slice.
    #[inline]
    pub(crate) fn from_header_and_iter<I>(header: H, items: I) -> Self
    where
        I: Iterator<Item = T> + ExactSizeIterator,
    {
        Self::from_header_and_iter_with(header, items, |_, item| item)
    }

    /// Creates a `ThinArc` whose slice holds `map(header, item)` for each item
    /// of `items`, in order.
    ///
    /// `map` can update the header from each item, for example to sum lengths.
    /// The header is stored after the last item has been mapped.
    ///
    /// # Panics
    ///
    /// Panics if `items` yields more or fewer items than its `len()`. If `map`
    /// or the iterator panics, the allocation and the items already written
    /// are leaked.
    #[inline]
    pub(crate) fn from_header_and_iter_with<I, F>(mut header: H, mut items: I, mut map: F) -> Self
    where
        I: ExactSizeIterator,
        F: FnMut(&mut H, I::Item) -> T,
    {
        let num_items = items.len();
        let ptr = Self::allocate(num_items);

        unsafe {
            let ptr = ptr.as_ptr();
            let mut current = ptr::addr_of_mut!((*ptr).data.slice) as *mut T;
            for _ in 0..num_items {
                let item = items
                    .next()
                    .expect("ExactSizeIterator over-reported length");
                ptr::write(current, map(&mut header, item));
                current = current.add(1);
            }
            assert!(
                items.next().is_none(),
                "ExactSizeIterator under-reported length"
            );
            ptr::write(ptr::addr_of_mut!((*ptr).data.header), header);
        }

        Self {
            ptr,
            phantom: PhantomData,
        }
    }

    /// Creates a `ThinArc` whose slice is a copy of `items`.
    #[inline]
    pub(crate) fn from_header_and_slice(header: H, items: &[T]) -> Self
    where
        T: Copy,
    {
        let ptr = Self::allocate(items.len());

        unsafe {
            let ptr = ptr.as_ptr();
            ptr::write(ptr::addr_of_mut!((*ptr).data.header), header);
            ptr::copy_nonoverlapping(
                items.as_ptr(),
                ptr::addr_of_mut!((*ptr).data.slice) as *mut T,
                items.len(),
            );
        }

        Self {
            ptr,
            phantom: PhantomData,
        }
    }
}

impl<H, T> Deref for ThinArc<H, T> {
    type Target = HeaderSlice<H, [T]>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &(*thin_to_thick(self.ptr.as_ptr())).data }
    }
}

impl<H, T> Clone for ThinArc<H, T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::with_arc(self, |a| Arc::into_thin(a.clone()))
    }
}

impl<H, T> Drop for ThinArc<H, T> {
    #[inline]
    fn drop(&mut self) {
        let _ = Arc::from_thin(Self {
            ptr: self.ptr,
            phantom: PhantomData,
        });
    }
}

impl<H, T> Arc<HeaderSlice<H, [T]>> {
    /// Converts an `Arc` into a `ThinArc`. This consumes the `Arc`, so the refcount
    /// is not modified.
    #[inline]
    pub(crate) fn into_thin(a: Self) -> ThinArc<H, T> {
        assert_eq!(
            a.length,
            a.slice.len(),
            "Length needs to be correct for ThinArc to work"
        );
        let fat_ptr: *mut ArcInner<HeaderSlice<H, [T]>> = a.ptr();
        mem::forget(a);
        let thin_ptr = fat_ptr as *mut [usize] as *mut usize;
        ThinArc {
            ptr: unsafe {
                ptr::NonNull::new_unchecked(thin_ptr as *mut ArcInner<HeaderSlice<H, [T; 0]>>)
            },
            phantom: PhantomData,
        }
    }

    /// Converts a `ThinArc` into an `Arc`. This consumes the `ThinArc`, so the refcount
    /// is not modified.
    #[inline]
    pub(crate) fn from_thin(a: ThinArc<H, T>) -> Self {
        let ptr = thin_to_thick(a.ptr.as_ptr());
        mem::forget(a);
        unsafe {
            Self {
                p: ptr::NonNull::new_unchecked(ptr),
                phantom: PhantomData,
            }
        }
    }
}

impl<H: PartialEq, T: PartialEq> PartialEq for ThinArc<H, T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl<H: Eq, T: Eq> Eq for ThinArc<H, T> {}

impl<H: Hash, T: Hash> Hash for ThinArc<H, T> {
    fn hash<HSR: Hasher>(&self, state: &mut HSR) {
        (**self).hash(state)
    }
}

#[cfg(test)]
mod tests {
    use super::ThinArc;

    #[test]
    fn from_header_and_slice_copies_items() {
        let arc = ThinArc::from_header_and_slice(7u16, b"hello");
        assert_eq!(arc.header, 7);
        assert_eq!(arc.len(), 5);
        assert_eq!(arc.slice(), b"hello");

        let empty = ThinArc::<u16, u8>::from_header_and_slice(1, &[]);
        assert_eq!(empty.header, 1);
        assert_eq!(empty.len(), 0);
        assert!(empty.slice().is_empty());
    }

    #[test]
    fn from_header_and_iter_with_updates_header_in_order() {
        let arc =
            ThinArc::from_header_and_iter_with(0u32, [1u32, 2, 3].into_iter(), |sum, item| {
                let offset = *sum;
                *sum += item;
                offset
            });
        assert_eq!(arc.header, 6);
        assert_eq!(arc.slice(), &[0, 1, 3]);

        let empty =
            ThinArc::from_header_and_iter_with(5u32, std::iter::empty::<u32>(), |_, item| item);
        assert_eq!(empty.header, 5);
        assert!(empty.slice().is_empty());
    }

    /// An iterator whose `len()` claims more items than it yields.
    struct OverReported;

    impl Iterator for OverReported {
        type Item = u32;

        fn next(&mut self) -> Option<u32> {
            None
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (2, Some(2))
        }
    }

    impl ExactSizeIterator for OverReported {}

    // The panic leaks the allocation by design, which Miri reports as an error.
    #[cfg_attr(miri, ignore)]
    #[test]
    #[should_panic(expected = "ExactSizeIterator over-reported length")]
    fn from_header_and_iter_with_panics_on_missing_items() {
        ThinArc::from_header_and_iter_with((), OverReported, |_, item| item);
    }

    /// An iterator whose `len()` claims fewer items than it yields.
    struct UnderReported(u32);

    impl Iterator for UnderReported {
        type Item = u32;

        fn next(&mut self) -> Option<u32> {
            self.0 += 1;
            (self.0 <= 2).then_some(self.0)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (1, Some(1))
        }
    }

    impl ExactSizeIterator for UnderReported {}

    // The panic leaks the allocation by design, which Miri reports as an error.
    #[cfg_attr(miri, ignore)]
    #[test]
    #[should_panic(expected = "ExactSizeIterator under-reported length")]
    fn from_header_and_iter_with_panics_on_extra_items() {
        ThinArc::from_header_and_iter_with((), UnderReported(0), |_, item| item);
    }
}

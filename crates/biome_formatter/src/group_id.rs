use std::num::NonZeroU32;
use std::sync::atomic::{AtomicU32, Ordering};

#[cfg(debug_assertions)]
mod debug {
    use super::*;

    #[derive(Clone, Copy, Eq, PartialEq, Hash)]
    pub struct GroupId {
        pub(super) value: NonZeroU32,
        name: &'static str,
    }

    impl GroupId {
        pub(super) fn new(value: NonZeroU32, debug_name: &'static str) -> Self {
            Self {
                value,
                name: debug_name,
            }
        }
    }

    impl std::fmt::Debug for GroupId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "#{}-{}", self.name, self.value)
        }
    }
}

#[cfg(not(debug_assertions))]
mod release {
    use super::*;

    /// Unique identification for a group.
    ///
    /// See [crate::Formatter::group_id] on how to get a unique id.
    #[repr(transparent)]
    #[derive(Clone, Copy, Eq, PartialEq, Hash)]
    pub struct GroupId {
        pub(super) value: NonZeroU32,
    }

    impl GroupId {
        /// Creates a new unique group id with the given debug name (only stored in debug builds)
        #[cfg_attr(debug_assertions, expect(unused))]
        pub(super) fn new(value: NonZeroU32, _: &'static str) -> Self {
            Self { value }
        }
    }

    impl std::fmt::Debug for GroupId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "#{}", self.value)
        }
    }
}

#[cfg(not(debug_assertions))]
pub type GroupId = release::GroupId;
#[cfg(debug_assertions)]
pub type GroupId = debug::GroupId;

impl From<GroupId> for u32 {
    fn from(id: GroupId) -> Self {
        id.value.get()
    }
}

/// Builder to construct unique group ids that are unique if created with the same builder.
pub(super) struct UniqueGroupIdBuilder {
    next_id: AtomicU32,
}

impl UniqueGroupIdBuilder {
    /// Creates a new unique group id with the given debug name.
    pub fn group_id(&self, debug_name: &'static str) -> GroupId {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let id = NonZeroU32::new(id).unwrap_or_else(|| panic!("Group ID counter overflowed"));

        GroupId::new(id, debug_name)
    }
}

impl Default for UniqueGroupIdBuilder {
    fn default() -> Self {
        UniqueGroupIdBuilder {
            // Start with 1 because `GroupId` wraps a `NonZeroU32` to reduce memory usage.
            next_id: AtomicU32::new(1),
        }
    }
}

use std::num::{NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8};

/// Simple trait to merge two types of the same type
pub trait MergeWith<T> {
    /// Merges `other` into `self`.
    ///
    /// Values that are non-`None` in `other` will take precedence over values
    /// in `self`. Complex types may get recursively merged instead of
    /// overwritten.
    fn merge_with(&mut self, other: T);
}

impl<M> MergeWith<Option<M>> for M
where
    M: MergeWith<M>,
{
    fn merge_with(&mut self, other: Option<M>) {
        if let Some(other) = other {
            self.merge_with(other);
        }
    }
}

/// This macro is used to implement [MergeWith] for all (primitive) types where
/// merging can simply be implemented through overwriting the value.
macro_rules! overwrite_on_merge {
    ( $ty:ident ) => {
        impl MergeWith<$ty> for $ty {
            fn merge_with(&mut self, other: $ty) {
                *self = other
            }
        }
    };
}

overwrite_on_merge!(bool);
overwrite_on_merge!(u8);
overwrite_on_merge!(u16);
overwrite_on_merge!(u32);
overwrite_on_merge!(u64);
overwrite_on_merge!(i8);
overwrite_on_merge!(i16);
overwrite_on_merge!(i32);
overwrite_on_merge!(i64);
overwrite_on_merge!(NonZeroU8);
overwrite_on_merge!(NonZeroU16);
overwrite_on_merge!(NonZeroU32);
overwrite_on_merge!(NonZeroU64);
overwrite_on_merge!(String);

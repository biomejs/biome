use std::num::{NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8};

/// Trait that allows deep merging of types, including injection of defaults.
pub trait Merge {
    /// Merges `other` into `self`.
    ///
    /// Values that are non-`None` in `other` will take precedence over values
    /// in `self`. Complex types may get recursively merged instead of
    /// overwritten.
    fn merge_with(&mut self, other: Self);
}

impl<T> Merge for Option<T>
where
    T: Merge,
{
    fn merge_with(&mut self, other: Self) {
        if let Some(other) = other {
            match self.as_mut() {
                Some(this) => this.merge_with(other),
                None => *self = Some(other),
            }
        }
    }
}

/// This macro is used to implement [Merge] for all (primitive) types where
/// merging can simply be implemented through overwriting the value.
macro_rules! overwrite_on_merge {
    ( $ty:ident ) => {
        impl Merge for $ty {
            fn merge_with(&mut self, other: Self) {
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

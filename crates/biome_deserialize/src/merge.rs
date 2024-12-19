use std::hash::{BuildHasher, Hash};

/// Trait that allows deep merging of types, including injection of defaults.
pub trait Merge {
    /// Merges `other` into `self`.
    ///
    /// Values that are non-`None` in `other` will take precedence over values
    /// in `self`. Complex types may get recursively merged instead of
    /// overwritten.
    fn merge_with(&mut self, other: Self);
}

impl<T: Merge> Merge for Box<T> {
    fn merge_with(&mut self, other: Self) {
        self.as_mut().merge_with(*other);
    }
}

impl<T: Merge> Merge for Option<T> {
    fn merge_with(&mut self, other: Self) {
        if let Some(other) = other {
            match self.as_mut() {
                Some(this) => this.merge_with(other),
                None => *self = Some(other),
            }
        }
    }
}

impl<T> Merge for Vec<T> {
    fn merge_with(&mut self, other: Self) {
        self.extend(other);
    }
}

impl<T: Eq + Hash, S: BuildHasher + Default> Merge for std::collections::HashSet<T, S> {
    fn merge_with(&mut self, other: Self) {
        self.extend(other);
    }
}

impl<K: Hash + Eq, V: Merge, S: Default + BuildHasher> Merge
    for std::collections::HashMap<K, V, S>
{
    fn merge_with(&mut self, other: Self) {
        for (k, v) in other {
            if let Some(self_value) = self.get_mut(&k) {
                self_value.merge_with(v);
            } else {
                self.insert(k, v);
            }
        }
    }
}

impl<V: Ord> Merge for std::collections::BTreeSet<V> {
    fn merge_with(&mut self, other: Self) {
        self.extend(other);
    }
}

impl<K: Ord, V: Merge> Merge for std::collections::BTreeMap<K, V> {
    fn merge_with(&mut self, other: Self) {
        for (k, v) in other {
            if let Some(self_value) = self.get_mut(&k) {
                self_value.merge_with(v);
            } else {
                self.insert(k, v);
            }
        }
    }
}

#[cfg(feature = "indexmap")]
impl<T: Hash + Eq> Merge for indexmap::IndexSet<T> {
    fn merge_with(&mut self, other: Self) {
        self.extend(other);
    }
}

#[cfg(feature = "indexmap")]
impl<K: Hash + Eq, V: Merge, S: Default + BuildHasher> Merge for indexmap::IndexMap<K, V, S> {
    fn merge_with(&mut self, other: Self) {
        for (k, v) in other {
            if let Some(self_value) = self.get_mut(&k) {
                self_value.merge_with(v);
            } else {
                self.insert(k, v);
            }
        }
    }
}

/// This macro is used to implement [Merge] for all (primitive) types where
/// merging can simply be implemented through overwriting the value.
macro_rules! overwrite_on_merge {
    ( $ty:path ) => {
        impl Merge for $ty {
            fn merge_with(&mut self, other: Self) {
                *self = other;
            }
        }
    };
}

overwrite_on_merge!(bool);
overwrite_on_merge!(f32);
overwrite_on_merge!(f64);
overwrite_on_merge!(i16);
overwrite_on_merge!(i32);
overwrite_on_merge!(i64);
overwrite_on_merge!(i8);
overwrite_on_merge!(isize);
overwrite_on_merge!(u16);
overwrite_on_merge!(u32);
overwrite_on_merge!(u64);
overwrite_on_merge!(u8);
overwrite_on_merge!(usize);

overwrite_on_merge!(std::num::NonZeroU16);
overwrite_on_merge!(std::num::NonZeroU32);
overwrite_on_merge!(std::num::NonZeroU64);
overwrite_on_merge!(std::num::NonZeroU8);
overwrite_on_merge!(std::num::NonZeroUsize);
overwrite_on_merge!(String);
overwrite_on_merge!(Box<str>);

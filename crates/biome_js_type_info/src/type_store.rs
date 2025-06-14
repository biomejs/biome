use std::{
    borrow::Cow,
    hash::{Hash, Hasher},
    num::NonZeroU32,
};

use hashbrown::{HashTable, hash_table::Entry};
use rustc_hash::FxHasher;

use crate::{Resolvable, ResolvedTypeId, TypeData, TypeId, TypeReference, TypeResolverLevel};

/// Type store with efficient lookup mechanism.
///
/// Type resolvers are responsible for storing types. The simplest resolvers can
/// store their types in a `Vec`, but this makes registration expensive when
/// they try to avoid registering duplicate types. For this reason, the
/// `TypeStore` exists.
///
/// `TypeStore` offers extremely fast lookups by `TypeId`, as fast as when using
/// a `Vec`, but offers the performance of a hash set when looking up a
/// `TypeData` reference.
///
/// Unlike a hash set, `TypeStore` allows duplicate types to be stored, even if
/// some amount of deduplication is preferred. The reason for this is that type
/// flattening can reduce disjoint types to the same type, at which point we
/// store both to preserve their indices.
#[derive(Debug, Default)]
pub struct TypeStore {
    types: Vec<TypeData>,
    table: HashTable<usize>,
}

impl TypeStore {
    pub fn from_types(types: Vec<TypeData>) -> Self {
        let mut table = HashTable::with_capacity(types.len());
        for (i, data) in types.iter().enumerate() {
            let hash = hash_data(data);
            table.insert_unique(hash, i, |_| {
                unreachable!("we should've reserved sufficient capacity")
            });
        }
        Self { types, table }
    }

    pub fn as_slice(&self) -> &[TypeData] {
        &self.types
    }

    /// Deduplicates all types in the store.
    ///
    /// Warning: Any `TypeId`s (including `ResolvedTypeId`s) that reference this
    ///          store may become invalidated. The return value gives you a
    ///          callback that can be used for updating `ResolvedTypeId`s.
    ///
    /// Returns `None` if there were no duplicate types.
    pub fn deduplicate(
        &mut self,
        level: TypeResolverLevel,
    ) -> Option<impl Fn(&mut ResolvedTypeId)> {
        struct MapEntry {
            mapped_index: Option<NonZeroU32>,
            num_removed_types_up_to_here: u32,
        }

        // First find all duplicates and create a map where the "original" can
        // be found. Whichever index the hash table points at is considered the
        // original.
        //
        // While populating the map, we also track how many types have been
        // removed at a given index. We need this for compensating the resolved
        // IDs.
        let mut num_removed_types = 0;
        let map: Vec<MapEntry> = self
            .types
            .iter()
            .enumerate()
            .map(|(i, data)| {
                let hash = hash_data(data);
                let expected_index = self.table.find(hash, |i| self.types[*i] == *data);

                let mapped_index = if expected_index.is_none_or(|index| *index == i) {
                    None
                } else {
                    num_removed_types += 1;
                    expected_index
                        // SAFETY: Index is already unsigned, so adding 1 is
                        //         guaranteed to make it non-zero. We also know
                        //         there can't be an overflow, since we don't
                        //         process files over 2GB in size.
                        .map(|index| unsafe { NonZeroU32::new_unchecked(*index as u32 + 1) })
                };

                MapEntry {
                    mapped_index,
                    num_removed_types_up_to_here: num_removed_types,
                }
            })
            .collect();

        if num_removed_types == 0 {
            return None;
        }

        // Now filter all types so only the "originals" remain.
        self.types = std::mem::take(&mut self.types)
            .into_iter()
            .enumerate()
            .filter_map(|(i, data)| map[i].mapped_index.is_none().then_some(data))
            .collect();

        let update_resolved_id = move |resolved_id: &mut ResolvedTypeId| {
            if resolved_id.level() == level {
                let old_index = resolved_id.index();
                let new_index = map[old_index]
                    .mapped_index
                    .map_or(old_index as u32, |index| index.get() - 1);
                let new_index = new_index - map[new_index as usize].num_removed_types_up_to_here;
                *resolved_id = ResolvedTypeId::new(level, TypeId::new(new_index as usize));
            }
        };

        // Update all references and the hash table.
        self.table.clear();
        self.table
            .shrink_to(self.types.len(), |_| unreachable!("table should be empty"));
        for (i, data) in self.types.iter_mut().enumerate() {
            data.update_all_references(|reference| {
                if let TypeReference::Resolved(resolved_id) = reference {
                    update_resolved_id(resolved_id)
                }
            });

            let hash = hash_data(data);
            self.table.insert_unique(hash, i, |_| {
                unreachable!("we should've reserved sufficient capacity")
            });
        }

        Some(update_resolved_id)
    }

    /// Returns the `TypeId` of the given `data`, if it is registered.
    pub fn find_type(&self, data: &TypeData) -> Option<TypeId> {
        let hash = hash_data(data);

        self.table
            .find(hash, |i| self.types[*i] == *data)
            .map(|index| TypeId::new(*index))
    }

    pub fn get_by_id(&self, id: TypeId) -> &TypeData {
        &self.types[id.index()]
    }

    pub fn is_empty(&self) -> bool {
        self.types.is_empty()
    }

    pub fn len(&self) -> usize {
        self.types.len()
    }

    /// Registers the given `data` if it is not registered yet.
    ///
    /// Returns the `TypeId` of the newly registered data, or the `TypeId` of
    /// an already registered equivalent type.
    pub fn register_type(&mut self, data: Cow<TypeData>) -> TypeId {
        let entry = self.table.entry(
            hash_data(&data),
            |i| &self.types[*i] == data.as_ref(),
            |i| hash_data(&self.types[*i]),
        );
        match entry {
            Entry::Occupied(entry) => TypeId::new(*entry.get()),
            Entry::Vacant(entry) => {
                let index = self.types.len();
                self.types.push(data.into_owned());
                entry.insert(index);
                TypeId::new(index)
            }
        }
    }

    /// Takes the type at the given `index` and swaps it for
    /// `TypeData::Unknown`.
    ///
    /// The returned type is correctly removed from the store's hash table, but
    /// the `TypeData::Unknown` that takes its place is never registered. This
    /// is done on the assumption that an updated type will be reinserted before
    /// any other lookups are performed. This allows us to avoid unnecessarily
    /// update the hash table twice, but makes the API slightly unsafe to use.
    ///
    /// # Safety
    ///
    /// Callers must promise to reinsert the (updated) value before new lookups
    /// are performed. They must use
    /// [`Self::reinsert_temporarily_taken_data()`] for this.
    pub unsafe fn take_from_index_temporarily(&mut self, index: usize) -> TypeData {
        let data = std::mem::take(&mut self.types[index]);

        let hash = hash_data(&data);

        if let Ok(occupied) = self.table.find_entry(hash, |i| self.types[*i] == data) {
            occupied.remove();
        }

        data
    }

    /// Reinserts an (updated) value that was taken using
    /// [`Self::take_from_index_temporarily()`].
    ///
    /// # Safety
    ///
    /// Callers must only call this once after calling
    /// [`Self::take_from_index_temporarily()`] with the same index, before any
    /// lookups are performed.
    pub unsafe fn reinsert_temporarily_taken_data(&mut self, index: usize, data: TypeData) {
        let hash = hash_data(&data);
        self.types[index] = data;
        self.table
            .insert_unique(hash, index, |i| hash_data(&self.types[*i]));
    }
}

impl From<TypeStore> for Box<[TypeData]> {
    fn from(store: TypeStore) -> Self {
        store.types.into()
    }
}

#[inline(always)]
fn hash_data(data: &TypeData) -> u64 {
    let mut hash = FxHasher::default();
    data.hash(&mut hash);
    hash.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deduplication() {
        let mut store = TypeStore::from_types(vec![
            TypeData::String,
            TypeData::Number,
            TypeData::String,
            TypeData::Reference(TypeReference::Resolved(ResolvedTypeId::new(
                TypeResolverLevel::Thin,
                TypeId::new(2),
            ))),
        ]);
        store.deduplicate(TypeResolverLevel::Thin);

        let expected = &[
            TypeData::String,
            TypeData::Number,
            TypeData::Reference(TypeReference::Resolved(ResolvedTypeId::new(
                TypeResolverLevel::Thin,
                TypeId::new(0),
            ))),
        ];
        assert_eq!(store.as_slice(), expected);

        let mut store = TypeStore::from_types(vec![
            TypeData::String,
            TypeData::Number,
            TypeData::String,
            TypeData::Reference(TypeReference::Resolved(ResolvedTypeId::new(
                TypeResolverLevel::Thin,
                TypeId::new(2),
            ))),
            TypeData::Reference(TypeReference::Resolved(ResolvedTypeId::new(
                TypeResolverLevel::Thin,
                TypeId::new(6),
            ))),
            TypeData::Number,
            TypeData::Null,
        ]);
        store.deduplicate(TypeResolverLevel::Thin);

        let expected = &[
            TypeData::String,
            TypeData::Number,
            TypeData::Reference(TypeReference::Resolved(ResolvedTypeId::new(
                TypeResolverLevel::Thin,
                TypeId::new(0),
            ))),
            TypeData::Reference(TypeReference::Resolved(ResolvedTypeId::new(
                TypeResolverLevel::Thin,
                TypeId::new(4),
            ))),
            TypeData::Null,
        ];
        assert_eq!(store.as_slice(), expected);
    }
}

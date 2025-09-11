use std::{
    borrow::Cow,
    hash::{Hash, Hasher},
    num::NonZeroU32,
    sync::Arc,
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
#[derive(Default)]
pub struct TypeStore {
    types: Vec<Arc<TypeData>>,
    table: HashTable<usize>,
}

impl TypeStore {
    pub fn from_types(types: impl Into<Vec<Arc<TypeData>>>) -> Self {
        let types = types.into();
        let mut table = HashTable::with_capacity(types.len());
        for (i, data) in types.iter().enumerate() {
            let hash = hash_data(data);
            table.insert_unique(hash, i, |_| {
                unreachable!("we should've reserved sufficient capacity")
            });
        }
        Self { types, table }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            types: Vec::with_capacity(capacity),
            table: HashTable::with_capacity(capacity),
        }
    }

    pub fn as_references(&self) -> Vec<&TypeData> {
        self.types.iter().map(Arc::as_ref).collect()
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
                let expected_index = self
                    .table
                    .find(hash, |i| self.types[*i].as_ref() == data.as_ref());

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
            let data = Arc::get_mut(data).expect("type data must be unique");
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
    pub fn find(&self, data: &TypeData) -> Option<TypeId> {
        let hash = hash_data(data);

        self.table
            .find(hash, |i| self.types[*i].as_ref() == data)
            .map(|index| TypeId::new(*index))
    }

    pub fn get(&self, index: usize) -> Arc<TypeData> {
        self.types[index].clone()
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

    /// Inserts the given `data` if it is not registered yet.
    ///
    /// Returns the `TypeId` of the newly registered data, or the `TypeId` of
    /// an already registered equivalent type.
    pub fn insert_arc(&mut self, data: &Arc<TypeData>) -> TypeId {
        let entry = self.table.entry(
            hash_data(data.as_ref()),
            |i| self.types[*i].as_ref() == data.as_ref(),
            |i| hash_data(&self.types[*i]),
        );
        match entry {
            Entry::Occupied(entry) => TypeId::new(*entry.get()),
            Entry::Vacant(entry) => {
                let index = self.types.len();
                self.types.push(data.clone());
                entry.insert(index);
                TypeId::new(index)
            }
        }
    }

    /// Inserts the given `data` if it is not registered yet.
    ///
    /// Returns the `TypeId` of the newly registered data, or the `TypeId` of
    /// an already registered equivalent type.
    pub fn insert_cow(&mut self, data: Cow<TypeData>) -> TypeId {
        let entry = self.table.entry(
            hash_data(&data),
            |i| self.types[*i].as_ref() == data.as_ref(),
            |i| hash_data(&self.types[*i]),
        );
        match entry {
            Entry::Occupied(entry) => TypeId::new(*entry.get()),
            Entry::Vacant(entry) => {
                let index = self.types.len();
                self.types.push(Arc::new(data.into_owned()));
                entry.insert(index);
                TypeId::new(index)
            }
        }
    }

    /// Replaces the type at the given index.
    ///
    /// The new type should be semantically equivalent to the old one, so as not
    /// to invalidate references pointing to the type by
    /// `TypeId`/`ResolvedTypeId`.
    ///
    /// For instance, this may be useful to update a type after it has been
    /// resolved and/or flattened.
    pub fn replace(&mut self, index: usize, data: TypeData) {
        let new_hash = hash_data(&data);
        let old_hash = hash_data(&self.types[index]);

        if new_hash != old_hash {
            if let Ok(occupied) = self.table.find_entry(old_hash, |i| *i == index) {
                occupied.remove();
            }

            let entry = self.table.entry(
                hash_data(&data),
                |i| self.types[*i].as_ref() == &data,
                |i| hash_data(&self.types[*i]),
            );
            match entry {
                Entry::Occupied(mut entry) => {
                    *entry.get_mut() = index;
                }
                Entry::Vacant(entry) => {
                    entry.insert(index);
                }
            }
        }

        self.types[index] = Arc::new(data);
    }
}

impl From<TypeStore> for Vec<Arc<TypeData>> {
    fn from(store: TypeStore) -> Self {
        store.types
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
        let mut store = TypeStore::from_types(
            vec![
                TypeData::String,
                TypeData::Number,
                TypeData::String,
                TypeData::Reference(TypeReference::Resolved(ResolvedTypeId::new(
                    TypeResolverLevel::Thin,
                    TypeId::new(2),
                ))),
            ]
            .into_iter()
            .map(Arc::new)
            .collect::<Vec<_>>(),
        );
        store.deduplicate(TypeResolverLevel::Thin);

        let expected = [
            TypeData::String,
            TypeData::Number,
            TypeData::Reference(TypeReference::Resolved(ResolvedTypeId::new(
                TypeResolverLevel::Thin,
                TypeId::new(0),
            ))),
        ];
        assert_eq!(store.as_references(), expected.iter().collect::<Vec<_>>());

        let mut store = TypeStore::from_types(
            vec![
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
            ]
            .into_iter()
            .map(Arc::new)
            .collect::<Vec<_>>(),
        );
        store.deduplicate(TypeResolverLevel::Thin);

        let expected = [
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
        assert_eq!(store.as_references(), expected.iter().collect::<Vec<_>>());
    }
}

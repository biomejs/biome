use std::{
    borrow::Cow,
    hash::{Hash, Hasher},
};

use hashbrown::{HashTable, hash_table::Entry};
use rustc_hash::FxHasher;

use crate::{TypeData, TypeId};

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
    pub fn as_slice(&self) -> &[TypeData] {
        &self.types
    }

    /// Returns the `TypeId` of the given `data`, if it is registered.
    pub fn find_type(&self, data: &TypeData) -> Option<TypeId> {
        let mut hash = FxHasher::default();
        data.hash(&mut hash);
        let hash = hash.finish();

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
        let mut hash = FxHasher::default();
        data.hash(&mut hash);
        let hash = hash.finish();

        let entry = self.table.entry(
            hash,
            |i| &self.types[*i] == data.as_ref(),
            |i| {
                let mut hash = FxHasher::default();
                self.types[*i].hash(&mut hash);
                hash.finish()
            },
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

        let mut hash = FxHasher::default();
        data.hash(&mut hash);
        let hash = hash.finish();

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
        let mut hash = FxHasher::default();
        data.hash(&mut hash);
        let hash = hash.finish();

        self.types[index] = data;

        self.table.insert_unique(hash, index, |i| {
            let mut hash = FxHasher::default();
            self.types[*i].hash(&mut hash);
            hash.finish()
        });
    }
}

impl From<TypeStore> for Box<[TypeData]> {
    fn from(store: TypeStore) -> Self {
        store.types.into()
    }
}

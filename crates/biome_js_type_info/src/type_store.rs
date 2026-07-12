use std::{
    borrow::Cow,
    hash::{Hash, Hasher},
};

use hashbrown::{HashTable, hash_table::Entry};
use rustc_hash::FxHasher;

use biome_js_semantic::ScopeId;
use biome_js_syntax::AnyJsExpression;

use crate::{RawTypeId, TypeData, TypeId, TypeReference, Union, globals::GLOBAL_UNDEFINED_ID};

/// Collector-side type table with indexed access and deduplicated insertion.
#[derive(Default)]
pub struct TypeStore {
    types: Vec<TypeData>,
    table: HashTable<usize>,
}

impl TypeStore {
    pub fn from_types(types: impl Into<Vec<TypeData>>) -> Self {
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
        self.types.iter().collect()
    }

    /// Returns the `TypeId` of the given `data`, if it is registered.
    pub fn find(&self, data: &TypeData) -> Option<TypeId> {
        let hash = hash_data(data);

        self.table
            .find(hash, |i| &self.types[*i] == data)
            .map(|index| TypeId::new(*index))
    }

    pub fn get_by_id(&self, id: TypeId) -> &TypeData {
        &self.types[id.index()]
    }

    /// Inserts the given `data` if it is not registered yet.
    ///
    /// Returns the `TypeId` of the newly registered data, or the `TypeId` of
    /// an already registered equivalent type.
    pub fn insert_cow(&mut self, data: Cow<TypeData>) -> TypeId {
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
}

impl From<TypeStore> for Vec<TypeData> {
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

/// Interface used while collecting a module's stable raw type table.
pub trait RawTypeCollector {
    fn find_type(&self, type_data: &TypeData) -> Option<TypeId>;
    fn get_by_id(&self, id: TypeId) -> &TypeData;
    fn register_type(&mut self, type_data: Cow<TypeData>) -> TypeId;
    fn resolve_expression(
        &mut self,
        scope_id: ScopeId,
        expression: &AnyJsExpression,
    ) -> Cow<'_, TypeData>;

    fn get_by_reference(&self, ty: &TypeReference) -> Option<&TypeData> {
        let TypeReference::Resolved(RawTypeId::Local(id)) = ty else {
            return None;
        };
        Some(self.get_by_id(*id))
    }

    fn reference_to_id(&self, id: TypeId) -> TypeReference {
        TypeReference::Resolved(RawTypeId::Local(id))
    }

    fn reference_to_data(&self, type_data: &TypeData) -> Option<TypeReference> {
        match type_data {
            TypeData::Reference(reference) => Some(reference.clone()),
            other => self.find_type(other).map(|id| self.reference_to_id(id)),
        }
    }

    fn reference_to_registered_data(&mut self, type_data: &TypeData) -> TypeReference {
        match type_data {
            TypeData::Reference(reference) => reference.clone(),
            _ => {
                let id = self.register_type(Cow::Borrowed(type_data));
                self.reference_to_id(id)
            }
        }
    }

    fn reference_to_owned_data(&mut self, type_data: TypeData) -> TypeReference {
        match type_data {
            TypeData::Reference(reference) => reference,
            _ => {
                let id = self.register_type(Cow::Owned(type_data));
                self.reference_to_id(id)
            }
        }
    }

    fn reference_to_resolved_expression(
        &mut self,
        scope_id: ScopeId,
        expression: &AnyJsExpression,
    ) -> TypeReference {
        let data = self.resolve_expression(scope_id, expression).into_owned();
        self.reference_to_owned_data(data)
    }

    fn register_and_resolve(&mut self, type_data: TypeData) -> RawTypeId {
        match type_data {
            TypeData::Reference(TypeReference::Resolved(id)) => id,
            type_data => RawTypeId::Local(self.register_type(Cow::Owned(type_data))),
        }
    }

    fn optional(&mut self, ty: TypeReference) -> TypeId {
        self.register_type(Cow::Owned(TypeData::Union(Box::new(Union(Box::new([
            ty,
            GLOBAL_UNDEFINED_ID.into(),
        ]))))))
    }
}

#[derive(Default)]
pub struct UnionCollector {
    types: Vec<TypeReference>,
}

impl UnionCollector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, ty: TypeReference) {
        self.types.push(ty);
    }

    pub fn finish(self) -> Cow<'static, TypeData> {
        if self.types.is_empty() {
            return Cow::Owned(TypeData::unknown());
        }
        Cow::Owned(TypeData::Union(Box::new(Union(
            self.types.into_boxed_slice(),
        ))))
    }
}

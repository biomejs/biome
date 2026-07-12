#![deny(clippy::wildcard_enum_match_arm)]

use crate::ModuleDb;
use biome_css_syntax::TextRange;
use biome_js_type_info::interned_types::{LocalTypeId, ModuleKey, TypeData as InferredTypeData};
use rustc_hash::FxHashMap;

mod expressions;
mod globals;
mod imports;
mod lookup;
mod qualifiers;
mod resolver;

pub(in crate::db) use resolver::resolve_raw_types;

#[derive(Clone, Copy, Debug, Eq, PartialEq, salsa::Update)]
pub struct BindingTypeData<'db> {
    pub ty: InferredTypeData<'db>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InferredModuleTypes<'db> {
    pub module_key: ModuleKey,
    pub named_type_ids: Box<[LocalTypeId]>,
    pub types: Box<[InferredTypeData<'db>]>,
    pub expressions: FxHashMap<TextRange, InferredTypeData<'db>>,
    pub binding_type_data: FxHashMap<TextRange, BindingTypeData<'db>>,
}

// SAFETY: This struct does not borrow from the database. It owns the ranges, and
// the types are small handles created by Salsa. Comparing the old maps with the
// new maps is safe; if they differ, replacing the old maps exposes the same data
// as updating each entry one by one.
unsafe impl salsa::Update for InferredModuleTypes<'_> {
    unsafe fn maybe_update(old_pointer: *mut Self, new_value: Self) -> bool {
        let old_value = unsafe { &mut *old_pointer };
        if *old_value == new_value {
            false
        } else {
            *old_value = new_value;
            true
        }
    }
}

impl<'db> InferredModuleTypes<'db> {
    pub fn resolve_type(
        &self,
        db: &'db dyn ModuleDb,
        ty: InferredTypeData<'db>,
    ) -> InferredTypeData<'db> {
        self.resolve_type_iterative(db, ty)
    }

    pub fn find_member_type(
        &self,
        db: &'db dyn ModuleDb,
        ty: InferredTypeData<'db>,
        name: &str,
    ) -> Option<InferredTypeData<'db>> {
        self.find_member_type_iterative(db, ty, name)
    }
}

pub(super) fn collected_type_result<'db>(
    db: &'db dyn ModuleDb,
    types: Vec<InferredTypeData<'db>>,
) -> Option<InferredTypeData<'db>> {
    if types.is_empty() {
        None
    } else {
        Some(InferredTypeData::union_from_types(db, types))
    }
}

pub(super) fn infer_module_types_cycle_result<'db>(
    _db: &'db dyn ModuleDb,
    _id: salsa::Id,
    _module: crate::module_graph::ModuleInfo,
) -> Option<InferredModuleTypes<'db>> {
    None
}

pub(super) fn normalize_type_cycle_result<'db>(
    _db: &'db dyn ModuleDb,
    _id: salsa::Id,
    _input: crate::db::queries::NormalizeTypeInput<'db>,
) -> InferredTypeData<'db> {
    InferredTypeData::Unknown
}

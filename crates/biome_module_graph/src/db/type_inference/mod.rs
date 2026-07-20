#![deny(clippy::wildcard_enum_match_arm)]

use crate::ModuleDb;
use biome_css_syntax::TextRange;
use biome_js_type_info::interned_types::{
    LocalTypeId, ModuleKey, TypeData as InferredTypeData, TypeTransformError,
};
use rustc_hash::FxHashMap;

use self::globals::global_type;

mod expressions;
mod globals;
mod imports;
mod lookup;
mod qualifiers;
mod resolver;

pub(in crate::db) use lookup::{apply_substitutions_to_root_body, substitutions_for_instance};
pub(in crate::db) use resolver::resolve_raw_types;

/// Type information attached to one binding declaration.
#[derive(Clone, Copy, Debug, Eq, PartialEq, salsa::Update)]
pub struct BindingTypeData<'db> {
    /// Inferred type of the declared binding.
    pub ty: InferredTypeData<'db>,
}

/// Resolved type tables produced for one JavaScript or TypeScript module.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InferredModuleTypes<'db> {
    /// Stable key stored in local type handles owned by this module.
    pub module_key: ModuleKey,
    /// Local type IDs that represent named declarations.
    pub named_type_ids: Box<[LocalTypeId]>,
    /// Resolved types indexed by [`LocalTypeId`].
    pub types: Box<[InferredTypeData<'db>]>,
    /// Expression types indexed by their source ranges.
    pub expressions: FxHashMap<TextRange, InferredTypeData<'db>>,
    /// Binding types indexed by declaration ranges.
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
    /// Resolves a chain of local type handles to its first non-local type.
    ///
    /// A cycle leaves the repeated local handle unresolved. A missing local
    /// type resolves to `Unknown`.
    pub fn resolve_type(
        &self,
        db: &'db dyn ModuleDb,
        ty: InferredTypeData<'db>,
    ) -> InferredTypeData<'db> {
        self.resolve_type_iterative(db, ty)
    }

    /// Finds a named member on a type or one of its inherited types.
    ///
    /// Type arguments from instances are substituted into the member type.
    /// Returns `None` when no reachable supported type defines `name`.
    pub fn find_member_type(
        &self,
        db: &'db dyn ModuleDb,
        ty: InferredTypeData<'db>,
        name: &str,
    ) -> Option<InferredTypeData<'db>> {
        self.find_member_type_iterative(db, ty, name)
    }

    /// Finds a named member available on a value of `ty`.
    ///
    /// Class values expose static members, while instances expose non-static
    /// members. Type arguments from instances are substituted into the member
    /// type. Returns `None` when no reachable supported type defines `name`.
    pub fn find_value_member_type(
        &self,
        db: &'db dyn ModuleDb,
        ty: InferredTypeData<'db>,
        name: &str,
    ) -> Option<InferredTypeData<'db>> {
        self.find_value_member_type_iterative(db, ty, name)
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

/// Expands a canonical global handle to the exact type stored in the database.
///
/// Use this for identity-aware operations such as direct member lookup.
fn expand_canonical_global<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
) -> InferredTypeData<'db> {
    if let InferredTypeData::GlobalType(id) = ty {
        global_type(db, id)
    } else {
        ty
    }
}

/// Expands only globals whose result is safe to use structurally.
///
/// Nominal classes and interfaces remain canonical handles so identity checks,
/// including Promise-class recognition, continue to observe the global ID.
fn expand_structural_global<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
) -> InferredTypeData<'db> {
    let InferredTypeData::GlobalType(id) = ty else {
        return ty;
    };
    match global_type(db, id) {
        InferredTypeData::Class(_)
        | InferredTypeData::Interface(_)
        | InferredTypeData::Module(_)
        | InferredTypeData::Namespace(_)
        | InferredTypeData::Object(_) => ty,
        expanded @ (InferredTypeData::Unknown
        | InferredTypeData::Divergent(_)
        | InferredTypeData::Global
        | InferredTypeData::GlobalType(_)
        | InferredTypeData::BigInt
        | InferredTypeData::Boolean
        | InferredTypeData::Null
        | InferredTypeData::Number
        | InferredTypeData::String
        | InferredTypeData::Symbol
        | InferredTypeData::Undefined
        | InferredTypeData::Conditional
        | InferredTypeData::Constructor(_)
        | InferredTypeData::Function(_)
        | InferredTypeData::Tuple(_)
        | InferredTypeData::Generic(_)
        | InferredTypeData::Local(_)
        | InferredTypeData::Intersection(_)
        | InferredTypeData::Union(_)
        | InferredTypeData::TypeOperator(_)
        | InferredTypeData::Literal(_)
        | InferredTypeData::InstanceOf(_)
        | InferredTypeData::MergedReference(_)
        | InferredTypeData::TypeofExpression(_)
        | InferredTypeData::TypeofType(_)
        | InferredTypeData::TypeofValue(_)
        | InferredTypeData::AnyKeyword
        | InferredTypeData::NeverKeyword
        | InferredTypeData::ObjectKeyword
        | InferredTypeData::ThisKeyword
        | InferredTypeData::UnknownKeyword
        | InferredTypeData::VoidKeyword) => expanded,
    }
}

pub(in crate::db) fn normalize_structural_type<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
    mut resolve_local: impl FnMut(InferredTypeData<'db>) -> InferredTypeData<'db>,
) -> Result<InferredTypeData<'db>, TypeTransformError> {
    ty.normalize_nested_types(db, |ty| {
        let ty = resolve_local(ty);
        expand_structural_global(db, ty)
    })
    .into_result()
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

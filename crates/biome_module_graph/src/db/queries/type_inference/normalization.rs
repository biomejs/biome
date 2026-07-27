//! Structural type normalization.
//!
//! The tracked normalization query resolves local handles and rebuilds the
//! structural wrappers that can contain them. Types without such wrappers are
//! returned unchanged.

use super::NormalizeTypeInput;
use crate::ModuleDb;
use crate::db::type_inference::{
    normalize_structural_type, normalize_type_cycle_result, resolve_local_type_on_demand,
};
use crate::type_inference::profiling::{
    TypeInferenceProfileOrigin, TypeInferenceQueryKind, execute_query,
};
use biome_js_type_info::interned_types::TypeData as InferredTypeData;

// #region NORMALIZATION QUERIES

/// Resolves local handles and simplifies structural wrappers in `input`.
///
/// A cycle, invalid structural rebuild, or exhausted normalization budget
/// returns [`InferredTypeData::Unknown`].
#[salsa::tracked(cycle_result=normalize_type_cycle_result)]
pub fn normalize_type<'db>(
    db: &'db dyn ModuleDb,
    input: NormalizeTypeInput<'db>,
) -> InferredTypeData<'db> {
    execute_query(
        TypeInferenceQueryKind::Normalization,
        TypeInferenceProfileOrigin::Inherited,
        "normalize_type",
        || {
            let ty = input.ty(db);
            if !needs_type_normalization(ty) {
                return ty;
            }
            normalize_structural_type(db, ty, |ty| resolve_local_type_on_demand(db, ty))
                .unwrap_or(InferredTypeData::Unknown)
        },
    )
}

// #endregion

// #region QUERY HELPER FUNCTIONS

fn needs_type_normalization(ty: InferredTypeData<'_>) -> bool {
    matches!(
        ty,
        InferredTypeData::InstanceOf(_)
            | InferredTypeData::Intersection(_)
            | InferredTypeData::Local(_)
            | InferredTypeData::MergedReference(_)
            | InferredTypeData::Tuple(_)
            | InferredTypeData::TypeofType(_)
            | InferredTypeData::TypeofValue(_)
            | InferredTypeData::Union(_)
    )
}

// #endregion

//! Converts global type IDs to the shared inferred definitions of built-in types.
//!
//! The definitions are stored by [`global_types()`]. This helper keeps that lookup
//! at the boundary between raw global IDs and inferred module types.

use crate::ModuleDb;
use biome_js_type_info::{
    global_types,
    resolved::{GlobalTypeId, InferredTypeData},
};

pub(in crate::db::type_inference) fn global_type<'db>(
    db: &'db dyn ModuleDb,
    type_id: GlobalTypeId,
) -> InferredTypeData<'db> {
    global_types(db).get(type_id)
}

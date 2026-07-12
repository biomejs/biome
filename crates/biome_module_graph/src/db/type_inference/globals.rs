//! Lookup of canonical database-native global type definitions.

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

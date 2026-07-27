//! Export traversal and exported-type queries.

use crate::db::type_inference::{
    ExportOriginResult, collect_namespace_export_names, find_export_origin,
    resolve_export_type_on_demand,
};
use crate::type_inference::profiling::{
    TypeInferenceProfileOrigin, TypeInferenceQueryKind, execute_query,
};
use crate::{ModuleDb, ModuleInfo, SymbolFromModuleInfo};
use biome_js_type_info::interned_types::TypeData as InferredTypeData;
use biome_rowan::Text;

// #region EXPORT QUERIES

/// Follows re-exports to the module and local name that define an export.
///
/// This query reads only module graph inputs and does not infer the export's
/// type. Separating these steps prevents export discovery from joining a type
/// inference cycle. Returns `Missing` when no reachable declaration owns the
/// name, `Ambiguous` when different `export *` paths provide it, and
/// `Indeterminate` when traversal reaches its work limit.
///
/// In the following example, resolving `result` from `facade.ts` returns the
/// `source.ts` module and the local name `value`.
///
/// ```ts
/// // source.ts
/// export const value = 1;
///
/// // facade.ts
/// export { value as result } from "./source";
/// ```
#[salsa::tracked(returns(ref))]
pub(crate) fn resolved_export_origin<'db>(
    db: &'db dyn ModuleDb,
    symbol: SymbolFromModuleInfo<'db>,
) -> ExportOriginResult {
    execute_query(
        TypeInferenceQueryKind::Exports,
        TypeInferenceProfileOrigin::document(*symbol.module(db)),
        "resolved_export_origin",
        || find_export_origin(db, *symbol.module(db), Text::from(symbol.name(db))),
    )
}

/// Infers the type exported under the requested symbol name.
///
/// Returns `None` when the module does not support inference. Missing,
/// ambiguous, indeterminate, or cyclic exports resolve to `Unknown`.
///
/// Requesting the export named `value` infers the numeric literal type `1`.
///
/// ```ts
/// export const value = 1;
/// ```
#[salsa::tracked(cycle_result=infer_export_type_cycle_result)]
pub fn infer_export_type<'db>(
    db: &'db dyn ModuleDb,
    symbol: SymbolFromModuleInfo<'db>,
) -> Option<InferredTypeData<'db>> {
    execute_query(
        TypeInferenceQueryKind::Exports,
        TypeInferenceProfileOrigin::document(*symbol.module(db)),
        "infer_export_type",
        || resolve_export_type_on_demand(db, *symbol.module(db), &symbol.name(db)),
    )
}

/// Lists the names available when a module is imported as a namespace.
///
/// The search follows `export *`, but those exports do not include `default`.
/// Returns `None` when an `export *` path cannot be resolved, a traversed module
/// is not JavaScript, inference is disabled for a traversed module, or the
/// search reaches its work limit.
///
/// In the following example, the namespace of `facade.ts` contains `local` and
/// `value`. A default export is not included through `export *`.
///
/// ```ts
/// // source.ts
/// export const value = 1;
/// export default 2;
///
/// // facade.ts
/// export const local = 3;
/// export * from "./source";
/// ```
#[salsa::tracked(returns(ref))]
pub(crate) fn namespace_export_names(db: &dyn ModuleDb, module: ModuleInfo) -> Option<Box<[Text]>> {
    execute_query(
        TypeInferenceQueryKind::Exports,
        TypeInferenceProfileOrigin::document(module),
        "namespace_export_names",
        || collect_namespace_export_names(db, module),
    )
}

// #endregion

// #region CYCLE RESULTS

fn infer_export_type_cycle_result<'db>(
    _db: &'db dyn ModuleDb,
    _id: salsa::Id,
    _symbol: SymbolFromModuleInfo<'db>,
) -> Option<InferredTypeData<'db>> {
    Some(InferredTypeData::Unknown)
}

// #endregion

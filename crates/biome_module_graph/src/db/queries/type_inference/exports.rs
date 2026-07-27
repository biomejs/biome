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

/// Finds the module and local name that own an exported symbol.
///
/// This query reads only module graph inputs and never resolves inferred types.
/// Keeping export discovery independent of inference prevents it from joining
/// an inference cycle. The caller resolves the type stored at the returned
/// origin.
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

/// Collects the export names visible through a module namespace.
///
/// This query reads only module graph inputs. An unresolved blanket re-export,
/// unsupported module, disabled inference, or exhausted traversal budget makes
/// the namespace indeterminate.
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

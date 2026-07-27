//! Complete module inference and dependency scheduling.
//!
//! [`infer_module_types`] resolves every raw type table for one module. The
//! untracked bottom-up entry point schedules imported modules explicitly before
//! invoking that query, which keeps long import chains off the Rust call stack.

use crate::db::type_inference::{
    ImportResolution, InferredModuleTypes, infer_module_types_cycle_result, resolve_raw_types,
};
use crate::module_graph::{ModuleInfo, ModuleInfoKind};
use crate::type_inference::profiling::{
    TypeInferenceProfileOrigin, TypeInferenceQueryKind, TypeInferenceWholeModuleReason,
    execute_query, is_recording as profiling_is_enabled, record_inferred_module,
    start_whole_module_inference, start_whole_module_inference_at,
};
use crate::{
    JsExport, JsOwnExport, ModuleDb, ResolvedPath, type_inference::TypeInferenceCodeReference,
};
use rustc_hash::FxHashSet;

// #region COMPLETE MODULE QUERY

/// Infers the complete type tables for a module.
///
/// The query returns `None` for non-JavaScript modules, modules whose type
/// inference is disabled, and dependency cycles that Salsa cannot recover.
/// Prefer the expression, binding, or local-type query when only one entry is
/// required.
///
/// Callers outside another database query should use
/// [`infer_module_types_bottom_up`] to prepare imports first.
#[salsa::tracked(returns(as_ref), cycle_result=infer_module_types_cycle_result)]
pub fn infer_module_types<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
) -> Option<InferredModuleTypes<'db>> {
    execute_query(
        TypeInferenceQueryKind::ModuleTypes,
        TypeInferenceProfileOrigin::document(module),
        "infer_module_types",
        || {
            let ModuleInfoKind::Js(js_info) = module.kind(db) else {
                return None;
            };
            if !js_info.infer_types {
                return None;
            }
            let whole_module = profiling_is_enabled().then(|| {
                let whole_module = start_whole_module_inference(
                    TypeInferenceWholeModuleReason::InternalDependency,
                    TypeInferenceProfileOrigin::document(module),
                    "infer_module_types",
                );
                record_inferred_module(
                    js_info.raw_types.len(),
                    js_info.raw_expressions.len(),
                    js_info.raw_binding_types.len(),
                );
                whole_module
            });
            let result = resolve_raw_types(db, module, &js_info, ImportResolution::OnDemand);
            if let Some(whole_module) = whole_module {
                whole_module.complete();
            }
            Some(result)
        },
    )
}

// #endregion

// #region EXTERNAL INFERENCE ENTRY POINTS

// The scheduler remains untracked because `infer_module_types` caches each
// module result while this explicit work list prevents recursive stack growth.
/// Infers a module after preparing every module it imports or re-exports.
///
/// This is the entry point for work initiated outside a database query, such
/// as a lint rule requesting type information after a file changes. It visits
/// dependencies before importers and uses an explicit work list, so long
/// import chains cannot overflow the Rust call stack.
///
/// For example, requesting types for `main.ts` prepares `values.ts` first:
///
/// ```ts
/// // values.ts
/// export const value = 1;
///
/// // main.ts
/// import { value } from "./values";
/// export const text = value.toString();
/// ```
///
/// Database queries should call [`infer_module_types`] directly because Salsa
/// already manages their dependency traversal.
pub fn infer_module_types_bottom_up<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
) -> Option<&'db InferredModuleTypes<'db>> {
    infer_module_types_bottom_up_impl(db, module)
}

pub(crate) fn infer_module_types_bottom_up_for_import_depth<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    implementation: TypeInferenceCodeReference,
) -> Option<&'db InferredModuleTypes<'db>> {
    let whole_module = start_whole_module_inference_at(
        TypeInferenceWholeModuleReason::ImportDepthLimit,
        TypeInferenceProfileOrigin::Inherited,
        implementation,
    );
    let result = infer_module_types_bottom_up_impl(db, module);
    whole_module.complete();
    result
}

fn infer_module_types_bottom_up_impl<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
) -> Option<&'db InferredModuleTypes<'db>> {
    let mut visited = FxHashSet::default();
    let mut stack = vec![(module, false)];

    while let Some((current, imports_visited)) = stack.pop() {
        if imports_visited {
            let inferred = infer_module_types(db, current);
            if current == module {
                return inferred;
            }
            continue;
        }
        if !visited.insert(current) {
            continue;
        }

        // Revisit this module to infer it once its imports below are done.
        stack.push((current, true));

        let ModuleInfoKind::Js(js_info) = current.kind(db) else {
            continue;
        };
        for import in js_info.static_imports.values() {
            push_inference_dependency(db, &visited, &mut stack, &import.resolved_path);
        }
        for reexport in js_info.blanket_reexports.iter() {
            push_inference_dependency(db, &visited, &mut stack, &reexport.import.resolved_path);
        }
        for export in js_info.exports.values() {
            match export {
                JsExport::Reexport(reexport) | JsExport::ReexportType(reexport) => {
                    push_inference_dependency(
                        db,
                        &visited,
                        &mut stack,
                        &reexport.import.resolved_path,
                    );
                }
                JsExport::Own(JsOwnExport::Namespace(reexport))
                | JsExport::OwnType(JsOwnExport::Namespace(reexport)) => {
                    push_inference_dependency(
                        db,
                        &visited,
                        &mut stack,
                        &reexport.import.resolved_path,
                    );
                }
                JsExport::Own(_) | JsExport::OwnType(_) => {}
            }
        }
    }

    None
}

// #endregion

// #region QUERY HELPER FUNCTIONS

fn push_inference_dependency(
    db: &dyn ModuleDb,
    visited: &FxHashSet<ModuleInfo>,
    stack: &mut Vec<(ModuleInfo, bool)>,
    resolved_path: &ResolvedPath,
) {
    if let Some(path) = resolved_path.as_path()
        && let Some(target) = db.module_for_path(path)
        && !visited.contains(&target)
    {
        stack.push((target, false));
    }
}

// #endregion

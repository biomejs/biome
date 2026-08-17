//! Complete module inference and dependency scheduling.
//!
//! [`infer_module_types`] resolves every raw type table for one module. The
//! untracked bottom-up entry point schedules resolved static imports and
//! re-exports explicitly before invoking that query. This keeps long acyclic
//! dependency chains off the Rust call stack.
//!
//! Complete module inference resolves every collected type, including types
//! imported from other modules. Prefer a lookup query when only one type is
//! needed.

use crate::db::queries::js_scc::compute_sccs;
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
    ModuleDb, ModuleGraphGeneration, ResolvedPath, type_inference::TypeInferenceCodeReference,
};
use rustc_hash::{FxHashMap, FxHashSet};

// #region COMPLETE MODULE QUERY

/// Infers the complete type tables for a module.
///
/// Returns `None` for non-JavaScript modules and modules whose type inference is
/// disabled. When imports form a cycle, inference blocks imports within that
/// cycle and continues with the information available from outside it. Types
/// that depend on a blocked import may be `Unknown`.
///
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
            let result = resolve_raw_types(db, module, &js_info, ImportResolution::on_demand());
            if let Some(whole_module) = whole_module {
                whole_module.complete();
            }
            Some(result)
        },
    )
}

#[salsa::tracked(returns(as_ref), cycle_result=infer_module_types_from_tables_cycle_result)]
pub(crate) fn infer_module_types_from_tables<'db>(
    db: &'db dyn ModuleDb,
    root: ModuleInfo,
    module: ModuleInfo,
) -> Option<InferredModuleTypes<'db>> {
    let ModuleInfoKind::Js(js_info) = module.kind(db) else {
        return None;
    };
    if !js_info.infer_types {
        return None;
    }

    Some(resolve_raw_types(
        db,
        module,
        &js_info,
        ImportResolution::FromTables { root },
    ))
}

fn infer_module_types_from_tables_cycle_result<'db>(
    db: &'db dyn ModuleDb,
    id: salsa::Id,
    _root: ModuleInfo,
    module: ModuleInfo,
) -> Option<InferredModuleTypes<'db>> {
    infer_module_types_cycle_result(db, id, module)
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct InferenceModuleSccs {
    component_by_module: FxHashMap<ModuleInfo, u32>,
    component_sizes: Box<[u32]>,
}

impl InferenceModuleSccs {
    pub(crate) fn contains_cycle_between(&self, from: ModuleInfo, to: ModuleInfo) -> bool {
        let (Some(&from_component), Some(&to_component)) = (
            self.component_by_module.get(&from),
            self.component_by_module.get(&to),
        ) else {
            return false;
        };

        from_component == to_component
            && self
                .component_sizes
                .get(from_component as usize)
                .is_some_and(|&size| size > 1)
    }
}

#[salsa::tracked(returns(ref))]
pub(crate) fn inference_module_sccs(
    db: &dyn ModuleDb,
    generation: ModuleGraphGeneration,
) -> InferenceModuleSccs {
    let _ = generation.value(db);
    let mut id_by_module = FxHashMap::default();

    db.for_each_module(&mut |module| {
        if matches!(module.kind(db), ModuleInfoKind::Js(_)) {
            id_by_module.insert(module, id_by_module.len() as u32);
        }
    });

    let mut edges = vec![Vec::new(); id_by_module.len()];
    db.for_each_module(&mut |module| {
        let ModuleInfoKind::Js(js_info) = module.kind(db) else {
            return;
        };
        let Some(&from_id) = id_by_module.get(&module) else {
            return;
        };
        let Some(edges) = edges.get_mut(from_id as usize) else {
            return;
        };
        push_inference_dependency_ids(db, &id_by_module, edges, &js_info);
    });

    let (component_by_id, component_sizes) = compute_sccs(&edges);
    let component_by_module = id_by_module
        .into_iter()
        .filter_map(|(module, id)| {
            component_by_id
                .get(id as usize)
                .copied()
                .map(|component| (module, component))
        })
        .collect();

    InferenceModuleSccs {
        component_by_module,
        component_sizes: component_sizes.into_boxed_slice(),
    }
}

// #endregion

// #region EXTERNAL INFERENCE ENTRY POINTS

// The scheduler is shared by tracked and untracked entry points. Its module
// queries cache inferred tables while this explicit work list prevents
// recursive stack growth.
/// Infers a module after preparing its resolved static import and re-export
/// dependencies.
///
/// This is the entry point for work initiated outside a database query, such
/// as a lint rule requesting type information after a file changes. It visits
/// acyclic dependencies before importers and uses an explicit work list, so
/// long import chains cannot overflow the Rust call stack. Cyclic dependencies
/// use the cycle recovery described by [`infer_module_types`].
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
    infer_module_types_bottom_up_impl(db, module, ModuleInferenceMode::OnDemand)
}

pub(crate) fn infer_module_types_bottom_up_for_import_depth<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
) -> Option<&'db InferredModuleTypes<'db>> {
    prepare_module_types_bottom_up_for_import_depth(db, module)
        .then(|| infer_module_types_from_tables(db, module, module))?
}

#[salsa::tracked]
fn prepare_module_types_bottom_up_for_import_depth(db: &dyn ModuleDb, module: ModuleInfo) -> bool {
    let whole_module = start_whole_module_inference_at(
        TypeInferenceWholeModuleReason::ImportDepthLimit,
        TypeInferenceProfileOrigin::Inherited,
        TypeInferenceCodeReference::new(
            file!(),
            line!(),
            "prepare_module_types_bottom_up_for_import_depth",
        ),
    );
    let prepared =
        infer_module_types_bottom_up_impl(db, module, ModuleInferenceMode::FromTables).is_some();
    whole_module.complete();
    prepared
}

fn infer_module_types_bottom_up_impl<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    mode: ModuleInferenceMode,
) -> Option<&'db InferredModuleTypes<'db>> {
    let mut visited = FxHashSet::default();
    let mut stack = vec![(module, false)];

    while let Some((current, imports_visited)) = stack.pop() {
        if imports_visited {
            let inferred = match mode {
                ModuleInferenceMode::OnDemand => infer_module_types(db, current),
                ModuleInferenceMode::FromTables => {
                    infer_module_types_from_tables(db, module, current)
                }
            };
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
        push_scheduled_inference_dependencies(db, &visited, &mut stack, &js_info);
    }

    None
}

#[derive(Clone, Copy)]
enum ModuleInferenceMode {
    OnDemand,
    FromTables,
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

fn push_scheduled_inference_dependencies(
    db: &dyn ModuleDb,
    visited: &FxHashSet<ModuleInfo>,
    stack: &mut Vec<(ModuleInfo, bool)>,
    js_info: &crate::JsModuleInfo,
) {
    for resolved_path in js_info.type_inference_dependency_paths() {
        push_inference_dependency(db, visited, stack, resolved_path);
    }
}

fn push_inference_dependency_ids(
    db: &dyn ModuleDb,
    id_by_module: &FxHashMap<ModuleInfo, u32>,
    edges: &mut Vec<u32>,
    js_info: &crate::JsModuleInfo,
) {
    let mut push = |resolved_path: &ResolvedPath| {
        if let Some(path) = resolved_path.as_path()
            && let Some(module) = db.module_for_path(path)
            && let Some(&id) = id_by_module.get(&module)
        {
            edges.push(id);
        }
    };

    for resolved_path in js_info.type_inference_dependency_paths() {
        push(resolved_path);
    }
}

// #endregion

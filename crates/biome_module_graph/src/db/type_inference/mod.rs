#![deny(clippy::wildcard_enum_match_arm)]

use crate::module_graph::{ModuleInfo, ModuleInfoKind};
use crate::{JsExport, JsOwnExport, ModuleDb, ResolvedPath};
use biome_css_syntax::TextRange;
use biome_js_type_info::interned_types::{
    LocalTypeId, ModuleKey, TypeData as InferredTypeData, TypeTransformError,
};
use rustc_hash::{FxHashMap, FxHashSet};

mod expressions;
mod globals;
mod imports;
mod lookup;
mod qualifiers;
mod resolver;

pub(in crate::db) use lookup::{apply_substitutions_to_root_body, substitutions_for_instance};
pub(in crate::db) use resolver::{ImportResolution, resolve_raw_types};

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

// SAFETY: None of the fields contains a Rust reference tied to `'db`.
// `InferredTypeData<'db>` uses the lifetime only to brand Salsa handles, whose
// `Update` implementations support comparison across revisions; all containers
// and map keys are owned. Each field is updated exactly once through its own
// `Update` implementation, and `maybe_update_range_map` either replaces an
// owned map or delegates updates to the values under an unchanged set of keys.
unsafe impl salsa::Update for InferredModuleTypes<'_> {
    unsafe fn maybe_update(old_pointer: *mut Self, new_value: Self) -> bool {
        let Self {
            module_key,
            named_type_ids,
            types,
            expressions,
            binding_type_data,
        } = new_value;
        let mut changed = false;
        changed |=
            unsafe { salsa::Update::maybe_update(&raw mut (*old_pointer).module_key, module_key) };
        changed |= unsafe {
            salsa::Update::maybe_update(&raw mut (*old_pointer).named_type_ids, named_type_ids)
        };
        changed |= unsafe { salsa::Update::maybe_update(&raw mut (*old_pointer).types, types) };
        changed |=
            unsafe { maybe_update_range_map(&raw mut (*old_pointer).expressions, expressions) };
        changed |= unsafe {
            maybe_update_range_map(&raw mut (*old_pointer).binding_type_data, binding_type_data)
        };
        changed
    }
}

unsafe fn maybe_update_range_map<V: salsa::Update>(
    old_pointer: *mut FxHashMap<TextRange, V>,
    new_map: FxHashMap<TextRange, V>,
) -> bool {
    let old_map = unsafe { &mut *old_pointer };
    if old_map.len() != new_map.len() || old_map.keys().any(|key| !new_map.contains_key(key)) {
        *old_map = new_map;
        return true;
    }

    let mut changed = false;
    for (key, new_value) in new_map {
        let old_value = old_map
            .get_mut(&key)
            .expect("range keys were checked above");
        changed |= unsafe { V::maybe_update(old_value, new_value) };
    }
    changed
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

pub(in crate::db) fn normalize_structural_type<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
    mut resolve_local: impl FnMut(InferredTypeData<'db>) -> InferredTypeData<'db>,
) -> Result<InferredTypeData<'db>, TypeTransformError> {
    ty.normalize_nested_types(db, |ty| {
        let ty = resolve_local(ty);
        ty.expand_structural_global(db)
    })
    .into_result()
}

pub(super) fn infer_module_types_cycle_result<'db>(
    db: &'db dyn ModuleDb,
    _id: salsa::Id,
    module: ModuleInfo,
) -> Option<InferredModuleTypes<'db>> {
    let ModuleInfoKind::Js(js_info) = module.kind(db) else {
        return None;
    };
    if !js_info.infer_types {
        return None;
    }

    let blocked = inference_scc(db, module);
    Some(resolve_raw_types(
        db,
        module,
        &js_info,
        ImportResolution::CycleFallback(&blocked),
    ))
}

fn inference_scc(db: &dyn ModuleDb, root: ModuleInfo) -> FxHashSet<ModuleInfo> {
    const MAX_DEPENDENCY_STEPS: usize = 1024;

    // Salsa invokes this from the cycle fallback. The first pass records every
    // dependency reachable from the root and builds reverse edges; the second
    // walks predecessors back to the root, retaining only its strongly
    // connected component for CycleFallback import suppression.
    let mut reachable = FxHashSet::default();
    let mut reverse = FxHashMap::<ModuleInfo, Vec<ModuleInfo>>::default();
    let mut pending = vec![root];
    let mut remaining_dependency_steps = MAX_DEPENDENCY_STEPS;
    reachable.insert(root);

    while let Some(source) = pending.pop() {
        db.unwind_if_revision_cancelled();
        for target in inferable_dependencies(db, source) {
            reverse.entry(target).or_default().push(source);
            if reachable.insert(target) {
                if remaining_dependency_steps == 0 {
                    // The search cannot determine the exact cycle within its
                    // step limit. Block every dependency found so far to avoid
                    // starting another long inference chain from the fallback.
                    return reachable;
                }
                remaining_dependency_steps -= 1;
                pending.push(target);
            }
        }
    }

    let mut scc = FxHashSet::default();
    let mut pending = vec![root];
    let mut remaining_dependency_steps = MAX_DEPENDENCY_STEPS;
    scc.insert(root);
    while let Some(target) = pending.pop() {
        db.unwind_if_revision_cancelled();
        if let Some(predecessors) = reverse.get(&target) {
            for predecessor in predecessors {
                if scc.insert(*predecessor) {
                    if remaining_dependency_steps == 0 {
                        return scc;
                    }
                    remaining_dependency_steps -= 1;
                    pending.push(*predecessor);
                }
            }
        }
    }
    scc
}

fn inferable_dependencies(db: &dyn ModuleDb, module: ModuleInfo) -> Vec<ModuleInfo> {
    let ModuleInfoKind::Js(js_info) = module.kind(db) else {
        return Vec::new();
    };
    if !js_info.infer_types {
        return Vec::new();
    }

    let mut dependencies = Vec::new();
    let mut push = |resolved_path: &ResolvedPath| {
        let Some(path) = resolved_path.as_path() else {
            return;
        };
        let Some(target) = db.module_for_path(path) else {
            return;
        };
        if matches!(target.kind(db), ModuleInfoKind::Js(info) if info.infer_types) {
            dependencies.push(target);
        }
    };

    for import in js_info.static_import_paths.values() {
        push(&import.resolved_path);
    }
    for reexport in &js_info.blanket_reexports {
        push(&reexport.import.resolved_path);
    }
    for export in js_info.exports.values() {
        match export {
            JsExport::Reexport(reexport) | JsExport::ReexportType(reexport) => {
                push(&reexport.import.resolved_path);
            }
            JsExport::Own(JsOwnExport::Namespace(reexport))
            | JsExport::OwnType(JsOwnExport::Namespace(reexport)) => {
                push(&reexport.import.resolved_path);
            }
            JsExport::Own(_) | JsExport::OwnType(_) => {}
        }
    }
    dependencies
}

pub(super) fn normalize_type_cycle_result<'db>(
    _db: &'db dyn ModuleDb,
    _id: salsa::Id,
    _input: crate::db::queries::NormalizeTypeInput<'db>,
) -> InferredTypeData<'db> {
    InferredTypeData::Unknown
}

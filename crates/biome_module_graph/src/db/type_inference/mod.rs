//! Database-backed JavaScript and TypeScript type inference.
//!
//! Every potentially cyclic or expanding walk has a deterministic query-local
//! budget. Exhaustion degrades to `Unknown` or `None`; deduplicated revisits do
//! not consume additional budget.

#![deny(clippy::wildcard_enum_match_arm)]

use crate::db::queries::NormalizeTypeInput;
use crate::db::type_inference::globals::global_type;
use crate::module_graph::{ModuleInfo, ModuleInfoKind};
use crate::{JsExport, JsOwnExport, ModuleDb, ResolvedPath};
use biome_css_syntax::TextRange;
use biome_js_type_info::resolved::{
    InferredLocalTypeId, InferredModuleKey, InferredTypeData, StructuralMapError,
};
use rustc_hash::{FxHashMap, FxHashSet};
use std::ops::ControlFlow;
use std::sync::Arc;

const MAX_TYPE_NORMALIZATION_STEPS: usize = 1024;

mod expressions;
mod globals;
mod imports;
mod lookup;
mod qualifiers;
mod resolver;

pub(in crate::db) use lookup::{apply_substitutions_to_root_body, substitutions_for_instance};
pub(in crate::db) use resolver::{ImportResolution, resolve_raw_types};

/// Type metadata for a binding.
///
/// This remains a wrapper because it is part of the public
/// [`InferredModuleTypes::binding_type_data`] value shape.
#[derive(Clone, Copy, Debug, Eq, PartialEq, salsa::Update)]
pub struct BindingTypeData<'db> {
    pub ty: InferredTypeData<'db>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InferredModuleTypes<'db> {
    pub module_key: InferredModuleKey,
    pub named_type_ids: Box<[InferredLocalTypeId]>,
    pub types: Box<[InferredTypeData<'db>]>,
    pub expressions: FxHashMap<TextRange, InferredTypeData<'db>>,
    pub binding_type_data: FxHashMap<TextRange, BindingTypeData<'db>>,
}

// SAFETY: `infer_module_types` returns this aggregate from a tracked query. The
// aggregate owns its containers and
// contains only Salsa handles, whose `Update` implementations perform the
// required cross-revision transition. Each field update delegates equality and
// replacement to those implementations, so old-revision handles are never
// compared directly with handles from the new revision.
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

pub(in crate::db) fn collected_type_result<'db>(
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
) -> Result<InferredTypeData<'db>, StructuralMapError> {
    ty.try_map_structural(
        db,
        MAX_TYPE_NORMALIZATION_STEPS,
        |ty| {
            let ty = resolve_local(ty);
            let ty = expand_structural_global(db, ty);
            if let InferredTypeData::TypeofValue(value) = ty
                && value.ty(db) == InferredTypeData::Unknown
            {
                ControlFlow::Break(ty)
            } else {
                ControlFlow::Continue(ty)
            }
        },
        |ty| match ty {
            InferredTypeData::InstanceOf(instance)
                if instance
                    .ty(db)
                    .should_flatten_instance(instance.type_parameters(db)) =>
            {
                instance.ty(db)
            }
            InferredTypeData::MergedReference(reference) => {
                let targets = reference.targets(db).collect::<Vec<_>>();
                match targets.first().copied() {
                    Some(first) if targets.iter().all(|target| *target == first) => first,
                    _ => InferredTypeData::MergedReference(reference),
                }
            }
            InferredTypeData::TypeofType(value) => value.ty(db),
            InferredTypeData::TypeofValue(value) => value.ty(db),
            ty @ (InferredTypeData::Unknown
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
            | InferredTypeData::Class(_)
            | InferredTypeData::Constructor(_)
            | InferredTypeData::Function(_)
            | InferredTypeData::Interface(_)
            | InferredTypeData::Module(_)
            | InferredTypeData::Namespace(_)
            | InferredTypeData::Object(_)
            | InferredTypeData::Tuple(_)
            | InferredTypeData::Generic(_)
            | InferredTypeData::Local(_)
            | InferredTypeData::Intersection(_)
            | InferredTypeData::Union(_)
            | InferredTypeData::TypeOperator(_)
            | InferredTypeData::Literal(_)
            | InferredTypeData::InstanceOf(_)
            | InferredTypeData::TypeofExpression(_)
            | InferredTypeData::AnyKeyword
            | InferredTypeData::NeverKeyword
            | InferredTypeData::ObjectKeyword
            | InferredTypeData::ThisKeyword
            | InferredTypeData::UnknownKeyword
            | InferredTypeData::VoidKeyword) => ty,
        },
    )
}

pub(in crate::db) fn infer_module_types_cycle_result<'db>(
    db: &'db dyn ModuleDb,
    _id: salsa::Id,
    module: ModuleInfo,
) -> Option<Arc<InferredModuleTypes<'db>>> {
    let ModuleInfoKind::Js(js_info) = module.kind(db) else {
        return None;
    };
    if !js_info.infer_types {
        return None;
    }

    let blocked = inference_scc(db, module);
    Some(Arc::new(resolve_raw_types(
        db,
        module,
        &js_info,
        ImportResolution::CycleFallback(&blocked),
    )))
}

fn inference_scc(db: &dyn ModuleDb, root: ModuleInfo) -> FxHashSet<ModuleInfo> {
    // Salsa invokes this from the cycle fallback. The first pass records every
    // dependency reachable from the root and builds reverse edges; the second
    // walks predecessors back to the root, retaining only its strongly
    // connected component for CycleFallback import suppression.
    let mut reachable = FxHashSet::default();
    let mut reverse = FxHashMap::<ModuleInfo, Vec<ModuleInfo>>::default();
    let mut pending = vec![root];
    reachable.insert(root);

    while let Some(source) = pending.pop() {
        for target in inferable_dependencies(db, source) {
            reverse.entry(target).or_default().push(source);
            if reachable.insert(target) {
                pending.push(target);
            }
        }
    }

    let mut scc = FxHashSet::default();
    let mut pending = vec![root];
    scc.insert(root);
    while let Some(target) = pending.pop() {
        if let Some(predecessors) = reverse.get(&target) {
            for predecessor in predecessors {
                if scc.insert(*predecessor) {
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

pub(in crate::db) fn normalize_type_cycle_result<'db>(
    _db: &'db dyn ModuleDb,
    _id: salsa::Id,
    _input: NormalizeTypeInput<'db>,
) -> InferredTypeData<'db> {
    InferredTypeData::Unknown
}

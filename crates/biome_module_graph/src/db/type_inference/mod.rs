#![deny(clippy::wildcard_enum_match_arm)]

use crate::db::queries::NormalizeTypeInput;
use crate::module_graph::{ModuleInfo, ModuleInfoKind};
use crate::{JsExport, JsOwnExport, ModuleDb, ResolvedPath};
use biome_css_syntax::TextRange;
use biome_js_type_info::interned_types::{
    LocalTypeId, ModuleKey, StructuralMapError, TypeData as InferredTypeData,
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
    pub module_key: ModuleKey,
    pub named_type_ids: Box<[LocalTypeId]>,
    pub types: Box<[InferredTypeData<'db>]>,
    pub expressions: FxHashMap<TextRange, InferredTypeData<'db>>,
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
    pub fn resolve_type(
        &self,
        db: &'db dyn ModuleDb,
        ty: InferredTypeData<'db>,
    ) -> InferredTypeData<'db> {
        self.resolve_type_iterative(db, ty)
    }

    pub fn find_member_type(
        &self,
        db: &'db dyn ModuleDb,
        ty: InferredTypeData<'db>,
        name: &str,
    ) -> Option<InferredTypeData<'db>> {
        self.find_member_type_iterative(db, ty, name)
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
) -> Result<InferredTypeData<'db>, StructuralMapError> {
    ty.try_map_structural(
        db,
        MAX_TYPE_NORMALIZATION_STEPS,
        |ty| {
            let ty = resolve_local(ty);
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

pub(super) fn infer_module_types_cycle_result<'db>(
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

pub(super) fn normalize_type_cycle_result<'db>(
    _db: &'db dyn ModuleDb,
    _id: salsa::Id,
    _input: NormalizeTypeInput<'db>,
) -> InferredTypeData<'db> {
    InferredTypeData::Unknown
}

#![deny(clippy::wildcard_enum_match_arm)]

use crate::module_graph::{ModuleInfo, ModuleInfoKind};
use crate::type_inference::profiling::record_cycle_recovery;
use crate::{JsModuleInfo, ModuleDb};
use biome_css_syntax::TextRange;
use biome_js_type_info::interned_types::{
    LocalTypeId, ModuleKey, TypeData as InferredTypeData, TypeTransformError,
};
use rustc_hash::{FxHashMap, FxHashSet};

mod expressions;
mod globals;
mod imports;
mod lookup;
mod promise_classification;
mod qualifiers;
mod resolver;

pub(in crate::db) use imports::{
    ExportOriginResult, collect_namespace_export_names, find_export_origin,
    resolve_export_type_on_demand,
};
pub(in crate::db) use lookup::{
    apply_substitutions_to_root_body, find_member_type_on_demand, find_value_member_type_on_demand,
    resolve_local_type_on_demand, substitutions_for_instance,
};
pub(in crate::db) use promise_classification::{
    PromiseClassification, classify_expression_array_promise, classify_expression_function_return,
    classify_expression_promise,
};
pub(in crate::db) use resolver::{ImportResolution, ResolutionCtx, resolve_raw_types};

/// Type information attached to one binding declaration.
#[derive(Clone, Copy, Debug, Eq, PartialEq, salsa::SalsaValue)]
pub struct BindingTypeData<'db> {
    /// Inferred type of the declared binding.
    pub ty: InferredTypeData<'db>,
}

/// Resolved type tables produced for one JavaScript or TypeScript module.
#[derive(Clone, Debug, Eq, PartialEq, salsa::SalsaValue)]
pub struct InferredModuleTypes<'db> {
    /// Stable key stored in local type handles owned by this module.
    pub module_key: ModuleKey,
    /// Local type IDs that represent named declarations.
    pub named_type_ids: Box<[LocalTypeId]>,
    /// Resolved types indexed by [`LocalTypeId`].
    pub types: Box<[InferredTypeData<'db>]>,
    /// Expression types indexed by their source ranges.
    // SAFETY: `TextRange` is two integers and doesn't implement `SalsaValue`
    // only because it's defined outside Salsa. The values are Salsa handles,
    // which use `'db` only as a brand.
    #[salsa_value(unsafe(prove_safe_to_retain_manually))]
    pub expressions: FxHashMap<TextRange, InferredTypeData<'db>>,
    /// Binding types indexed by declaration ranges.
    // SAFETY: See `expressions`.
    #[salsa_value(unsafe(prove_safe_to_retain_manually))]
    pub binding_type_data: FxHashMap<TextRange, BindingTypeData<'db>>,
}

impl<'db> InferredModuleTypes<'db> {
    /// Resolves a chain of local type handles to its first non-local type.
    ///
    /// A cycle leaves the repeated local handle unresolved. A missing local
    /// type resolves to `Unknown`. At most 1024 handle resolutions are
    /// performed; if the chain is still local, the next handle is returned
    /// unresolved.
    pub fn resolve_type(
        &self,
        db: &'db dyn ModuleDb,
        ty: InferredTypeData<'db>,
    ) -> InferredTypeData<'db> {
        self.resolve_type_iterative(db, ty)
    }

    /// Finds a named member on a type or one of its inherited types.
    ///
    /// This lookup does not distinguish between the static and instance sides
    /// of a class. Type arguments from instances are substituted into the
    /// member type. Compound types may produce a union from the branches that
    /// define `name`; branches without that member do not make the lookup fail.
    /// Work is bounded, and exhausting the work limit returns `Unknown`.
    ///
    /// For example, looking up `value` on `Text | Count | Empty` returns
    /// `string | number`; the `Empty` branch contributes no member:
    ///
    /// ```ts
    /// type Text = { value: string };
    /// type Count = { value: number };
    /// type Empty = { other: boolean };
    /// ```
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
    /// Unlike [`Self::find_member_type`], this lookup respects which side of a
    /// class the value represents. A class value exposes static members; an
    /// instance exposes non-static members. Other object-like values expose
    /// their ordinary members. Type arguments from instances are substituted
    /// into the member type. Compound types and the work limit follow the same
    /// rules as [`Self::find_member_type`].
    ///
    /// In this example, value lookup on `Box` can find `kind` but not `value`;
    /// lookup on a `Box` instance can find `value` but not `kind`:
    ///
    /// ```ts
    /// class Box {
    ///     static kind: string;
    ///     value: number;
    /// }
    /// ```
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

    record_cycle_recovery();
    let blocked = inference_scc(db, module, js_info);
    Some(resolve_raw_types(
        db,
        module,
        js_info,
        ImportResolution::CycleFallback(&blocked),
    ))
}

/// Returns the modules to block during cycle fallback for `root`.
///
/// The cycle fallback blocks imports within this set to stop recursive Salsa
/// queries while continuing to infer dependencies outside the cycle. When both
/// graph walks complete, the set is the strongly connected component containing
/// `root`: every member is reachable from `root` and can also reach `root`.
///
fn inference_scc(
    db: &dyn ModuleDb,
    root: ModuleInfo,
    root_info: &JsModuleInfo,
) -> FxHashSet<ModuleInfo> {
    // Record every dependency reachable from the root while building the
    // reverse graph needed to determine which modules can reach it again.
    let mut reachable = FxHashSet::default();
    let mut reverse = FxHashMap::<ModuleInfo, Vec<ModuleInfo>>::default();
    let mut pending = vec![(root, root_info)];
    reachable.insert(root);

    while let Some((source, source_info)) = pending.pop() {
        db.unwind_if_revision_cancelled();

        for import in source_info.type_inference_dependencies() {
            let resolved = import.resolve_js(db, source);
            let Some(path) = resolved.path().as_path() else {
                continue;
            };
            let Some(target) = db.module_for_path(path) else {
                continue;
            };
            let ModuleInfoKind::Js(target_info) = target.kind(db) else {
                continue;
            };
            if !target_info.infer_types {
                continue;
            }

            reverse.entry(target).or_default().push(source);
            if reachable.insert(target) {
                pending.push((target, target_info));
            }
        }
    }

    // Walking predecessors from the root intersects the reachable set with
    // modules that can reach the root, yielding its strongly connected
    // component. Acyclic dependencies have no reverse path and remain usable.
    let mut scc = FxHashSet::default();
    let mut pending = vec![root];
    scc.insert(root);
    while let Some(target) = pending.pop() {
        db.unwind_if_revision_cancelled();
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

pub(super) fn normalize_type_cycle_result<'db>(
    _db: &'db dyn ModuleDb,
    _id: salsa::Id,
    _input: crate::db::queries::NormalizeTypeInput<'db>,
) -> InferredTypeData<'db> {
    InferredTypeData::Unknown
}

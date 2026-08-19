use super::{
    InferredModuleTypes,
    globals::global_type,
    resolver::{MAX_RAW_TYPE_RESOLUTION_DEPTH, ResolutionCtx},
};
use crate::db::queries::{
    BindingTypeInput, BindingTypeWithImportBudgetInput, LocalTypeInput,
    LocalTypeWithImportBudgetInput, SymbolFromModuleInfo, infer_binding_type,
    infer_binding_type_with_import_budget, infer_local_type, infer_local_type_with_import_budget,
    infer_module_types_bottom_up_for_import_depth, inference_module_sccs, namespace_export_names,
    resolved_export_origin,
};
use crate::module_graph::{ModuleInfo, ModuleInfoKind};
use crate::{JsExport, JsImport, JsOwnExport, ModuleDb, ModuleGraphGeneration, ResolvedPath};
use biome_js_type_info::{
    GlobalTypeId, ImportSymbol, Path, ResolvedTypeId, TypeImportQualifier, TypeReference,
    TypeResolverLevel,
    interned_types::{
        InternedNamespace as InferredNamespace, LocalTypeHandle, LocalTypeId, ModuleKey,
        TypeData as InferredTypeData, TypeMember as InferredTypeMember,
        TypeMemberKind as InferredTypeMemberKind,
    },
};
use biome_rowan::{Text, TextRange};
use rustc_hash::FxHashSet;
use salsa::plumbing::AsId;

const MAX_EXPORT_RESOLUTION_STEPS: usize = 1024;

/// Result of searching for the declaration behind a named export.
#[derive(Clone, Debug, Eq, PartialEq, salsa::Update)]
pub(crate) enum ExportOriginResult {
    /// The completed search found no declaration.
    ///
    /// Unresolved paths, unavailable modules, non-JavaScript modules, and
    /// modules with type inference disabled are skipped, so `Missing` does not
    /// prove that the export is absent from source code.
    Missing,
    /// The module and local export name of the declaration.
    ///
    /// The local name may differ from the requested name after a named
    /// re-export.
    Found { module: ModuleInfo, name: Text },
    /// Blanket re-exports lead to more than one distinct declaration.
    ///
    /// Reaching the same declaration through multiple paths is not ambiguous.
    Ambiguous,
    /// The work limit was reached before the search completed.
    Indeterminate,
}

struct NamespaceExportCollection {
    names: Vec<Text>,
    seen_names: FxHashSet<Text>,
    seen_modules: FxHashSet<ModuleKey>,
    stack: Vec<(ModuleInfo, bool)>,
    remaining_steps: usize,
}

impl NamespaceExportCollection {
    fn new() -> Self {
        Self {
            names: Vec::new(),
            seen_names: FxHashSet::default(),
            seen_modules: FxHashSet::default(),
            stack: Vec::new(),
            remaining_steps: MAX_EXPORT_RESOLUTION_STEPS,
        }
    }
}

#[derive(Clone, PartialEq)]
struct ExportIdentity {
    module: ModuleKey,
    own_export: JsOwnExport,
}

struct ResolvedExport<'db> {
    identity: ExportIdentity,
    ty: InferredTypeData<'db>,
}

struct ExportOrigin {
    identity: ExportIdentity,
    module: ModuleInfo,
    name: Text,
}

enum ExportResolution<'db> {
    Missing,
    Resolved(ResolvedExport<'db>),
    Ambiguous,
    Indeterminate,
}

enum ExportResolutionStep<'db> {
    Continue,
    Resolved(ResolvedExport<'db>),
}

enum ExportOriginStep {
    Continue,
    Found(ExportOrigin),
}

/// Finds the declaration behind `root_name` without resolving its type.
///
/// Explicit re-exports follow the selected source name. Blanket re-exports are
/// searched only when a module has no explicit export with the requested name.
/// Distinct declarations reached through blanket re-exports are ambiguous;
/// repeated paths to the same declaration are accepted. Cycles are skipped.
/// The search returns [`ExportOriginResult::Indeterminate`] after 1024 distinct
/// module-and-name steps beyond the root.
///
/// For example, searching for `Public` below returns the declaration named
/// `Internal` from `source.ts`:
///
/// ```ts
/// // source.ts
/// export type Internal = string;
///
/// // index.ts
/// export { Internal as Public } from "./source";
/// ```
pub(in crate::db) fn find_export_origin(
    db: &dyn ModuleDb,
    root_module: ModuleInfo,
    root_name: Text,
) -> ExportOriginResult {
    let mut stack = Vec::new();
    let mut seen = FxHashSet::default();
    let mut resolved = None;
    let mut remaining_steps = MAX_EXPORT_RESOLUTION_STEPS;

    seen.insert((ModuleKey::new(root_module.as_id()), root_name.clone()));
    match find_export_origin_in_module(db, root_module, &root_name, &mut stack) {
        ExportOriginStep::Continue => {}
        ExportOriginStep::Found(candidate) => resolved = Some(candidate),
    }

    while let Some((module, name)) = stack.pop() {
        if !seen.insert((ModuleKey::new(module.as_id()), name.clone())) {
            continue;
        }
        if remaining_steps == 0 {
            return ExportOriginResult::Indeterminate;
        }
        remaining_steps -= 1;

        match find_export_origin_in_module(db, module, &name, &mut stack) {
            ExportOriginStep::Continue => {}
            ExportOriginStep::Found(candidate) => {
                if let Some(previous) = &resolved {
                    if previous.identity != candidate.identity {
                        return ExportOriginResult::Ambiguous;
                    }
                } else {
                    resolved = Some(candidate);
                }
            }
        }
    }

    resolved.map_or(ExportOriginResult::Missing, |origin| {
        ExportOriginResult::Found {
            module: origin.module,
            name: origin.name,
        }
    })
}

/// Collects the names visible on a module namespace without resolving types.
///
/// Explicit exports from the root include `default`. Names reached through
/// `export *` exclude `default`, and duplicate names are returned once. The
/// function returns `None` if any traversed module is unsupported, has type
/// inference disabled, has an unavailable blanket re-export, or if traversal
/// needs more than 1024 blanket-reexported modules beyond the root. It does not
/// return a partial list in those cases.
///
/// Thus the namespace for `index.ts` contains `default` and `named`, but not the
/// default export from `source.ts`:
///
/// ```ts
/// // source.ts
/// export default 1;
/// export const named = 2;
///
/// // index.ts
/// export default 3;
/// export * from "./source";
/// ```
pub(in crate::db) fn collect_namespace_export_names(
    db: &dyn ModuleDb,
    module: ModuleInfo,
) -> Option<Box<[Text]>> {
    let mut collection = NamespaceExportCollection::new();

    collection
        .seen_modules
        .insert(ModuleKey::new(module.as_id()));
    if !collect_namespace_names_in_module(db, module, true, &mut collection) {
        return None;
    }

    while let Some((module, include_default)) = collection.stack.pop() {
        let module_key = ModuleKey::new(module.as_id());
        if collection.seen_modules.contains(&module_key) {
            continue;
        }
        if collection.remaining_steps == 0 {
            return None;
        }
        collection.remaining_steps -= 1;
        collection.seen_modules.insert(module_key);

        if !collect_namespace_names_in_module(db, module, include_default, &mut collection) {
            return None;
        }
    }

    Some(collection.names.into_boxed_slice())
}

fn collect_namespace_names_in_module(
    db: &dyn ModuleDb,
    module: ModuleInfo,
    include_default: bool,
    collection: &mut NamespaceExportCollection,
) -> bool {
    let ModuleInfoKind::Js(js_info) = module.kind(db) else {
        return false;
    };
    if !js_info.infer_types {
        return false;
    }

    for (name, _) in js_info.exports.iter() {
        if !include_default && name.text() == "default" {
            continue;
        }
        if collection.seen_names.insert(name.clone()) {
            collection.names.push(name.clone());
        }
    }

    for reexport in js_info.blanket_reexports.iter().rev() {
        let Some(path) = reexport.import.resolved_path.as_path() else {
            return false;
        };
        let Some(module) = db.module_for_path(path) else {
            return false;
        };
        collection.stack.push((module, false));
    }

    true
}

fn find_export_origin_in_module(
    db: &dyn ModuleDb,
    module: ModuleInfo,
    name: &Text,
    stack: &mut Vec<(ModuleInfo, Text)>,
) -> ExportOriginStep {
    let module_key = ModuleKey::new(module.as_id());
    let ModuleInfoKind::Js(js_info) = module.kind(db) else {
        return ExportOriginStep::Continue;
    };
    if !js_info.infer_types {
        return ExportOriginStep::Continue;
    }

    match js_info.exports.get(name.text()) {
        Some(JsExport::Own(own_export) | JsExport::OwnType(own_export)) => {
            ExportOriginStep::Found(ExportOrigin {
                identity: ExportIdentity {
                    module: module_key,
                    own_export: own_export.clone(),
                },
                module,
                name: name.clone(),
            })
        }
        Some(JsExport::Reexport(reexport) | JsExport::ReexportType(reexport)) => {
            if let Some(path) = reexport.import.resolved_path.as_path()
                && let Some(module) = db.module_for_path(path)
            {
                let name = match &reexport.import.symbol {
                    ImportSymbol::All => name.clone(),
                    ImportSymbol::Default => Text::from("default"),
                    ImportSymbol::Named(name) => name.clone(),
                };
                stack.push((module, name));
            }
            ExportOriginStep::Continue
        }
        None => {
            for reexport in js_info.blanket_reexports.iter().rev() {
                let Some(path) = reexport.import.resolved_path.as_path() else {
                    continue;
                };
                if let Some(module) = db.module_for_path(path) {
                    stack.push((module, name.clone()));
                }
            }
            ExportOriginStep::Continue
        }
    }
}

/// Resolves one exported type without inferring the module's complete tables.
///
/// Returns `None` when `module` is not a JavaScript module or has type inference
/// disabled. Once inference is supported, the function returns `Some`; a
/// missing, ambiguous, or indeterminate origin is represented by
/// `Some(Unknown)`. A declaration whose binding or local type cannot be inferred
/// also produces `Some(Unknown)`.
pub(in crate::db) fn resolve_export_type_on_demand<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    name: &str,
) -> Option<InferredTypeData<'db>> {
    let ModuleInfoKind::Js(js_info) = module.kind(db) else {
        return None;
    };
    if !js_info.infer_types {
        return None;
    }

    let ctx = ResolutionCtx::new(db, module, &js_info, super::ImportResolution::on_demand());
    Some(ctx.resolve_export_name_on_demand(module, name))
}

impl<'db> ResolutionCtx<'db, '_> {
    pub(in crate::db::type_inference) fn resolve_import(
        &mut self,
        qualifier: &TypeImportQualifier,
    ) -> InferredTypeData<'db> {
        let Some(module) = self.module_for_resolved_path(&qualifier.resolved_path) else {
            return InferredTypeData::Unknown;
        };

        self.resolve_import_symbol(module, &qualifier.symbol)
    }

    fn module_for_resolved_path(&self, resolved_path: &ResolvedPath) -> Option<ModuleInfo> {
        let path = resolved_path.as_path()?;
        self.db.module_for_path(path)
    }

    /// Resolves an import from tables produced by whole-module inference.
    ///
    /// Cycle fallback uses this path because its blocked strongly connected
    /// component is local to the active module query and cannot be cached by
    /// the on-demand lookup queries.
    fn resolve_import_symbol_from_tables(
        &self,
        module: ModuleInfo,
        inferred_types: &InferredModuleTypes<'db>,
        symbol: &ImportSymbol,
    ) -> InferredTypeData<'db> {
        match symbol {
            ImportSymbol::All => self.namespace_for_module_from_tables(module, inferred_types),
            ImportSymbol::Default => {
                self.resolve_export_name_from_tables(module, inferred_types, "default")
            }
            ImportSymbol::Named(name) => {
                self.resolve_export_name_from_tables(module, inferred_types, name.text())
            }
        }
    }

    /// Resolves only the requested import through export and lookup queries.
    ///
    /// Unlike [`Self::resolve_import_symbol_from_tables`], this path does not
    /// infer the imported module's complete type tables. Dependency graphs that
    /// exceed the on-demand traversal budget use bottom-up whole-module
    /// inference instead. The fallback returns `Unknown` if those module tables
    /// cannot be inferred.
    fn resolve_import_symbol_on_demand(
        &self,
        module: ModuleInfo,
        symbol: &ImportSymbol,
    ) -> InferredTypeData<'db> {
        match symbol {
            ImportSymbol::All => self.namespace_for_module_on_demand(module),
            ImportSymbol::Default => self.resolve_export_name_on_demand(module, "default"),
            ImportSymbol::Named(name) => self.resolve_export_name_on_demand(module, name.text()),
        }
    }

    fn resolve_import_symbol(
        &self,
        module: ModuleInfo,
        symbol: &ImportSymbol,
    ) -> InferredTypeData<'db> {
        match self.import_resolution {
            super::ImportResolution::FromTables { .. }
            | super::ImportResolution::CycleFallback(_) => self
                .infer_imported_module(module)
                .map_or(InferredTypeData::Unknown, |types| {
                    self.resolve_import_symbol_from_tables(module, types, symbol)
                }),
            super::ImportResolution::OnDemand { remaining } => {
                let sccs = inference_module_sccs(self.db, ModuleGraphGeneration::get(self.db));
                if module == self.module || sccs.contains_cycle_between(self.module, module) {
                    return InferredTypeData::Unknown;
                }
                if remaining == 0 {
                    return infer_module_types_bottom_up_for_import_depth(self.db, module)
                        .map_or(InferredTypeData::Unknown, |types| {
                            self.resolve_import_symbol_from_tables(module, types, symbol)
                        });
                }

                let ModuleInfoKind::Js(js_info) = module.kind(self.db) else {
                    return InferredTypeData::Unknown;
                };
                if !js_info.infer_types {
                    return InferredTypeData::Unknown;
                }
                let ctx = ResolutionCtx::new(
                    self.db,
                    module,
                    &js_info,
                    super::ImportResolution::OnDemand {
                        remaining: remaining - 1,
                    },
                );
                ctx.resolve_import_symbol_on_demand(module, symbol)
            }
        }
    }

    fn resolve_js_import(&self, import: &JsImport) -> InferredTypeData<'db> {
        let resolution_depth = self.resolution_depth.get();
        if resolution_depth >= MAX_RAW_TYPE_RESOLUTION_DEPTH {
            return InferredTypeData::Unknown;
        }
        self.resolution_depth.set(resolution_depth + 1);
        let result = self
            .module_for_resolved_path(&import.resolved_path)
            .map_or(InferredTypeData::Unknown, |module| {
                self.resolve_import_symbol(module, &import.symbol)
            });
        self.resolution_depth.set(resolution_depth);
        result
    }

    /// Builds a namespace by resolving each discovered export through lookup queries.
    ///
    /// Missing and ambiguous exports are omitted, so the namespace may be
    /// partial. An indeterminate export makes the entire namespace `Unknown`.
    /// Export-name discovery can also make the entire namespace `Unknown`; see
    /// [`collect_namespace_export_names`].
    ///
    /// For example, if two blanket re-exports provide different declarations
    /// named `shared`, the namespace keeps `local` and omits `shared`:
    ///
    /// ```ts
    /// // left.ts
    /// export const shared = 1;
    ///
    /// // right.ts
    /// export const shared = "right";
    ///
    /// // index.ts
    /// export * from "./left";
    /// export * from "./right";
    /// export const local = 1;
    /// ```
    fn namespace_for_module_on_demand(&self, module: ModuleInfo) -> InferredTypeData<'db> {
        let Some(names) = namespace_export_names(self.db, module) else {
            return InferredTypeData::Unknown;
        };
        let mut members = Vec::with_capacity(names.len());
        for name in names {
            match self.resolve_export_name_result_on_demand(module, name.text()) {
                ExportResolution::Resolved(resolved) => members.push(InferredTypeMember {
                    kind: InferredTypeMemberKind::Named(name.clone()),
                    ty: resolved.ty,
                }),
                ExportResolution::Missing | ExportResolution::Ambiguous => {}
                ExportResolution::Indeterminate => return InferredTypeData::Unknown,
            };
        }

        InferredTypeData::Namespace(InferredNamespace::new(
            self.db,
            members.into_boxed_slice(),
            Path::from(Text::from(module.path(self.db).to_string())),
        ))
    }

    /// Resolves one exported name without inferring complete module tables.
    ///
    /// Missing, ambiguous, indeterminate, and unresolved exports all return
    /// `Unknown`.
    pub(in crate::db) fn resolve_export_name_on_demand(
        &self,
        module: ModuleInfo,
        name: &str,
    ) -> InferredTypeData<'db> {
        match self.resolve_export_name_result_on_demand(module, name) {
            ExportResolution::Resolved(resolved) => resolved.ty,
            ExportResolution::Missing
            | ExportResolution::Ambiguous
            | ExportResolution::Indeterminate => InferredTypeData::Unknown,
        }
    }

    /// Resolves an export origin, then requests only its binding or local type.
    fn resolve_export_name_result_on_demand(
        &self,
        module: ModuleInfo,
        name: &str,
    ) -> ExportResolution<'db> {
        let symbol = SymbolFromModuleInfo::new(self.db, name.to_string(), module);
        let (module, name) = match resolved_export_origin(self.db, symbol) {
            ExportOriginResult::Found { module, name } => (module, name),
            ExportOriginResult::Missing => return ExportResolution::Missing,
            ExportOriginResult::Ambiguous => return ExportResolution::Ambiguous,
            ExportOriginResult::Indeterminate => return ExportResolution::Indeterminate,
        };
        let ModuleInfoKind::Js(js_info) = module.kind(self.db) else {
            return ExportResolution::Missing;
        };
        let Some(JsExport::Own(own_export) | JsExport::OwnType(own_export)) =
            js_info.exports.get(name.text())
        else {
            return ExportResolution::Missing;
        };

        let ty = match own_export {
            JsOwnExport::Binding(range) => inferred_type_from_binding_on_demand(
                self.db,
                *module,
                &js_info,
                *range,
                self.import_resolution,
            ),
            JsOwnExport::Type(resolved_id) => inferred_type_from_resolved_id_on_demand(
                self.db,
                *module,
                &js_info,
                ResolvedTypeId::Local(*resolved_id),
                self.import_resolution,
            ),
            JsOwnExport::Namespace(reexport) => self.resolve_js_import(&reexport.import),
        };
        ExportResolution::Resolved(ResolvedExport {
            identity: ExportIdentity {
                module: ModuleKey::new(module.as_id()),
                own_export: own_export.clone(),
            },
            ty,
        })
    }

    /// Builds a namespace from whole-module tables during cycle fallback.
    ///
    /// Missing and ambiguous exports are omitted. An unavailable dependency,
    /// failed name collection, exhausted work limit, or indeterminate export
    /// makes the entire namespace `Unknown`.
    fn namespace_for_module_from_tables(
        &self,
        module: ModuleInfo,
        inferred_types: &InferredModuleTypes<'db>,
    ) -> InferredTypeData<'db> {
        let mut collection = NamespaceExportCollection::new();

        collection
            .seen_modules
            .insert(ModuleKey::new(module.as_id()));
        if !self.collect_namespace_members(module, true, &mut collection) {
            return InferredTypeData::Unknown;
        }

        while let Some((module, include_default)) = collection.stack.pop() {
            let module_key = ModuleKey::new(module.as_id());
            if collection.seen_modules.contains(&module_key) {
                continue;
            }
            if collection.remaining_steps == 0 {
                return InferredTypeData::Unknown;
            }
            collection.remaining_steps -= 1;
            collection.seen_modules.insert(module_key);

            if self.infer_imported_module(module).is_none() {
                return InferredTypeData::Unknown;
            }

            if !self.collect_namespace_members(module, include_default, &mut collection) {
                return InferredTypeData::Unknown;
            }
        }

        self.namespace_from_table_names(module, inferred_types, collection.names)
    }

    fn namespace_from_table_names(
        &self,
        module: ModuleInfo,
        inferred_types: &InferredModuleTypes<'db>,
        names: impl IntoIterator<Item = Text>,
    ) -> InferredTypeData<'db> {
        let names = names.into_iter();
        let (min_size, _) = names.size_hint();
        let mut members = Vec::with_capacity(min_size);
        for name in names {
            match self.resolve_export_name_result_from_tables(module, inferred_types, name.text()) {
                ExportResolution::Resolved(resolved) => members.push(InferredTypeMember {
                    kind: InferredTypeMemberKind::Named(name),
                    ty: resolved.ty,
                }),
                ExportResolution::Missing | ExportResolution::Ambiguous => {}
                ExportResolution::Indeterminate => return InferredTypeData::Unknown,
            };
        }

        InferredTypeData::Namespace(InferredNamespace::new(
            self.db,
            members.into_boxed_slice(),
            Path::from(Text::from(module.path(self.db).to_string())),
        ))
    }

    fn collect_namespace_members(
        &self,
        module: ModuleInfo,
        include_default: bool,
        collection: &mut NamespaceExportCollection,
    ) -> bool {
        let ModuleInfoKind::Js(js_info) = module.kind(self.db) else {
            return false;
        };

        for (name, export) in js_info.exports.iter() {
            if !is_namespace_export_collectible(js_info.infer_types, export) {
                continue;
            }
            if !include_default && name.text() == "default" {
                continue;
            }

            if !collection.seen_names.insert(name.clone()) {
                continue;
            }

            collection.names.push(name.clone());
        }

        for reexport in js_info.blanket_reexports.iter().rev() {
            let Some(module) = self.module_for_resolved_path(&reexport.import.resolved_path) else {
                return false;
            };
            collection.stack.push((module, false));
        }

        true
    }

    fn resolve_export_name_from_tables(
        &self,
        module: ModuleInfo,
        inferred_types: &InferredModuleTypes<'db>,
        name: &str,
    ) -> InferredTypeData<'db> {
        match self.resolve_export_name_result_from_tables(module, inferred_types, name) {
            ExportResolution::Resolved(resolved) => resolved.ty,
            ExportResolution::Missing
            | ExportResolution::Ambiguous
            | ExportResolution::Indeterminate => InferredTypeData::Unknown,
        }
    }

    /// Follows exports while reading types from tables prepared for cycle fallback.
    fn resolve_export_name_result_from_tables(
        &self,
        module: ModuleInfo,
        inferred_types: &InferredModuleTypes<'db>,
        name: &str,
    ) -> ExportResolution<'db> {
        let mut stack = Vec::new();
        let mut seen = FxHashSet::default();
        let mut resolved: Option<ResolvedExport<'db>> = None;
        let mut remaining_steps = MAX_EXPORT_RESOLUTION_STEPS;

        seen.insert((ModuleKey::new(module.as_id()), name.to_string()));
        match self.resolve_export_name_in_module_from_tables(
            module,
            inferred_types,
            name,
            &mut stack,
        ) {
            ExportResolutionStep::Continue => {}
            ExportResolutionStep::Resolved(candidate) => resolved = Some(candidate),
        }

        while let Some((module, name)) = stack.pop() {
            if !seen.insert((ModuleKey::new(module.as_id()), name.clone())) {
                continue;
            }
            if remaining_steps == 0 {
                return ExportResolution::Indeterminate;
            }
            remaining_steps -= 1;

            let Some(inferred_types) = self.infer_imported_module(module) else {
                continue;
            };

            match self.resolve_export_name_in_module_from_tables(
                module,
                inferred_types,
                &name,
                &mut stack,
            ) {
                ExportResolutionStep::Continue => {}
                ExportResolutionStep::Resolved(candidate) => {
                    if let Some(previous) = &resolved {
                        if previous.identity != candidate.identity {
                            return ExportResolution::Ambiguous;
                        }
                    } else {
                        resolved = Some(candidate);
                    }
                }
            }
        }

        resolved.map_or(ExportResolution::Missing, ExportResolution::Resolved)
    }

    fn resolve_export_name_in_module_from_tables(
        &self,
        module: ModuleInfo,
        inferred_types: &InferredModuleTypes<'db>,
        name: &str,
        stack: &mut Vec<(ModuleInfo, String)>,
    ) -> ExportResolutionStep<'db> {
        let module_key = ModuleKey::new(module.as_id());
        let ModuleInfoKind::Js(js_info) = module.kind(self.db) else {
            return ExportResolutionStep::Continue;
        };

        match js_info.exports.get(name) {
            Some(JsExport::Own(own_export) | JsExport::OwnType(own_export)) => {
                ExportResolutionStep::Resolved(ResolvedExport {
                    identity: ExportIdentity {
                        module: module_key,
                        own_export: own_export.clone(),
                    },
                    ty: self.resolve_own_export_from_tables(inferred_types, own_export),
                })
            }
            Some(JsExport::Reexport(reexport) | JsExport::ReexportType(reexport)) => {
                self.push_reexport_target(reexport.import.clone(), name, stack);
                ExportResolutionStep::Continue
            }
            None => {
                for reexport in js_info.blanket_reexports.iter().rev() {
                    if let Some(module) =
                        self.module_for_resolved_path(&reexport.import.resolved_path)
                    {
                        stack.push((module, name.to_string()));
                    }
                }
                ExportResolutionStep::Continue
            }
        }
    }

    fn push_reexport_target(
        &self,
        import: JsImport,
        fallback_name: &str,
        stack: &mut Vec<(ModuleInfo, String)>,
    ) {
        let Some(module) = self.module_for_resolved_path(&import.resolved_path) else {
            return;
        };

        match import.symbol {
            ImportSymbol::All => {
                stack.push((module, fallback_name.to_string()));
            }
            ImportSymbol::Default => stack.push((module, "default".to_string())),
            ImportSymbol::Named(name) => stack.push((module, name.text().to_string())),
        }
    }

    fn resolve_own_export_from_tables(
        &self,
        inferred_types: &InferredModuleTypes<'db>,
        own_export: &JsOwnExport,
    ) -> InferredTypeData<'db> {
        match own_export {
            JsOwnExport::Binding(range) => inferred_types
                .binding_type_data
                .get(range)
                .map_or(InferredTypeData::Unknown, |data| data.ty),
            JsOwnExport::Type(resolved_id) => inferred_type_from_resolved_id_from_tables(
                self.db,
                inferred_types,
                ResolvedTypeId::Local(*resolved_id),
            ),
            JsOwnExport::Namespace(reexport) => self.resolve_js_import(&reexport.import),
        }
    }
}

fn is_namespace_export_collectible(infer_types: bool, export: &JsExport) -> bool {
    infer_types
        || !matches!(
            export,
            JsExport::Own(JsOwnExport::Type(_)) | JsExport::OwnType(JsOwnExport::Type(_))
        )
}

fn inferred_type_from_binding_on_demand<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    js_info: &crate::JsModuleInfo,
    range: TextRange,
    import_resolution: super::ImportResolution<'_>,
) -> InferredTypeData<'db> {
    if let Some(TypeReference::Resolved(resolved_id)) = js_info.raw_binding_types.get(&range)
        && resolved_id.level() == TypeResolverLevel::Thin
        && js_info.is_named_type(resolved_id.id())
    {
        return InferredTypeData::Local(LocalTypeHandle::new(
            db,
            ModuleKey::new(module.as_id()),
            LocalTypeId::new(resolved_id.index()),
        ));
    }

    let input = BindingTypeInput::new(db, module, range);
    match import_resolution {
        super::ImportResolution::OnDemand { remaining } => {
            let input = BindingTypeWithImportBudgetInput::new(db, input, remaining);
            infer_binding_type_with_import_budget(db, input)
        }
        super::ImportResolution::FromTables { .. } | super::ImportResolution::CycleFallback(_) => {
            infer_binding_type(db, input)
        }
    }
    .unwrap_or(InferredTypeData::Unknown)
}

/// Resolves an exported type ID from complete inferred module tables.
fn inferred_type_from_resolved_id_from_tables<'db>(
    db: &'db dyn ModuleDb,
    inferred_types: &InferredModuleTypes<'db>,
    resolved_id: ResolvedTypeId,
) -> InferredTypeData<'db> {
    match resolved_id.level() {
        TypeResolverLevel::Thin => {
            let local_type_id = LocalTypeId::new(resolved_id.index());
            if inferred_types
                .named_type_ids
                .binary_search(&local_type_id)
                .is_ok()
            {
                InferredTypeData::Local(LocalTypeHandle::new(
                    db,
                    inferred_types.module_key,
                    local_type_id,
                ))
            } else {
                inferred_types
                    .types
                    .get(resolved_id.index())
                    .copied()
                    .unwrap_or(InferredTypeData::Unknown)
            }
        }
        TypeResolverLevel::Global => GlobalTypeId::try_from_type_id(resolved_id.id())
            .map_or(InferredTypeData::Unknown, |id| global_type(db, id)),
        TypeResolverLevel::Full | TypeResolverLevel::Import => InferredTypeData::Unknown,
    }
}

/// Resolves an exported type ID without inferring complete module tables.
///
/// Named declarations remain symbolic local handles so recursive types retain
/// their module identity. Other local types are requested through the lookup
/// query.
fn inferred_type_from_resolved_id_on_demand<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    js_info: &crate::JsModuleInfo,
    resolved_id: ResolvedTypeId,
    import_resolution: super::ImportResolution<'_>,
) -> InferredTypeData<'db> {
    match resolved_id.level() {
        TypeResolverLevel::Thin => {
            let local_type_id = LocalTypeId::new(resolved_id.index());
            if js_info.is_named_type(resolved_id.id()) {
                InferredTypeData::Local(LocalTypeHandle::new(
                    db,
                    ModuleKey::new(module.as_id()),
                    local_type_id,
                ))
            } else {
                let input = LocalTypeInput::new(db, module, local_type_id);
                match import_resolution {
                    super::ImportResolution::OnDemand { remaining } => {
                        let input = LocalTypeWithImportBudgetInput::new(db, input, remaining);
                        infer_local_type_with_import_budget(db, input)
                    }
                    super::ImportResolution::FromTables { .. }
                    | super::ImportResolution::CycleFallback(_) => infer_local_type(db, input),
                }
                .unwrap_or(InferredTypeData::Unknown)
            }
        }
        TypeResolverLevel::Global => GlobalTypeId::try_from_type_id(resolved_id.id())
            .map_or(InferredTypeData::Unknown, |id| global_type(db, id)),
        TypeResolverLevel::Full | TypeResolverLevel::Import => InferredTypeData::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_js_type_info::TypeId;

    #[test]
    fn disabled_inference_only_hides_exports_with_local_type_ids() {
        let type_export = JsExport::Own(JsOwnExport::Type(TypeId::new(0)));
        let binding_export = JsExport::Own(JsOwnExport::Binding(TextRange::default()));

        assert!(!is_namespace_export_collectible(false, &type_export));
        assert!(is_namespace_export_collectible(false, &binding_export));
        assert!(is_namespace_export_collectible(true, &type_export));
    }
}

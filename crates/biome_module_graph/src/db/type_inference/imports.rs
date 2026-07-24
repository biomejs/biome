use super::{InferredModuleTypes, globals::global_type, resolver::ResolutionCtx};
use crate::module_graph::{ModuleInfo, ModuleInfoKind};
use crate::{JsExport, JsImport, JsOwnExport, ModuleDb, ResolvedPath};
use biome_js_type_info::{
    GlobalTypeId, ImportSymbol, Path, ResolvedTypeId, TypeImportQualifier, TypeResolverLevel,
    interned_types::{
        InternedNamespace as InferredNamespace, LocalTypeHandle, LocalTypeId, ModuleKey,
        TypeData as InferredTypeData, TypeMember as InferredTypeMember,
        TypeMemberKind as InferredTypeMemberKind,
    },
};
use biome_rowan::Text;
use rustc_hash::FxHashSet;
use salsa::plumbing::AsId;

const MAX_EXPORT_RESOLUTION_STEPS: usize = 1024;

struct NamespaceExportCollection {
    names: Vec<Text>,
    seen_names: FxHashSet<String>,
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

impl<'db> ResolutionCtx<'db, '_> {
    pub(in crate::db::type_inference) fn resolve_import(
        &mut self,
        qualifier: &TypeImportQualifier,
    ) -> InferredTypeData<'db> {
        let Some(module) = self.module_for_resolved_path(&qualifier.resolved_path) else {
            return InferredTypeData::Unknown;
        };

        self.infer_imported_module(module)
            .map_or(InferredTypeData::Unknown, |types| {
                self.resolve_import_symbol(module, types, &qualifier.symbol)
            })
    }

    fn module_for_resolved_path(&self, resolved_path: &ResolvedPath) -> Option<ModuleInfo> {
        let path = resolved_path.as_path()?;
        self.db.module_for_path(path)
    }

    fn resolve_import_symbol(
        &self,
        module: ModuleInfo,
        inferred_types: &InferredModuleTypes<'db>,
        symbol: &ImportSymbol,
    ) -> InferredTypeData<'db> {
        match symbol {
            ImportSymbol::All => self.namespace_for_module(module, inferred_types),
            ImportSymbol::Default => self.resolve_export_name(module, inferred_types, "default"),
            ImportSymbol::Named(name) => {
                self.resolve_export_name(module, inferred_types, name.text())
            }
        }
    }

    fn resolve_js_import(&self, import: &JsImport) -> InferredTypeData<'db> {
        self.module_for_resolved_path(&import.resolved_path)
            .and_then(|module| {
                self.infer_imported_module(module)
                    .map(|types| self.resolve_import_symbol(module, types, &import.symbol))
            })
            .unwrap_or(InferredTypeData::Unknown)
    }

    fn namespace_for_module(
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

        let mut members = Vec::with_capacity(collection.names.len());
        for name in collection.names {
            match self.resolve_export_name_result(module, inferred_types, name.text()) {
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

        for (name, _) in js_info.raw_exports.iter() {
            if !include_default && name.text() == "default" {
                continue;
            }

            if !collection.seen_names.insert(name.text().to_string()) {
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

    fn resolve_export_name(
        &self,
        module: ModuleInfo,
        inferred_types: &InferredModuleTypes<'db>,
        name: &str,
    ) -> InferredTypeData<'db> {
        match self.resolve_export_name_result(module, inferred_types, name) {
            ExportResolution::Resolved(resolved) => resolved.ty,
            ExportResolution::Missing
            | ExportResolution::Ambiguous
            | ExportResolution::Indeterminate => InferredTypeData::Unknown,
        }
    }

    fn resolve_export_name_result(
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
        match self.resolve_export_name_in_module(module, inferred_types, name, &mut stack) {
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

            match self.resolve_export_name_in_module(module, inferred_types, &name, &mut stack) {
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

    fn resolve_export_name_in_module(
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

        match js_info.raw_exports.get(name) {
            Some(JsExport::Own(own_export) | JsExport::OwnType(own_export)) => {
                ExportResolutionStep::Resolved(ResolvedExport {
                    identity: ExportIdentity {
                        module: module_key,
                        own_export: own_export.clone(),
                    },
                    ty: self.resolve_own_export(inferred_types, own_export),
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

    fn resolve_own_export(
        &self,
        inferred_types: &InferredModuleTypes<'db>,
        own_export: &JsOwnExport,
    ) -> InferredTypeData<'db> {
        match own_export {
            JsOwnExport::Binding(range) => inferred_types
                .binding_type_data
                .get(range)
                .map_or(InferredTypeData::Unknown, |data| data.ty),
            JsOwnExport::Type(resolved_id) => {
                inferred_type_from_resolved_id(self.db, inferred_types, *resolved_id)
            }
            JsOwnExport::Namespace(reexport) => self.resolve_js_import(&reexport.import),
        }
    }
}

fn inferred_type_from_resolved_id<'db>(
    db: &'db dyn ModuleDb,
    inferred_types: &InferredModuleTypes<'db>,
    resolved_id: ResolvedTypeId,
) -> InferredTypeData<'db> {
    match resolved_id.level() {
        TypeResolverLevel::Thin => {
            let local_type_id = LocalTypeId::new(resolved_id.index());
            if inferred_types.named_type_ids.contains(&local_type_id) {
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

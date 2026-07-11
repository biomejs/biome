use super::{InferredModuleTypes, globals::resolve_global_type_id, resolver::ResolutionCtx};
use crate::module_graph::{ModuleInfo, ModuleInfoKind};
use crate::{JsExport, JsImport, JsOwnExport, ModuleDb, ResolvedPath};
use biome_js_type_info::{
    ImportSymbol, Path, ResolvedTypeId, TypeImportQualifier, TypeResolverLevel,
    interned_types::{
        InternedNamespace as InferredNamespace, LocalTypeHandle, LocalTypeId, ModuleKey,
        TypeData as InferredTypeData, TypeMember as InferredTypeMember,
        TypeMemberKind as InferredTypeMemberKind,
    },
};
use biome_rowan::Text;
use rustc_hash::{FxHashMap, FxHashSet};
use salsa::plumbing::AsId;

const MAX_EXPORT_RESOLUTION_STEPS: usize = 1024;

struct NamespaceExportCollection<'db> {
    members: Vec<InferredTypeMember<'db>>,
    seen_names: FxHashSet<String>,
    seen_modules: FxHashSet<ModuleKey>,
    stack: Vec<(ModuleInfo, bool)>,
    remaining_steps: usize,
}

impl NamespaceExportCollection<'_> {
    fn new() -> Self {
        Self {
            members: Vec::new(),
            seen_names: FxHashSet::default(),
            seen_modules: FxHashSet::default(),
            stack: Vec::new(),
            remaining_steps: MAX_EXPORT_RESOLUTION_STEPS,
        }
    }
}

impl<'db> ResolutionCtx<'db, '_> {
    pub(in crate::db::type_inference) fn resolve_import(
        &mut self,
        qualifier: &TypeImportQualifier,
    ) -> InferredTypeData<'db> {
        self.resolve_import_qualifier(qualifier)
    }

    fn resolve_import_qualifier(&self, qualifier: &TypeImportQualifier) -> InferredTypeData<'db> {
        let Some(module) = self.module_for_resolved_path(&qualifier.resolved_path) else {
            return InferredTypeData::Unknown;
        };

        self.infer_imported_module(module)
            .map_or(InferredTypeData::Unknown, |types| {
                self.resolve_import_symbol(module, &types, &qualifier.symbol)
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
                    .map(|types| self.resolve_import_symbol(module, &types, &import.symbol))
            })
            .unwrap_or(InferredTypeData::Unknown)
    }

    fn namespace_for_module(
        &self,
        module: ModuleInfo,
        inferred_types: &InferredModuleTypes<'db>,
    ) -> InferredTypeData<'db> {
        let mut collection = NamespaceExportCollection::new();

        self.collect_namespace_members(module, inferred_types, true, &mut collection);

        while let Some((module, include_default)) = collection.stack.pop() {
            if collection.remaining_steps == 0 {
                break;
            }
            collection.remaining_steps -= 1;

            let Some(inferred_types) = self.infer_imported_module(module) else {
                continue;
            };

            self.collect_namespace_members(
                module,
                &inferred_types,
                include_default,
                &mut collection,
            );
        }

        InferredTypeData::Namespace(InferredNamespace::new(
            self.db,
            collection.members.into_boxed_slice(),
            Path::from(Text::from(module.path(self.db).to_string())),
        ))
    }

    fn collect_namespace_members(
        &self,
        module: ModuleInfo,
        inferred_types: &InferredModuleTypes<'db>,
        include_default: bool,
        collection: &mut NamespaceExportCollection<'db>,
    ) {
        let module_key = ModuleKey::new(module.as_id());
        if !collection.seen_modules.insert(module_key) {
            return;
        }

        let ModuleInfoKind::Js(js_info) = module.kind(self.db) else {
            return;
        };

        for (name, _) in js_info.exports.iter() {
            if !include_default && name.text() == "default" {
                continue;
            }

            if !collection.seen_names.insert(name.text().to_string()) {
                continue;
            }

            collection.members.push(InferredTypeMember {
                kind: InferredTypeMemberKind::Named(name.clone()),
                ty: self.resolve_export_name(module, inferred_types, name.text()),
            });
        }

        for reexport in js_info.blanket_reexports.iter().rev() {
            if let Some(module) = self.module_for_resolved_path(&reexport.import.resolved_path) {
                collection.stack.push((module, false));
            }
        }
    }

    fn resolve_export_name(
        &self,
        module: ModuleInfo,
        inferred_types: &InferredModuleTypes<'db>,
        name: &str,
    ) -> InferredTypeData<'db> {
        let mut stack = Vec::new();
        let mut seen = FxHashSet::default();
        // Shared across direct stack pops and blanket reexport scans.
        let mut remaining_steps = MAX_EXPORT_RESOLUTION_STEPS;

        if let Some(ty) = self.resolve_export_name_in_module(
            module,
            inferred_types,
            name,
            &mut stack,
            &mut seen,
            &mut remaining_steps,
        ) {
            return ty;
        }

        while let Some((module, name)) = stack.pop() {
            if remaining_steps == 0 {
                return InferredTypeData::Unknown;
            }
            remaining_steps -= 1;

            let Some(inferred_types) = self.infer_imported_module(module) else {
                continue;
            };

            if let Some(ty) = self.resolve_export_name_in_module(
                module,
                &inferred_types,
                &name,
                &mut stack,
                &mut seen,
                &mut remaining_steps,
            ) {
                return ty;
            }
        }

        InferredTypeData::Unknown
    }

    fn resolve_export_name_in_module(
        &self,
        module: ModuleInfo,
        inferred_types: &InferredModuleTypes<'db>,
        name: &str,
        stack: &mut Vec<(ModuleInfo, String)>,
        seen: &mut FxHashSet<(ModuleKey, String)>,
        remaining_steps: &mut usize,
    ) -> Option<InferredTypeData<'db>> {
        let module_key = ModuleKey::new(module.as_id());
        if !seen.insert((module_key, name.to_string())) {
            return None;
        }

        let ModuleInfoKind::Js(js_info) = module.kind(self.db) else {
            return None;
        };

        match js_info.exports.get(name) {
            Some(JsExport::Own(own_export) | JsExport::OwnType(own_export)) => {
                Some(self.resolve_own_export(inferred_types, own_export))
            }
            Some(JsExport::Reexport(reexport) | JsExport::ReexportType(reexport)) => {
                self.push_reexport_target(reexport.import.clone(), name, stack);
                None
            }
            None => {
                for reexport in js_info.blanket_reexports.iter().rev() {
                    if *remaining_steps == 0 {
                        return Some(InferredTypeData::Unknown);
                    }
                    *remaining_steps -= 1;
                    if let Some(module) =
                        self.module_for_resolved_path(&reexport.import.resolved_path)
                    {
                        stack.push((module, name.to_string()));
                    }
                }
                None
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
        TypeResolverLevel::Global => {
            let mut resolved_globals = FxHashMap::default();
            resolve_global_type_id(db, resolved_id.id(), &mut resolved_globals)
        }
        TypeResolverLevel::Full | TypeResolverLevel::Import => InferredTypeData::Unknown,
    }
}

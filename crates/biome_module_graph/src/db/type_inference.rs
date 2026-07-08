use crate::module_graph::{ModuleInfo, ModuleInfoKind};
use crate::{JsExport, JsModuleInfo, JsOwnExport, ModuleDb, ResolvedPath};
use biome_css_syntax::TextRange;
use biome_js_semantic::JsDeclarationKind;
use biome_js_type_info::{
    GLOBAL_RESOLVER, ImportSymbol, ResolvedTypeId, TypeId, TypeImportQualifier, TypeReference,
    TypeResolver, TypeResolverLevel,
    interned_types::{LocalTypeHandle, LocalTypeId, ModuleKey, TypeData as InferredTypeData},
};
use rustc_hash::{FxHashMap, FxHashSet};
use salsa::plumbing::AsId;

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

pub(super) fn infer_module_types_cycle_result<'db>(
    _db: &'db dyn ModuleDb,
    _id: salsa::Id,
    _module: ModuleInfo,
) -> Option<InferredModuleTypes<'db>> {
    None
}

pub(super) fn normalize_type_cycle_result<'db>(
    _db: &'db dyn ModuleDb,
    _id: salsa::Id,
    _module: ModuleInfo,
    _ty: InferredTypeData<'db>,
) -> InferredTypeData<'db> {
    InferredTypeData::Unknown
}

struct ResolutionCtx<'db, 'a> {
    db: &'db dyn ModuleDb,
    module_key: ModuleKey,
    js_info: &'a JsModuleInfo,
    import_types: &'a FxHashMap<ResolvedPath, InferredModuleTypes<'db>>,
    named_type_ids: FxHashSet<TypeId>,
    resolved: FxHashMap<TypeId, InferredTypeData<'db>>,
    in_progress: FxHashSet<TypeId>,
}

pub(super) fn resolve_raw_types<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    js_info: &JsModuleInfo,
    import_types: &FxHashMap<ResolvedPath, InferredModuleTypes<'db>>,
) -> InferredModuleTypes<'db> {
    let module_key = ModuleKey::new(module.as_id());
    let named_type_ids = named_type_ids(js_info);
    let mut ctx = ResolutionCtx {
        db,
        module_key,
        js_info,
        import_types,
        named_type_ids,
        resolved: FxHashMap::default(),
        in_progress: FxHashSet::default(),
    };

    let mut named_type_ids = ctx
        .named_type_ids
        .iter()
        .map(|type_id| LocalTypeId::new(type_id.index()))
        .collect::<Vec<_>>();
    named_type_ids.sort_unstable();

    let types = (0..js_info.raw_types.len())
        .map(|index| ctx.resolve_raw_type_id(TypeId::new(index)))
        .collect();

    let expressions = js_info
        .raw_expressions
        .iter()
        .map(|(range, reference)| (*range, ctx.resolve(reference)))
        .collect();

    let binding_type_data = js_info
        .raw_binding_types
        .iter()
        .map(|(range, reference)| {
            (
                *range,
                BindingTypeData {
                    ty: ctx.resolve(reference),
                },
            )
        })
        .collect();

    InferredModuleTypes {
        module_key,
        named_type_ids: named_type_ids.into_boxed_slice(),
        types,
        expressions,
        binding_type_data,
    }
}

fn named_type_ids(js_info: &JsModuleInfo) -> FxHashSet<TypeId> {
    js_info
        .raw_binding_types
        .iter()
        .filter_map(|(range, reference)| {
            let binding = js_info.semantic_model.as_binding_by_range(*range)?;
            if !is_named_type_declaration(binding.declaration_kind()) {
                return None;
            }
            let TypeReference::Resolved(resolved_id) = reference else {
                return None;
            };
            (resolved_id.level() == TypeResolverLevel::Thin).then(|| resolved_id.id())
        })
        .collect()
}

fn is_named_type_declaration(declaration_kind: JsDeclarationKind) -> bool {
    matches!(
        declaration_kind,
        JsDeclarationKind::Class
            | JsDeclarationKind::Enum
            | JsDeclarationKind::Interface
            | JsDeclarationKind::Module
            | JsDeclarationKind::Namespace
            | JsDeclarationKind::Type
    )
}

impl<'db> ResolutionCtx<'db, '_> {
    fn resolve(&mut self, reference: &TypeReference) -> InferredTypeData<'db> {
        match reference {
            TypeReference::Resolved(resolved_id) => self.resolve_resolved_id(*resolved_id),
            TypeReference::Qualifier(qualifier) => self.resolve_qualifier(qualifier),
            TypeReference::Import(import) => self.resolve_import(import),
        }
    }

    fn resolve_resolved_id(&mut self, resolved_id: ResolvedTypeId) -> InferredTypeData<'db> {
        match resolved_id.level() {
            TypeResolverLevel::Thin => self.resolve_raw_type_reference(resolved_id.id()),
            TypeResolverLevel::Global => InferredTypeData::from_raw_lossy(
                self.db,
                GLOBAL_RESOLVER.get_by_id(resolved_id.id()),
            ),
            TypeResolverLevel::Full | TypeResolverLevel::Import => InferredTypeData::Unknown,
        }
    }

    fn resolve_raw_type_reference(&mut self, type_id: TypeId) -> InferredTypeData<'db> {
        if self.named_type_ids.contains(&type_id) {
            return self.local_type(type_id);
        }

        self.resolve_raw_type_id(type_id)
    }

    fn local_type(&self, type_id: TypeId) -> InferredTypeData<'db> {
        InferredTypeData::Local(LocalTypeHandle::new(
            self.db,
            self.module_key,
            LocalTypeId::new(type_id.index()),
        ))
    }

    fn resolve_raw_type_id(&mut self, type_id: TypeId) -> InferredTypeData<'db> {
        if let Some(ty) = self.resolved.get(&type_id) {
            return *ty;
        }

        if !self.in_progress.insert(type_id) {
            return InferredTypeData::Unknown;
        }

        let ty = self.js_info.raw_types.get(type_id.index()).cloned().map_or(
            InferredTypeData::Unknown,
            |raw| {
                let db = self.db;
                InferredTypeData::from_raw_with_resolver(db, &raw, &mut |reference| {
                    self.resolve(reference)
                })
            },
        );

        self.in_progress.remove(&type_id);
        self.resolved.insert(type_id, ty);
        ty
    }

    fn resolve_qualifier(
        &mut self,
        qualifier: &biome_js_type_info::TypeReferenceQualifier,
    ) -> InferredTypeData<'db> {
        let Some(identifier) = qualifier.path.iter().next() else {
            return InferredTypeData::Unknown;
        };

        let mut scope = self
            .js_info
            .semantic_model
            .scope_from_id(qualifier.scope_id);
        loop {
            if let Some(binding) = scope.get_binding(identifier.text()) {
                if binding.is_imported()
                    && let Some(import) = self.js_info.static_imports.get(identifier.text())
                {
                    return self.resolve_import(&TypeImportQualifier {
                        symbol: import.symbol.clone(),
                        resolved_path: import.resolved_path.clone(),
                        type_only: qualifier.type_only,
                    });
                }

                return self
                    .js_info
                    .raw_binding_types
                    .get(&binding.syntax().text_trimmed_range())
                    .cloned()
                    .map_or(InferredTypeData::Unknown, |reference| {
                        self.resolve(&reference)
                    });
            }

            match scope.parent() {
                Some(parent) => scope = parent,
                None => break,
            }
        }

        GLOBAL_RESOLVER
            .resolve_qualifier(qualifier)
            .map_or(InferredTypeData::Unknown, |resolved_id| {
                self.resolve_resolved_id(resolved_id)
            })
    }

    fn resolve_import(&mut self, qualifier: &TypeImportQualifier) -> InferredTypeData<'db> {
        let Some(imported_types) = self.import_types.get(&qualifier.resolved_path) else {
            return InferredTypeData::Unknown;
        };

        let Some(path) = qualifier.resolved_path.as_path() else {
            return InferredTypeData::Unknown;
        };
        let Some(module) = self.db.module_for_path(path) else {
            return InferredTypeData::Unknown;
        };
        let ModuleInfoKind::Js(js_info) = module.kind(self.db) else {
            return InferredTypeData::Unknown;
        };

        let export_name = match &qualifier.symbol {
            ImportSymbol::Default => "default",
            ImportSymbol::Named(name) => name.text(),
            ImportSymbol::All => return InferredTypeData::Unknown,
        };

        match js_info
            .exports
            .get(export_name)
            .and_then(JsExport::as_own_export)
        {
            Some(JsOwnExport::Binding(range)) => imported_types
                .binding_type_data
                .get(range)
                .map_or(InferredTypeData::Unknown, |data| data.ty),
            Some(JsOwnExport::Type(resolved_id)) => {
                inferred_type_from_resolved_id(self.db, imported_types, *resolved_id)
            }
            Some(JsOwnExport::Namespace(_)) | None => InferredTypeData::Unknown,
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
            InferredTypeData::from_raw_lossy(db, GLOBAL_RESOLVER.get_by_id(resolved_id.id()))
        }
        TypeResolverLevel::Full | TypeResolverLevel::Import => InferredTypeData::Unknown,
    }
}

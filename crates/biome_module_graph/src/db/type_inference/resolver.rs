use super::{BindingTypeData, InferredModuleTypes, globals::global_type};
use crate::db::queries::{
    LocalTypeInput, infer_local_type, infer_module_types, infer_module_types_from_tables,
    inference_module_sccs,
};
use crate::module_graph::ModuleInfo;
use crate::{JsModuleInfo, ModuleDb, ModuleGraphGeneration, module_for_key};
use biome_js_syntax::TsModuleDeclaration;
use biome_js_type_info::{
    GlobalTypeId, RawTypeData, ResolvedTypeId, ScopeId, TypeId, TypeReference,
    TypeReferenceQualifier, TypeResolverLevel,
    interned_types::{
        InternedModule as InferredModule, InternedNamespace as InferredNamespace,
        InternedTypeofValue, LocalTypeHandle, LocalTypeId, ModuleKey, TypeData as InferredTypeData,
        TypeMember as InferredTypeMember, TypeMemberKind as InferredTypeMemberKind,
    },
};
use biome_rowan::AstNode;
use rustc_hash::{FxHashMap, FxHashSet};
use salsa::plumbing::AsId;
use std::cell::Cell;

// This limit guards Rust stack recursion. Each `ResolutionCtx::resolve` frame
// clones a raw type and runs a conversion walk. Named declarations become local
// handles before recursion, so the remaining depth comes from structural types
// nested within one declaration.
pub(super) const MAX_RAW_TYPE_RESOLUTION_DEPTH: usize = 64;
pub(super) const MAX_ON_DEMAND_IMPORT_DEPTH: u8 = 128;
const MAX_INFERRED_EXPRESSION_WRAPPER_STEPS: usize = 64;
const MAX_LOCAL_TYPE_RESOLUTION_STEPS: usize = 1024;

/// Selects how a resolution context follows references into imported modules.
///
/// Regular queries resolve only the requested imported symbol. Salsa cycle
/// recovery cannot recursively request members of the strongly connected
/// component that caused the cycle. Cycle fallback therefore reads complete
/// inferred tables for dependencies outside that component and treats imports
/// into the component as unavailable.
#[derive(Clone, Copy)]
pub(in crate::db) enum ImportResolution<'a> {
    /// Resolves imported symbols through export and lookup queries.
    OnDemand { remaining: u8 },
    /// Reads imports from complete module tables while blocking cyclic edges.
    FromTables { root: ModuleInfo },
    /// Reads imports from complete module tables while blocking the active cycle.
    CycleFallback(&'a FxHashSet<ModuleInfo>),
}

impl ImportResolution<'_> {
    pub(in crate::db) fn on_demand() -> Self {
        Self::OnDemand {
            remaining: MAX_ON_DEMAND_IMPORT_DEPTH,
        }
    }
}

pub(in crate::db) struct ResolutionCtx<'db, 'a> {
    pub(in crate::db::type_inference) db: &'db dyn ModuleDb,
    pub(in crate::db::type_inference) module: ModuleInfo,
    pub(in crate::db::type_inference) module_key: ModuleKey,
    pub(in crate::db::type_inference) js_info: &'a JsModuleInfo,
    pub(in crate::db::type_inference) import_resolution: ImportResolution<'a>,
    pub(in crate::db::type_inference) resolved: FxHashMap<TypeId, InferredTypeData<'db>>,
    pub(in crate::db::type_inference) in_progress: FxHashSet<TypeId>,
    pub(in crate::db::type_inference) resolution_depth: Cell<usize>,
}

pub(in crate::db) fn resolve_raw_types<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    js_info: &JsModuleInfo,
    import_resolution: ImportResolution<'_>,
) -> InferredModuleTypes<'db> {
    let mut ctx = ResolutionCtx::new(db, module, js_info, import_resolution);

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
        module_key: ctx.module_key,
        named_type_ids: js_info.named_type_ids.clone(),
        types,
        expressions,
        binding_type_data,
    }
}

impl<'db, 'a> ResolutionCtx<'db, 'a> {
    pub(in crate::db) fn new(
        db: &'db dyn ModuleDb,
        module: ModuleInfo,
        js_info: &'a JsModuleInfo,
        import_resolution: ImportResolution<'a>,
    ) -> Self {
        Self {
            db,
            module,
            module_key: ModuleKey::new(module.as_id()),
            js_info,
            import_resolution,
            resolved: FxHashMap::default(),
            in_progress: FxHashSet::default(),
            resolution_depth: Cell::new(0),
        }
    }

    /// Infers `module` as a dependency of the module currently being resolved.
    ///
    /// The tracked query records the imported result as a dependency of the
    /// current inference. Returns `None` for unsupported modules, disabled type
    /// inference, or an import cycle.
    pub(in crate::db::type_inference) fn infer_imported_module(
        &self,
        module: ModuleInfo,
    ) -> Option<&'db InferredModuleTypes<'db>> {
        match self.import_resolution {
            ImportResolution::OnDemand { .. } => infer_module_types(self.db, module),
            ImportResolution::FromTables { root } => {
                let sccs = inference_module_sccs(self.db, ModuleGraphGeneration::get(self.db));
                if module == self.module || sccs.contains_cycle_between(self.module, module) {
                    None
                } else {
                    infer_module_types_from_tables(self.db, root, module)
                }
            }
            ImportResolution::CycleFallback(blocked) => {
                if blocked.contains(&module) {
                    None
                } else {
                    infer_module_types(self.db, module)
                }
            }
        }
    }

    /// Converts a raw type reference to an inferred type.
    ///
    /// Recursive conversion is limited to 64 active calls shared by resolved
    /// IDs, qualifiers, and imports in this context. A reference that would
    /// exceed the limit resolves to `Unknown`; its caller may still construct a
    /// surrounding type containing that `Unknown`. Named declarations usually
    /// avoid this recursion by resolving to symbolic local handles.
    pub(in crate::db) fn resolve(&mut self, reference: &TypeReference) -> InferredTypeData<'db> {
        let resolution_depth = self.resolution_depth.get();
        if resolution_depth >= MAX_RAW_TYPE_RESOLUTION_DEPTH {
            return InferredTypeData::Unknown;
        }

        self.resolution_depth.set(resolution_depth + 1);
        let resolved = match reference {
            TypeReference::Resolved(resolved_id) => self.resolve_resolved_id(*resolved_id),
            TypeReference::Qualifier(qualifier) => self.resolve_qualifier(qualifier),
            TypeReference::Import(import) => self.resolve_import(import),
        };
        self.resolution_depth.set(resolution_depth);
        resolved
    }

    pub(in crate::db::type_inference) fn resolve_resolved_id(
        &mut self,
        resolved_id: ResolvedTypeId,
    ) -> InferredTypeData<'db> {
        match resolved_id.level() {
            TypeResolverLevel::Thin => self.resolve_raw_type_reference(resolved_id.id()),
            TypeResolverLevel::Global => GlobalTypeId::try_from_type_id(resolved_id.id())
                .map_or(InferredTypeData::Unknown, |id| global_type(self.db, id)),
            TypeResolverLevel::Full | TypeResolverLevel::Import => InferredTypeData::Unknown,
        }
    }

    fn resolve_raw_type_reference(&mut self, type_id: TypeId) -> InferredTypeData<'db> {
        if self.js_info.is_named_type(type_id) {
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

    pub(in crate::db) fn resolve_raw_type_id(&mut self, type_id: TypeId) -> InferredTypeData<'db> {
        if let Some(ty) = self.resolved.get(&type_id) {
            return *ty;
        }

        if !self.in_progress.insert(type_id) {
            // A resolution cycle keeps the reference symbolic: lookups that
            // are aware of in-progress types can still read the raw
            // declaration behind the handle, which `Unknown` would rule out.
            return self.local_type(type_id);
        }

        let js_info = self.js_info;
        let ty = js_info
            .raw_types
            .get(type_id.index())
            .map_or(InferredTypeData::Unknown, |raw| self.resolve_raw_type(raw));
        let ty = self.with_namespace_members(type_id, ty);

        self.in_progress.remove(&type_id);
        self.resolved.insert(type_id, ty);
        ty
    }

    fn with_namespace_members(
        &mut self,
        type_id: TypeId,
        fallback: InferredTypeData<'db>,
    ) -> InferredTypeData<'db> {
        enum NamespaceMetadata {
            Module(biome_rowan::Text),
            Namespace(biome_js_type_info::Path),
        }

        let metadata = match self.js_info.raw_types.get(type_id.index()) {
            Some(RawTypeData::Module(module)) => NamespaceMetadata::Module(module.name.clone()),
            Some(RawTypeData::Namespace(namespace)) => {
                NamespaceMetadata::Namespace(namespace.path.clone())
            }
            _ => return fallback,
        };
        let members = self
            .js_info
            .semantic_model
            .all_bindings()
            .filter_map(|binding| {
                let reference = self
                    .js_info
                    .raw_binding_types
                    .get(&binding.syntax().text_trimmed_range())?;
                if !matches!(
                    reference,
                    TypeReference::Resolved(id)
                        if id.level() == TypeResolverLevel::Thin && id.id() == type_id
                ) {
                    return None;
                }
                let declaration = binding.tree().declaration()?;
                TsModuleDeclaration::cast(declaration.syntax().clone())
            })
            .flat_map(|declaration| {
                self.js_info
                    .semantic_model
                    .scope(declaration.syntax())
                    .bindings()
            })
            .filter_map(|binding| {
                let name = binding
                    .tree()
                    .name_token()
                    .ok()?
                    .token_text_trimmed()
                    .into();
                let reference = self
                    .js_info
                    .raw_binding_types
                    .get(&binding.syntax().text_trimmed_range())?
                    .clone();
                Some((name, reference))
            })
            .map(|(name, reference)| InferredTypeMember {
                kind: InferredTypeMemberKind::NamedStatic(name),
                ty: self.resolve(&reference),
            })
            .collect::<Box<[_]>>();

        match metadata {
            NamespaceMetadata::Module(name) => {
                InferredTypeData::Module(InferredModule::new(self.db, members, name))
            }
            NamespaceMetadata::Namespace(path) => {
                InferredTypeData::Namespace(InferredNamespace::new(self.db, members, path))
            }
        }
    }

    fn resolve_raw_type(&mut self, raw: &RawTypeData) -> InferredTypeData<'db> {
        if let RawTypeData::TypeofExpression(expression) = raw
            && let Some(ty) = self.resolve_typeof_expression(expression)
        {
            return ty;
        }

        if let RawTypeData::TypeofValue(value) = raw {
            let ty = if value.ty.is_unknown() {
                self.resolve_qualifier(&TypeReferenceQualifier::from_path(
                    value.scope_id.unwrap_or(ScopeId::GLOBAL),
                    value.identifier.clone(),
                ))
            } else {
                self.resolve(&value.ty)
            };
            return InferredTypeData::TypeofValue(InternedTypeofValue::new(
                self.db,
                ty,
                value.identifier.clone(),
                value.scope_id,
            ));
        }

        let db = self.db;
        let ty = InferredTypeData::from_raw_with_resolver(db, raw, false, &mut |reference| {
            self.resolve(reference)
        });
        self.resolve_inferred_expression_wrappers(ty)
    }

    fn resolve_inferred_expression_wrappers(
        &mut self,
        mut ty: InferredTypeData<'db>,
    ) -> InferredTypeData<'db> {
        for _ in 0..MAX_INFERRED_EXPRESSION_WRAPPER_STEPS {
            match ty {
                InferredTypeData::TypeofExpression(expression) => {
                    ty = self
                        .resolve_inferred_typeof_expression(expression.expression(self.db))
                        .unwrap_or(InferredTypeData::Unknown);
                }
                InferredTypeData::InstanceOf(instance) => {
                    let target = instance.ty(self.db);
                    let InferredTypeData::TypeofExpression(expression) = target else {
                        return ty;
                    };
                    let target = self
                        .resolve_inferred_typeof_expression(expression.expression(self.db))
                        .unwrap_or(InferredTypeData::Unknown);
                    if target.should_flatten_instance(instance.type_parameters(self.db)) {
                        ty = target;
                    } else {
                        return InferredTypeData::instance_of(
                            self.db,
                            target,
                            instance
                                .type_parameters(self.db)
                                .to_vec()
                                .into_boxed_slice(),
                        );
                    }
                }
                InferredTypeData::Unknown
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
                | InferredTypeData::MergedReference(_)
                | InferredTypeData::TypeofType(_)
                | InferredTypeData::TypeofValue(_)
                | InferredTypeData::AnyKeyword
                | InferredTypeData::NeverKeyword
                | InferredTypeData::ObjectKeyword
                | InferredTypeData::ThisKeyword
                | InferredTypeData::UnknownKeyword
                | InferredTypeData::VoidKeyword => return ty,
            }
        }

        ty
    }

    pub(in crate::db::type_inference) fn resolve_inferred_type(
        &mut self,
        mut ty: InferredTypeData<'db>,
    ) -> InferredTypeData<'db> {
        let mut seen = FxHashSet::default();

        for _ in 0..MAX_LOCAL_TYPE_RESOLUTION_STEPS {
            let InferredTypeData::Local(local) = ty else {
                return ty;
            };

            let module_key = local.module(self.db);
            let local_type_id = local.type_id(self.db);
            if !seen.insert((module_key, local_type_id)) {
                return ty;
            }

            ty = if module_key == self.module_key {
                self.resolve_raw_type_id(TypeId::new(local_type_id.index()))
            } else {
                module_for_key(self.db, module_key)
                    .and_then(|module| {
                        let input = LocalTypeInput::new(self.db, module, local_type_id);
                        infer_local_type(self.db, input)
                    })
                    .unwrap_or(InferredTypeData::Unknown)
            };
        }

        ty
    }
}

use super::{BindingTypeData, InferredModuleTypes, globals::global_type, lookup::module_for_key};
use crate::db::queries::infer_module_types;
use crate::module_graph::ModuleInfo;
use crate::{JsModuleInfo, ModuleDb, ResolvedPath};
use biome_js_semantic::JsDeclarationKind;
use biome_js_type_info::{
    GlobalTypeId, RawTypeData, ResolvedTypeId, ScopeId, TypeId, TypeReference,
    TypeReferenceQualifier, TypeResolverLevel,
    interned_types::{
        InternedTypeofValue, LocalTypeHandle, LocalTypeId, ModuleKey, TypeData as InferredTypeData,
    },
};
use rustc_hash::{FxHashMap, FxHashSet};
use salsa::plumbing::AsId;

/// Unlike the other limits, this one guards actual stack recursion: each level
/// of `ResolutionCtx::resolve` clones a raw type and runs the conversion walk,
/// so the frames are heavy. Named declarations already short-circuit to
/// `TypeData::Local` handles, which leaves only structural nesting within a
/// single declaration on the stack -- real-world code stays well below this.
const MAX_RAW_TYPE_RESOLUTION_DEPTH: usize = 64;
const MAX_INFERRED_EXPRESSION_WRAPPER_STEPS: usize = 64;
const MAX_LOCAL_TYPE_RESOLUTION_STEPS: usize = 1024;

pub(in crate::db::type_inference) struct ResolutionCtx<'db, 'a> {
    pub(in crate::db::type_inference) db: &'db dyn ModuleDb,
    pub(in crate::db::type_inference) module_key: ModuleKey,
    pub(in crate::db::type_inference) js_info: &'a JsModuleInfo,
    pub(in crate::db::type_inference) import_types:
        &'a FxHashMap<ResolvedPath, &'db InferredModuleTypes<'db>>,
    pub(in crate::db::type_inference) named_type_ids: FxHashSet<TypeId>,
    pub(in crate::db::type_inference) resolved: FxHashMap<TypeId, InferredTypeData<'db>>,
    pub(in crate::db::type_inference) in_progress: FxHashSet<TypeId>,
    pub(in crate::db::type_inference) resolution_depth: usize,
}

pub(in crate::db) fn resolve_raw_types<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    js_info: &JsModuleInfo,
    import_types: &FxHashMap<ResolvedPath, &'db InferredModuleTypes<'db>>,
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
        resolution_depth: 0,
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
    pub(in crate::db::type_inference) fn resolve(
        &mut self,
        reference: &TypeReference,
    ) -> InferredTypeData<'db> {
        if self.resolution_depth >= MAX_RAW_TYPE_RESOLUTION_DEPTH {
            return InferredTypeData::Unknown;
        }

        self.resolution_depth += 1;
        let resolved = match reference {
            TypeReference::Resolved(resolved_id) => self.resolve_resolved_id(*resolved_id),
            TypeReference::Qualifier(qualifier) => self.resolve_qualifier(qualifier),
            TypeReference::Import(import) => self.resolve_import(import),
        };
        self.resolution_depth -= 1;
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

        self.in_progress.remove(&type_id);
        self.resolved.insert(type_id, ty);
        ty
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
                    .and_then(|module| infer_module_types(self.db, module))
                    .and_then(|types| types.types.get(local_type_id.index()).copied())
                    .unwrap_or(InferredTypeData::Unknown)
            };
        }

        ty
    }
}

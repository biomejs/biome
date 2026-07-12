use super::{BindingTypeData, InferredModuleTypes, globals::global_type, lookup::module_for_key};
use crate::db::queries::infer_module_types;
use crate::js_module_info::is_named_type_declaration;
use crate::module_graph::ModuleInfo;
use crate::{JsModuleInfo, ModuleDb};
use biome_js_type_info::{
    RawTypeData, ResolvedTypeId, ScopeId, TypeId, TypeReference, TypeReferenceQualifier,
    TypeResolverLevel,
    resolved::{
        GlobalTypeId, InferredLocalTypeHandle, InferredLocalTypeId, InferredModuleKey,
        InferredTypeData, InternedTypeofValue,
    },
};
use rustc_hash::{FxHashMap, FxHashSet};
use salsa::plumbing::AsId;
use std::{hash::Hash, ops::ControlFlow, sync::Arc};

/// Unlike the other limits, this one guards actual stack recursion: each level
/// of `ResolutionCtx::resolve` clones a raw type and runs the conversion walk,
/// so the frames are heavy. Named declarations already short-circuit to
/// `TypeData::Local` handles, which leaves only structural nesting within a
/// single declaration on the stack -- real-world code stays well below this.
const MAX_RAW_TYPE_RESOLUTION_DEPTH: usize = 64;
const MAX_INFERRED_EXPRESSION_WRAPPER_STEPS: usize = 64;

#[derive(Clone, Copy)]
pub(in crate::db) enum ImportResolution<'a> {
    Full,
    CycleFallback(&'a FxHashSet<ModuleInfo>),
}

pub(in crate::db::type_inference) struct ResolutionCtx<'db, 'a> {
    pub(in crate::db::type_inference) db: &'db dyn ModuleDb,
    pub(in crate::db::type_inference) module_key: InferredModuleKey,
    pub(in crate::db::type_inference) js_info: &'a JsModuleInfo,
    pub(in crate::db::type_inference) import_resolution: ImportResolution<'a>,
    pub(in crate::db::type_inference) named_type_ids: FxHashSet<TypeId>,
    pub(in crate::db::type_inference) resolved: FxHashMap<TypeId, InferredTypeData<'db>>,
    pub(in crate::db::type_inference) in_progress: FxHashSet<TypeId>,
    pub(in crate::db::type_inference) resolution_depth: usize,
}

pub(in crate::db) fn resolve_raw_types<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    js_info: &JsModuleInfo,
    import_resolution: ImportResolution<'_>,
) -> InferredModuleTypes<'db> {
    let module_key = InferredModuleKey::new(module.as_id());
    let named_type_ids = named_type_ids(js_info);
    let mut ctx = ResolutionCtx {
        db,
        module_key,
        js_info,
        import_resolution,
        named_type_ids,
        resolved: FxHashMap::default(),
        in_progress: FxHashSet::default(),
        resolution_depth: 0,
    };

    let mut named_type_ids = ctx
        .named_type_ids
        .iter()
        .map(|type_id| InferredLocalTypeId::new(type_id.index()))
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

impl<'db> ResolutionCtx<'db, '_> {
    pub(in crate::db::type_inference) fn infer_imported_module(
        &self,
        module: ModuleInfo,
    ) -> Option<Arc<InferredModuleTypes<'db>>> {
        match self.import_resolution {
            ImportResolution::Full => infer_module_types(self.db, module),
            ImportResolution::CycleFallback(blocked) => {
                if blocked.contains(&module) {
                    None
                } else {
                    infer_module_types(self.db, module)
                }
            }
        }
    }

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
            TypeResolverLevel::Global => self.resolve_global_type_id(resolved_id.id()),
            TypeResolverLevel::Full | TypeResolverLevel::Import => InferredTypeData::Unknown,
        }
    }

    fn resolve_global_type_id(&mut self, type_id: TypeId) -> InferredTypeData<'db> {
        GlobalTypeId::try_from_type_id(type_id).map_or_else(
            || {
                debug_assert!(
                    false,
                    "global resolved TypeId must index the predefined global manifest"
                );
                InferredTypeData::Unknown
            },
            |type_id| global_type(self.db, type_id),
        )
    }

    fn resolve_raw_type_reference(&mut self, type_id: TypeId) -> InferredTypeData<'db> {
        if self.named_type_ids.contains(&type_id) {
            return self.local_type(type_id);
        }

        self.resolve_raw_type_id(type_id)
    }

    fn local_type(&self, type_id: TypeId) -> InferredTypeData<'db> {
        InferredTypeData::Local(InferredLocalTypeHandle::new(
            self.db,
            self.module_key,
            InferredLocalTypeId::new(type_id.index()),
        ))
    }

    fn resolve_raw_type_id(&mut self, type_id: TypeId) -> InferredTypeData<'db> {
        if let Some(ty) = self.resolved.get(&type_id) {
            return *ty;
        }

        if !self.in_progress.insert(type_id) {
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
        let ty = InferredTypeData::from_raw_with_resolver(db, raw, &mut |reference| {
            self.resolve(reference)
        });
        self.resolve_inferred_expression_wrappers(ty)
    }

    fn resolve_inferred_expression_wrappers(
        &mut self,
        ty: InferredTypeData<'db>,
    ) -> InferredTypeData<'db> {
        walk_wrapper_chain(ty, InferredTypeData::Unknown, |ty| match ty {
            InferredTypeData::TypeofExpression(expression) => ControlFlow::Continue(
                self.resolve_inferred_typeof_expression(expression.expression(self.db))
                    .unwrap_or(InferredTypeData::Unknown),
            ),
            InferredTypeData::InstanceOf(instance) => {
                let target = instance.ty(self.db);
                let InferredTypeData::TypeofExpression(expression) = target else {
                    return ControlFlow::Break(ty);
                };
                let target = self
                    .resolve_inferred_typeof_expression(expression.expression(self.db))
                    .unwrap_or(InferredTypeData::Unknown);
                if target.should_flatten_instance(instance.type_parameters(self.db)) {
                    ControlFlow::Continue(target)
                } else {
                    ControlFlow::Break(InferredTypeData::instance_of(
                        self.db,
                        target,
                        instance
                            .type_parameters(self.db)
                            .to_vec()
                            .into_boxed_slice(),
                    ))
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
            | InferredTypeData::VoidKeyword => ControlFlow::Break(ty),
        })
    }

    pub(in crate::db::type_inference) fn resolve_inferred_type(
        &mut self,
        mut ty: InferredTypeData<'db>,
    ) -> InferredTypeData<'db> {
        let mut seen = FxHashSet::default();

        loop {
            let InferredTypeData::Local(local) = ty else {
                return ty;
            };

            let module_key = local.module(self.db);
            let local_type_id = local.type_id(self.db);
            if !seen.insert((module_key, local_type_id)) {
                return InferredTypeData::Unknown;
            }
            if seen.len() % 256 == 0 {
                self.db.unwind_if_revision_cancelled();
            }

            ty = if module_key == self.module_key {
                self.resolve_raw_type_id(TypeId::new(local_type_id.index()))
            } else {
                module_for_key(self.db, module_key)
                    .and_then(|module| self.infer_imported_module(module))
                    .and_then(|types| types.types.get(local_type_id.index()).copied())
                    .unwrap_or(InferredTypeData::Unknown)
            };
        }
    }
}

fn walk_wrapper_chain<T: Copy + Eq + Hash>(
    mut value: T,
    fallback: T,
    mut step: impl FnMut(T) -> ControlFlow<T, T>,
) -> T {
    let mut seen = FxHashSet::default();
    for _ in 0..MAX_INFERRED_EXPRESSION_WRAPPER_STEPS {
        if !seen.insert(value) {
            return fallback;
        }
        match step(value) {
            ControlFlow::Break(result) => return result,
            ControlFlow::Continue(next) => value = next,
        }
    }
    fallback
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inferred_expression_wrapper_cycle_returns_fallback() {
        let result = walk_wrapper_chain(0, usize::MAX, |value| {
            ControlFlow::Continue((value + 1) % 2)
        });
        assert_eq!(result, usize::MAX);
    }

    #[test]
    fn inferred_expression_wrapper_exhaustion_returns_fallback() {
        let result = walk_wrapper_chain(0, usize::MAX, |value| ControlFlow::Continue(value + 1));
        assert_eq!(result, usize::MAX);
    }
}

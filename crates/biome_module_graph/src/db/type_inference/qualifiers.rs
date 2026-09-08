use super::{
    ImportResolution, imports::MAX_NAMESPACE_IMPORT_MEMBER_STEPS, resolver::ResolutionCtx,
};
use crate::db::queries::inference_module_sccs;
use crate::{
    ModuleGraphGeneration, js_module_info::TsBindingReferenceExt, module_for_key,
    module_graph::ModuleInfoKind,
};
use biome_js_type_info::{
    Path, TypeImportQualifier, TypeReference, TypeReferenceQualifier, TypeResolverLevel,
    global_type_id_for_qualifier,
    interned_types::{
        Literal as InferredLiteral, LocalTypeHandle, LocalTypeId, TypeData as InferredTypeData,
        TypeMember as InferredTypeMember, TypeMemberKind as InferredTypeMemberKind,
        well_known_symbol_type,
    },
};
use biome_rowan::Text;

const MAX_SCOPE_RESOLUTION_STEPS: usize = 1024;
const MAX_LOCAL_TYPE_RESOLUTION_STEPS: usize = 1024;

impl<'db> ResolutionCtx<'db, '_> {
    /// Resolves a qualifier that names a binding whose type is still being
    /// inferred to a local type handle.
    ///
    /// Only single-identifier qualifiers match; the identifier's binding is
    /// searched upward through the scope chain. Returns `None` when the
    /// binding is missing, its type reference is not a thin resolved ID, or
    /// the referenced type is not currently in progress. The search examines
    /// at most 1024 scopes and also returns `None` if it cannot reach the root
    /// scope within that limit.
    pub(super) fn resolve_in_progress_this_qualifier(
        &self,
        qualifier: &TypeReferenceQualifier,
    ) -> Option<InferredTypeData<'db>> {
        let mut path = qualifier.path.iter();
        let identifier = path.next()?;
        if path.next().is_some() {
            return None;
        }

        let mut scope = self
            .js_info
            .semantic_model
            .scope_from_id(qualifier.scope_id);
        for _ in 0..MAX_SCOPE_RESOLUTION_STEPS {
            let binding = scope
                .get_binding_reference(identifier.text())
                .and_then(|reference| reference.get_binding_id_for_qualifier(qualifier))
                .and_then(|id| self.js_info.semantic_model.binding_by_id(id));
            if let Some(binding) = binding {
                let TypeReference::Resolved(resolved_id) = self
                    .js_info
                    .raw_binding_types
                    .get(&binding.syntax().text_trimmed_range())?
                else {
                    return None;
                };
                if resolved_id.level() != TypeResolverLevel::Thin
                    || !self.in_progress.contains(&resolved_id.id())
                {
                    return None;
                }
                return Some(InferredTypeData::Local(LocalTypeHandle::new(
                    self.db,
                    self.module_key,
                    LocalTypeId::new(resolved_id.id().index()),
                )));
            }
            scope = scope.parent()?;
        }
        None
    }

    /// Resolves a scoped name such as `ns.Widget<T>`.
    ///
    /// The first path segment is searched from `qualifier.scope_id` toward the
    /// root scope. When it names a binding, the remaining segments are looked
    /// up as static members and the supplied type arguments are applied to the
    /// result. Built-in utility types and global names are considered only
    /// after the search reaches the root without finding a binding.
    ///
    /// At most 1024 scopes are examined. If the root is deeper than that, the
    /// result is `Unknown`; global fallback is not attempted because an unseen
    /// lexical binding could shadow the global name.
    pub(in crate::db::type_inference) fn resolve_qualifier(
        &mut self,
        qualifier: &TypeReferenceQualifier,
    ) -> InferredTypeData<'db> {
        let mut path = qualifier.path.iter();
        let Some(identifier) = path.next() else {
            return InferredTypeData::Unknown;
        };
        let members = path.collect::<Vec<_>>();

        let mut scope = self
            .js_info
            .semantic_model
            .scope_from_id(qualifier.scope_id);
        let mut reached_root_scope = false;
        for _ in 0..MAX_SCOPE_RESOLUTION_STEPS {
            let binding = scope
                .get_binding_reference(identifier.text())
                .and_then(|reference| reference.get_binding_id_for_qualifier(qualifier))
                .and_then(|id| self.js_info.semantic_model.binding_by_id(id));
            if let Some(binding) = binding {
                let binding_is_imported = binding.is_imported();
                let resolves_declarations_directly = self.resolves_declarations_directly();
                // Project the selected namespace member before resolving its
                // base. Building the complete namespace here would add
                // unrelated exports to the declaration graph and could turn
                // an acyclic lookup into a dependency cycle.
                let projected_member = self.import_resolution.is_on_demand().then(|| {
                    members.first().and_then(|member| {
                        self.resolve_namespace_import_member(
                            &TypeReference::Qualifier(Box::new(TypeReferenceQualifier {
                                path: Path::from(identifier.clone()),
                                type_parameters: Box::default(),
                                scope_id: qualifier.scope_id,
                                type_only: qualifier.type_only,
                                excluded_binding_id: qualifier.excluded_binding_id,
                            })),
                            member,
                        )
                    })
                });
                let projected_member = projected_member.flatten();
                let consumed_first_member = projected_member.is_some();
                let mut target = if let Some(projected_member) = projected_member {
                    projected_member
                } else if binding_is_imported
                    && let Some(import) = self.js_info.static_imports.get(identifier.text())
                {
                    self.resolve_import(&TypeImportQualifier {
                        symbol: import.symbol.clone(),
                        resolved_path: import.resolved_path.clone(),
                        type_only: qualifier.type_only,
                    })
                } else if resolves_declarations_directly {
                    self.resolve_local_binding(binding.syntax().text_trimmed_range())
                } else {
                    self.js_info
                        .raw_binding_types
                        .get(&binding.syntax().text_trimmed_range())
                        .cloned()
                        .map_or(InferredTypeData::Unknown, |reference| {
                            self.resolve(&reference)
                        })
                };

                for member in members.iter().skip(usize::from(consumed_first_member)) {
                    let Some(member_ty) =
                        self.resolve_static_member_expression(target, member.text())
                    else {
                        return InferredTypeData::Unknown;
                    };
                    target = member_ty;
                }

                return self.apply_qualifier_type_parameters(target, qualifier);
            }

            match scope.parent() {
                Some(parent) => scope = parent,
                None => {
                    reached_root_scope = true;
                    break;
                }
            }
        }

        if !reached_root_scope {
            return InferredTypeData::Unknown;
        }

        if qualifier.is_record() && qualifier.type_parameters.len() == 2 {
            let key_ty = self.resolve(&qualifier.type_parameters[0]);
            let value_ty = self.resolve(&qualifier.type_parameters[1]);
            return InferredTypeData::object_from_members(
                self.db,
                Vec::from([InferredTypeMember {
                    kind: InferredTypeMemberKind::IndexSignature(key_ty),
                    ty: value_ty,
                }]),
            );
        }

        if (qualifier.is_pick() || qualifier.is_omit()) && qualifier.type_parameters.len() == 2 {
            return self.resolve_pick_or_omit(qualifier);
        }

        if (qualifier.is_partial() || qualifier.is_required())
            && qualifier.type_parameters.len() == 1
        {
            return self.resolve_partial_or_required(qualifier);
        }

        if qualifier.is_readonly() && qualifier.type_parameters.len() == 1 {
            return self.resolve_readonly(qualifier);
        }

        if qualifier.is_array() && qualifier.has_known_type_parameters() {
            return InferredTypeData::array_instance(
                self.db,
                qualifier
                    .type_parameters
                    .iter()
                    .map(|parameter| self.resolve(parameter))
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            );
        }

        if qualifier.is_map() && qualifier.has_known_type_parameters() {
            return InferredTypeData::map_instance(
                self.db,
                qualifier
                    .type_parameters
                    .iter()
                    .map(|parameter| self.resolve(parameter))
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            );
        }

        if qualifier.is_promise() && qualifier.has_known_type_parameters() {
            return InferredTypeData::promise_instance(
                self.db,
                qualifier
                    .type_parameters
                    .iter()
                    .map(|parameter| self.resolve(parameter))
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            );
        }

        if qualifier.is_set() && qualifier.has_known_type_parameters() {
            return InferredTypeData::set_instance(
                self.db,
                qualifier
                    .type_parameters
                    .iter()
                    .map(|parameter| self.resolve(parameter))
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            );
        }

        if qualifier.is_weak_map() && qualifier.has_known_type_parameters() {
            return InferredTypeData::weak_map_instance(
                self.db,
                qualifier
                    .type_parameters
                    .iter()
                    .map(|parameter| self.resolve(parameter))
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            );
        }

        if let Some(ty) = self.resolve_global_member_qualifier(qualifier) {
            return ty;
        }

        if let Some(id) = global_type_id_for_qualifier(qualifier) {
            return super::globals::global_type(self.db, id);
        }

        InferredTypeData::Unknown
    }

    /// Projects one member from a reference that leads to a namespace import.
    ///
    /// On-demand resolution uses this path to avoid resolving every
    /// export on the namespace. Returns `None` when the reference does not lead
    /// to a supported namespace import or the bounded projection cannot finish.
    pub(super) fn resolve_namespace_import_member(
        &mut self,
        reference: &TypeReference,
        member: &Text,
    ) -> Option<InferredTypeData<'db>> {
        self.resolve_namespace_import_member_with_steps(
            reference,
            member,
            MAX_NAMESPACE_IMPORT_MEMBER_STEPS,
        )
    }

    /// Follows local aliases and qualifiers until it reaches the import that
    /// supplies `member`.
    ///
    /// Each iteration through the `TypeReference` chain consumes one step.
    /// Entering `resolve_import_member_with_steps` consumes another step for
    /// the cross-module projection. Walking parent scopes has a separate limit
    /// and does not consume this budget.
    pub(super) fn resolve_namespace_import_member_with_steps(
        &mut self,
        reference: &TypeReference,
        member: &Text,
        mut remaining_projection_steps: usize,
    ) -> Option<InferredTypeData<'db>> {
        let mut reference = reference.clone();
        while remaining_projection_steps > 0 {
            remaining_projection_steps -= 1;
            if let TypeReference::Import(import) = &reference {
                return self.resolve_import_member_with_steps(
                    import,
                    member,
                    remaining_projection_steps,
                );
            }

            if let TypeReference::Resolved(resolved_id) = &reference {
                if resolved_id.level() != TypeResolverLevel::Thin {
                    return None;
                }
                let raw = self.js_info.raw_types.get(resolved_id.id().index())?;
                if let biome_js_type_info::RawTypeData::Reference(next) = raw {
                    reference = next.clone();
                    continue;
                }
                if let biome_js_type_info::RawTypeData::TypeofValue(value) = raw
                    && value.ty.is_unknown()
                {
                    reference =
                        TypeReference::Qualifier(Box::new(TypeReferenceQualifier::from_path(
                            value.scope_id.unwrap_or(biome_js_semantic::ScopeId::GLOBAL),
                            value.identifier.clone(),
                        )));
                    continue;
                }
                return None;
            }

            let TypeReference::Qualifier(qualifier) = &reference else {
                return None;
            };
            let identifier = qualifier.path.identifier()?;
            let mut scope = self
                .js_info
                .semantic_model
                .scope_from_id(qualifier.scope_id);
            let mut next = None;
            for _ in 0..MAX_SCOPE_RESOLUTION_STEPS {
                let binding = scope
                    .get_binding_reference(identifier.text())
                    .and_then(|binding_reference| {
                        binding_reference.get_binding_id_for_qualifier(qualifier)
                    })
                    .and_then(|id| self.js_info.semantic_model.binding_by_id(id));
                if let Some(binding) = binding {
                    if binding.is_imported() {
                        let import = self.js_info.static_imports.get(identifier.text())?;
                        return self.resolve_import_member_with_steps(
                            &TypeImportQualifier {
                                symbol: import.symbol.clone(),
                                resolved_path: import.resolved_path.clone(),
                                type_only: qualifier.type_only,
                            },
                            member,
                            remaining_projection_steps,
                        );
                    }
                    next = self
                        .js_info
                        .raw_binding_types
                        .get(&binding.syntax().text_trimmed_range())
                        .cloned();
                    break;
                }
                let Some(parent) = scope.parent() else {
                    break;
                };
                scope = parent;
            }
            reference = next?;
        }

        None
    }

    fn resolve_global_member_qualifier(
        &mut self,
        qualifier: &TypeReferenceQualifier,
    ) -> Option<InferredTypeData<'db>> {
        let mut parts = qualifier.path.iter();
        let first = parts.next()?;
        let members = parts.collect::<Vec<_>>();
        if first.text() == "Symbol"
            && let [member] = members.as_slice()
            && let Some(ty) = well_known_symbol_type(member.text())
        {
            return Some(ty);
        }

        let member = members.first()?;
        let mut target = {
            let base = global_type_id_for_qualifier(&TypeReferenceQualifier {
                path: Path::from(first.clone()),
                type_parameters: Box::default(),
                scope_id: qualifier.scope_id,
                type_only: qualifier.type_only,
                excluded_binding_id: qualifier.excluded_binding_id,
            })
            .map(|id| super::globals::global_type(self.db, id))?;
            self.resolve_static_member_expression(base, member.text())
        }?;

        for member in members.iter().skip(1) {
            target = self.resolve_static_member_expression(target, member.text())?;
        }

        Some(self.apply_qualifier_type_parameters(target, qualifier))
    }

    fn apply_qualifier_type_parameters(
        &mut self,
        target: InferredTypeData<'db>,
        qualifier: &TypeReferenceQualifier,
    ) -> InferredTypeData<'db> {
        if qualifier.type_parameters.is_empty() {
            return target;
        }

        let Some(declared_parameters) = self.declared_type_parameters(target) else {
            return target;
        };

        let incoming_parameters = qualifier
            .type_parameters
            .iter()
            .map(|parameter| self.resolve(parameter))
            .collect::<Vec<_>>();
        let merged_parameters = declared_parameters
            .iter()
            .enumerate()
            .map(|(index, parameter)| {
                incoming_parameters
                    .get(index)
                    .copied()
                    .unwrap_or(*parameter)
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();

        InferredTypeData::instance_of(self.db, target, merged_parameters)
    }

    fn declared_type_parameters(
        &mut self,
        target: InferredTypeData<'db>,
    ) -> Option<Box<[InferredTypeData<'db>]>> {
        if let InferredTypeData::Local(local) = target {
            let module_key = local.module(self.db);
            let type_id = local.type_id(self.db);
            if module_key == self.module_key {
                if let Some(parameters) = self
                    .js_info
                    .raw_types
                    .get(type_id.index())
                    .and_then(|raw| raw.type_parameters())
                    .map(<[_]>::to_vec)
                {
                    return Some(
                        parameters
                            .iter()
                            .map(|parameter| self.resolve(parameter))
                            .collect(),
                    );
                }
            } else if let Some(module) = module_for_key(self.db, module_key)
                && let ModuleInfoKind::Js(js_info) = module.kind(self.db)
                && js_info.infer_types
                && let Some(parameters) = js_info
                    .raw_types
                    .get(type_id.index())
                    .and_then(|raw| raw.type_parameters())
                    .map(<[_]>::to_vec)
            {
                let mut ctx = match self.import_resolution {
                    ImportResolution::OnDemand { remaining } => {
                        let resolve_declarations_directly = if self.resolves_declarations_directly()
                        {
                            let sccs =
                                inference_module_sccs(self.db, ModuleGraphGeneration::get(self.db));
                            sccs.contains_cycle_between(self.module, module)
                        } else {
                            false
                        };
                        self.for_on_demand_import(
                            module,
                            &js_info,
                            remaining,
                            resolve_declarations_directly,
                        )
                    }
                    import_resolution @ (ImportResolution::FromTables { .. }
                    | ImportResolution::CycleFallback(_)) => {
                        ResolutionCtx::new(self.db, module, &js_info, import_resolution)
                    }
                };
                return Some(
                    parameters
                        .iter()
                        .map(|parameter| ctx.resolve(parameter))
                        .collect(),
                );
            }
        }

        match self.resolve_inferred_type(target) {
            InferredTypeData::Class(class) => Some(class.type_parameters(self.db).to_vec().into()),
            InferredTypeData::Function(function) => {
                Some(function.type_parameters(self.db).to_vec().into())
            }
            InferredTypeData::InstanceOf(instance) => {
                Some(instance.type_parameters(self.db).to_vec().into())
            }
            InferredTypeData::Interface(interface) => {
                Some(interface.type_parameters(self.db).to_vec().into())
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
            | InferredTypeData::Constructor(_)
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
            | InferredTypeData::TypeofExpression(_)
            | InferredTypeData::TypeofType(_)
            | InferredTypeData::TypeofValue(_)
            | InferredTypeData::AnyKeyword
            | InferredTypeData::NeverKeyword
            | InferredTypeData::ObjectKeyword
            | InferredTypeData::ThisKeyword
            | InferredTypeData::UnknownKeyword
            | InferredTypeData::VoidKeyword => None,
        }
    }

    fn resolve_pick_or_omit(
        &mut self,
        qualifier: &TypeReferenceQualifier,
    ) -> InferredTypeData<'db> {
        let target_ty = self.resolve(&qualifier.type_parameters[0]);
        let key_ty = self.resolve(&qualifier.type_parameters[1]);
        let Some(key_names) = self.string_literal_keys(key_ty) else {
            return InferredTypeData::Unknown;
        };
        let Some(members) = self.own_members(target_ty) else {
            return InferredTypeData::Unknown;
        };

        if qualifier.is_pick() {
            InferredTypeData::pick_members(self.db, members, &key_names)
        } else {
            InferredTypeData::omit_members(self.db, members, &key_names)
        }
    }

    fn resolve_partial_or_required(
        &mut self,
        qualifier: &TypeReferenceQualifier,
    ) -> InferredTypeData<'db> {
        let target_ty = self.resolve(&qualifier.type_parameters[0]);
        let Some(members) = self.own_members(target_ty) else {
            return InferredTypeData::Unknown;
        };

        if qualifier.is_partial() {
            InferredTypeData::with_all_optional_members(self.db, members)
        } else {
            InferredTypeData::with_all_required_members(self.db, members)
        }
    }

    fn resolve_readonly(&mut self, qualifier: &TypeReferenceQualifier) -> InferredTypeData<'db> {
        let target_ty = self.resolve(&qualifier.type_parameters[0]);
        self.own_members(target_ty)
            .map_or(InferredTypeData::Unknown, |members| {
                InferredTypeData::object_from_members(self.db, members)
            })
    }

    fn own_members(&mut self, ty: InferredTypeData<'db>) -> Option<Vec<InferredTypeMember<'db>>> {
        let mut ty = ty;

        for _ in 0..MAX_LOCAL_TYPE_RESOLUTION_STEPS {
            match self.resolve_inferred_type(ty) {
                InferredTypeData::Class(class) => return Some(class.members(self.db).to_vec()),
                InferredTypeData::Interface(interface) => {
                    return Some(interface.members(self.db).to_vec());
                }
                InferredTypeData::InstanceOf(instance) => ty = instance.ty(self.db),
                InferredTypeData::Literal(literal) => match literal.literal(self.db) {
                    InferredLiteral::Object(members) => return Some(members.to_vec()),
                    InferredLiteral::BigInt(_)
                    | InferredLiteral::Boolean(_)
                    | InferredLiteral::Number(_)
                    | InferredLiteral::RegExp(_)
                    | InferredLiteral::String(_)
                    | InferredLiteral::Template(_) => return None,
                },
                InferredTypeData::Module(module) => return Some(module.members(self.db).to_vec()),
                InferredTypeData::Namespace(namespace) => {
                    return Some(namespace.members(self.db).to_vec());
                }
                InferredTypeData::Object(object) => return Some(object.members(self.db).to_vec()),
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
                | InferredTypeData::Constructor(_)
                | InferredTypeData::Function(_)
                | InferredTypeData::Tuple(_)
                | InferredTypeData::Generic(_)
                | InferredTypeData::Local(_)
                | InferredTypeData::Intersection(_)
                | InferredTypeData::Union(_)
                | InferredTypeData::TypeOperator(_)
                | InferredTypeData::MergedReference(_)
                | InferredTypeData::TypeofExpression(_)
                | InferredTypeData::TypeofType(_)
                | InferredTypeData::TypeofValue(_)
                | InferredTypeData::AnyKeyword
                | InferredTypeData::NeverKeyword
                | InferredTypeData::ObjectKeyword
                | InferredTypeData::ThisKeyword
                | InferredTypeData::UnknownKeyword
                | InferredTypeData::VoidKeyword => return None,
            }
        }

        None
    }

    fn string_literal_keys(&mut self, ty: InferredTypeData<'db>) -> Option<Vec<Text>> {
        match self.resolve_inferred_type(ty) {
            InferredTypeData::Literal(literal) => match literal.literal(self.db) {
                InferredLiteral::String(value) => Some(vec![value.as_ref().clone()]),
                InferredLiteral::BigInt(_)
                | InferredLiteral::Boolean(_)
                | InferredLiteral::Number(_)
                | InferredLiteral::Object(_)
                | InferredLiteral::RegExp(_)
                | InferredLiteral::Template(_) => None,
            },
            InferredTypeData::Union(union) => Some(
                union
                    .types(self.db)
                    .to_vec()
                    .into_iter()
                    .filter_map(|ty| self.string_literal_key(ty))
                    .collect(),
            ),
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
            | InferredTypeData::TypeOperator(_)
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
            | InferredTypeData::VoidKeyword => None,
        }
    }

    fn string_literal_key(&mut self, ty: InferredTypeData<'db>) -> Option<Text> {
        match self.resolve_inferred_type(ty) {
            InferredTypeData::Literal(literal) => match literal.literal(self.db) {
                InferredLiteral::String(value) => Some(value.as_ref().clone()),
                InferredLiteral::BigInt(_)
                | InferredLiteral::Boolean(_)
                | InferredLiteral::Number(_)
                | InferredLiteral::Object(_)
                | InferredLiteral::RegExp(_)
                | InferredLiteral::Template(_) => None,
            },
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
            | InferredTypeData::VoidKeyword => None,
        }
    }
}

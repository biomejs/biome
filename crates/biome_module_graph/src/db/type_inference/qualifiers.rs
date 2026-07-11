use super::{lookup::declared_type_parameters, resolver::ResolutionCtx};
use crate::js_module_info::TsBindingReferenceExt;
use biome_js_type_info::{
    GLOBAL_RESOLVER, Path, TypeImportQualifier, TypeReferenceQualifier, TypeResolver,
    resolved::{InferredLiteral, InferredTypeData, InferredTypeMember, InferredTypeMemberKind},
};
use biome_rowan::Text;

const MAX_SCOPE_RESOLUTION_STEPS: usize = 1024;
const MAX_LOCAL_TYPE_RESOLUTION_STEPS: usize = 1024;

impl<'db> ResolutionCtx<'db, '_> {
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
                let mut target = if binding.is_imported()
                    && let Some(import) = self.js_info.static_imports.get(identifier.text())
                {
                    self.resolve_import(&TypeImportQualifier {
                        symbol: import.symbol.clone(),
                        resolved_path: import.resolved_path.clone(),
                        type_only: qualifier.type_only,
                    })
                } else {
                    self.js_info
                        .raw_binding_types
                        .get(&binding.syntax().text_trimmed_range())
                        .cloned()
                        .map_or(InferredTypeData::Unknown, |reference| {
                            self.resolve(&reference)
                        })
                };

                for member in &members {
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

        if let Some(resolved_id) = GLOBAL_RESOLVER.resolve_qualifier(qualifier) {
            return self.resolve_resolved_id(resolved_id);
        }

        InferredTypeData::Unknown
    }

    fn resolve_global_member_qualifier(
        &mut self,
        qualifier: &TypeReferenceQualifier,
    ) -> Option<InferredTypeData<'db>> {
        let mut parts = qualifier.path.iter();
        let first = parts.next()?;
        let mut target = parts.next().and_then(|member| {
            let base = GLOBAL_RESOLVER
                .resolve_qualifier(&TypeReferenceQualifier {
                    path: Path::from(first.clone()),
                    type_parameters: Box::default(),
                    scope_id: qualifier.scope_id,
                    type_only: qualifier.type_only,
                    excluded_binding_id: qualifier.excluded_binding_id,
                })
                .map(|resolved_id| self.resolve_resolved_id(resolved_id))?;
            self.resolve_static_member_expression(base, member.text())
        })?;

        for member in parts {
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

        let declared_target = self.resolve_inferred_type(target);
        let Some(declared_parameters) = declared_type_parameters(self.db, declared_target) else {
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

use super::{InferredModuleTypes, collected_type_result, expand_canonical_global};
use crate::ModuleDb;
use crate::db::queries::infer_module_types_query;
use crate::module_graph::ModuleInfo;
use biome_js_type_info::resolved::{
    InferredLiteral, InferredLocalTypeHandle, InferredModuleKey, InferredReturnType,
    InferredTypeData, InferredTypeMember, InferredTypeMemberKind, InferredTypeSubstitution,
    StructuralMapError,
};
use rustc_hash::FxHashSet;
use salsa::plumbing::{AsId, FromId};
use std::rc::Rc;

const MAX_MEMBER_LOOKUP_STEPS: usize = 1024;

impl<'db> InferredModuleTypes<'db> {
    pub(in crate::db::type_inference) fn resolve_type_iterative(
        &self,
        db: &'db dyn ModuleDb,
        mut ty: InferredTypeData<'db>,
    ) -> InferredTypeData<'db> {
        let mut seen = FxHashSet::default();

        loop {
            let InferredTypeData::Local(local) = ty else {
                return ty;
            };

            let module_key = local.module(db);
            let type_id = local.type_id(db);
            if !seen.insert((module_key, type_id)) {
                return InferredTypeData::Unknown;
            }
            if seen.len() % 256 == 0 {
                db.unwind_if_revision_cancelled();
            }

            ty = self
                .type_for_local_handle(db, local)
                .unwrap_or(InferredTypeData::Unknown);
        }
    }

    fn type_for_local_handle(
        &self,
        db: &'db dyn ModuleDb,
        local: InferredLocalTypeHandle<'db>,
    ) -> Option<InferredTypeData<'db>> {
        let module_key = local.module(db);
        let type_id = local.type_id(db);
        if module_key == self.module_key {
            return self.types.get(type_id.index()).copied();
        }

        let module = module_for_key(db, module_key)?;
        infer_module_types_query(db, module)
            .and_then(|types| types.types.get(type_id.index()).copied())
    }

    pub(in crate::db::type_inference) fn find_member_type_iterative(
        &self,
        db: &'db dyn ModuleDb,
        ty: InferredTypeData<'db>,
        name: &str,
    ) -> Option<InferredTypeData<'db>> {
        let mut seen = FxHashSet::default();
        let mut pending = vec![MemberLookupState::new(ty, MemberLookup::Any, false)];
        let mut found = Vec::new();
        let mut remaining_steps = MAX_MEMBER_LOOKUP_STEPS;
        let mut exhausted = false;

        while let Some(mut state) = pending.pop() {
            let lookup = state.lookup;
            let collect = state.collect;
            let Some(ty) = self.resolve_type_for_member_lookup(db, state.ty) else {
                if collect {
                    found.push(InferredTypeData::Unknown);
                    continue;
                }
                return Some(InferredTypeData::Unknown);
            };
            let ty = expand_canonical_global(db, ty);
            let (ty, lookup) = match ty {
                InferredTypeData::InstanceOf(instance) => {
                    let Some(target) = self.resolve_type_for_member_lookup(db, instance.ty(db))
                    else {
                        if collect {
                            found.push(InferredTypeData::Unknown);
                            continue;
                        }
                        return Some(InferredTypeData::Unknown);
                    };
                    let target = expand_canonical_global(db, target);
                    let Ok(substitutions) = substitutions_for_instance(
                        db,
                        target,
                        instance.type_parameters(db),
                        &state.substitutions,
                    ) else {
                        return Some(InferredTypeData::Unknown);
                    };
                    state.substitutions = substitutions.into();
                    (target, MemberLookup::Instance)
                }
                ty @ (InferredTypeData::Unknown
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
                | InferredTypeData::TypeofExpression(_)
                | InferredTypeData::TypeofType(_)
                | InferredTypeData::TypeofValue(_)
                | InferredTypeData::AnyKeyword
                | InferredTypeData::NeverKeyword
                | InferredTypeData::ObjectKeyword
                | InferredTypeData::ThisKeyword
                | InferredTypeData::UnknownKeyword
                | InferredTypeData::VoidKeyword) => (ty, lookup),
            };

            if !seen.insert((ty, lookup, state.substitutions.clone())) {
                continue;
            }

            // Deduplicated entries above don't count against the budget, so
            // the limit measures distinct types visited, not queue churn.
            if remaining_steps == 0 {
                exhausted = true;
                break;
            }
            remaining_steps -= 1;

            if let Some(member_ty) = self.find_own_member_type(db, ty, name, lookup) {
                let Ok(member_ty) = apply_substitutions(db, member_ty, &state.substitutions) else {
                    return Some(InferredTypeData::Unknown);
                };
                let member_ty = if matches!(
                    member_ty,
                    InferredTypeData::Unknown
                        | InferredTypeData::AnyKeyword
                        | InferredTypeData::UnknownKeyword
                ) {
                    InferredTypeData::Unknown
                } else {
                    member_ty
                };
                if collect {
                    found.push(member_ty);
                    continue;
                }
                return Some(member_ty);
            }

            match ty {
                InferredTypeData::Class(class) => {
                    if let Some(mut extends) = class.extends(db) {
                        if pending.len() >= remaining_steps {
                            return Some(InferredTypeData::Unknown);
                        }
                        if matches!(lookup, MemberLookup::Any) {
                            extends = class_side_type(db, extends);
                        }
                        let Ok(extends) = apply_substitutions(db, extends, &state.substitutions)
                        else {
                            return Some(InferredTypeData::Unknown);
                        };
                        pending.push(MemberLookupState {
                            ty: extends,
                            lookup,
                            collect,
                            substitutions: state.substitutions.clone(),
                        });
                    }
                }
                InferredTypeData::Interface(interface) => {
                    if interface.extends(db).len() > remaining_steps.saturating_sub(pending.len()) {
                        return Some(InferredTypeData::Unknown);
                    }
                    for ty in interface.extends(db).iter().rev() {
                        let Ok(ty) = apply_substitutions(db, *ty, &state.substitutions) else {
                            return Some(InferredTypeData::Unknown);
                        };
                        pending.push(MemberLookupState {
                            ty,
                            lookup,
                            collect,
                            substitutions: state.substitutions.clone(),
                        });
                    }
                }
                InferredTypeData::Generic(generic) => {
                    if let Some(constraint) = generic.constraint(db) {
                        if pending.len() >= remaining_steps {
                            return Some(InferredTypeData::Unknown);
                        }
                        let Ok(constraint) =
                            apply_substitutions(db, constraint, &state.substitutions)
                        else {
                            return Some(InferredTypeData::Unknown);
                        };
                        pending.push(MemberLookupState {
                            ty: constraint,
                            lookup,
                            collect,
                            substitutions: state.substitutions.clone(),
                        });
                    }
                }
                InferredTypeData::Intersection(intersection) => {
                    if intersection.types(db).len() > remaining_steps.saturating_sub(pending.len())
                    {
                        return Some(InferredTypeData::Unknown);
                    }
                    for ty in intersection.types(db).iter().rev() {
                        let Ok(ty) = apply_substitutions(db, *ty, &state.substitutions) else {
                            return Some(InferredTypeData::Unknown);
                        };
                        pending.push(MemberLookupState {
                            ty,
                            lookup,
                            collect: true,
                            substitutions: state.substitutions.clone(),
                        });
                    }
                }
                InferredTypeData::MergedReference(reference) => {
                    if reference.targets(db).count() > remaining_steps.saturating_sub(pending.len())
                    {
                        return Some(InferredTypeData::Unknown);
                    }
                    for ty in reference.targets(db) {
                        let Ok(ty) = apply_substitutions(db, ty, &state.substitutions) else {
                            return Some(InferredTypeData::Unknown);
                        };
                        pending.push(MemberLookupState {
                            ty,
                            lookup,
                            collect: true,
                            substitutions: state.substitutions.clone(),
                        });
                    }
                }
                InferredTypeData::Object(object) => {
                    if let Some(prototype) = object.prototype(db) {
                        if pending.len() >= remaining_steps {
                            return Some(InferredTypeData::Unknown);
                        }
                        let Ok(prototype) =
                            apply_substitutions(db, prototype, &state.substitutions)
                        else {
                            return Some(InferredTypeData::Unknown);
                        };
                        pending.push(MemberLookupState {
                            ty: prototype,
                            lookup,
                            collect,
                            substitutions: state.substitutions.clone(),
                        });
                    }
                }
                InferredTypeData::Union(union) => {
                    if union.types(db).len() > remaining_steps.saturating_sub(pending.len()) {
                        return Some(InferredTypeData::Unknown);
                    }
                    for ty in union.types(db).iter().rev() {
                        let Ok(ty) = apply_substitutions(db, *ty, &state.substitutions) else {
                            return Some(InferredTypeData::Unknown);
                        };
                        pending.push(MemberLookupState {
                            ty,
                            lookup,
                            collect: true,
                            substitutions: state.substitutions.clone(),
                        });
                    }
                }
                InferredTypeData::Unknown
                | InferredTypeData::AnyKeyword
                | InferredTypeData::UnknownKeyword => {
                    if collect {
                        found.push(InferredTypeData::Unknown);
                    } else {
                        return Some(InferredTypeData::Unknown);
                    }
                }
                InferredTypeData::Divergent(_)
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
                | InferredTypeData::Module(_)
                | InferredTypeData::Namespace(_)
                | InferredTypeData::Tuple(_)
                | InferredTypeData::Local(_)
                | InferredTypeData::TypeOperator(_)
                | InferredTypeData::Literal(_)
                | InferredTypeData::InstanceOf(_)
                | InferredTypeData::TypeofExpression(_)
                | InferredTypeData::TypeofType(_)
                | InferredTypeData::TypeofValue(_)
                | InferredTypeData::NeverKeyword
                | InferredTypeData::ObjectKeyword
                | InferredTypeData::ThisKeyword
                | InferredTypeData::VoidKeyword => {}
            }
        }

        if exhausted || found.contains(&InferredTypeData::Unknown) {
            Some(InferredTypeData::Unknown)
        } else {
            collected_type_result(db, found)
        }
    }

    fn resolve_type_for_member_lookup(
        &self,
        db: &'db dyn ModuleDb,
        mut ty: InferredTypeData<'db>,
    ) -> Option<InferredTypeData<'db>> {
        let mut seen = FxHashSet::default();

        loop {
            let InferredTypeData::Local(local) = ty else {
                return Some(ty);
            };

            let key = (local.module(db), local.type_id(db));
            if !seen.insert(key) {
                return None;
            }
            if seen.len() % 256 == 0 {
                db.unwind_if_revision_cancelled();
            }
            ty = self.type_for_local_handle(db, local)?;
        }
    }

    fn find_own_member_type(
        &self,
        db: &'db dyn ModuleDb,
        ty: InferredTypeData<'db>,
        name: &str,
        lookup: MemberLookup,
    ) -> Option<InferredTypeData<'db>> {
        match ty {
            InferredTypeData::Class(class) => find_member_type(
                db,
                class.members(db),
                name,
                lookup,
                matches!(lookup, MemberLookup::Instance),
            ),
            InferredTypeData::Interface(interface) => {
                find_member_type(db, interface.members(db), name, lookup, true)
            }
            InferredTypeData::Literal(literal) => match literal.literal(db) {
                InferredLiteral::Object(members) => {
                    find_member_type(db, members, name, lookup, true)
                }
                InferredLiteral::BigInt(_)
                | InferredLiteral::Boolean(_)
                | InferredLiteral::Number(_)
                | InferredLiteral::RegExp(_)
                | InferredLiteral::String(_)
                | InferredLiteral::Template(_) => None,
            },
            InferredTypeData::Module(module) => {
                find_member_type(db, module.members(db), name, lookup, true)
            }
            InferredTypeData::Namespace(namespace) => {
                find_member_type(db, namespace.members(db), name, lookup, true)
            }
            InferredTypeData::Object(object) => {
                find_member_type(db, object.members(db), name, lookup, true)
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
            | InferredTypeData::Constructor(_)
            | InferredTypeData::Function(_)
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

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum MemberLookup {
    Any,
    Instance,
}

#[derive(Clone, Copy)]
pub(in crate::db::type_inference) enum StaticMemberMode {
    Class,
    Instance,
}

impl StaticMemberMode {
    fn allows_named_member(self, kind: &InferredTypeMemberKind<'_>) -> bool {
        match self {
            Self::Class => kind.is_static() && !kind.is_constructor(),
            Self::Instance => !kind.is_static(),
        }
    }

    fn allows_index_signature(self) -> bool {
        matches!(self, Self::Instance)
    }
}

#[derive(Clone)]
struct MemberLookupState<'db> {
    ty: InferredTypeData<'db>,
    lookup: MemberLookup,
    collect: bool,
    substitutions: Rc<[InferredTypeSubstitution<'db>]>,
}

impl<'db> MemberLookupState<'db> {
    fn new(ty: InferredTypeData<'db>, lookup: MemberLookup, collect: bool) -> Self {
        Self {
            ty,
            lookup,
            collect,
            substitutions: Rc::default(),
        }
    }
}

pub(in crate::db) fn substitutions_for_instance<'db>(
    db: &'db dyn ModuleDb,
    target: InferredTypeData<'db>,
    type_parameters: &[InferredTypeData<'db>],
    inherited: &[InferredTypeSubstitution<'db>],
) -> Result<Vec<InferredTypeSubstitution<'db>>, StructuralMapError> {
    let Some(declared_parameters) = declared_type_parameters(db, target) else {
        return Ok(inherited.to_vec());
    };
    if declared_parameters.is_empty() {
        return Ok(inherited.to_vec());
    }

    let mut substitutions = inherited.to_vec();
    for (declared, replacement) in declared_parameters.iter().zip(type_parameters) {
        let declared = apply_substitutions(db, *declared, inherited)?;
        let replacement = apply_substitutions(db, *replacement, inherited)?;
        let declared_instance = InferredTypeData::instance_of(db, declared, Box::default());
        if declared_instance != declared {
            substitutions.push(InferredTypeSubstitution {
                generic: declared_instance,
                replacement,
            });
        }
        substitutions.push(InferredTypeSubstitution {
            generic: declared,
            replacement,
        });
    }

    Ok(substitutions)
}

pub(super) fn declared_type_parameters<'db>(
    db: &'db dyn ModuleDb,
    target: InferredTypeData<'db>,
) -> Option<&'db [InferredTypeData<'db>]> {
    match target {
        InferredTypeData::Class(class) => Some(class.type_parameters(db)),
        InferredTypeData::Function(function) => Some(function.type_parameters(db)),
        InferredTypeData::InstanceOf(instance) => Some(instance.type_parameters(db)),
        InferredTypeData::Interface(interface) => Some(interface.type_parameters(db)),
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

pub(in crate::db) fn apply_substitutions<'db>(
    db: &'db dyn ModuleDb,
    mut ty: InferredTypeData<'db>,
    substitutions: &[InferredTypeSubstitution<'db>],
) -> Result<InferredTypeData<'db>, StructuralMapError> {
    for substitution in substitutions {
        ty = ty.substitute_type(db, *substitution)?;
    }
    Ok(ty)
}

pub(in crate::db) fn apply_substitutions_to_root_body<'db>(
    db: &'db dyn ModuleDb,
    mut ty: InferredTypeData<'db>,
    substitutions: &[InferredTypeSubstitution<'db>],
) -> Result<InferredTypeData<'db>, StructuralMapError> {
    for substitution in substitutions {
        ty = ty.substitute_type_in_root_body(db, *substitution)?;
    }
    Ok(ty)
}

pub(super) fn class_side_type<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
) -> InferredTypeData<'db> {
    match ty {
        InferredTypeData::InstanceOf(instance) => instance.ty(db),
        ty @ (InferredTypeData::Unknown
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
        | InferredTypeData::TypeofExpression(_)
        | InferredTypeData::TypeofType(_)
        | InferredTypeData::TypeofValue(_)
        | InferredTypeData::AnyKeyword
        | InferredTypeData::NeverKeyword
        | InferredTypeData::ObjectKeyword
        | InferredTypeData::ThisKeyword
        | InferredTypeData::UnknownKeyword
        | InferredTypeData::VoidKeyword) => ty,
    }
}

pub(in crate::db::type_inference) fn module_for_key(
    db: &dyn ModuleDb,
    module_key: InferredModuleKey,
) -> Option<ModuleInfo> {
    let module = ModuleInfo::from_id(module_key.as_id());
    let current = db.module_for_path(module.path(db))?;
    (InferredModuleKey::new(current.as_id()) == module_key).then_some(current)
}

fn find_member_type<'db>(
    db: &'db dyn ModuleDb,
    members: &[InferredTypeMember<'db>],
    name: &str,
    lookup: MemberLookup,
    allow_index_signature: bool,
) -> Option<InferredTypeData<'db>> {
    find_member_in_members(
        db,
        members,
        name,
        |kind| !(matches!(lookup, MemberLookup::Instance) && kind.is_static()),
        allow_index_signature,
    )
    .map(|(ty, _)| ty)
}

fn find_member_in_members<'db>(
    db: &'db dyn ModuleDb,
    members: &[InferredTypeMember<'db>],
    name: &str,
    allows_named_member: impl Fn(&InferredTypeMemberKind<'db>) -> bool,
    allow_index_signature: bool,
) -> Option<(InferredTypeData<'db>, bool)> {
    let named_member = members
        .iter()
        .find(|member| allows_named_member(&member.kind) && member.kind.has_name(name))
        .map(|member| (member_value_type(db, member), member.kind.is_optional()));
    if named_member.is_some() {
        return named_member;
    }

    let computed_member = members.iter().find_map(|member| {
        member
            .kind
            .computed_value_type()
            .is_some_and(|ty| {
                ty.is_string_literal_key(db, name)
                    || allow_index_signature && ty.is_string_key_type(db)
            })
            .then_some((member_value_type(db, member), false))
    });
    if computed_member.is_some() {
        return computed_member;
    }

    allow_index_signature.then(|| {
        members.iter().find_map(|member| {
            member
                .kind
                .index_signature_type()
                .is_some_and(|ty| ty.is_string_key_type(db) || ty.is_string_literal_key(db, name))
                .then_some((member.ty, false))
        })
    })?
}

fn member_value_type<'db>(
    db: &'db dyn ModuleDb,
    member: &InferredTypeMember<'db>,
) -> InferredTypeData<'db> {
    if matches!(
        member.kind,
        InferredTypeMemberKind::Getter(_) | InferredTypeMemberKind::ConstAssertedGetter(_)
    ) && let InferredTypeData::Function(function) = member.ty
        && let InferredReturnType::Type(return_ty) = function.return_type(db)
    {
        *return_ty
    } else {
        member.ty
    }
}

pub(in crate::db::type_inference) fn find_member_in_members_for_mode<'db>(
    db: &'db dyn ModuleDb,
    members: &[InferredTypeMember<'db>],
    name: &str,
    mode: StaticMemberMode,
) -> Option<(InferredTypeData<'db>, bool)> {
    find_member_in_members(
        db,
        members,
        name,
        |kind| mode.allows_named_member(kind),
        mode.allows_index_signature(),
    )
}

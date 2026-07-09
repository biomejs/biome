use super::{InferredModuleTypes, collected_type_result};
use crate::ModuleDb;
use crate::db::queries::infer_module_types;
use crate::module_graph::ModuleInfo;
use biome_js_type_info::interned_types::{
    Literal as InferredLiteral, LocalTypeHandle, ModuleKey, TypeData as InferredTypeData,
    TypeMember as InferredTypeMember, TypeMemberKind as InferredTypeMemberKind,
    TypeSubstitution as InferredTypeSubstitution,
};
use rustc_hash::FxHashSet;
use salsa::plumbing::{AsId, FromId};

const MAX_LOCAL_TYPE_RESOLUTION_STEPS: usize = 1024;
const MAX_MEMBER_LOOKUP_STEPS: usize = 1024;

impl<'db> InferredModuleTypes<'db> {
    pub(in crate::db::type_inference) fn resolve_type_iterative(
        &self,
        db: &'db dyn ModuleDb,
        mut ty: InferredTypeData<'db>,
    ) -> InferredTypeData<'db> {
        let mut seen = FxHashSet::default();

        for _ in 0..MAX_LOCAL_TYPE_RESOLUTION_STEPS {
            let InferredTypeData::Local(local) = ty else {
                return ty;
            };

            let module_key = local.module(db);
            let type_id = local.type_id(db);
            if !seen.insert((module_key, type_id)) {
                return ty;
            }

            ty = self
                .type_for_local_handle(db, local)
                .unwrap_or(InferredTypeData::Unknown);
        }

        ty
    }

    fn type_for_local_handle(
        &self,
        db: &'db dyn ModuleDb,
        local: LocalTypeHandle<'db>,
    ) -> Option<InferredTypeData<'db>> {
        let module_key = local.module(db);
        let type_id = local.type_id(db);
        if module_key == self.module_key {
            return self.types.get(type_id.index()).copied();
        }

        let module = module_for_key(db, module_key)?;
        infer_module_types(db, module).and_then(|types| types.types.get(type_id.index()).copied())
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

        while let Some(mut state) = pending.pop() {
            let lookup = state.lookup;
            let collect = state.collect;
            let ty = self.resolve_type_iterative(db, state.ty);
            let (ty, lookup) = match ty {
                InferredTypeData::InstanceOf(instance) => {
                    let target = self.resolve_type_iterative(db, instance.ty(db));
                    state.substitutions = substitutions_for_instance(
                        db,
                        target,
                        instance.type_parameters(db),
                        &state.substitutions,
                    );
                    (target, MemberLookup::Instance)
                }
                ty @ (InferredTypeData::Unknown
                | InferredTypeData::Divergent(_)
                | InferredTypeData::Global
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
                break;
            }
            remaining_steps -= 1;

            if let Some(member_ty) = self.find_own_member_type(db, ty, name, lookup) {
                let member_ty = apply_substitutions(db, member_ty, &state.substitutions);
                if collect {
                    found.push(member_ty);
                    continue;
                }
                return Some(member_ty);
            }

            match ty {
                InferredTypeData::Class(class) => {
                    if let Some(mut extends) = class.extends(db) {
                        if matches!(lookup, MemberLookup::Any) {
                            extends = class_side_type(db, extends);
                        }
                        pending.push(MemberLookupState {
                            ty: apply_substitutions(db, extends, &state.substitutions),
                            lookup,
                            collect,
                            substitutions: state.substitutions.clone(),
                        });
                    }
                }
                InferredTypeData::Interface(interface) => {
                    pending.extend(interface.extends(db).iter().rev().copied().map(|ty| {
                        MemberLookupState {
                            ty: apply_substitutions(db, ty, &state.substitutions),
                            lookup,
                            collect,
                            substitutions: state.substitutions.clone(),
                        }
                    }));
                }
                InferredTypeData::Generic(generic) => {
                    if let Some(constraint) = generic.constraint(db) {
                        pending.push(MemberLookupState {
                            ty: apply_substitutions(db, constraint, &state.substitutions),
                            lookup,
                            collect,
                            substitutions: state.substitutions.clone(),
                        });
                    }
                }
                InferredTypeData::Intersection(intersection) => {
                    pending.extend(intersection.types(db).iter().rev().copied().map(|ty| {
                        MemberLookupState {
                            ty: apply_substitutions(db, ty, &state.substitutions),
                            lookup,
                            collect: true,
                            substitutions: state.substitutions.clone(),
                        }
                    }));
                }
                InferredTypeData::MergedReference(reference) => {
                    pending.extend(reference.targets(db).map(|ty| MemberLookupState {
                        ty: apply_substitutions(db, ty, &state.substitutions),
                        lookup,
                        collect: true,
                        substitutions: state.substitutions.clone(),
                    }));
                }
                InferredTypeData::Object(object) => {
                    if let Some(prototype) = object.prototype(db) {
                        pending.push(MemberLookupState {
                            ty: apply_substitutions(db, prototype, &state.substitutions),
                            lookup,
                            collect,
                            substitutions: state.substitutions.clone(),
                        });
                    }
                }
                InferredTypeData::Union(union) => {
                    pending.extend(union.types(db).iter().rev().copied().map(|ty| {
                        MemberLookupState {
                            ty: apply_substitutions(db, ty, &state.substitutions),
                            lookup,
                            collect: true,
                            substitutions: state.substitutions.clone(),
                        }
                    }));
                }
                InferredTypeData::Unknown
                | InferredTypeData::Divergent(_)
                | InferredTypeData::Global
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
                | InferredTypeData::AnyKeyword
                | InferredTypeData::NeverKeyword
                | InferredTypeData::ObjectKeyword
                | InferredTypeData::ThisKeyword
                | InferredTypeData::UnknownKeyword
                | InferredTypeData::VoidKeyword => {}
            }
        }

        collected_type_result(db, found)
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
    substitutions: Vec<InferredTypeSubstitution<'db>>,
}

impl<'db> MemberLookupState<'db> {
    fn new(ty: InferredTypeData<'db>, lookup: MemberLookup, collect: bool) -> Self {
        Self {
            ty,
            lookup,
            collect,
            substitutions: Vec::new(),
        }
    }
}

pub(in crate::db::type_inference) fn substitutions_for_instance<'db>(
    db: &'db dyn ModuleDb,
    target: InferredTypeData<'db>,
    type_parameters: &[InferredTypeData<'db>],
    inherited: &[InferredTypeSubstitution<'db>],
) -> Vec<InferredTypeSubstitution<'db>> {
    let Some(declared_parameters) = declared_type_parameters(db, target) else {
        return inherited.to_vec();
    };
    if declared_parameters.is_empty() {
        return inherited.to_vec();
    }

    let mut substitutions = inherited.to_vec();
    for (declared, replacement) in declared_parameters.iter().zip(type_parameters) {
        let declared = apply_substitutions(db, *declared, inherited);
        let replacement = apply_substitutions(db, *replacement, inherited);
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

    substitutions
}

fn declared_type_parameters<'db>(
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

pub(in crate::db::type_inference) fn apply_substitutions<'db>(
    db: &'db dyn ModuleDb,
    mut ty: InferredTypeData<'db>,
    substitutions: &[InferredTypeSubstitution<'db>],
) -> InferredTypeData<'db> {
    for substitution in substitutions {
        ty = ty.substitute_type(db, *substitution);
    }
    ty
}

fn class_side_type<'db>(db: &'db dyn ModuleDb, ty: InferredTypeData<'db>) -> InferredTypeData<'db> {
    match ty {
        InferredTypeData::InstanceOf(instance) => instance.ty(db),
        ty @ (InferredTypeData::Unknown
        | InferredTypeData::Divergent(_)
        | InferredTypeData::Global
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
    module_key: ModuleKey,
) -> Option<ModuleInfo> {
    let module = ModuleInfo::from_id(module_key.as_id());
    let current = db.module_for_path(module.path(db))?;
    (ModuleKey::new(current.as_id()) == module_key).then_some(current)
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
        .map(|member| (member.ty, member.kind.is_optional()));
    if named_member.is_some() {
        return named_member;
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

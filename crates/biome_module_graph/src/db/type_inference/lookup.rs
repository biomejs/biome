use super::{InferredModuleTypes, collected_type_result};
use crate::ModuleDb;
use crate::db::queries::infer_module_types;
use crate::module_graph::ModuleInfo;
use biome_js_type_info::interned_types::{
    Literal as InferredLiteral, LocalTypeHandle, ModuleKey, ReturnType as InferredReturnType,
    TypeData as InferredTypeData, TypeMember as InferredTypeMember,
    TypeMemberKind as InferredTypeMemberKind, TypeSubstitution as InferredTypeSubstitution,
    TypeTransformResult,
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
        let mut resolver = self;
        find_member_type_with_resolver(db, &mut resolver, ty, name, MemberLookupMode::Any)
    }

    pub(in crate::db::type_inference) fn find_value_member_type_iterative(
        &self,
        db: &'db dyn ModuleDb,
        ty: InferredTypeData<'db>,
        name: &str,
    ) -> Option<InferredTypeData<'db>> {
        let mut resolver = self;
        find_member_type_with_resolver(db, &mut resolver, ty, name, MemberLookupMode::Value)
    }
}

/// Selects which side of a type participates in member lookup.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub(in crate::db::type_inference) enum MemberLookupMode {
    /// Accepts both class-side and instance-side members.
    Any,
    /// Accepts static members, excluding constructors.
    Class,
    /// Accepts non-static members and index signatures.
    Instance,
    /// Selects class-side or instance-side members from the traversed value type.
    Value,
}

impl MemberLookupMode {
    fn allows_named_member(self, kind: &InferredTypeMemberKind<'_>) -> bool {
        match self {
            Self::Any => true,
            Self::Class => kind.is_static() && !kind.is_constructor(),
            Self::Instance => !kind.is_static(),
            Self::Value => false,
        }
    }

    fn allows_index_signature(self) -> bool {
        !matches!(self, Self::Class | Self::Value)
    }
}

/// Adapts member traversal to the type-resolution phase that invokes it.
///
/// Member lookup runs both while raw module types are being converted and
/// after inferred type tables are available. Implementations supply the local
/// type resolution and member finalization appropriate for either phase.
pub(in crate::db::type_inference) trait MemberLookupResolver<'db> {
    /// Resolves local indirection before the type is inspected for members.
    /// A resolution cycle may leave a local handle unresolved.
    fn resolve_type(
        &mut self,
        db: &'db dyn ModuleDb,
        ty: InferredTypeData<'db>,
    ) -> InferredTypeData<'db>;

    /// Produces the type returned for an own member.
    ///
    /// `substitutions` contains mappings accumulated from enclosing
    /// `InstanceOf` types. `crossed_instance` indicates whether lookup crossed
    /// at least one such type.
    fn finalize_member_type(
        &mut self,
        db: &'db dyn ModuleDb,
        ty: InferredTypeData<'db>,
        is_optional: bool,
        substitutions: &[InferredTypeSubstitution<'db>],
        crossed_instance: bool,
    ) -> InferredTypeData<'db>;
}

impl<'db> MemberLookupResolver<'db> for &InferredModuleTypes<'db> {
    fn resolve_type(
        &mut self,
        db: &'db dyn ModuleDb,
        ty: InferredTypeData<'db>,
    ) -> InferredTypeData<'db> {
        self.resolve_type_iterative(db, ty)
    }

    fn finalize_member_type(
        &mut self,
        db: &'db dyn ModuleDb,
        ty: InferredTypeData<'db>,
        _is_optional: bool,
        substitutions: &[InferredTypeSubstitution<'db>],
        _crossed_instance: bool,
    ) -> InferredTypeData<'db> {
        apply_substitutions(db, ty, substitutions)
    }
}

#[derive(Clone)]
struct MemberLookupState<'db> {
    ty: InferredTypeData<'db>,
    mode: MemberLookupMode,
    collect_result: bool,
    crossed_instance: bool,
    substitutions: Vec<InferredTypeSubstitution<'db>>,
}

impl<'db> MemberLookupState<'db> {
    fn new(ty: InferredTypeData<'db>, mode: MemberLookupMode) -> Self {
        Self {
            ty,
            mode,
            collect_result: false,
            crossed_instance: false,
            substitutions: Vec::new(),
        }
    }

    fn child(
        &self,
        db: &'db dyn ModuleDb,
        ty: InferredTypeData<'db>,
        collect_result: bool,
    ) -> Self {
        Self {
            ty: apply_substitutions(db, ty, &self.substitutions),
            mode: self.mode,
            collect_result,
            crossed_instance: self.crossed_instance,
            substitutions: self.substitutions.clone(),
        }
    }
}

/// Finds a member through own properties, inheritance, and compound types.
///
/// Instance wrappers contribute substitutions that are applied while the
/// lookup advances. Members found through unions, intersections, or merged
/// references are collected into a union. Returns `None` when no traversed
/// type exposes `name` within the requested `mode`. Traversal is bounded;
/// reaching the limit returns any members collected so far.
pub(in crate::db::type_inference) fn find_member_type_with_resolver<'db>(
    db: &'db dyn ModuleDb,
    resolver: &mut impl MemberLookupResolver<'db>,
    ty: InferredTypeData<'db>,
    name: &str,
    mode: MemberLookupMode,
) -> Option<InferredTypeData<'db>> {
    let mut seen = FxHashSet::default();
    let mut pending = vec![MemberLookupState::new(ty, mode)];
    let mut found = Vec::new();
    let mut remaining_steps = MAX_MEMBER_LOOKUP_STEPS;

    while let Some(mut state) = pending.pop() {
        let ty = resolver.resolve_type(db, state.ty);
        if !seen.insert((
            ty,
            state.mode,
            state.collect_result,
            state.crossed_instance,
            state.substitutions.clone(),
        )) {
            continue;
        }

        // Deduplicated entries above don't count against the budget, so
        // the limit measures distinct types visited, not queue churn.
        if remaining_steps == 0 {
            break;
        }
        remaining_steps -= 1;

        if let InferredTypeData::InstanceOf(instance) = ty {
            let target = resolver.resolve_type(db, instance.ty(db));
            state.substitutions = substitutions_for_instance(
                db,
                target,
                instance.type_parameters(db),
                &state.substitutions,
            );
            state.ty = target;
            state.mode = MemberLookupMode::Instance;
            state.crossed_instance = true;
            pending.push(state);
            continue;
        }

        if let Some((member_ty, is_optional)) = find_own_member_type(db, ty, name, state.mode) {
            let member_ty = resolver.finalize_member_type(
                db,
                member_ty,
                is_optional,
                &state.substitutions,
                state.crossed_instance,
            );
            if state.collect_result {
                found.push(member_ty);
                continue;
            }
            return Some(member_ty);
        }

        match ty {
            InferredTypeData::Class(class) => {
                if let Some(mut extends) = class.extends(db) {
                    if matches!(
                        state.mode,
                        MemberLookupMode::Any | MemberLookupMode::Class | MemberLookupMode::Value
                    ) {
                        extends = class_side_type(db, extends);
                    }
                    pending.push(state.child(db, extends, state.collect_result));
                }
            }
            InferredTypeData::Interface(interface) => {
                pending.extend(
                    interface
                        .extends(db)
                        .iter()
                        .rev()
                        .copied()
                        .map(|ty| state.child(db, ty, state.collect_result)),
                );
            }
            InferredTypeData::Generic(generic) => {
                if let Some(constraint) = generic.constraint(db) {
                    pending.push(state.child(db, constraint, state.collect_result));
                }
            }
            InferredTypeData::Intersection(intersection) => {
                pending.extend(
                    intersection
                        .types(db)
                        .iter()
                        .rev()
                        .copied()
                        .map(|ty| state.child(db, ty, true)),
                );
            }
            InferredTypeData::MergedReference(reference) => {
                pending.extend(reference.targets(db).map(|ty| state.child(db, ty, true)));
            }
            InferredTypeData::Object(object) => {
                if let Some(prototype) = object.prototype(db) {
                    pending.push(state.child(db, prototype, state.collect_result));
                }
            }
            InferredTypeData::Union(union) => {
                pending.extend(
                    union
                        .types(db)
                        .iter()
                        .rev()
                        .copied()
                        .map(|ty| state.child(db, ty, true)),
                );
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

pub(in crate::db) fn substitutions_for_instance<'db>(
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
        let TypeTransformResult::Transformed(substituted) = ty.substitute_type(db, *substitution)
        else {
            return InferredTypeData::Unknown;
        };
        ty = substituted;
    }
    ty
}

pub(in crate::db) fn apply_substitutions_to_root_body<'db>(
    db: &'db dyn ModuleDb,
    mut ty: InferredTypeData<'db>,
    substitutions: &[InferredTypeSubstitution<'db>],
) -> InferredTypeData<'db> {
    for substitution in substitutions {
        let TypeTransformResult::Transformed(substituted) =
            ty.substitute_type_in_root_body(db, *substitution)
        else {
            return InferredTypeData::Unknown;
        };
        ty = substituted;
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

/// Finds a member defined directly on `ty` without traversing related types.
///
/// Named, computed, and index-signature members are filtered according to
/// `mode`. Modules and namespaces accept members from either side. Getter
/// members produce their return type. The returned boolean indicates whether
/// the member declaration is optional.
fn find_own_member_type<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
    name: &str,
    mode: MemberLookupMode,
) -> Option<(InferredTypeData<'db>, bool)> {
    let find = |members, mode: MemberLookupMode, allow_index_signature| {
        find_member_in_members(
            db,
            members,
            name,
            |kind| mode.allows_named_member(kind),
            allow_index_signature,
        )
    };

    match ty {
        InferredTypeData::Class(class) => {
            let mode = if matches!(mode, MemberLookupMode::Value) {
                MemberLookupMode::Class
            } else {
                mode
            };
            find(
                class.members(db),
                mode,
                matches!(mode, MemberLookupMode::Instance),
            )
        }
        InferredTypeData::Interface(interface) => {
            let mode = if matches!(mode, MemberLookupMode::Value) {
                MemberLookupMode::Instance
            } else {
                mode
            };
            find(interface.members(db), mode, mode.allows_index_signature())
        }
        InferredTypeData::Literal(literal) => match literal.literal(db) {
            InferredLiteral::Object(members) => {
                let mode = if matches!(mode, MemberLookupMode::Value) {
                    MemberLookupMode::Instance
                } else {
                    mode
                };
                find(members, mode, mode.allows_index_signature())
            }
            InferredLiteral::BigInt(_)
            | InferredLiteral::Boolean(_)
            | InferredLiteral::Number(_)
            | InferredLiteral::RegExp(_)
            | InferredLiteral::String(_)
            | InferredLiteral::Template(_) => None,
        },
        InferredTypeData::Module(module) => find(module.members(db), MemberLookupMode::Any, true),
        InferredTypeData::Namespace(namespace) => {
            find(namespace.members(db), MemberLookupMode::Any, true)
        }
        InferredTypeData::Object(object) => {
            let mode = if matches!(mode, MemberLookupMode::Value) {
                MemberLookupMode::Instance
            } else {
                mode
            };
            find(object.members(db), mode, mode.allows_index_signature())
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

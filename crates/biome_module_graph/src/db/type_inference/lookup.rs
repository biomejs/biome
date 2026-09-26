use super::{InferredModuleTypes, collected_type_result};
use crate::db::queries::{LocalTypeInput, infer_local_type, infer_module_types};
use crate::{ModuleDb, module_for_key};
use biome_js_type_info::interned_types::{
    Literal as InferredLiteral, LocalTypeHandle, ReturnType as InferredReturnType,
    TypeData as InferredTypeData, TypeMember as InferredTypeMember,
    TypeMemberKind as InferredTypeMemberKind, TypeSubstitution as InferredTypeSubstitution,
    TypeTransformResult,
};
use rustc_hash::{FxHashSet, FxHasher};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

const MAX_LOCAL_TYPE_RESOLUTION_STEPS: usize = 1024;
const MAX_MEMBER_LOOKUP_STEPS: usize = 1024;

/// Follows local type handles through lookup queries.
///
/// Returns the first non-local type. A cycle returns the repeated local handle,
/// and a missing module or local type returns `Unknown`. At most 1024 handle
/// resolutions are performed; if the chain is still local, the next handle is
/// returned unresolved.
pub(in crate::db) fn resolve_local_type_on_demand<'db>(
    db: &'db dyn ModuleDb,
    mut ty: InferredTypeData<'db>,
) -> InferredTypeData<'db> {
    let mut seen = FxHashSet::default();

    for _ in 0..MAX_LOCAL_TYPE_RESOLUTION_STEPS {
        let InferredTypeData::Local(local) = ty else {
            return ty;
        };
        let key = (local.module(db), local.type_id(db));
        if !seen.insert(key) {
            return ty;
        }
        let Some(module) = module_for_key(db, key.0) else {
            return InferredTypeData::Unknown;
        };
        let input = LocalTypeInput::new(db, module, key.1);
        ty = infer_local_type(db, input).unwrap_or(InferredTypeData::Unknown);
    }

    ty
}

/// Finds a member without distinguishing class values from class instances.
///
/// Local handles are resolved through lookup queries. Inherited and compound
/// types are traversed subject to the work limit documented on
/// [`find_member_type_with_resolver`].
pub(in crate::db) fn find_member_type_on_demand<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
    name: &str,
) -> Option<InferredTypeData<'db>> {
    find_member_type_with_resolver(
        db,
        &mut OnDemandMemberLookupResolver,
        ty,
        name,
        MemberLookupMode::Any,
    )
}

/// Finds a member that is available on a value.
///
/// A class value exposes static members, while a class instance exposes
/// non-static members. This differs from [`find_member_type_on_demand`], which
/// accepts either side. Local handles are resolved through lookup queries, and
/// inherited and compound types are subject to the work limit documented on
/// [`find_member_type_with_resolver`].
pub(in crate::db) fn find_value_member_type_on_demand<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
    name: &str,
) -> Option<InferredTypeData<'db>> {
    find_member_type_with_resolver(
        db,
        &mut OnDemandMemberLookupResolver,
        ty,
        name,
        MemberLookupMode::Value,
    )
}

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
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

struct OnDemandMemberLookupResolver;

impl<'db> MemberLookupResolver<'db> for OnDemandMemberLookupResolver {
    fn resolve_type(
        &mut self,
        db: &'db dyn ModuleDb,
        ty: InferredTypeData<'db>,
    ) -> InferredTypeData<'db> {
        resolve_local_type_on_demand(db, ty)
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

#[derive(Clone, Debug)]
struct MemberLookupState<'db> {
    ty: InferredTypeData<'db>,
    mode: MemberLookupMode,
    collect_result: bool,
    crossed_instance: bool,
    substitutions: Rc<[InferredTypeSubstitution<'db>]>,
}

// Substitutions are applied simultaneously, so their order does not distinguish
// lookup states. Each substitution has a distinct `generic` value.
impl PartialEq for MemberLookupState<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty
            && self.mode == other.mode
            && self.collect_result == other.collect_result
            && self.crossed_instance == other.crossed_instance
            && self.substitutions.len() == other.substitutions.len()
            && (self.substitutions == other.substitutions
                || self
                    .substitutions
                    .iter()
                    .all(|substitution| other.substitutions.contains(substitution)))
    }
}

impl Eq for MemberLookupState<'_> {}

impl Hash for MemberLookupState<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ty.hash(state);
        self.mode.hash(state);
        self.collect_result.hash(state);
        self.crossed_instance.hash(state);
        self.substitutions.len().hash(state);
        let mut hash = 0u64;
        for substitution in self.substitutions.iter() {
            let mut hasher = FxHasher::default();
            substitution.hash(&mut hasher);
            hash = hash.wrapping_add(hasher.finish());
        }
        hash.hash(state);
    }
}

impl<'db> MemberLookupState<'db> {
    fn new(ty: InferredTypeData<'db>, mode: MemberLookupMode) -> Self {
        Self {
            ty,
            mode,
            collect_result: false,
            crossed_instance: false,
            substitutions: Rc::default(),
        }
    }

    fn child(&self, ty: InferredTypeData<'db>, collect_result: bool) -> Self {
        Self {
            ty,
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
/// references are collected into a union. A branch without `name` contributes
/// nothing; it does not discard members found on other branches. Consequently,
/// `Some` may describe only part of a compound type, and `None` means only that
/// no member was found in the supported portion that was traversed.
///
/// Traversal processes at most 1024 distinct states. A state includes the type,
/// lookup mode, accumulated substitutions, and whether a compound or instance
/// boundary has been crossed. Repeated states do not consume work. If the limit
/// is reached, the result is `Unknown` because unvisited states may contain a
/// different member type.
///
/// For example, lookup of `item` on `Left | Right | Missing` returns
/// `string | number`, even though `Missing` has no `item`:
///
/// ```ts
/// type Left = { item: string };
/// type Right = { item: number };
/// type Missing = {};
/// ```
pub(in crate::db::type_inference) fn find_member_type_with_resolver<'db>(
    db: &'db dyn ModuleDb,
    resolver: &mut impl MemberLookupResolver<'db>,
    ty: InferredTypeData<'db>,
    name: &str,
    mode: MemberLookupMode,
) -> Option<InferredTypeData<'db>> {
    find_member_key_type_with_resolver(db, resolver, ty, MemberLookupKey::Name(name), mode)
}

#[derive(Clone, Copy)]
pub(in crate::db::type_inference) enum MemberLookupKey<'a, 'db> {
    Name(&'a str),
    /// A resolved symbol identity, kept distinct from its display name.
    Symbol(InferredTypeData<'db>),
}

/// Looks up a string name or symbol identity with the traversal and work limits
/// of [`find_member_type_with_resolver`]. Symbol keys match only computed members
/// and index signatures with the same identity.
pub(in crate::db::type_inference) fn find_member_key_type_with_resolver<'db>(
    db: &'db dyn ModuleDb,
    resolver: &mut impl MemberLookupResolver<'db>,
    ty: InferredTypeData<'db>,
    key: MemberLookupKey<'_, 'db>,
    mode: MemberLookupMode,
) -> Option<InferredTypeData<'db>> {
    let mut seen = FxHashSet::default();
    let mut pending = vec![MemberLookupState::new(ty, mode)];
    let mut found = Vec::new();
    let mut remaining_steps = MAX_MEMBER_LOOKUP_STEPS;

    while let Some(mut state) = pending.pop() {
        let ty = resolver
            .resolve_type(db, state.ty)
            .expand_canonical_global(db);
        let ty = if let Some(substitution) = state
            .substitutions
            .iter()
            .find(|substitution| substitution.generic == ty)
        {
            resolver
                .resolve_type(db, substitution.replacement)
                .expand_canonical_global(db)
        } else {
            ty
        };
        state.ty = ty;
        if !seen.insert(state.clone()) {
            continue;
        }

        // Deduplicated entries above don't count against the budget, so the
        // limit measures distinct traversal states rather than queue churn.
        if remaining_steps == 0 {
            return Some(InferredTypeData::Unknown);
        }
        remaining_steps -= 1;

        if let InferredTypeData::InstanceOf(instance) = ty {
            let target = resolver
                .resolve_type(db, instance.ty(db))
                .expand_canonical_global(db);
            let substitutions = substitutions_for_instance(
                db,
                target,
                instance.type_parameters(db),
                &state.substitutions,
            );
            if substitutions.as_slice() != state.substitutions.as_ref() {
                state.substitutions = substitutions.into();
            }
            state.ty = target;
            state.mode = MemberLookupMode::Instance;
            state.crossed_instance = true;
            pending.push(state);
            continue;
        }

        if let Some((member_ty, is_optional)) = find_own_member_type(db, ty, key, state.mode) {
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
            InferredTypeData::Literal(literal)
                if matches!(literal.literal(db), InferredLiteral::RegExp(_)) =>
            {
                state.ty = InferredTypeData::instance_of(
                    db,
                    InferredTypeData::regexp_class(),
                    Box::default(),
                );
                pending.push(state);
            }
            InferredTypeData::Class(class) => {
                if let Some(mut extends) = class.extends(db) {
                    if matches!(
                        state.mode,
                        MemberLookupMode::Any | MemberLookupMode::Class | MemberLookupMode::Value
                    ) {
                        extends = class_side_type(db, extends);
                    }
                    pending.push(state.child(extends, state.collect_result));
                }
            }
            InferredTypeData::Interface(interface) => {
                pending.extend(
                    interface
                        .extends(db)
                        .iter()
                        .rev()
                        .copied()
                        .map(|ty| state.child(ty, state.collect_result)),
                );
            }
            InferredTypeData::Generic(generic) => {
                if let Some(constraint) = generic.constraint(db) {
                    pending.push(state.child(constraint, state.collect_result));
                }
            }
            InferredTypeData::Intersection(intersection) => {
                pending.extend(
                    intersection
                        .types(db)
                        .iter()
                        .rev()
                        .copied()
                        .map(|ty| state.child(ty, true)),
                );
            }
            InferredTypeData::MergedReference(reference) => {
                pending.extend(reference.targets(db).map(|ty| state.child(ty, true)));
            }
            InferredTypeData::Object(object) => {
                if let Some(prototype) = object.prototype(db) {
                    pending.push(state.child(prototype, state.collect_result));
                }
            }
            InferredTypeData::Union(union) => {
                pending.extend(
                    union
                        .types(db)
                        .iter()
                        .rev()
                        .copied()
                        .map(|ty| state.child(ty, true)),
                );
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
            | InferredTypeData::Function(_)
            | InferredTypeData::Module(_)
            | InferredTypeData::Namespace(_)
            | InferredTypeData::Tuple(_)
            | InferredTypeData::Local(_)
            | InferredTypeData::TypeOperator(_)
            | InferredTypeData::IndexedAccess(_)
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

/// Collects replacements for an instance's generic parameters.
///
/// Arguments are substituted using the inherited replacements first. A new
/// replacement for the same parameter updates its entry rather than adding a
/// duplicate. The inherited list is left unchanged.
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
        let declared = *declared;
        if !declared.is_generic_reference(db) {
            continue;
        }
        let replacement = apply_substitutions(db, *replacement, inherited);
        let declared_instance = InferredTypeData::instance_of(db, declared, Box::default());
        for generic in [declared_instance, declared] {
            if let Some(substitution) = substitutions
                .iter_mut()
                .find(|substitution| substitution.generic == generic)
            {
                substitution.replacement = replacement;
            } else {
                substitutions.push(InferredTypeSubstitution {
                    generic,
                    replacement,
                });
            }
        }
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
        | InferredTypeData::IndexedAccess(_)
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

/// Replaces each generic once without substituting inside its replacement.
pub(in crate::db::type_inference) fn apply_substitutions<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
    substitutions: &[InferredTypeSubstitution<'db>],
) -> InferredTypeData<'db> {
    ty.substitute_types(db, substitutions)
        .map_or(InferredTypeData::Unknown, |ty| ty)
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
        | InferredTypeData::IndexedAccess(_)
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

/// Finds a member defined directly on `ty` without traversing related types.
///
/// Named, computed, and index-signature members are filtered according to
/// `mode`. Modules and namespaces accept members from either side. Getter
/// members produce their return type. The returned boolean indicates whether
/// the member declaration is optional.
fn find_own_member_type<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
    key: MemberLookupKey<'_, 'db>,
    mode: MemberLookupMode,
) -> Option<(InferredTypeData<'db>, bool)> {
    let find = |members, mode: MemberLookupMode, allow_index_signature| {
        find_member_in_members(
            db,
            members,
            key,
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
        | InferredTypeData::IndexedAccess(_)
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
    key: MemberLookupKey<'_, 'db>,
    allows_named_member: impl Fn(&InferredTypeMemberKind<'db>) -> bool,
    allow_index_signature: bool,
) -> Option<(InferredTypeData<'db>, bool)> {
    let name = match key {
        MemberLookupKey::Name(name) => name,
        MemberLookupKey::Symbol(symbol) => {
            return members.iter().find_map(|member| {
                if !allows_named_member(&member.kind) {
                    return None;
                }
                let key = member.kind.computed_value_type().or_else(|| {
                    allow_index_signature
                        .then(|| member.kind.index_signature_type())
                        .flatten()
                })?;
                (key == symbol)
                    .then_some((member_value_type(db, member), member.kind.is_optional()))
            });
        }
    };
    let named_member = members
        .iter()
        .find(|member| {
            allows_named_member(&member.kind)
                && member.kind.computed_name().is_none()
                && member.kind.has_name(name)
        })
        .map(|member| (member_value_type(db, member), member.kind.is_optional()));
    if named_member.is_some() {
        return named_member;
    }

    let computed_member = members.iter().find_map(|member| {
        if !allows_named_member(&member.kind) {
            return None;
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use biome_js_type_info::interned_types::{InternedGenericTypeParameter, InternedInterface};
    use biome_rowan::Text;

    #[salsa::db]
    #[derive(Default)]
    struct TestDb {
        storage: salsa::Storage<Self>,
    }

    #[salsa::db]
    impl salsa::Database for TestDb {}

    #[salsa::db]
    impl biome_db::Db for TestDb {
        fn parsed_source_for_path(
            &self,
            _path: &camino::Utf8Path,
        ) -> Option<biome_db::ParsedSource> {
            None
        }
    }

    #[salsa::db]
    impl biome_js_type_info::TypeDb for TestDb {}

    #[salsa::db]
    impl biome_languages::LanguageDb for TestDb {
        fn source_from_index(&self, _index: usize) -> Option<biome_languages::DocumentFileSource> {
            None
        }
    }

    #[salsa::db]
    impl ModuleDb for TestDb {
        fn module_for_path(&self, _path: &camino::Utf8Path) -> Option<crate::ModuleInfo> {
            None
        }

        fn for_each_module(&self, _f: &mut dyn FnMut(crate::ModuleInfo)) {}
    }

    #[test]
    fn instance_substitutions_rebind_without_changing_other_branches() {
        let db = TestDb::default();
        let generic = InferredTypeData::Generic(InternedGenericTypeParameter::new(
            &db,
            false,
            None,
            None,
            Text::from("T"),
        ));
        let target = InferredTypeData::Interface(InternedInterface::new(
            &db,
            vec![generic].into_boxed_slice(),
            Box::default(),
            Box::default(),
            Text::from("Base"),
        ));
        let parent = substitutions_for_instance(&db, target, &[InferredTypeData::Number], &[]);
        let unchanged =
            substitutions_for_instance(&db, target, &[InferredTypeData::Number], &parent);
        let rebound = substitutions_for_instance(&db, target, &[InferredTypeData::Null], &parent);
        assert_eq!(parent, unchanged);
        assert_eq!(parent.len(), rebound.len());
        for reference in [
            generic,
            InferredTypeData::instance_of(&db, generic, Box::default()),
        ] {
            assert_eq!(
                apply_substitutions(&db, reference, &parent),
                InferredTypeData::Number
            );
            assert_eq!(
                apply_substitutions(&db, reference, &unchanged),
                InferredTypeData::Number
            );
            assert_eq!(
                apply_substitutions(&db, reference, &rebound),
                InferredTypeData::Null
            );
        }
        let reference = InferredTypeData::instance_of(&db, generic, Box::default());
        let alias = InferredTypeData::instance_of(&db, target, vec![reference].into_boxed_slice());
        let alias_bound =
            substitutions_for_instance(&db, alias, &[InferredTypeData::Null], &parent);
        assert_eq!(
            apply_substitutions(&db, generic, &alias_bound),
            InferredTypeData::Number
        );
        assert_eq!(
            apply_substitutions(&db, reference, &alias_bound),
            InferredTypeData::Null
        );
        let restored =
            substitutions_for_instance(&db, target, &[InferredTypeData::Number], &alias_bound);
        assert_eq!(
            apply_substitutions(&db, generic, &restored),
            InferredTypeData::Number
        );
        assert_eq!(
            apply_substitutions(&db, reference, &restored),
            InferredTypeData::Number
        );
    }

    #[test]
    fn instance_substitutions_resolve_arguments_before_rebinding_parameters() {
        let db = TestDb::default();
        let generic = |name| {
            InferredTypeData::Generic(InternedGenericTypeParameter::new(
                &db,
                false,
                None,
                None,
                Text::from(name),
            ))
        };
        let t = generic("T");
        let u = generic("U");
        let target = InferredTypeData::Interface(InternedInterface::new(
            &db,
            vec![t, u].into_boxed_slice(),
            Box::default(),
            Box::default(),
            Text::from("Pair"),
        ));
        let parent = substitutions_for_instance(
            &db,
            target,
            &[InferredTypeData::String, InferredTypeData::Number],
            &[],
        );
        let swapped = substitutions_for_instance(&db, target, &[u, t], &parent);
        assert_eq!(swapped.len(), parent.len());
        assert_eq!(
            apply_substitutions(&db, t, &swapped),
            InferredTypeData::Number
        );
        assert_eq!(
            apply_substitutions(&db, u, &swapped),
            InferredTypeData::String
        );
        let symbolic = substitutions_for_instance(&db, target, &[u, t], &[]);
        assert_eq!(apply_substitutions(&db, t, &symbolic), u);
        assert_eq!(apply_substitutions(&db, u, &symbolic), t);
    }

    #[test]
    fn recursive_instance_substitutions_stop_growing_after_unknown_indexed_access() {
        use biome_js_type_info::interned_types::InternedIndexedAccessType;

        let db = TestDb::default();
        let t = InferredTypeData::Generic(InternedGenericTypeParameter::new(
            &db,
            false,
            None,
            None,
            Text::from("T"),
        ));
        let target = InferredTypeData::Interface(InternedInterface::new(
            &db,
            vec![t].into_boxed_slice(),
            Box::default(),
            Box::default(),
            Text::from("Recursive"),
        ));
        let argument = InferredTypeData::IndexedAccess(InternedIndexedAccessType::new(
            &db,
            t,
            InferredTypeData::Unknown,
        ));
        let initial = substitutions_for_instance(&db, target, &[InferredTypeData::String], &[]);
        let first = substitutions_for_instance(&db, target, &[argument], &initial);
        let second = substitutions_for_instance(&db, target, &[argument], &first);

        assert_eq!(first, second);
        assert_eq!(
            apply_substitutions(&db, t, &first),
            InferredTypeData::Unknown
        );
    }

    #[test]
    fn member_lookup_state_ignores_substitution_order() {
        let mut first =
            MemberLookupState::new(InferredTypeData::ObjectKeyword, MemberLookupMode::Any);
        first.substitutions = vec![
            InferredTypeSubstitution {
                generic: InferredTypeData::String,
                replacement: InferredTypeData::Number,
            },
            InferredTypeSubstitution {
                generic: InferredTypeData::Boolean,
                replacement: InferredTypeData::BigInt,
            },
        ]
        .into();
        let mut second = first.clone();
        Rc::make_mut(&mut second.substitutions).reverse();
        assert_eq!(first, second);
        let mut seen = FxHashSet::default();
        assert!(seen.insert(first.clone()));
        assert!(!seen.insert(second));
        let mut rebound = first;
        Rc::make_mut(&mut rebound.substitutions)[0].replacement = InferredTypeData::Boolean;
        assert!(seen.insert(rebound));
    }
}

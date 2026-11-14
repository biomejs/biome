//! Evaluates TypeScript conditional types such as `T extends U ? X : Y`.
//!
//! A conditional type selects its true branch when the check type is
//! assignable to the extends type, and its false branch when it is not. The
//! assignability relation implemented here is partial: it answers only when
//! the operands are resolved well enough to be certain. Otherwise the
//! conditional remains unevaluated, and consumers treat it as indeterminate
//! instead of assuming either branch.
//!
//! The operands must already be normalized, as with the operations in
//! [`crate::type_operations`].

use crate::generated::global_types::ids::{
    BOOLEAN_ID_GLOBAL_TYPE_ID, FUNCTION_ID_GLOBAL_TYPE_ID, NUMBER_ID_GLOBAL_TYPE_ID,
    OBJECT_ID_GLOBAL_TYPE_ID, PROMISE_ALL_ID_GLOBAL_TYPE_ID, PROMISE_ALL_SETTLED_ID_GLOBAL_TYPE_ID,
    PROMISE_ANY_ID_GLOBAL_TYPE_ID, PROMISE_CATCH_ID_GLOBAL_TYPE_ID,
    PROMISE_CONSTRUCT_ID_GLOBAL_TYPE_ID, PROMISE_FINALLY_ID_GLOBAL_TYPE_ID,
    PROMISE_RACE_ID_GLOBAL_TYPE_ID, PROMISE_REJECT_ID_GLOBAL_TYPE_ID,
    PROMISE_RESOLVE_ID_GLOBAL_TYPE_ID, PROMISE_THEN_ID_GLOBAL_TYPE_ID,
    PROMISE_TRY_ID_GLOBAL_TYPE_ID, STRING_ID_GLOBAL_TYPE_ID, SYMBOL_ID_GLOBAL_TYPE_ID,
};
use crate::interned_types::{
    FunctionParameter, InternedClass, InternedConstructor, InternedExtendsType, InternedFunction,
    InternedGenericTypeParameter, InternedIntersection, InternedLiteral, InternedTuple,
    InternedTypeInstance, InternedUnion, Literal, ReturnType, TupleElementType, TypeData, TypeDb,
    TypeMember, TypeMemberKind, well_known_symbol_name,
};
use crate::literal::StringLiteral;
use crate::type_operations::keyof;
use crate::type_transform::TypeSubstitution;
use crate::{GlobalTypeId, TypeOperator, TypeofKind};
use biome_js_syntax::numbers::canonicalize_js_bigint_literal;
use rustc_hash::FxHashSet;

/// Maximum number of type pairs compared while evaluating one conditional.
const MAX_RELATION_STEPS: usize = 256;

/// Maximum number of inheritance levels followed while looking up a member.
const MAX_MEMBER_LOOKUP_DEPTH: usize = 16;

/// Maximum number of types inspected while looking for a free generic.
const MAX_FREE_GENERIC_STEPS: usize = 256;

/// Maximum number of types inspected while looking for an undetermined type.
const MAX_UNDETERMINED_TYPE_STEPS: usize = 64;

/// Maximum number of members compared structurally for one pair of types.
const MAX_STRUCTURAL_MEMBERS: usize = 64;

/// Built-in methods whose generated signatures are simplified stand-ins, such
/// as `Promise.prototype.then`, which is declared without parameters.
/// Comparing a stand-in with another signature would give wrong answers, so
/// such comparisons stay undetermined.
const SIMPLIFIED_SIGNATURES: [GlobalTypeId; 11] = [
    PROMISE_CONSTRUCT_ID_GLOBAL_TYPE_ID,
    PROMISE_CATCH_ID_GLOBAL_TYPE_ID,
    PROMISE_FINALLY_ID_GLOBAL_TYPE_ID,
    PROMISE_THEN_ID_GLOBAL_TYPE_ID,
    PROMISE_ALL_ID_GLOBAL_TYPE_ID,
    PROMISE_ALL_SETTLED_ID_GLOBAL_TYPE_ID,
    PROMISE_ANY_ID_GLOBAL_TYPE_ID,
    PROMISE_RACE_ID_GLOBAL_TYPE_ID,
    PROMISE_REJECT_ID_GLOBAL_TYPE_ID,
    PROMISE_RESOLVE_ID_GLOBAL_TYPE_ID,
    PROMISE_TRY_ID_GLOBAL_TYPE_ID,
];

/// Resolves local handles and structural globals in a normalized type.
pub(crate) type TypeResolver<'db, 'a> = dyn FnMut(TypeData<'db>) -> TypeData<'db> + 'a;

/// Result of evaluating a conditional type.
pub(crate) struct EvaluatedExtends<'db> {
    /// The selected branch, with `infer` generics replaced by their bindings.
    pub(crate) ty: TypeData<'db>,
    /// Whether `ty` was changed after normalization and must be normalized again.
    pub(crate) needs_normalization: bool,
}

/// Selects the branch of a conditional type.
///
/// Returns `None` when the check type still refers to an unsubstituted
/// generic, or when assignability cannot be decided:
///
/// ```ts
/// type IsString<T> = T extends string ? "yes" : "no";
///
/// type Yes = IsString<"a">;  // "yes"
/// type No = IsString<1>;     // "no"
/// type Pending = IsString<T>; // unevaluated until `T` is known
/// ```
///
/// A distributive conditional whose check type is `never` evaluates to `never`.
/// A union or `boolean` check type is distributed when the generic is
/// substituted, so one reaching this function could not be distributed and is
/// left unevaluated. A check type of `any` selects both branches.
pub(crate) fn evaluate_extends<'db>(
    db: &'db dyn TypeDb,
    extends: InternedExtendsType<'db>,
    resolve: &mut TypeResolver<'db, '_>,
) -> Option<EvaluatedExtends<'db>> {
    evaluate_extends_with(
        db,
        extends,
        extends.check_type(db),
        extends.extends_type(db),
        resolve,
    )
}

/// Selects the branch of `extends` as [`evaluate_extends`] does, but with the
/// given check and extends types in place of its own.
fn evaluate_extends_with<'db>(
    db: &'db dyn TypeDb,
    extends: InternedExtendsType<'db>,
    check_type: TypeData<'db>,
    extends_type: TypeData<'db>,
    resolve: &mut TypeResolver<'db, '_>,
) -> Option<EvaluatedExtends<'db>> {
    let mut check = AssignabilityCheck::new(db, resolve, extends.infer_types(db));
    let check_type = check.resolve(check_type);
    // Like TypeScript, defer a conditional whose check type is generic, even if
    // the generic's constraint would select a branch. Otherwise a generic
    // declaration would be evaluated before its type arguments are known.
    if may_refer_to_free_generic(db, check_type, &[]) {
        return None;
    }
    if extends.distributive(db) {
        match check_type {
            TypeData::NeverKeyword => {
                return Some(EvaluatedExtends {
                    ty: TypeData::NeverKeyword,
                    needs_normalization: false,
                });
            }
            TypeData::Union(_) | TypeData::Boolean => return None,
            _ => {}
        }
    }

    if check_type == TypeData::AnyKeyword {
        let extends_type = check.resolve(extends_type);
        let true_type = check.instantiate_true_type(extends.true_type(db))?;
        let ty = if matches!(
            extends_type,
            TypeData::AnyKeyword | TypeData::UnknownKeyword
        ) {
            true_type.ty
        } else {
            TypeData::union_from_types(db, Vec::from([true_type.ty, extends.false_type(db)]))
        };
        return Some(EvaluatedExtends {
            ty,
            needs_normalization: true_type.needs_normalization,
        });
    }

    let result = match check.is_assignable(check_type, extends_type) {
        Assignability::Assignable => check.are_inferences_consistent(),
        result => result,
    };
    match result {
        Assignability::Assignable => check.instantiate_true_type(extends.true_type(db)),
        Assignability::NotAssignable => Some(EvaluatedExtends {
            ty: extends.false_type(db),
            needs_normalization: false,
        }),
        Assignability::Undetermined => None,
    }
}

impl<'db> InternedExtendsType<'db> {
    /// Selects the branch of this conditional without normalizing either branch.
    ///
    /// `check_type` and `extends_type` replace the conditional's own operands.
    /// They must already have the caller's type arguments substituted and be
    /// normalized. The selected branch is returned as declared, except that
    /// `infer` generics are replaced by the types they matched. Callers that
    /// substitute type arguments lazily, such as member lookup, can then apply
    /// their substitutions to the branch alone.
    ///
    /// Returns `None` under the same conditions as [`evaluate_extends`].
    pub fn select_branch(
        self,
        db: &'db dyn TypeDb,
        check_type: TypeData<'db>,
        extends_type: TypeData<'db>,
        mut resolve: impl FnMut(TypeData<'db>) -> TypeData<'db>,
    ) -> Option<TypeData<'db>> {
        evaluate_extends_with(db, self, check_type, extends_type, &mut resolve)
            .map(|evaluated| evaluated.ty)
    }
}

/// Decides whether a value whose `typeof` is `kind` can have the object type
/// `target`, by comparing the value's apparent type with `target`.
///
/// The apparent type of a primitive is its wrapper, such as `String` for
/// `"string"`, together with the members of `Object`. For `"function"` it is
/// `Function`. `target` must be normalized. The result is:
///
/// - `Assignable` when every such value has the type, as every string has
///   `{ length: number }` and every value that is not nullish has `{}`;
/// - `NotAssignable` when no such value has it: `target` requires a member
///   that the apparent type lacks, all of its members are optional and the
///   apparent type has none of them, or it has a call or construct signature,
///   which only functions have;
/// - `Undetermined` otherwise.
///
/// Only missing members are conclusive. A member whose type does not match
/// leaves the result undetermined, because the generated globals come from
/// the ES2015 library files, and newer ones add overloads to some members,
/// such as `localeCompare` of `String`. For the same reason, a member that a
/// newer TypeScript library adds to a wrapper, such as `padStart` of
/// `String`, counts as present with an unknown type. So do the members
/// `toString`, `toLocaleString`, and `valueOf` of a `bigint`, which has no
/// wrapper among the generated globals. Members that the project adds to a
/// wrapper through `declare global` are not seen.
///
/// For `"function"`, a `target` that has or may inherit a call or construct
/// signature is undetermined, because a function of that type keeps it. A
/// value whose `typeof` is `"undefined"` never has an object type, and one
/// whose `typeof` is `"object"` is always undetermined.
pub(crate) fn primitive_can_satisfy<'db>(
    db: &'db dyn TypeDb,
    kind: TypeofKind,
    target: TypeData<'db>,
    resolve: &mut TypeResolver<'db, '_>,
) -> Assignability {
    AssignabilityCheck::new(db, resolve, &[]).primitive_can_satisfy(kind, target)
}

/// Returns the members that TypeScript 6.0.3's library files declare on the
/// wrapper of a `typeof` kind, but that the generated globals lack, because
/// they come from library files newer than ES2015.
fn members_from_newer_libraries(kind: TypeofKind) -> &'static [&'static str] {
    match kind {
        TypeofKind::String => &[
            "at",
            "isWellFormed",
            "matchAll",
            "padEnd",
            "padStart",
            "replaceAll",
            "toWellFormed",
            "trimEnd",
            "trimLeft",
            "trimRight",
            "trimStart",
        ],
        TypeofKind::Symbol => &["description"],
        // The generated globals have no `BigInt` wrapper.
        TypeofKind::BigInt => &["toLocaleString", "toString", "valueOf"],
        TypeofKind::Boolean
        | TypeofKind::Function
        | TypeofKind::Number
        | TypeofKind::Object
        | TypeofKind::Undefined => &[],
    }
}

impl<'db> TypeData<'db> {
    /// Returns the generic referenced by this type, if it is a reference to one.
    ///
    /// See [`referenced_generic`] for the representations this recognizes.
    pub fn as_generic_reference(
        self,
        db: &'db dyn TypeDb,
    ) -> Option<InternedGenericTypeParameter<'db>> {
        referenced_generic(db, self)
    }

    /// Returns whether this type may refer to a generic it doesn't declare
    /// itself. A conditional type with such a check type can't be evaluated.
    pub fn may_refer_to_free_generic(self, db: &'db dyn TypeDb) -> bool {
        may_refer_to_free_generic(db, self, &[])
    }

    /// Returns the types a distributive conditional evaluates separately when
    /// this type replaces its check type.
    ///
    /// A union gives its members, `boolean` gives `true` and `false`, and
    /// `never` gives no types. Any other type isn't distributed and gives
    /// `None`.
    pub fn distributive_members(self, db: &'db dyn TypeDb) -> Option<Vec<Self>> {
        match self {
            Self::Union(union) => Some(union.types(db).to_vec()),
            Self::Boolean => Some(boolean_literals(db).to_vec()),
            Self::NeverKeyword => Some(Vec::new()),
            // Annotations such as `boolean` are instances of the keyword.
            Self::InstanceOf(instance) if instance.type_parameters(db).is_empty() => {
                instance.ty(db).distributive_members(db)
            }
            _ => None,
        }
    }
}

/// Returns the `true` and `false` literal types.
fn boolean_literals(db: &dyn TypeDb) -> [TypeData<'_>; 2] {
    [true, false]
        .map(|value| TypeData::Literal(InternedLiteral::new(db, Literal::Boolean(value.into()))))
}

/// Returns the generic referenced by `ty`.
///
/// References to a generic `T` are represented either as `T` itself or as an
/// instance of `T` without type arguments. Built-in types refer to their
/// generics through global handles.
pub(crate) fn referenced_generic<'db>(
    db: &'db dyn TypeDb,
    ty: TypeData<'db>,
) -> Option<InternedGenericTypeParameter<'db>> {
    match ty.expand_canonical_global(db) {
        TypeData::Generic(generic) => Some(generic),
        TypeData::InstanceOf(instance) if instance.type_parameters(db).is_empty() => {
            match instance.ty(db).expand_canonical_global(db) {
                TypeData::Generic(generic) => Some(generic),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Returns whether `ty` may refer to a generic that is neither in `bound` nor
/// declared by `ty` itself.
///
/// For example, `Promise<T>` refers to `T`, while `<T>(value: T) => T` only
/// refers to the `T` it declares. Types too large to inspect are assumed to
/// refer to a generic.
pub(crate) fn may_refer_to_free_generic<'db>(
    db: &'db dyn TypeDb,
    ty: TypeData<'db>,
    bound: &[InternedGenericTypeParameter<'db>],
) -> bool {
    let mut pending = Vec::from([ty]);
    let mut seen = FxHashSet::default();
    let mut declared = bound
        .iter()
        .map(|generic| TypeData::Generic(*generic))
        .collect::<FxHashSet<_>>();
    let mut remaining_steps = MAX_FREE_GENERIC_STEPS;
    while let Some(ty) = pending.pop() {
        if !seen.insert(ty) {
            continue;
        }
        if remaining_steps == 0 {
            return true;
        }
        remaining_steps -= 1;

        // An instance of an alias replaces the parameters of the alias
        // declaration with its arguments, so only the arguments can be free.
        if let TypeData::InstanceOf(instance) = ty
            && let TypeData::InstanceOf(declaration) = instance.ty(db)
            && !declaration.type_parameters(db).is_empty()
        {
            pending.extend(instance.type_parameters(db).iter().copied());
            continue;
        }

        let declared_here: &[TypeData<'db>] = match ty {
            TypeData::Generic(_) => {
                if !declared.contains(&ty) {
                    return true;
                }
                continue;
            }
            TypeData::Class(class) => class.type_parameters(db),
            TypeData::Constructor(constructor) => constructor.type_parameters(db),
            TypeData::Function(function) => function.type_parameters(db),
            TypeData::Interface(interface) => interface.type_parameters(db),
            TypeData::Extends(extends) => extends.infer_types(db),
            TypeData::MappedType(mapped) => std::slice::from_ref(mapped.type_parameter(db)),
            _ => &[],
        };
        declared.extend(
            declared_here
                .iter()
                .filter_map(|ty| referenced_generic(db, *ty).map(TypeData::Generic)),
        );
        pending.extend(ty.type_slots(db).iter());
    }
    false
}

/// Returns whether `ty` may contain a type that inference couldn't determine.
///
/// Every such type has the same representation, so two types containing one
/// can be equal without being the same type. In this example, inference
/// determines neither `keyof` type, yet the conditional must not select
/// `"yes"`:
///
/// ```ts
/// type Different = [keyof { 1: number }] extends [keyof { 2: number }] ? "yes" : "no";
/// ```
///
/// Classes and interfaces are not inspected. Two equal declarations are the
/// same declaration, which is assignable to itself whatever its members are.
/// Types too large to inspect are assumed to contain an undetermined type.
fn may_contain_undetermined_type<'db>(db: &'db dyn TypeDb, ty: TypeData<'db>) -> bool {
    let mut pending = Vec::from([ty]);
    let mut remaining_steps = MAX_UNDETERMINED_TYPE_STEPS;
    while let Some(ty) = pending.pop() {
        if remaining_steps == 0 {
            return true;
        }
        remaining_steps -= 1;

        match ty {
            TypeData::Unknown => return true,
            TypeData::Class(_) | TypeData::Interface(_) => {}
            ty => pending.extend(ty.type_slots(db).iter()),
        }
    }
    false
}

/// Returns the substitutions replacing both representations of a generic
/// reference with `replacement`.
pub(crate) fn generic_substitutions<'db>(
    db: &'db dyn TypeDb,
    generic: InternedGenericTypeParameter<'db>,
    replacement: TypeData<'db>,
) -> [TypeSubstitution<'db>; 2] {
    let generic = TypeData::Generic(generic);
    [
        TypeSubstitution {
            generic: TypeData::instance_of(db, generic, Box::default()),
            replacement,
        },
        TypeSubstitution {
            generic,
            replacement,
        },
    ]
}

/// Returns the substitutions replacing a declared type parameter with
/// `replacement`, together with the parameter's generic.
///
/// Built-in types declare their type parameters as canonical global handles,
/// which their members also use to refer to them.
pub(crate) fn type_parameter_substitutions<'db>(
    db: &'db dyn TypeDb,
    type_parameter: TypeData<'db>,
    replacement: TypeData<'db>,
) -> Option<(
    InternedGenericTypeParameter<'db>,
    Vec<TypeSubstitution<'db>>,
)> {
    let generic = referenced_generic(db, type_parameter)?;
    let mut substitutions = generic_substitutions(db, generic, replacement).to_vec();
    if matches!(type_parameter, TypeData::GlobalType(_)) {
        substitutions.push(TypeSubstitution {
            generic: TypeData::instance_of(db, type_parameter, Box::default()),
            replacement,
        });
        substitutions.push(TypeSubstitution {
            generic: type_parameter,
            replacement,
        });
    }
    Some((generic, substitutions))
}

/// Applies `substitutions` simultaneously, or returns `None` if substitution
/// fails.
pub(crate) fn substitute_all<'db>(
    db: &'db dyn TypeDb,
    ty: TypeData<'db>,
    substitutions: impl IntoIterator<Item = TypeSubstitution<'db>>,
) -> Option<TypeData<'db>> {
    let substitutions = substitutions.into_iter().collect::<Vec<_>>();
    ty.substitute_types(db, &substitutions).into_result().ok()
}

/// Members of a type that is compared structurally.
struct StructuralMembers<'db> {
    /// Name, type, and optionality of each named member.
    named: Vec<(biome_rowan::Text, TypeData<'db>, bool)>,
    /// Whether the type has other members, such as members with symbol keys,
    /// index or call signatures, or inherited members.
    has_unlisted_members: bool,
}

impl StructuralMembers<'_> {
    /// Returns whether every member is a known optional property.
    ///
    /// A source that shares no property with such a type isn't assignable to
    /// it, even though no required property is missing. This conditional
    /// selects `"no"`:
    ///
    /// ```ts
    /// type Weak = { a: number } extends { b?: string } ? "yes" : "no";
    /// ```
    fn is_weak(&self) -> bool {
        !self.has_unlisted_members
            && !self.named.is_empty()
            && self.named.iter().all(|(_, _, is_optional)| *is_optional)
    }
}

/// Whether the values of an object type are functions, because the type has a
/// call or construct signature.
enum Signatures {
    Present,
    Absent,
    /// The type may have a signature that isn't listed, or inherit one from a
    /// parent that can't be inspected.
    Unknown,
}

/// Outcome of looking up a member while comparing object types.
enum MemberLookup<'db> {
    Found {
        ty: TypeData<'db>,
        is_optional: bool,
    },
    Missing,
    Unknown,
}

/// Variance of the position where an `infer` generic matched a type, which
/// decides how several matches of one generic combine.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum InferencePosition {
    /// A position such as a property type, a return type, or any type inside
    /// the parameters of a method. The generic is inferred as the union of its
    /// covariant matches:
    ///
    /// ```ts
    /// type Co<T> = T extends { a: infer U; b: infer U } ? U : never;
    /// type Method<T> = T extends { a(x: infer U): void; b(x: infer U): void } ? U : never;
    /// // string | number
    /// type FromProperties = Co<{ a: string; b: number }>;
    /// // string | number
    /// type FromMethods = Method<{ a(x: string): void; b(x: number): void }>;
    /// ```
    Covariant,
    /// A position such as the parameter of a function type. A generic with
    /// only contravariant matches is inferred as their intersection:
    ///
    /// ```ts
    /// type Contra<T> =
    ///     T extends { a: (x: infer U) => void; b: (x: infer U) => void } ? U : never;
    /// // never, the intersection of string and number
    /// type FromParameters = Contra<{ a: (x: string) => void; b: (x: number) => void }>;
    /// ```
    ///
    /// A generic with covariant matches as well is inferred from those, and the
    /// result must be assignable to every contravariant match. See
    /// [`AssignabilityCheck::are_inferences_consistent`] for an example.
    Contravariant,
}

/// Whether a candidate of an `infer` generic counts when the generic has
/// other candidates.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum InferencePriority {
    /// A type that the generic matched.
    Direct,
    /// A type that a bare `infer` member of a union received although other
    /// members matched it. Such candidates count only when the generic has no
    /// direct candidates. Here `U` is inferred as `number`:
    ///
    /// ```ts
    /// type Fallback = [string, number] extends [string | (infer U), infer U] ? U : never;
    /// ```
    Fallback,
}

/// How the parameters of two function signatures are compared.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ParameterVariance {
    /// The target parameter must be assignable to the source parameter.
    Contravariant,
    /// Assignability in either direction is enough, as for methods.
    Bivariant,
}

/// A type matched by an `infer` generic.
struct InferenceCandidate<'db> {
    generic: InternedGenericTypeParameter<'db>,
    ty: TypeData<'db>,
    position: InferencePosition,
    priority: InferencePriority,
}

/// Outcome of an assignability check.
///
/// A check answers only when it is certain. A conditional whose check is
/// undetermined stays unevaluated.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Assignability {
    Assignable,
    NotAssignable,
    /// The available type information can't decide the relation. For example,
    /// a class can declare private members, which aren't modeled, so matching
    /// members don't prove that an unrelated class is assignable to it.
    Undetermined,
}

impl Assignability {
    /// Combines the outcomes of two checks that must both succeed. A mismatch
    /// in either is conclusive, even when the other is undetermined.
    fn and(self, other: Self) -> Self {
        match (self, other) {
            (Self::NotAssignable, _) | (_, Self::NotAssignable) => Self::NotAssignable,
            (Self::Assignable, Self::Assignable) => Self::Assignable,
            _ => Self::Undetermined,
        }
    }
}

impl From<bool> for Assignability {
    fn from(is_assignable: bool) -> Self {
        if is_assignable {
            Self::Assignable
        } else {
            Self::NotAssignable
        }
    }
}

/// Decides assignability between normalized types, recording the types
/// matched by `infer` generics along the way.
struct AssignabilityCheck<'db, 'a> {
    db: &'db dyn TypeDb,
    resolve: &'a mut TypeResolver<'db, 'a>,
    infer_types: Vec<InternedGenericTypeParameter<'db>>,
    /// Types matched by each `infer` generic, in the order they were found.
    candidates: Vec<InferenceCandidate<'db>>,
    /// Position recorded for an `infer` generic found as the source of a
    /// comparison. Such a generic is in a parameter, which is contravariant
    /// unless the parameter belongs to a method.
    parameter_position: InferencePosition,
    remaining_steps: usize,
}

impl<'db, 'a> AssignabilityCheck<'db, 'a> {
    fn new(
        db: &'db dyn TypeDb,
        resolve: &'a mut TypeResolver<'db, 'a>,
        infer_types: &[TypeData<'db>],
    ) -> Self {
        Self {
            db,
            resolve,
            infer_types: infer_types
                .iter()
                .filter_map(|ty| referenced_generic(db, *ty))
                .collect(),
            candidates: Vec::new(),
            parameter_position: InferencePosition::Contravariant,
            remaining_steps: MAX_RELATION_STEPS,
        }
    }

    /// Resolves handles and removes wrappers that don't affect assignability.
    fn resolve(&mut self, mut ty: TypeData<'db>) -> TypeData<'db> {
        for _ in 0..MAX_MEMBER_LOOKUP_DEPTH {
            let resolved = (self.resolve)(ty).expand_global_local(self.db);
            let next = match resolved {
                TypeData::TypeofType(typeof_type) => typeof_type.ty(self.db),
                TypeData::TypeofValue(typeof_value) => typeof_value.ty(self.db),
                TypeData::MergedReference(reference) => match reference.ty(self.db) {
                    Some(ty) => ty,
                    None => return resolved,
                },
                TypeData::InstanceOf(instance)
                    if instance.type_parameters(self.db).is_empty()
                        && instance.ty(self.db).should_flatten_instance(&[]) =>
                {
                    instance.ty(self.db)
                }
                // Operands substituted lazily, as by member lookup, are not
                // normalized, so `keyof` may still be unevaluated.
                TypeData::TypeOperator(operator)
                    if operator.operator(self.db) == TypeOperator::Keyof =>
                {
                    match keyof(self.db, operator.ty(self.db), self.resolve) {
                        Some(keys) => keys,
                        None => return resolved,
                    }
                }
                _ => return resolved,
            };
            if next == ty {
                return next;
            }
            ty = next;
        }
        ty
    }

    fn infer_type(&self, ty: TypeData<'db>) -> Option<InternedGenericTypeParameter<'db>> {
        referenced_generic(self.db, ty).filter(|generic| self.infer_types.contains(generic))
    }

    /// Records that the `infer` generic matched `ty` in the given position.
    fn bind(
        &mut self,
        generic: InternedGenericTypeParameter<'db>,
        ty: TypeData<'db>,
        position: InferencePosition,
    ) -> Assignability {
        self.bind_with_priority(generic, ty, position, InferencePriority::Direct)
    }

    /// Records a candidate of the `infer` generic, unless `ty` violates the
    /// generic's constraint.
    fn bind_with_priority(
        &mut self,
        generic: InternedGenericTypeParameter<'db>,
        ty: TypeData<'db>,
        position: InferencePosition,
        priority: InferencePriority,
    ) -> Assignability {
        if let Some(constraint) = generic.constraint(self.db) {
            match self.is_assignable(ty, constraint) {
                Assignability::Assignable => {}
                result => return result,
            }
        }
        self.candidates.push(InferenceCandidate {
            generic,
            ty,
            position,
            priority,
        });
        Assignability::Assignable
    }

    /// Returns the types the `infer` generic matched in the given position.
    ///
    /// [`InferencePriority::Fallback`] candidates are returned only when the
    /// generic has no direct candidates in any position.
    fn candidate_types(
        &self,
        generic: InternedGenericTypeParameter<'db>,
        position: InferencePosition,
    ) -> Vec<TypeData<'db>> {
        let candidates = || {
            self.candidates
                .iter()
                .filter(move |candidate| candidate.generic == generic)
        };
        let priority =
            if candidates().any(|candidate| candidate.priority == InferencePriority::Direct) {
                InferencePriority::Direct
            } else {
                InferencePriority::Fallback
            };
        candidates()
            .filter(|candidate| candidate.position == position && candidate.priority == priority)
            .map(|candidate| candidate.ty)
            .collect()
    }

    /// Checks whether the type inferred for each `infer` generic satisfies
    /// the positions it was not inferred from.
    ///
    /// A generic matched in both kinds of position is inferred from its
    /// covariant matches, which must then be assignable to every contravariant
    /// match. Here `U` is inferred as `string`, which `(x: number) => void`
    /// doesn't accept, so the conditional selects `"no"`:
    ///
    /// ```ts
    /// type Mixed = { a: string; b: (x: number) => void } extends
    ///     { a: infer U; b: (x: infer U) => void } ? "yes" : "no";
    /// ```
    fn are_inferences_consistent(&mut self) -> Assignability {
        let mut pairs = Vec::new();
        for generic in &self.infer_types {
            let contravariant = self.candidate_types(*generic, InferencePosition::Contravariant);
            let covariant = self.candidate_types(*generic, InferencePosition::Covariant);
            if contravariant.is_empty() || covariant.is_empty() {
                continue;
            }
            let inferred = TypeData::union_from_types(self.db, covariant);
            pairs.extend(contravariant.into_iter().map(|ty| (inferred, ty)));
        }
        self.all_assignable(pairs)
    }

    /// Replaces the `infer` generics in `true_type` with the types they matched.
    ///
    /// A generic matched in a covariant position becomes the union of those
    /// matches. A generic matched only in contravariant positions becomes the
    /// intersection of those matches. An unmatched generic becomes its
    /// constraint, or unknown without one.
    fn instantiate_true_type(&mut self, true_type: TypeData<'db>) -> Option<EvaluatedExtends<'db>> {
        if self.infer_types.is_empty() {
            return Some(EvaluatedExtends {
                ty: true_type,
                needs_normalization: false,
            });
        }

        let db = self.db;
        let mut substitutions = Vec::with_capacity(self.infer_types.len() * 2);
        for generic in &self.infer_types {
            let covariant = self.candidate_types(*generic, InferencePosition::Covariant);
            let contravariant = self.candidate_types(*generic, InferencePosition::Contravariant);
            let replacement = if !covariant.is_empty() {
                TypeData::union_from_types(db, covariant)
            } else if !contravariant.is_empty() {
                TypeData::intersection_from_types(db, contravariant)
            } else {
                generic.constraint(db).unwrap_or(TypeData::Unknown)
            };
            substitutions.extend(generic_substitutions(db, *generic, replacement));
        }
        Some(EvaluatedExtends {
            ty: substitute_all(db, true_type, substitutions)?,
            needs_normalization: true,
        })
    }

    fn is_assignable(&mut self, source: TypeData<'db>, target: TypeData<'db>) -> Assignability {
        if self.remaining_steps == 0 {
            return Assignability::Undetermined;
        }
        self.remaining_steps -= 1;

        // Resolving a simplified signature loses the handle that identifies it.
        let is_simplified = is_simplified_signature(source) || is_simplified_signature(target);
        let source = self.resolve(source);
        let target = self.resolve(target);
        if let Some(generic) = self.infer_type(target) {
            return self.bind(generic, source, InferencePosition::Covariant);
        }
        // Parameters are compared in the opposite direction, so an `infer`
        // generic in a parameter position appears as the source.
        if let Some(generic) = self.infer_type(source) {
            return self.bind(generic, target, self.parameter_position);
        }
        if source == target && !may_contain_undetermined_type(self.db, source) {
            return Assignability::Assignable;
        }
        if is_simplified && !matches!(target, TypeData::AnyKeyword | TypeData::UnknownKeyword) {
            return Assignability::Undetermined;
        }

        match (source, target) {
            (_, TypeData::AnyKeyword | TypeData::UnknownKeyword) | (TypeData::NeverKeyword, _) => {
                return Assignability::Assignable;
            }
            (TypeData::AnyKeyword, _) => {
                return Assignability::from(target != TypeData::NeverKeyword);
            }
            // A bare `infer` member is inferred from the whole source, so a
            // union source isn't split here.
            (_, TypeData::Union(union)) if self.has_bare_infer_member(union) => {
                return self.is_assignable_to_union(source, union);
            }
            (TypeData::Union(union), _) => {
                return self.all_assignable(union.types(self.db).iter().map(|ty| (*ty, target)));
            }
            (TypeData::Boolean, TypeData::Union(_)) => {
                let literals = boolean_literals(self.db);
                return self.all_assignable(literals.map(|literal| (literal, target)));
            }
            (TypeData::UnknownKeyword, TypeData::Union(union))
                if self.accepts_every_value(union) =>
            {
                return Assignability::Assignable;
            }
            (_, TypeData::Union(union)) => {
                return self.is_assignable_to_union(source, union);
            }
            (_, TypeData::Intersection(intersection)) => {
                return self.is_assignable_to_intersection(source, intersection);
            }
            (TypeData::Intersection(intersection), _) => {
                // Members of an intersection may combine to satisfy an object
                // type, so failing every member is only conclusive for primitives.
                let result =
                    self.any_assignable(intersection.types(self.db).iter().map(|ty| (*ty, target)));
                return match result {
                    Assignability::NotAssignable if primitive_kind(self.db, target).is_none() => {
                        Assignability::Undetermined
                    }
                    result => result,
                };
            }
            _ => {}
        }

        if let Some(generic) = referenced_generic(self.db, source) {
            let Some(constraint) = generic.constraint(self.db) else {
                return Assignability::Undetermined;
            };
            // A generic can be narrower than its constraint, so only a match
            // is conclusive.
            return match self.is_assignable(constraint, target) {
                Assignability::Assignable => Assignability::Assignable,
                _ => Assignability::Undetermined,
            };
        }
        if referenced_generic(self.db, target).is_some() {
            return Assignability::Undetermined;
        }
        if is_unresolved(self.db, source) || is_unresolved(self.db, target) {
            return Assignability::Undetermined;
        }
        if source == TypeData::UnknownKeyword {
            return Assignability::NotAssignable;
        }

        let (source, source_is_readonly) = strip_readonly(self.db, source);
        let (target, target_is_readonly) = strip_readonly(self.db, target);
        if target == TypeData::NeverKeyword {
            return Assignability::NotAssignable;
        }

        match (
            primitive_kind(self.db, source),
            primitive_kind(self.db, target),
        ) {
            (Some(source_kind), Some(target_kind)) => {
                return compare_literal_values(self.db, source, target)
                    .unwrap_or_else(|| source_kind.is_assignable_to(target_kind));
            }
            (Some(source_kind), None) => {
                return self.is_primitive_assignable_to_object(source_kind, target);
            }
            (None, Some(_)) => {
                return if self.is_object_like(source) {
                    Assignability::NotAssignable
                } else {
                    Assignability::Undetermined
                };
            }
            (None, None) => {}
        }

        if target == TypeData::ObjectKeyword {
            return if self.is_object_like(source) {
                Assignability::Assignable
            } else {
                Assignability::Undetermined
            };
        }
        if source == TypeData::ObjectKeyword {
            return if self.is_empty_object(target) {
                Assignability::Assignable
            } else {
                Assignability::Undetermined
            };
        }

        if !source_is_readonly || target_is_readonly {
            // Readonly arrays and tuples are not assignable to mutable ones.
        } else if matches!(target, TypeData::Tuple(_)) || self.array_element(target).is_some() {
            return Assignability::NotAssignable;
        }

        match (source, target) {
            (TypeData::Tuple(source), TypeData::Tuple(target)) => {
                self.is_tuple_assignable(source, target)
            }
            (TypeData::Tuple(source), _) if let Some(element) = self.array_element(target) => {
                if source.is_inferred_array(self.db)
                    || source
                        .elements(self.db)
                        .iter()
                        .any(|element| element.is_rest)
                {
                    return Assignability::Undetermined;
                }
                self.all_assignable(
                    source
                        .elements(self.db)
                        .iter()
                        .map(|source| (source.ty, element)),
                )
            }
            (_, TypeData::Tuple(target)) if self.array_element(source).is_some() => {
                match target.elements(self.db).as_ref() {
                    [rest] if rest.is_rest => self.is_assignable(source, rest.ty),
                    elements
                        if elements
                            .iter()
                            .any(|element| !element.is_optional && !element.is_rest) =>
                    {
                        Assignability::NotAssignable
                    }
                    _ => Assignability::Undetermined,
                }
            }
            (TypeData::Function(source), TypeData::Function(target)) => {
                self.is_function_assignable(source, target, ParameterVariance::Contravariant)
            }
            (TypeData::Class(class), TypeData::Constructor(constructor)) => {
                let instance =
                    TypeData::instance_of(self.db, TypeData::Class(class), Box::default());
                let parameters = self.class_constructor_parameters(class);
                self.is_constructor_assignable(instance, parameters, constructor)
            }
            (TypeData::Constructor(source), TypeData::Constructor(target)) => {
                let Some(source_return) = source.return_type(self.db) else {
                    return Assignability::Undetermined;
                };
                let parameters = source
                    .type_parameters(self.db)
                    .is_empty()
                    .then(|| constructor_parameters(self.db, source));
                self.is_constructor_assignable(source_return, parameters, target)
            }
            (TypeData::Function(_), TypeData::Constructor(_))
            | (TypeData::Class(_) | TypeData::Constructor(_), TypeData::Function(_)) => {
                Assignability::NotAssignable
            }
            (TypeData::InstanceOf(source), TypeData::InstanceOf(target))
                if self.same_instance_target(source, target) =>
            {
                self.are_type_arguments_assignable(source, target)
            }
            _ => self.is_structurally_assignable(source, target),
        }
    }

    /// Checks whether every pair is assignable.
    ///
    /// A pair that isn't assignable is conclusive, even when other pairs are
    /// undetermined. Otherwise, any undetermined pair makes the outcome
    /// undetermined.
    fn all_assignable(
        &mut self,
        pairs: impl IntoIterator<Item = (TypeData<'db>, TypeData<'db>)>,
    ) -> Assignability {
        let mut result = Assignability::Assignable;
        for (source, target) in pairs {
            match self.is_assignable(source, target) {
                Assignability::Assignable => {}
                Assignability::NotAssignable => return Assignability::NotAssignable,
                Assignability::Undetermined => result = Assignability::Undetermined,
            }
        }
        result
    }

    /// Checks whether `source` is assignable to every member of an
    /// intersection.
    ///
    /// A member of `any` makes the whole intersection `any`. A mismatch stays
    /// undetermined while a member is generic, because that member may become
    /// `any`, which is how `0 extends 1 & T` detects `any`. A mismatch also
    /// stays undetermined when a member has only optional properties. Such a
    /// member only needs a property shared with the whole intersection. Here
    /// `{ value: number }` shares no property with `{ b?: string }`, yet it is
    /// assignable to the intersection because it shares `value` with `Named`:
    ///
    /// ```ts
    /// interface Named { value: number }
    /// type Shared = { value: number } extends Named & { b?: string } ? "yes" : "no";
    /// ```
    fn is_assignable_to_intersection(
        &mut self,
        source: TypeData<'db>,
        intersection: InternedIntersection<'db>,
    ) -> Assignability {
        let members = intersection.types(self.db);
        if members
            .iter()
            .any(|member| self.resolve(*member) == TypeData::AnyKeyword)
        {
            return Assignability::Assignable;
        }
        let result = self.all_assignable(members.iter().map(|member| (source, *member)));
        if result == Assignability::NotAssignable
            && (may_refer_to_free_generic(
                self.db,
                TypeData::Intersection(intersection),
                &self.infer_types,
            ) || members.iter().any(|member| self.is_weak(*member)))
        {
            return Assignability::Undetermined;
        }
        result
    }

    /// Returns whether `ty` is an object type with only optional properties.
    fn is_weak(&mut self, ty: TypeData<'db>) -> bool {
        let ty = self.resolve(ty);
        self.structural_members(ty)
            .is_some_and(|members| members.is_weak())
    }

    /// Checks whether any pair is assignable.
    ///
    /// An assignable pair is conclusive, even when other pairs are
    /// undetermined. Otherwise, any undetermined pair makes the outcome
    /// undetermined. Inference candidates recorded while trying an
    /// unsuccessful pair are discarded.
    fn any_assignable(
        &mut self,
        pairs: impl IntoIterator<Item = (TypeData<'db>, TypeData<'db>)>,
    ) -> Assignability {
        let mut result = Assignability::NotAssignable;
        for (source, target) in pairs {
            let candidates = self.candidates.len();
            match self.is_assignable(source, target) {
                Assignability::Assignable => return Assignability::Assignable,
                Assignability::NotAssignable => {}
                Assignability::Undetermined => result = Assignability::Undetermined,
            }
            self.candidates.truncate(candidates);
        }
        result
    }

    /// Returns whether a member of the union is a bare `infer` generic.
    fn has_bare_infer_member(&self, union: InternedUnion<'db>) -> bool {
        !self.infer_types.is_empty()
            && union
                .types(self.db)
                .iter()
                .any(|member| self.infer_type(*member).is_some())
    }

    /// Checks whether `source` is assignable to a member of a union.
    ///
    /// See [`Self::is_assignable_to_any_member`], and
    /// [`Self::is_assignable_with_bare_infer`] for a union with bare `infer`
    /// members.
    fn is_assignable_to_union(
        &mut self,
        source: TypeData<'db>,
        union: InternedUnion<'db>,
    ) -> Assignability {
        let (bare, members): (Vec<_>, Vec<_>) = union
            .types(self.db)
            .iter()
            .copied()
            .partition(|member| self.infer_type(*member).is_some());
        if bare.is_empty() {
            self.is_assignable_to_any_member(source, &members)
        } else {
            self.is_assignable_with_bare_infer(source, &bare, &members)
        }
    }

    /// Checks whether `source` is assignable to one of `members`.
    ///
    /// Like TypeScript, `infer` generics take their candidates from every
    /// member that `source` is assignable to. Here `P` is inferred as
    /// `string | number`:
    ///
    /// ```ts
    /// type Payload<T> = T extends { data: infer P } | { error: infer P } ? P : never;
    /// type Both = Payload<{ data: string; error: number }>;
    /// ```
    ///
    /// A member that may refer to an `infer` generic but remains undetermined
    /// might add candidates, so the outcome is then undetermined.
    fn is_assignable_to_any_member(
        &mut self,
        source: TypeData<'db>,
        members: &[TypeData<'db>],
    ) -> Assignability {
        let mut result = Assignability::NotAssignable;
        let mut may_miss_candidates = false;
        for &member in members {
            let may_infer =
                !self.infer_types.is_empty() && may_refer_to_free_generic(self.db, member, &[]);
            if result == Assignability::Assignable && !may_infer {
                continue;
            }
            let candidates = self.candidates.len();
            match self.is_assignable(source, member) {
                Assignability::Assignable => {
                    result = Assignability::Assignable;
                    continue;
                }
                Assignability::NotAssignable => {}
                Assignability::Undetermined => {
                    may_miss_candidates |= may_infer;
                    if result == Assignability::NotAssignable {
                        result = Assignability::Undetermined;
                    }
                }
            }
            self.candidates.truncate(candidates);
        }
        match result {
            Assignability::Assignable if may_miss_candidates => Assignability::Undetermined,
            result => result,
        }
    }

    /// Checks whether `source` is assignable to a union whose `bare` members
    /// are `infer` generics, and infers those generics as TypeScript does.
    ///
    /// A bare generic accepts anything, so the outcome only depends on its
    /// constraint. Each member of a union `source` is considered separately:
    ///
    /// - A source member identical to one of the other `members` gives the
    ///   bare generics nothing. Neither does a string or number literal when
    ///   `members` include its primitive.
    /// - A sole bare generic receives the remaining source members that no
    ///   other member infers anything from. Here `U` is inferred as `number`:
    ///
    ///   ```ts
    ///   type Rest = ("a" | number) extends string | (infer U) ? U : never;
    ///   ```
    ///
    /// - Otherwise the bare generics receive the remaining source members, or
    ///   the whole source if none remain, as an
    ///   [`InferencePriority::Fallback`] candidate. Here `U` is inferred as
    ///   `string`:
    ///
    ///   ```ts
    ///   type Whole = string extends string | (infer U) ? U : never;
    ///   ```
    ///
    /// Whether two object types are identical isn't modeled, so an object
    /// source member that other members accept without inferring anything
    /// leaves the outcome undetermined.
    fn is_assignable_with_bare_infer(
        &mut self,
        source: TypeData<'db>,
        bare: &[TypeData<'db>],
        members: &[TypeData<'db>],
    ) -> Assignability {
        let sources = source
            .distributive_members(self.db)
            .filter(|sources| !sources.is_empty())
            .unwrap_or_else(|| vec![source]);
        let mut remaining = Vec::new();
        let mut unmatched = Vec::new();
        // Source members that no other member accepts.
        let mut rejected = Vec::new();
        for member in sources {
            if members
                .iter()
                .any(|target| self.is_identical_member(member, *target))
            {
                continue;
            }
            remaining.push(member);
            let candidates = self.candidates.len();
            match self.is_assignable_to_any_member(member, members) {
                Assignability::Assignable if self.candidates.len() > candidates => {}
                Assignability::Assignable
                    if primitive_kind(self.db, self.resolve(member)).is_some() =>
                {
                    unmatched.push(member);
                }
                Assignability::NotAssignable => {
                    unmatched.push(member);
                    rejected.push(member);
                }
                _ => return Assignability::Undetermined,
            }
        }

        let generics: Vec<_> = bare
            .iter()
            .filter_map(|member| self.infer_type(*member))
            .collect();
        if let [generic] = generics.as_slice() {
            let (ty, priority) = if !unmatched.is_empty() {
                let ty = TypeData::union_from_types(self.db, unmatched);
                (ty, InferencePriority::Direct)
            } else if remaining.is_empty() {
                (source, InferencePriority::Fallback)
            } else {
                let ty = TypeData::union_from_types(self.db, remaining);
                (ty, InferencePriority::Fallback)
            };
            let position = InferencePosition::Covariant;
            return match self.bind_with_priority(*generic, ty, position, priority) {
                // TypeScript replaces a candidate that violates the constraint
                // with the constraint, which must then accept the source
                // members that no other member accepts.
                Assignability::NotAssignable => match generic.constraint(self.db) {
                    Some(constraint) => {
                        self.all_assignable(rejected.into_iter().map(|member| (member, constraint)))
                    }
                    None => Assignability::NotAssignable,
                },
                result => result,
            };
        }

        let ty = if remaining.is_empty() {
            source
        } else {
            TypeData::union_from_types(self.db, remaining)
        };
        let mut result = Assignability::Assignable;
        for generic in generics {
            let position = InferencePosition::Covariant;
            match self.bind_with_priority(generic, ty, position, InferencePriority::Fallback) {
                Assignability::Assignable => {}
                _ => result = Assignability::Undetermined,
            }
        }
        result
    }

    /// Returns whether TypeScript matches the source member `source` with the
    /// target member `target` before inferring bare `infer` members: when
    /// both are the same type, or when `source` is a string or number literal
    /// of the primitive `target`.
    fn is_identical_member(&mut self, source: TypeData<'db>, target: TypeData<'db>) -> bool {
        let source = self.resolve(source);
        let target = self.resolve(target);
        if source == target {
            return true;
        }
        let (Some(source_kind), Some(target_kind)) = (
            primitive_kind(self.db, source),
            primitive_kind(self.db, target),
        ) else {
            return false;
        };
        let is_literal_of_target = source_kind.is_literal
            && !source_kind.is_template
            && !target_kind.is_literal
            && source_kind.primitive == target_kind.primitive
            && matches!(source_kind.primitive, Primitive::Number | Primitive::String);
        is_literal_of_target
            || compare_literal_values(self.db, source, target) == Some(Assignability::Assignable)
    }

    /// Returns whether a union includes `{}`, `null`, and `undefined`, which
    /// together accept every value, so even `unknown` is assignable to it.
    fn accepts_every_value(&mut self, union: InternedUnion<'db>) -> bool {
        let (mut has_empty_object, mut has_null, mut has_undefined) = (false, false, false);
        for member in union.types(self.db) {
            match self.resolve(*member) {
                TypeData::Null => has_null = true,
                TypeData::Undefined => has_undefined = true,
                member => has_empty_object |= self.is_empty_object(member),
            }
        }
        has_empty_object && has_null && has_undefined
    }

    fn is_primitive_assignable_to_object(
        &mut self,
        source: PrimitiveKind,
        target: TypeData<'db>,
    ) -> Assignability {
        if matches!(
            source.primitive,
            Primitive::Null | Primitive::Undefined | Primitive::Void
        ) {
            return Assignability::NotAssignable;
        }
        if self.is_empty_object(target) {
            return Assignability::Assignable;
        }
        match target {
            TypeData::ObjectKeyword
            | TypeData::Tuple(_)
            | TypeData::Function(_)
            | TypeData::Constructor(_)
            | TypeData::Class(_) => Assignability::NotAssignable,
            // Primitives have apparent members, such as a string's `length`,
            // so they are assignable to `Object` and to their wrapper types.
            // Other built-in types are compared with the wrapper's members.
            TypeData::InstanceOf(instance) => {
                let Some(name) = self.builtin_name(instance.ty(self.db)) else {
                    return Assignability::Undetermined;
                };
                if name == "Object" || Some(name.text()) == source.primitive.wrapper_name() {
                    return Assignability::Assignable;
                }
                let Some(wrapper) = source.primitive.wrapper_type() else {
                    return Assignability::Undetermined;
                };
                let wrapper = TypeData::instance_of(self.db, wrapper, Box::default());
                self.is_structurally_assignable(wrapper, target)
            }
            _ => Assignability::Undetermined,
        }
    }

    /// See [`primitive_can_satisfy`].
    fn primitive_can_satisfy(&mut self, kind: TypeofKind, target: TypeData<'db>) -> Assignability {
        let apparent = match kind {
            TypeofKind::Undefined => return Assignability::NotAssignable,
            TypeofKind::Object => return Assignability::Undetermined,
            TypeofKind::BigInt => TypeData::BigInt,
            TypeofKind::Boolean => TypeData::Boolean,
            TypeofKind::Number => TypeData::Number,
            TypeofKind::String => TypeData::String,
            TypeofKind::Symbol => TypeData::Symbol,
            TypeofKind::Function => TypeData::instance_of(
                self.db,
                TypeData::GlobalType(FUNCTION_ID_GLOBAL_TYPE_ID),
                Box::default(),
            ),
        };
        let target = self.resolve(target);
        if self.is_empty_object(target) {
            return Assignability::Assignable;
        }
        match (kind, self.signatures(target, 0)) {
            (TypeofKind::Function, Signatures::Present | Signatures::Unknown) => {
                return Assignability::Undetermined;
            }
            (_, Signatures::Present) => return Assignability::NotAssignable,
            _ => {}
        }
        let Some(target_members) = self.structural_members(target) else {
            return Assignability::Undetermined;
        };
        if target_members.named.len() > MAX_STRUCTURAL_MEMBERS {
            return Assignability::Undetermined;
        }

        let target_is_weak = target_members.is_weak();
        let mut found = Vec::with_capacity(target_members.named.len());
        let mut may_share_member = false;
        let mut result = if target_members.has_unlisted_members {
            Assignability::Undetermined
        } else {
            Assignability::Assignable
        };
        for (name, target_ty, target_is_optional) in target_members.named {
            match self.find_primitive_member(kind, &name) {
                MemberLookup::Found {
                    ty: source_ty,
                    is_optional: source_is_optional,
                } => {
                    may_share_member = true;
                    if source_is_optional && !target_is_optional {
                        result = Assignability::Undetermined;
                    }
                    found.push((name, source_ty, target_ty, target_is_optional));
                }
                MemberLookup::Missing if target_is_optional => {}
                MemberLookup::Missing => return Assignability::NotAssignable,
                MemberLookup::Unknown => {
                    may_share_member = true;
                    result = Assignability::Undetermined;
                }
            }
        }
        if target_is_weak && !may_share_member {
            return Assignability::NotAssignable;
        }
        if result != Assignability::Assignable {
            return result;
        }

        for (name, source_ty, target_ty, target_is_optional) in found {
            let target_ty = if target_is_optional {
                with_undefined(self.db, target_ty)
            } else {
                target_ty
            };
            if self.is_member_assignable(source_ty, target_ty, &name) != Assignability::Assignable {
                return Assignability::Undetermined;
            }
        }
        if self.is_unrelated_class_instance(apparent, target) {
            return Assignability::Undetermined;
        }
        Assignability::Assignable
    }

    /// Returns whether the values of the object type `ty` have a call or
    /// construct signature, of their own or from a parent, which `depth`
    /// counts.
    ///
    /// An instance of a class has one only through a built-in parent class
    /// such as `Function`. The signatures listed on a built-in class, such as
    /// the call signature of `String`, belong to its constructor.
    fn signatures(&mut self, ty: TypeData<'db>, depth: usize) -> Signatures {
        if depth > MAX_MEMBER_LOOKUP_DEPTH {
            return Signatures::Unknown;
        }
        let ty = match self.resolve(ty) {
            TypeData::InstanceOf(instance) => self.resolve(instance.ty(self.db)),
            ty => ty,
        };
        let (members, parents, mut result) = match ty.expand_canonical_global(self.db) {
            TypeData::Class(class) => {
                if depth > 0 && class.is_builtin(self.db) {
                    return Signatures::Unknown;
                }
                return match class.extends(self.db) {
                    Some(parent) => self.signatures(parent, depth + 1),
                    None => Signatures::Absent,
                };
            }
            TypeData::Object(object) => (
                object.members(self.db).as_ref(),
                object.prototype(self.db).into_iter().collect(),
                if object.has_unknown_members(self.db) {
                    Signatures::Unknown
                } else {
                    Signatures::Absent
                },
            ),
            TypeData::Literal(literal) => match literal.literal(self.db) {
                Literal::Object(members) => (members.as_ref(), Vec::new(), Signatures::Absent),
                _ => return Signatures::Unknown,
            },
            TypeData::Interface(interface) => (
                interface.members(self.db).as_ref(),
                interface.extends(self.db).to_vec(),
                Signatures::Absent,
            ),
            _ => return Signatures::Unknown,
        };
        if members
            .iter()
            .any(|member| member.kind.is_call_signature() || member.kind.is_constructor())
        {
            return Signatures::Present;
        }
        for parent in parents {
            match self.signatures(parent, depth + 1) {
                Signatures::Present => return Signatures::Present,
                Signatures::Absent => {}
                Signatures::Unknown => result = Signatures::Unknown,
            }
        }
        result
    }

    /// Looks up a member of the apparent type of a value whose `typeof` is
    /// `kind`, as described by [`primitive_can_satisfy`].
    fn find_primitive_member(&mut self, kind: TypeofKind, name: &str) -> MemberLookup<'db> {
        if members_from_newer_libraries(kind).contains(&name) {
            return MemberLookup::Unknown;
        }
        let wrapper = match kind {
            TypeofKind::Boolean => Some(BOOLEAN_ID_GLOBAL_TYPE_ID),
            TypeofKind::Function => Some(FUNCTION_ID_GLOBAL_TYPE_ID),
            TypeofKind::Number => Some(NUMBER_ID_GLOBAL_TYPE_ID),
            TypeofKind::String => Some(STRING_ID_GLOBAL_TYPE_ID),
            TypeofKind::Symbol => Some(SYMBOL_ID_GLOBAL_TYPE_ID),
            TypeofKind::BigInt => None,
            TypeofKind::Object | TypeofKind::Undefined => return MemberLookup::Unknown,
        };
        if let Some(wrapper) = wrapper {
            let TypeData::Class(class) =
                TypeData::GlobalType(wrapper).expand_canonical_global(self.db)
            else {
                return MemberLookup::Unknown;
            };
            let members = class.members(self.db);
            if let Some(member) = members
                .iter()
                .find(|member| !member.kind.is_static() && member.kind.has_name(name))
            {
                return MemberLookup::Found {
                    ty: member_type(self.db, member),
                    is_optional: member.kind.is_optional(),
                };
            }
            // Unlike `find_member`, a member keyed by a well-known symbol,
            // such as `[Symbol.iterator]` of `String`, can't have this name.
            let is_numeric_name = name.parse::<f64>().is_ok();
            if members.iter().any(|member| {
                member.kind.index_signature_type().is_some_and(|key| {
                    is_numeric_name || !matches!(key, TypeData::Number | TypeData::Symbol)
                }) || matches!(
                    member.kind,
                    TypeMemberKind::ComputedValue(key) | TypeMemberKind::ConstAssertedComputedValue(key)
                        if well_known_symbol_name(key).is_none()
                )
            }) {
                return MemberLookup::Unknown;
            }
        }
        let object = TypeData::instance_of(
            self.db,
            TypeData::GlobalType(OBJECT_ID_GLOBAL_TYPE_ID),
            Box::default(),
        );
        self.find_member(object, name, 0, &mut FxHashSet::default())
    }

    fn is_object_like(&mut self, mut ty: TypeData<'db>) -> bool {
        for _ in 0..MAX_MEMBER_LOOKUP_DEPTH {
            match ty {
                TypeData::Class(_)
                | TypeData::Constructor(_)
                | TypeData::Function(_)
                | TypeData::Interface(_)
                | TypeData::Object(_)
                | TypeData::ObjectKeyword
                | TypeData::Tuple(_) => return true,
                TypeData::Literal(literal) => {
                    return matches!(
                        literal.literal(self.db),
                        Literal::Object(_) | Literal::RegExp(_)
                    );
                }
                TypeData::InstanceOf(instance) => ty = self.resolve(instance.ty(self.db)),
                TypeData::GlobalType(_) => ty = ty.expand_canonical_global(self.db),
                _ => return false,
            }
        }
        false
    }

    /// Returns whether `ty` is `{}`, which accepts every non-nullish value.
    fn is_empty_object(&mut self, ty: TypeData<'db>) -> bool {
        match ty {
            TypeData::Object(object) => {
                object.prototype(self.db).is_none()
                    && !object.has_unknown_members(self.db)
                    && object.members(self.db).is_empty()
            }
            TypeData::Literal(literal) => {
                matches!(literal.literal(self.db), Literal::Object(members) if members.is_empty())
            }
            TypeData::Interface(interface) => {
                interface.members(self.db).is_empty() && interface.extends(self.db).is_empty()
            }
            _ => false,
        }
    }

    /// Returns the name of a built-in class or interface.
    fn builtin_name(&mut self, ty: TypeData<'db>) -> Option<biome_rowan::Text> {
        let is_global = matches!(ty, TypeData::GlobalType(_));
        match self.resolve(ty).expand_canonical_global(self.db) {
            TypeData::Class(class) if is_global || class.is_builtin(self.db) => {
                class.name(self.db).clone()
            }
            TypeData::Interface(interface) if is_global => Some(interface.name(self.db).clone()),
            _ => None,
        }
    }

    /// Returns the element type of an array instance.
    fn array_element(&mut self, ty: TypeData<'db>) -> Option<TypeData<'db>> {
        let TypeData::InstanceOf(instance) = ty else {
            return None;
        };
        let target = self.resolve(instance.ty(self.db));
        target.is_array_class(self.db).then(|| {
            instance
                .type_parameters(self.db)
                .first()
                .copied()
                .unwrap_or(TypeData::Unknown)
        })
    }

    fn same_instance_target(
        &mut self,
        source: InternedTypeInstance<'db>,
        target: InternedTypeInstance<'db>,
    ) -> bool {
        let source = self.resolve(source.ty(self.db));
        let target = self.resolve(target.ty(self.db));
        source == target
            || source.expand_canonical_global(self.db) == target.expand_canonical_global(self.db)
            // Normalization can intern a second copy of a built-in class.
            || self
                .builtin_name(source)
                .is_some_and(|name| Some(name) == self.builtin_name(target))
    }

    /// Compares two instances of the same type.
    ///
    /// Type arguments are assumed to be covariant for built-in containers such
    /// as `Promise` and `Array`, so a mismatch there is conclusive. Other types
    /// may use their arguments differently, so they are compared by their
    /// members when a target argument is known. A target whose arguments are
    /// all generic, such as `Box<infer U>`, compares the arguments only, and a
    /// mismatch leaves the result undetermined.
    fn are_type_arguments_assignable(
        &mut self,
        source: InternedTypeInstance<'db>,
        target: InternedTypeInstance<'db>,
    ) -> Assignability {
        let is_builtin = self.builtin_name(target.ty(self.db)).is_some();
        if !is_builtin
            && target
                .type_parameters(self.db)
                .iter()
                .any(|argument| !may_refer_to_free_generic(self.db, *argument, &[]))
        {
            return self.is_structurally_assignable(
                TypeData::InstanceOf(source),
                TypeData::InstanceOf(target),
            );
        }
        let pairs = source
            .type_parameters(self.db)
            .iter()
            .copied()
            .zip(target.type_parameters(self.db).iter().copied())
            .collect::<Vec<_>>();
        match self.all_assignable(pairs) {
            Assignability::NotAssignable if !is_builtin => Assignability::Undetermined,
            result => result,
        }
    }

    fn is_tuple_assignable(
        &mut self,
        source: InternedTuple<'db>,
        target: InternedTuple<'db>,
    ) -> Assignability {
        if source.is_inferred_array(self.db) || target.is_inferred_array(self.db) {
            return Assignability::Undetermined;
        }
        let source_elements = source.elements(self.db);
        if source_elements.iter().any(|element| element.is_rest) {
            return Assignability::Undetermined;
        }
        let (fixed, rest) = match target.elements(self.db).as_ref() {
            [fixed @ .., rest] if rest.is_rest => (fixed, Some(rest.ty)),
            fixed => (fixed, None),
        };
        if fixed.iter().any(|element| element.is_rest) {
            return Assignability::Undetermined;
        }

        let required = fixed.iter().filter(|element| !element.is_optional).count();
        if source_elements.len() < required || rest.is_none() && source_elements.len() > fixed.len()
        {
            return Assignability::NotAssignable;
        }
        if source_elements
            .iter()
            .zip(fixed)
            .any(|(source, target)| source.is_optional && !target.is_optional)
        {
            return Assignability::NotAssignable;
        }

        let db = self.db;
        let mut result =
            self.all_assignable(source_elements.iter().zip(fixed).map(|(source, target)| {
                let target_ty = if target.is_optional {
                    with_undefined(db, target.ty)
                } else {
                    target.ty
                };
                (source.ty, target_ty)
            }));
        if result == Assignability::NotAssignable {
            return result;
        }

        if let Some(rest) = rest {
            let remaining = source_elements.get(fixed.len()..).unwrap_or_default();
            let rest_result = if let Some(generic) = self.infer_type(rest) {
                let tuple = TypeData::Tuple(InternedTuple::new(
                    self.db,
                    remaining.to_vec().into_boxed_slice(),
                    false,
                ));
                self.bind(generic, tuple, InferencePosition::Covariant)
            } else if let rest = self.resolve(rest)
                && let Some(element) = self.array_element(rest)
            {
                self.all_assignable(remaining.iter().map(|source| (source.ty, element)))
            } else {
                Assignability::Undetermined
            };
            result = result.and(rest_result);
        }
        result
    }

    /// Compares function signatures.
    ///
    /// Return types are compared covariantly, and parameters as `variance`
    /// says. See [`Self::are_parameters_assignable`] for parameter lists.
    ///
    /// A generic source is compared with its type parameters replaced by
    /// their constraints. TypeScript instead infers them from the target, and
    /// may infer narrower types, so only a match is conclusive. Here `T` is
    /// inferred as `"a"`, which this comparison can't conclude:
    ///
    /// ```ts
    /// declare function id<T extends string>(x: T): T;
    /// type Narrowed = typeof id extends (x: "a") => "a" ? "yes" : "no";
    /// ```
    fn is_function_assignable(
        &mut self,
        source: InternedFunction<'db>,
        target: InternedFunction<'db>,
        variance: ParameterVariance,
    ) -> Assignability {
        if !target.type_parameters(self.db).is_empty() {
            return Assignability::Undetermined;
        }
        if source.type_parameters(self.db).is_empty() {
            return self.is_signature_assignable(source, target, variance);
        }
        let Some(source) = self.erase_type_parameters(source) else {
            return Assignability::Undetermined;
        };
        match self.is_signature_assignable(source, target, variance) {
            Assignability::NotAssignable => Assignability::Undetermined,
            result => result,
        }
    }

    /// Compares function signatures that declare no type parameters.
    fn is_signature_assignable(
        &mut self,
        source: InternedFunction<'db>,
        target: InternedFunction<'db>,
        variance: ParameterVariance,
    ) -> Assignability {
        let return_result = match (source.return_type(self.db), target.return_type(self.db)) {
            (_, ReturnType::Type(TypeData::VoidKeyword)) => Assignability::Assignable,
            (ReturnType::Type(source), ReturnType::Type(target)) => {
                self.is_assignable(*source, *target)
            }
            (ReturnType::Predicate(_), ReturnType::Type(target)) => {
                self.is_assignable(TypeData::Boolean, *target)
            }
            (ReturnType::Asserts(_), ReturnType::Type(target)) => {
                self.is_assignable(TypeData::VoidKeyword, *target)
            }
            (_, ReturnType::Predicate(_) | ReturnType::Asserts(_)) => Assignability::Undetermined,
        };
        if return_result == Assignability::NotAssignable {
            return Assignability::NotAssignable;
        }

        let source_parameters = parameters_without_this(source.parameters(self.db));
        let target_parameters = parameters_without_this(target.parameters(self.db));
        // Every type inside the parameters of a method is a covariant position
        // for inference.
        let outer_position = self.parameter_position;
        if variance == ParameterVariance::Bivariant {
            self.parameter_position = InferencePosition::Covariant;
        }
        let result = self.are_parameters_assignable(
            &source_parameters,
            &target_parameters,
            return_result,
            variance,
        );
        self.parameter_position = outer_position;
        result
    }

    /// Compares parameter lists, combining the outcome with `initial_result`.
    ///
    /// A target rest parameter of `any`, `any[]`, or an `infer` generic accepts
    /// any remaining source parameters; an `infer` generic becomes a tuple of
    /// them, as in `Parameters<F>`.
    fn are_parameters_assignable(
        &mut self,
        source_parameters: &[FunctionParameter<'db>],
        target_parameters: &[FunctionParameter<'db>],
        initial_result: Assignability,
        variance: ParameterVariance,
    ) -> Assignability {
        let (fixed, rest) = match target_parameters {
            [fixed @ .., rest] if rest.is_rest() => (fixed, Some(rest.ty())),
            fixed => (fixed, None),
        };
        if fixed.iter().any(|parameter| parameter.is_rest())
            || source_parameters
                .iter()
                .take(fixed.len())
                .any(|parameter| parameter.is_rest())
        {
            return Assignability::Undetermined;
        }

        let mut result = initial_result;

        for (index, source_parameter) in source_parameters.iter().enumerate() {
            let parameter_result = match fixed.get(index) {
                Some(target_parameter) => match variance {
                    ParameterVariance::Contravariant => {
                        self.is_assignable(target_parameter.ty(), source_parameter.ty())
                    }
                    ParameterVariance::Bivariant => self.any_assignable([
                        (target_parameter.ty(), source_parameter.ty()),
                        (source_parameter.ty(), target_parameter.ty()),
                    ]),
                },
                None if rest.is_some() => continue,
                // A source that requires more arguments than the target
                // passes is not assignable.
                None => Assignability::from(
                    source_parameter.is_optional() || source_parameter.is_rest(),
                ),
            };
            result = result.and(parameter_result);
            if result == Assignability::NotAssignable {
                return result;
            }
        }

        if let Some(rest) = rest {
            let remaining = source_parameters.get(fixed.len()..).unwrap_or_default();
            let rest_result = if let Some(generic) = self.infer_type(rest) {
                let elements: Box<[_]> = remaining
                    .iter()
                    .map(|parameter| TupleElementType {
                        ty: parameter.ty(),
                        name: match parameter {
                            FunctionParameter::Named(named) => Some(named.name.clone()),
                            FunctionParameter::Pattern(_) => None,
                        },
                        is_optional: parameter.is_optional(),
                        is_rest: parameter.is_rest(),
                    })
                    .collect();
                let tuple = TypeData::Tuple(InternedTuple::new(self.db, elements, false));
                self.bind(generic, tuple, self.parameter_position)
            } else if self.accepts_any_arguments(rest) {
                Assignability::Assignable
            } else {
                Assignability::Undetermined
            };
            result = result.and(rest_result);
        }
        result
    }

    /// Returns whether a rest parameter of type `rest` is `any` or `any[]`.
    fn accepts_any_arguments(&mut self, rest: TypeData<'db>) -> bool {
        let rest = self.resolve(rest);
        if rest == TypeData::AnyKeyword {
            return true;
        }
        self.array_element(rest)
            .is_some_and(|element| self.resolve(element) == TypeData::AnyKeyword)
    }

    /// Compares a constructor with the constructor type `target`.
    ///
    /// `source_return` is the type the constructor creates, and
    /// `source_parameters` its parameters. When the parameters aren't known,
    /// the result is determined only for a target that accepts any arguments,
    /// such as `new (...args: any) => any`.
    fn is_constructor_assignable(
        &mut self,
        source_return: TypeData<'db>,
        source_parameters: Option<Vec<FunctionParameter<'db>>>,
        target: InternedConstructor<'db>,
    ) -> Assignability {
        if !target.type_parameters(self.db).is_empty() {
            return Assignability::Undetermined;
        }
        let Some(target_return) = target.return_type(self.db) else {
            return Assignability::Undetermined;
        };
        let return_result = self.is_assignable(source_return, target_return);
        if return_result == Assignability::NotAssignable {
            return Assignability::NotAssignable;
        }

        let target_parameters = constructor_parameters(self.db, target);
        let Some(source_parameters) = source_parameters else {
            return match target_parameters.as_slice() {
                [rest] if rest.is_rest() && self.accepts_any_arguments(rest.ty()) => return_result,
                _ => Assignability::Undetermined,
            };
        };
        self.are_parameters_assignable(
            &source_parameters,
            &target_parameters,
            return_result,
            ParameterVariance::Contravariant,
        )
    }

    /// Returns the parameters needed to construct `class`.
    ///
    /// A class without a constructor of its own takes the parameters of its
    /// parent, and takes none without a parent. Returns `None` when the
    /// parameters aren't known: for overloaded constructors, and for generic
    /// classes, whose parameters refer to type parameters left unsubstituted.
    fn class_constructor_parameters(
        &mut self,
        mut class: InternedClass<'db>,
    ) -> Option<Vec<FunctionParameter<'db>>> {
        for _ in 0..MAX_MEMBER_LOOKUP_DEPTH {
            if !class.type_parameters(self.db).is_empty() {
                return None;
            }
            let mut constructors = class
                .members(self.db)
                .iter()
                .filter(|member| member.kind.is_constructor());
            match (constructors.next(), constructors.next()) {
                (Some(member), None) => {
                    return match self.resolve(member.ty) {
                        TypeData::Constructor(constructor) => {
                            Some(constructor_parameters(self.db, constructor))
                        }
                        _ => None,
                    };
                }
                (Some(_), Some(_)) => return None,
                (None, _) => {}
            }
            let Some(parent) = class.extends(self.db) else {
                return Some(Vec::new());
            };
            class = match self.resolve(parent) {
                TypeData::Class(parent) => parent,
                TypeData::InstanceOf(instance) => self.instance_class(instance)?,
                _ => return None,
            };
        }
        None
    }

    /// Replaces a generic function's type parameters with their constraints,
    /// or with unknown without one.
    fn erase_type_parameters(
        &mut self,
        function: InternedFunction<'db>,
    ) -> Option<InternedFunction<'db>> {
        let type_parameters = function.type_parameters(self.db);
        if type_parameters.is_empty() {
            return Some(function);
        }
        let db = self.db;
        let mut substitutions = Vec::with_capacity(type_parameters.len() * 2);
        for type_parameter in type_parameters {
            let generic = referenced_generic(db, *type_parameter)?;
            let replacement = generic.constraint(db).unwrap_or(TypeData::Unknown);
            substitutions.extend(type_parameter_substitutions(db, *type_parameter, replacement)?.1);
        }
        let erased = substitute_all(
            db,
            TypeData::Function(InternedFunction::new(
                db,
                Box::default(),
                function.parameters(db).clone(),
                function.return_type(db).clone(),
                function.is_async(db),
                function.name(db).clone(),
            )),
            substitutions,
        )?;
        match erased {
            TypeData::Function(function) => Some(function),
            _ => None,
        }
    }

    /// Compares the members of object types.
    ///
    /// Every required member of the target must exist on the source with an
    /// assignable type. A missing member is conclusive only when every member
    /// of the source is known. Assignability is never concluded for targets
    /// whose members can't all be compared, such as members with symbol keys
    /// or index signatures, but a missing named member is still conclusive.
    fn is_structurally_assignable(
        &mut self,
        source: TypeData<'db>,
        target: TypeData<'db>,
    ) -> Assignability {
        let Some(target_members) = self.structural_members(target) else {
            return Assignability::Undetermined;
        };
        if target_members.named.len() > MAX_STRUCTURAL_MEMBERS {
            return Assignability::Undetermined;
        }

        let target_is_weak = target_members.is_weak();

        // Look for a missing member first, since that is conclusive without
        // comparing any member types.
        let mut found = Vec::with_capacity(target_members.named.len());
        let mut result = if target_members.has_unlisted_members {
            Assignability::Undetermined
        } else {
            Assignability::Assignable
        };
        for (name, target_ty, target_is_optional) in target_members.named {
            match self.find_apparent_member(source, &name) {
                MemberLookup::Found {
                    ty: source_ty,
                    is_optional: source_is_optional,
                } => found.push((
                    name,
                    source_ty,
                    source_is_optional,
                    target_ty,
                    target_is_optional,
                )),
                MemberLookup::Missing if target_is_optional => {}
                MemberLookup::Missing => return Assignability::NotAssignable,
                MemberLookup::Unknown => result = Assignability::Undetermined,
            }
        }

        // A type with only optional properties rejects a source that shares
        // none of them.
        if target_is_weak && found.is_empty() && result != Assignability::Undetermined {
            return match self.structural_members(source) {
                Some(members) if !members.named.is_empty() => Assignability::NotAssignable,
                Some(members) if !members.has_unlisted_members => Assignability::Assignable,
                _ => Assignability::Undetermined,
            };
        }

        for (name, source_ty, source_is_optional, target_ty, target_is_optional) in found {
            let member_result = if source_is_optional && !target_is_optional {
                Assignability::NotAssignable
            } else if target_is_optional {
                self.is_member_assignable(source_ty, with_undefined(self.db, target_ty), &name)
            } else {
                self.is_member_assignable(source_ty, target_ty, &name)
            };
            match member_result {
                Assignability::Assignable => {}
                Assignability::NotAssignable => return Assignability::NotAssignable,
                Assignability::Undetermined => result = Assignability::Undetermined,
            }
        }
        if result == Assignability::Assignable && self.is_unrelated_class_instance(source, target) {
            return Assignability::Undetermined;
        }
        result
    }

    /// Returns whether `target` is an instance of a user-defined class that
    /// `source` doesn't inherit from.
    ///
    /// Such a class may declare private members, which only its subclasses
    /// provide, and member accessibility isn't modeled. Matching member names
    /// and types therefore can't prove assignability.
    fn is_unrelated_class_instance(
        &mut self,
        source: TypeData<'db>,
        target: TypeData<'db>,
    ) -> bool {
        if let TypeData::InstanceOf(target) = target
            && self.builtin_name(target.ty(self.db)).is_none()
            && let Some(target_class) = self.instance_class(target)
        {
            let mut current = source;
            for _ in 0..MAX_MEMBER_LOOKUP_DEPTH {
                let class = match self.resolve(current) {
                    TypeData::InstanceOf(instance) => self.instance_class(instance),
                    TypeData::Class(class) => Some(class),
                    _ => None,
                };
                if class == Some(target_class) {
                    return false;
                }
                let Some(parent) = class.and_then(|class| class.extends(self.db)) else {
                    return true;
                };
                current = parent;
            }
            true
        } else {
            false
        }
    }

    /// Returns the class that `instance` is an instance of, if any.
    fn instance_class(
        &mut self,
        instance: InternedTypeInstance<'db>,
    ) -> Option<InternedClass<'db>> {
        match self
            .resolve(instance.ty(self.db))
            .expand_canonical_global(self.db)
        {
            TypeData::Class(class) => Some(class),
            _ => None,
        }
    }

    /// Compares a source member with the target member called `name`.
    ///
    /// The parameters of a target method are compared bivariantly, and those
    /// of a property holding a function contravariantly. Here `Method` selects
    /// `"yes"` and `Property` selects `"no"`:
    ///
    /// ```ts
    /// interface MethodSource { handle(x: "a"): void }
    /// interface MethodTarget { handle(x: string): void }
    /// interface PropertySource { handle: (x: "a") => void }
    /// interface PropertyTarget { handle: (x: string) => void }
    /// type Method = MethodSource extends MethodTarget ? "yes" : "no";
    /// type Property = PropertySource extends PropertyTarget ? "yes" : "no";
    /// ```
    fn is_member_assignable(
        &mut self,
        source: TypeData<'db>,
        target: TypeData<'db>,
        name: &str,
    ) -> Assignability {
        if is_simplified_signature(source) || is_simplified_signature(target) {
            return self.is_assignable(source, target);
        }
        let source = self.resolve(source);
        let target = self.resolve(target);
        if let Some(target_method) = self.optional_function(target).filter(|function| {
            function
                .name(self.db)
                .as_ref()
                .is_some_and(|function_name| function_name.text() == name)
        }) && let Some(source_function) = match source {
            TypeData::Union(_) if !matches!(target, TypeData::Union(_)) => None,
            source => self.optional_function(source),
        } {
            return self.is_function_assignable(
                source_function,
                target_method,
                ParameterVariance::Bivariant,
            );
        }
        self.is_assignable(source, target)
    }

    /// Returns the function in `ty`, which must be a function or the union of
    /// a function with `undefined`, as an optional method is.
    fn optional_function(&mut self, ty: TypeData<'db>) -> Option<InternedFunction<'db>> {
        match ty {
            TypeData::Function(function) => Some(function),
            TypeData::Union(union) => {
                let mut function = None;
                for member in union.types(self.db) {
                    match self.resolve(*member) {
                        TypeData::Function(found) if function.is_none() => function = Some(found),
                        TypeData::Undefined => {}
                        _ => return None,
                    }
                }
                function
            }
            _ => None,
        }
    }

    /// Returns the named instance members of an object type, together with
    /// their types and whether they are optional.
    ///
    /// Returns `None` for types whose named members can't all be listed.
    fn structural_members(&mut self, ty: TypeData<'db>) -> Option<StructuralMembers<'db>> {
        let (members, substitutions, is_complete) = match ty {
            TypeData::Object(object) => (
                object.members(self.db).to_vec(),
                Vec::new(),
                object.prototype(self.db).is_none() && !object.has_unknown_members(self.db),
            ),
            TypeData::Literal(literal) => match literal.literal(self.db) {
                Literal::Object(members) => (members.to_vec(), Vec::new(), true),
                _ => return None,
            },
            TypeData::Interface(interface) => (
                interface.members(self.db).to_vec(),
                Vec::new(),
                interface.extends(self.db).is_empty(),
            ),
            TypeData::InstanceOf(instance) => {
                let target = self
                    .resolve(instance.ty(self.db))
                    .expand_canonical_global(self.db);
                let (type_parameters, members, is_complete) = match target {
                    TypeData::Interface(interface) => (
                        interface.type_parameters(self.db),
                        interface.members(self.db),
                        interface.extends(self.db).is_empty(),
                    ),
                    TypeData::Class(class) => (
                        class.type_parameters(self.db),
                        class.members(self.db),
                        class.extends(self.db).is_none(),
                    ),
                    _ => return None,
                };
                let substitutions = self
                    .instance_substitutions(type_parameters, instance.type_parameters(self.db))?;
                (members.to_vec(), substitutions, is_complete)
            }
            _ => return None,
        };

        let mut named = Vec::new();
        let mut has_unlisted_members = !is_complete;
        for member in members {
            if member.kind.is_static() {
                continue;
            }
            match member.kind.name() {
                Some(name) if member.kind.computed_value_type().is_none() => {
                    let ty = substitute_all(
                        self.db,
                        member_type(self.db, &member),
                        substitutions.clone(),
                    )?;
                    named.push((name, ty, member.kind.is_optional()));
                }
                _ => has_unlisted_members = true,
            }
        }
        Some(StructuralMembers {
            named,
            has_unlisted_members,
        })
    }

    /// Looks up a member the way property access does, so a member that `ty`
    /// lacks is also looked for on `Object`.
    fn find_apparent_member(&mut self, ty: TypeData<'db>, name: &str) -> MemberLookup<'db> {
        match self.find_member(ty, name, 0, &mut FxHashSet::default()) {
            MemberLookup::Missing => {
                let object = TypeData::instance_of(
                    self.db,
                    TypeData::GlobalType(OBJECT_ID_GLOBAL_TYPE_ID),
                    Box::default(),
                );
                self.find_member(object, name, 0, &mut FxHashSet::default())
            }
            result => result,
        }
    }

    /// Looks up an instance member of `ty` by name, following inheritance.
    ///
    /// `visited` holds the types already searched. A type reached again
    /// through another parent, as in a diamond-shaped hierarchy, is reported
    /// as missing the member, since its first search already decided the
    /// outcome.
    fn find_member(
        &mut self,
        ty: TypeData<'db>,
        name: &str,
        depth: usize,
        visited: &mut FxHashSet<TypeData<'db>>,
    ) -> MemberLookup<'db> {
        if depth > MAX_MEMBER_LOOKUP_DEPTH {
            return MemberLookup::Unknown;
        }
        let ty = self.resolve(ty);
        if !visited.insert(ty) {
            return MemberLookup::Missing;
        }
        let (target, type_arguments) = match ty {
            TypeData::InstanceOf(instance) => (
                self.resolve(instance.ty(self.db))
                    .expand_canonical_global(self.db),
                instance.type_parameters(self.db).as_ref(),
            ),
            TypeData::GlobalType(_) => (ty.expand_canonical_global(self.db), &[][..]),
            ty => (ty, &[][..]),
        };

        let (members, type_parameters, inherited, is_closed) = match target {
            TypeData::Object(object) => (
                object.members(self.db).as_ref(),
                &[][..],
                object.prototype(self.db).into_iter().collect::<Vec<_>>(),
                !object.has_unknown_members(self.db),
            ),
            TypeData::Literal(literal) => match literal.literal(self.db) {
                Literal::Object(members) => (members.as_ref(), &[][..], Vec::new(), true),
                _ => return MemberLookup::Unknown,
            },
            TypeData::Interface(interface) => (
                interface.members(self.db).as_ref(),
                interface.type_parameters(self.db).as_ref(),
                interface.extends(self.db).to_vec(),
                true,
            ),
            TypeData::Class(class) if matches!(ty, TypeData::InstanceOf(_)) => (
                class.members(self.db).as_ref(),
                class.type_parameters(self.db).as_ref(),
                class.extends(self.db).into_iter().collect(),
                true,
            ),
            _ => return MemberLookup::Unknown,
        };

        let Some(substitutions) = self.instance_substitutions(type_parameters, type_arguments)
        else {
            return MemberLookup::Unknown;
        };
        let member = members
            .iter()
            .find(|member| !member.kind.is_static() && member.kind.has_name(name));
        if let Some(member) = member {
            return match substitute_all(self.db, member_type(self.db, member), substitutions) {
                Some(ty) => MemberLookup::Found {
                    ty,
                    is_optional: member.kind.is_optional(),
                },
                None => MemberLookup::Unknown,
            };
        }
        // A numeric or symbol index signature can't provide a member whose
        // name isn't numeric, but other index signatures may. So may a member
        // whose computed key has no known name.
        let is_numeric_name = name.parse::<f64>().is_ok();
        if members.iter().any(|member| {
            member.kind.index_signature_type().is_some_and(|key| {
                is_numeric_name || !matches!(key, TypeData::Number | TypeData::Symbol)
            }) || matches!(
                member.kind,
                TypeMemberKind::ComputedValue(_) | TypeMemberKind::ConstAssertedComputedValue(_)
            )
        }) {
            return MemberLookup::Unknown;
        }

        let mut result = if is_closed {
            MemberLookup::Missing
        } else {
            MemberLookup::Unknown
        };
        for parent in inherited {
            let Some(parent) = substitute_all(self.db, parent, substitutions.clone()) else {
                return MemberLookup::Unknown;
            };
            // An `extends` clause refers to the parent class, but instance
            // members are looked up on an instance of it.
            let parent = match self.resolve(parent) {
                class @ TypeData::Class(_) => TypeData::instance_of(self.db, class, Box::default()),
                parent => parent,
            };
            match self.find_member(parent, name, depth + 1, visited) {
                found @ MemberLookup::Found { .. } => return found,
                MemberLookup::Missing => {}
                MemberLookup::Unknown => result = MemberLookup::Unknown,
            }
        }
        result
    }

    /// Returns the substitutions that replace declared type parameters with an
    /// instance's type arguments.
    ///
    /// Parameters without an argument use their defaults, or remain generic.
    fn instance_substitutions(
        &self,
        type_parameters: &[TypeData<'db>],
        type_arguments: &[TypeData<'db>],
    ) -> Option<Vec<TypeSubstitution<'db>>> {
        let mut substitutions = Vec::with_capacity(type_parameters.len() * 2);
        for (index, type_parameter) in type_parameters.iter().enumerate() {
            let generic = referenced_generic(self.db, *type_parameter)?;
            let Some(argument) = type_arguments
                .get(index)
                .copied()
                .or_else(|| generic.default(self.db))
            else {
                continue;
            };
            substitutions
                .extend(type_parameter_substitutions(self.db, *type_parameter, argument)?.1);
        }
        Some(substitutions)
    }
}

/// Returns the type of a member as seen when reading it.
fn member_type<'db>(db: &'db dyn TypeDb, member: &TypeMember<'db>) -> TypeData<'db> {
    let ty = member.ty.expand_global_local(db);
    if member.kind.name().is_some()
        && matches!(
            member.kind,
            TypeMemberKind::Getter(_) | TypeMemberKind::ConstAssertedGetter(_)
        )
        && let TypeData::Function(function) = ty
        && let ReturnType::Type(return_type) = function.return_type(db)
    {
        *return_type
    } else {
        ty
    }
}

/// Returns the parameters declared by `constructor`.
fn constructor_parameters<'db>(
    db: &'db dyn TypeDb,
    constructor: InternedConstructor<'db>,
) -> Vec<FunctionParameter<'db>> {
    constructor
        .parameters(db)
        .iter()
        .map(|parameter| parameter.parameter.clone())
        .collect()
}

fn parameters_without_this<'db>(
    parameters: &[FunctionParameter<'db>],
) -> Vec<FunctionParameter<'db>> {
    parameters
        .iter()
        .filter(|parameter| !parameter.is_this())
        .cloned()
        .collect()
}

/// Removes a `readonly` operator, returning whether one was present.
fn strip_readonly<'db>(db: &'db dyn TypeDb, ty: TypeData<'db>) -> (TypeData<'db>, bool) {
    match ty {
        TypeData::TypeOperator(operator) if operator.operator(db) == TypeOperator::Readonly => {
            (operator.ty(db), true)
        }
        ty => (ty, false),
    }
}

/// Returns whether `ty` must be resolved further before comparing it.
fn is_unresolved<'db>(db: &'db dyn TypeDb, ty: TypeData<'db>) -> bool {
    match ty {
        TypeData::TypeOperator(operator) => operator.operator(db) != TypeOperator::Readonly,
        TypeData::Unknown
        | TypeData::Global
        | TypeData::Conditional
        | TypeData::Module(_)
        | TypeData::Namespace(_)
        | TypeData::Generic(_)
        | TypeData::Local(_)
        | TypeData::GlobalLocal(_)
        | TypeData::IndexedAccess(_)
        | TypeData::MappedType(_)
        | TypeData::Extends(_)
        | TypeData::MergedReference(_)
        | TypeData::TypeofExpression(_)
        | TypeData::TypeofType(_)
        | TypeData::TypeofValue(_)
        | TypeData::ThisKeyword => true,
        TypeData::BigInt
        | TypeData::Boolean
        | TypeData::Null
        | TypeData::Number
        | TypeData::String
        | TypeData::Symbol
        | TypeData::Undefined
        | TypeData::Class(_)
        | TypeData::Constructor(_)
        | TypeData::Function(_)
        | TypeData::Interface(_)
        | TypeData::Object(_)
        | TypeData::Tuple(_)
        | TypeData::GlobalType(_)
        | TypeData::Intersection(_)
        | TypeData::Union(_)
        | TypeData::Literal(_)
        | TypeData::InstanceOf(_)
        | TypeData::AnyKeyword
        | TypeData::NeverKeyword
        | TypeData::ObjectKeyword
        | TypeData::UnknownKeyword
        | TypeData::VoidKeyword => false,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Primitive {
    BigInt,
    Boolean,
    Null,
    Number,
    String,
    Symbol,
    Undefined,
    Void,
}

impl Primitive {
    /// Returns the name of the built-in type that wraps values of this type.
    fn wrapper_name(self) -> Option<&'static str> {
        match self {
            Self::BigInt => Some("BigInt"),
            Self::Boolean => Some("Boolean"),
            Self::Number => Some("Number"),
            Self::String => Some("String"),
            Self::Symbol => Some("Symbol"),
            Self::Null | Self::Undefined | Self::Void => None,
        }
    }

    /// Returns the built-in type that provides this primitive's members.
    fn wrapper_type<'db>(self) -> Option<TypeData<'db>> {
        let id = match self {
            Self::Boolean => BOOLEAN_ID_GLOBAL_TYPE_ID,
            Self::Number => NUMBER_ID_GLOBAL_TYPE_ID,
            Self::String => STRING_ID_GLOBAL_TYPE_ID,
            Self::Symbol => SYMBOL_ID_GLOBAL_TYPE_ID,
            Self::BigInt | Self::Null | Self::Undefined | Self::Void => return None,
        };
        Some(TypeData::GlobalType(id))
    }
}

/// A primitive type, or a literal of a primitive type.
#[derive(Clone, Copy, Debug)]
struct PrimitiveKind {
    primitive: Primitive,
    is_literal: bool,
    /// Whether this is a template literal type, whose values follow a pattern.
    is_template: bool,
}

impl PrimitiveKind {
    /// Checks whether a value of this kind is assignable to `target`.
    ///
    /// Two different types are compared here, since identical types were
    /// already accepted, and literals of one primitive were already compared
    /// by value. So a literal is assignable only to its primitive.
    /// Whether a string matches a template literal type such as `` `a${string}` ``
    /// is not evaluated.
    fn is_assignable_to(self, target: Self) -> Assignability {
        if target.is_template || self.is_template && target.is_literal {
            return if self.primitive == target.primitive {
                Assignability::Undetermined
            } else {
                Assignability::NotAssignable
            };
        }
        if target.is_literal {
            return Assignability::NotAssignable;
        }
        Assignability::from(
            self.primitive == target.primitive
                || self.primitive == Primitive::Undefined && target.primitive == Primitive::Void,
        )
    }
}

/// Compares two literal types of the same primitive by value.
///
/// Different spellings can denote the same value, such as `1` and `1.0`, or
/// `16n` and `0x10n`. Escape sequences in string literals aren't decoded, so
/// a string literal that contains one leaves the outcome undetermined.
///
/// Returns `None` for types that aren't number, bigint, or string literals of
/// the same primitive.
fn compare_literal_values<'db>(
    db: &'db dyn TypeDb,
    source: TypeData<'db>,
    target: TypeData<'db>,
) -> Option<Assignability> {
    let (TypeData::Literal(source), TypeData::Literal(target)) = (source, target) else {
        return None;
    };
    let is_equal = match (source.literal(db), target.literal(db)) {
        (Literal::Number(source), Literal::Number(target)) => source
            .to_f64()
            .zip(target.to_f64())
            .map(|(source, target)| source == target),
        (Literal::BigInt(source), Literal::BigInt(target)) => {
            canonicalize_js_bigint_literal(source.text())
                .zip(canonicalize_js_bigint_literal(target.text()))
                .map(|(source, target)| source == target)
        }
        (Literal::String(source), Literal::String(target)) => {
            let is_escaped = |literal: &StringLiteral| literal.as_str().contains('\\');
            (!is_escaped(source) && !is_escaped(target)).then(|| source == target)
        }
        _ => return None,
    };
    Some(match is_equal {
        Some(true) => Assignability::Assignable,
        Some(false) => Assignability::NotAssignable,
        None => Assignability::Undetermined,
    })
}

/// Returns the intersection of two different primitive types.
///
/// The intersection is the narrower type when one includes the other, such as
/// `"a"` for `string & "a"`, and `never` when no value belongs to both, such
/// as for `"a" & "b"`. Returns `None` when this can't be decided, such as for
/// a template literal type and a string literal, or for types that aren't
/// primitives.
pub(crate) fn intersect_primitives<'db>(
    db: &'db dyn TypeDb,
    left: TypeData<'db>,
    right: TypeData<'db>,
) -> Option<TypeData<'db>> {
    let left_kind = primitive_kind(db, left)?;
    let right_kind = primitive_kind(db, right)?;
    let is_left_in_right = compare_literal_values(db, left, right)
        .unwrap_or_else(|| left_kind.is_assignable_to(right_kind));
    let is_right_in_left = compare_literal_values(db, right, left)
        .unwrap_or_else(|| right_kind.is_assignable_to(left_kind));
    match (is_left_in_right, is_right_in_left) {
        (Assignability::Assignable, _) => Some(left),
        (_, Assignability::Assignable) => Some(right),
        (Assignability::NotAssignable, Assignability::NotAssignable) => {
            Some(TypeData::NeverKeyword)
        }
        _ => None,
    }
}

/// Returns `ty` together with `undefined`, which an optional element or
/// member also accepts.
fn with_undefined<'db>(db: &'db dyn TypeDb, ty: TypeData<'db>) -> TypeData<'db> {
    TypeData::union_from_types(db, vec![ty, TypeData::Undefined])
}

/// Returns whether `ty` is a handle to one of the [`SIMPLIFIED_SIGNATURES`].
fn is_simplified_signature(ty: TypeData) -> bool {
    matches!(ty, TypeData::GlobalType(id) if SIMPLIFIED_SIGNATURES.contains(&id))
}

fn primitive_kind<'db>(db: &'db dyn TypeDb, ty: TypeData<'db>) -> Option<PrimitiveKind> {
    let (primitive, is_literal, is_template) = match ty {
        TypeData::BigInt => (Primitive::BigInt, false, false),
        TypeData::Boolean => (Primitive::Boolean, false, false),
        TypeData::Null => (Primitive::Null, false, false),
        TypeData::Number => (Primitive::Number, false, false),
        TypeData::String => (Primitive::String, false, false),
        TypeData::Symbol => (Primitive::Symbol, false, false),
        TypeData::Undefined => (Primitive::Undefined, false, false),
        TypeData::VoidKeyword => (Primitive::Void, false, false),
        TypeData::Literal(literal) => match literal.literal(db) {
            Literal::BigInt(_) => (Primitive::BigInt, true, false),
            Literal::Boolean(_) => (Primitive::Boolean, true, false),
            Literal::Number(_) => (Primitive::Number, true, false),
            Literal::String(_) => (Primitive::String, true, false),
            Literal::Template(_) => (Primitive::String, true, true),
            Literal::Object(_) | Literal::RegExp(_) => return None,
        },
        _ => return None,
    };
    Some(PrimitiveKind {
        primitive,
        is_literal,
        is_template,
    })
}

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

use crate::TypeOperator;
use crate::interned_types::{
    FunctionParameter, InternedExtendsType, InternedFunction, InternedGenericTypeParameter,
    InternedTuple, InternedTypeInstance, Literal, ReturnType, TupleElementType, TypeData, TypeDb,
    TypeMember,
};
use crate::type_transform::TypeSubstitution;
use rustc_hash::FxHashSet;

/// Maximum number of type pairs compared while evaluating one conditional.
const MAX_RELATION_STEPS: usize = 256;

/// Maximum number of inheritance levels followed while looking up a member.
const MAX_MEMBER_LOOKUP_DEPTH: usize = 16;

/// Maximum number of types inspected while looking for a free generic.
const MAX_FREE_GENERIC_STEPS: usize = 256;

/// Maximum number of members compared structurally for one pair of types.
const MAX_STRUCTURAL_MEMBERS: usize = 64;

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
/// A union check type is distributed when the generic is substituted, so a
/// union reaching this function could not be distributed and is left
/// unevaluated. A check type of `any` selects both branches.
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
    if may_refer_to_free_generic(db, check_type) {
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
            TypeData::Union(_) => return None,
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

    if check.is_assignable(check_type, extends_type)? {
        check.instantiate_true_type(extends.true_type(db))
    } else {
        Some(EvaluatedExtends {
            ty: extends.false_type(db),
            needs_normalization: false,
        })
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
        may_refer_to_free_generic(db, self)
    }
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

/// Returns whether `ty` may refer to a generic it doesn't declare itself.
///
/// For example, `Promise<T>` refers to `T`, while `<T>(value: T) => T` only
/// refers to the `T` it declares. Types too large to inspect are assumed to
/// refer to a generic.
pub(crate) fn may_refer_to_free_generic<'db>(db: &'db dyn TypeDb, ty: TypeData<'db>) -> bool {
    let mut pending = Vec::from([ty]);
    let mut seen = FxHashSet::default();
    let mut declared = FxHashSet::default();
    let mut remaining_steps = MAX_FREE_GENERIC_STEPS;
    while let Some(ty) = pending.pop() {
        if !seen.insert(ty) {
            continue;
        }
        if remaining_steps == 0 {
            return true;
        }
        remaining_steps -= 1;

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

/// Outcome of looking up a member while comparing object types.
enum MemberLookup<'db> {
    Found {
        ty: TypeData<'db>,
        is_optional: bool,
    },
    Missing,
    Unknown,
}

/// Decides assignability between normalized types, recording the types
/// matched by `infer` generics along the way.
///
/// Every check returns `Some(true)` when assignability is certain,
/// `Some(false)` when non-assignability is certain, and `None` otherwise.
struct AssignabilityCheck<'db, 'a> {
    db: &'db dyn TypeDb,
    resolve: &'a mut TypeResolver<'db, 'a>,
    infer_types: Vec<InternedGenericTypeParameter<'db>>,
    /// Types matched by each `infer` generic, in the order they were found.
    bindings: Vec<(InternedGenericTypeParameter<'db>, TypeData<'db>)>,
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
            bindings: Vec::new(),
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

    /// Records that the `infer` generic matched `ty`.
    fn bind(
        &mut self,
        generic: InternedGenericTypeParameter<'db>,
        ty: TypeData<'db>,
    ) -> Option<bool> {
        if let Some(constraint) = generic.constraint(self.db)
            && !self.is_assignable(ty, constraint)?
        {
            return Some(false);
        }
        self.bindings.push((generic, ty));
        Some(true)
    }

    /// Replaces the `infer` generics in `true_type` with the types they matched.
    ///
    /// A generic matched more than once becomes the union of its matches. An
    /// unmatched generic becomes its constraint, or unknown without one.
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
            let matches = self
                .bindings
                .iter()
                .filter(|(bound, _)| bound == generic)
                .map(|(_, ty)| *ty)
                .collect::<Vec<_>>();
            let replacement = if matches.is_empty() {
                generic.constraint(db).unwrap_or(TypeData::Unknown)
            } else {
                TypeData::union_from_types(db, matches)
            };
            substitutions.extend(generic_substitutions(db, *generic, replacement));
        }
        Some(EvaluatedExtends {
            ty: substitute_all(db, true_type, substitutions)?,
            needs_normalization: true,
        })
    }

    fn is_assignable(&mut self, source: TypeData<'db>, target: TypeData<'db>) -> Option<bool> {
        if self.remaining_steps == 0 {
            return None;
        }
        self.remaining_steps -= 1;

        let source = self.resolve(source);
        let target = self.resolve(target);
        if let Some(generic) = self.infer_type(target) {
            return self.bind(generic, source);
        }
        // Parameters are compared in the opposite direction, so an `infer`
        // generic in a parameter position appears as the source.
        if let Some(generic) = self.infer_type(source) {
            return self.bind(generic, target);
        }
        if source == target {
            return Some(true);
        }

        match (source, target) {
            (_, TypeData::AnyKeyword | TypeData::UnknownKeyword) | (TypeData::NeverKeyword, _) => {
                return Some(true);
            }
            (TypeData::AnyKeyword, _) => return Some(target != TypeData::NeverKeyword),
            (TypeData::Union(union), _) => {
                return self.all_assignable(union.types(self.db).iter().map(|ty| (*ty, target)));
            }
            (TypeData::Boolean, TypeData::Union(_)) => {
                let [true_literal, false_literal] = [true, false].map(|value| {
                    TypeData::Literal(crate::interned_types::InternedLiteral::new(
                        self.db,
                        Literal::Boolean(value.into()),
                    ))
                });
                return self.all_assignable([(true_literal, target), (false_literal, target)]);
            }
            (_, TypeData::Union(union)) => {
                return self.any_assignable(union.types(self.db).iter().map(|ty| (source, *ty)));
            }
            (_, TypeData::Intersection(intersection)) => {
                return self
                    .all_assignable(intersection.types(self.db).iter().map(|ty| (source, *ty)));
            }
            (TypeData::Intersection(intersection), _) => {
                // Members of an intersection may combine to satisfy an object
                // type, so failing every member is only conclusive for primitives.
                let result =
                    self.any_assignable(intersection.types(self.db).iter().map(|ty| (*ty, target)));
                return match result {
                    Some(false) if primitive_kind(self.db, target).is_none() => None,
                    result => result,
                };
            }
            _ => {}
        }

        if let Some(generic) = referenced_generic(self.db, source) {
            let constraint = generic.constraint(self.db)?;
            return self
                .is_assignable(constraint, target)
                .filter(|result| *result);
        }
        if referenced_generic(self.db, target).is_some() {
            return None;
        }
        if is_unresolved(self.db, source) || is_unresolved(self.db, target) {
            return None;
        }

        let (source, source_is_readonly) = strip_readonly(self.db, source);
        let (target, target_is_readonly) = strip_readonly(self.db, target);
        if target == TypeData::NeverKeyword {
            return Some(false);
        }

        match (
            primitive_kind(self.db, source),
            primitive_kind(self.db, target),
        ) {
            (Some(source_kind), Some(target_kind)) => {
                return source_kind.is_assignable_to(target_kind);
            }
            (Some(source_kind), None) => {
                return self.is_primitive_assignable_to_object(source_kind, target);
            }
            (None, Some(_)) => return self.is_object_like(source).then_some(false),
            (None, None) => {}
        }

        if target == TypeData::ObjectKeyword {
            return self.is_object_like(source).then_some(true);
        }
        if source == TypeData::ObjectKeyword {
            return self.is_empty_object(target).then_some(true);
        }

        if !source_is_readonly || target_is_readonly {
            // Readonly arrays and tuples are not assignable to mutable ones.
        } else if matches!(target, TypeData::Tuple(_)) || self.array_element(target).is_some() {
            return Some(false);
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
                    return None;
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
                        Some(false)
                    }
                    _ => None,
                }
            }
            (TypeData::Function(source), TypeData::Function(target)) => {
                self.is_function_assignable(source, target)
            }
            (TypeData::Class(class), TypeData::Constructor(constructor)) => {
                let instance =
                    TypeData::instance_of(self.db, TypeData::Class(class), Box::default());
                match constructor.return_type(self.db) {
                    Some(return_type) => self.is_assignable(instance, return_type),
                    None => None,
                }
            }
            (TypeData::Constructor(source), TypeData::Constructor(target)) => {
                match (source.return_type(self.db), target.return_type(self.db)) {
                    (Some(source), Some(target)) => self.is_assignable(source, target),
                    _ => None,
                }
            }
            (TypeData::Function(_), TypeData::Constructor(_))
            | (TypeData::Class(_) | TypeData::Constructor(_), TypeData::Function(_)) => Some(false),
            (TypeData::InstanceOf(source), TypeData::InstanceOf(target))
                if self.same_instance_target(source, target) =>
            {
                self.are_type_arguments_assignable(source, target)
            }
            _ => self.is_structurally_assignable(source, target),
        }
    }

    /// Returns whether every pair is assignable.
    fn all_assignable(
        &mut self,
        pairs: impl IntoIterator<Item = (TypeData<'db>, TypeData<'db>)>,
    ) -> Option<bool> {
        let mut result = Some(true);
        for (source, target) in pairs {
            match self.is_assignable(source, target) {
                Some(true) => {}
                Some(false) => return Some(false),
                None => result = None,
            }
        }
        result
    }

    /// Returns whether any pair is assignable.
    ///
    /// Bindings recorded while trying an unsuccessful pair are discarded.
    fn any_assignable(
        &mut self,
        pairs: impl IntoIterator<Item = (TypeData<'db>, TypeData<'db>)>,
    ) -> Option<bool> {
        let mut result = Some(false);
        for (source, target) in pairs {
            let bindings = self.bindings.len();
            match self.is_assignable(source, target) {
                Some(true) => return Some(true),
                Some(false) => {}
                None => result = None,
            }
            self.bindings.truncate(bindings);
        }
        result
    }

    fn is_primitive_assignable_to_object(
        &mut self,
        source: PrimitiveKind,
        target: TypeData<'db>,
    ) -> Option<bool> {
        if matches!(
            source.primitive,
            Primitive::Null | Primitive::Undefined | Primitive::Void
        ) {
            return Some(false);
        }
        if self.is_empty_object(target) {
            return Some(true);
        }
        match target {
            TypeData::ObjectKeyword
            | TypeData::Tuple(_)
            | TypeData::Function(_)
            | TypeData::Constructor(_)
            | TypeData::Class(_) => Some(false),
            // Primitives have apparent members, such as a string's `length`,
            // so they are assignable to `Object` and to their wrapper types.
            // They never provide the members of other built-in types.
            TypeData::InstanceOf(instance) => {
                let name = self.builtin_name(instance.ty(self.db))?;
                Some(name == "Object" || Some(name.text()) == source.primitive.wrapper_name())
            }
            _ => None,
        }
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
    }

    /// Compares the type arguments of two instances of the same type.
    ///
    /// Type arguments are assumed to be covariant. That holds for built-in
    /// containers such as `Promise` and `Array`, so a mismatch there is
    /// conclusive. Other types may use their arguments differently, so a
    /// mismatch there leaves the result undetermined.
    fn are_type_arguments_assignable(
        &mut self,
        source: InternedTypeInstance<'db>,
        target: InternedTypeInstance<'db>,
    ) -> Option<bool> {
        let is_builtin = self.builtin_name(target.ty(self.db)).is_some();
        let pairs = source
            .type_parameters(self.db)
            .iter()
            .copied()
            .zip(target.type_parameters(self.db).iter().copied())
            .collect::<Vec<_>>();
        match self.all_assignable(pairs) {
            Some(false) if !is_builtin => None,
            result => result,
        }
    }

    fn is_tuple_assignable(
        &mut self,
        source: InternedTuple<'db>,
        target: InternedTuple<'db>,
    ) -> Option<bool> {
        if source.is_inferred_array(self.db) || target.is_inferred_array(self.db) {
            return None;
        }
        let source_elements = source.elements(self.db);
        if source_elements.iter().any(|element| element.is_rest) {
            return None;
        }
        let (fixed, rest) = match target.elements(self.db).as_ref() {
            [fixed @ .., rest] if rest.is_rest => (fixed, Some(rest.ty)),
            fixed => (fixed, None),
        };
        if fixed.iter().any(|element| element.is_rest) {
            return None;
        }

        let required = fixed.iter().filter(|element| !element.is_optional).count();
        if source_elements.len() < required || rest.is_none() && source_elements.len() > fixed.len()
        {
            return Some(false);
        }
        if source_elements
            .iter()
            .zip(fixed)
            .any(|(source, target)| source.is_optional && !target.is_optional)
        {
            return Some(false);
        }

        let mut result = self.all_assignable(
            source_elements
                .iter()
                .zip(fixed)
                .map(|(source, target)| (source.ty, target.ty)),
        );
        if result == Some(false) {
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
                self.bind(generic, tuple)
            } else if let rest = self.resolve(rest)
                && let Some(element) = self.array_element(rest)
            {
                self.all_assignable(remaining.iter().map(|source| (source.ty, element)))
            } else {
                None
            };
            result = match (result, rest_result) {
                (_, Some(false)) => return Some(false),
                (Some(true), Some(true)) => Some(true),
                _ => None,
            };
        }
        result
    }

    /// Compares function signatures.
    ///
    /// Return types are compared covariantly, and parameters contravariantly.
    /// A target rest parameter of `any`, `any[]`, or an `infer` generic accepts
    /// any remaining source parameters; an `infer` generic becomes a tuple of
    /// them, as in `Parameters<F>`.
    fn is_function_assignable(
        &mut self,
        source: InternedFunction<'db>,
        target: InternedFunction<'db>,
    ) -> Option<bool> {
        if !target.type_parameters(self.db).is_empty() {
            return None;
        }
        let source = self.erase_type_parameters(source)?;

        let return_result = match (source.return_type(self.db), target.return_type(self.db)) {
            (_, ReturnType::Type(TypeData::VoidKeyword)) => Some(true),
            (ReturnType::Type(source), ReturnType::Type(target)) => {
                self.is_assignable(*source, *target)
            }
            (ReturnType::Predicate(_), ReturnType::Type(target)) => {
                self.is_assignable(TypeData::Boolean, *target)
            }
            (ReturnType::Asserts(_), ReturnType::Type(target)) => {
                self.is_assignable(TypeData::VoidKeyword, *target)
            }
            (_, ReturnType::Predicate(_) | ReturnType::Asserts(_)) => None,
        };
        if return_result == Some(false) {
            return Some(false);
        }

        let source_parameters = parameters_without_this(source.parameters(self.db));
        let target_parameters = parameters_without_this(target.parameters(self.db));
        let (fixed, rest) = match target_parameters.as_slice() {
            [fixed @ .., rest] if rest.is_rest() => (fixed, Some(rest.ty())),
            fixed => (fixed, None),
        };
        if fixed.iter().any(|parameter| parameter.is_rest())
            || source_parameters
                .iter()
                .take(fixed.len())
                .any(|parameter| parameter.is_rest())
        {
            return None;
        }

        let combine =
            |result: Option<bool>, parameter_result: Option<bool>| match (result, parameter_result)
            {
                (_, Some(false)) | (Some(false), _) => Some(false),
                (Some(true), Some(true)) => Some(true),
                _ => None,
            };
        let mut result = return_result;

        for (index, source_parameter) in source_parameters.iter().enumerate() {
            let parameter_result = match fixed.get(index) {
                Some(target_parameter) => {
                    self.is_assignable(target_parameter.ty(), source_parameter.ty())
                }
                None if rest.is_some() => continue,
                // A source that requires more arguments than the target
                // passes is not assignable.
                None => Some(source_parameter.is_optional() || source_parameter.is_rest()),
            };
            result = combine(result, parameter_result);
            if result == Some(false) {
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
                self.bind(generic, tuple)
            } else {
                match self.resolve(rest) {
                    TypeData::AnyKeyword => Some(true),
                    rest => match self
                        .array_element(rest)
                        .map(|element| self.resolve(element))
                    {
                        Some(TypeData::AnyKeyword) => Some(true),
                        _ => None,
                    },
                }
            };
            result = combine(result, rest_result);
        }
        result
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
    ) -> Option<bool> {
        let target_members = self.structural_members(target)?;
        if target_members.named.len() > MAX_STRUCTURAL_MEMBERS {
            return None;
        }

        // Look for a missing member first, since that is conclusive without
        // comparing any member types.
        let mut found = Vec::with_capacity(target_members.named.len());
        let mut result = (!target_members.has_unlisted_members).then_some(true);
        for (name, target_ty, target_is_optional) in target_members.named {
            match self.find_member(source, &name, 0) {
                MemberLookup::Found {
                    ty: source_ty,
                    is_optional: source_is_optional,
                } => found.push((source_ty, source_is_optional, target_ty, target_is_optional)),
                MemberLookup::Missing if target_is_optional => {}
                MemberLookup::Missing => return Some(false),
                MemberLookup::Unknown => result = None,
            }
        }

        for (source_ty, source_is_optional, target_ty, target_is_optional) in found {
            let member_result = if source_is_optional && !target_is_optional {
                Some(false)
            } else {
                self.is_assignable(source_ty, target_ty)
            };
            match member_result {
                Some(true) => {}
                Some(false) => return Some(false),
                None => result = None,
            }
        }
        result
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

    /// Looks up an instance member of `ty` by name, following inheritance.
    fn find_member(&mut self, ty: TypeData<'db>, name: &str, depth: usize) -> MemberLookup<'db> {
        if depth > MAX_MEMBER_LOOKUP_DEPTH {
            return MemberLookup::Unknown;
        }
        let ty = self.resolve(ty);
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
        // name isn't numeric, but other index signatures may.
        let is_numeric_name = name.parse::<f64>().is_ok();
        if members.iter().any(|member| {
            member.kind.index_signature_type().is_some_and(|key| {
                is_numeric_name || !matches!(key, TypeData::Number | TypeData::Symbol)
            })
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
            match self.find_member(parent, name, depth + 1) {
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
            crate::interned_types::TypeMemberKind::Getter(_)
                | crate::interned_types::TypeMemberKind::ConstAssertedGetter(_)
        )
        && let TypeData::Function(function) = ty
        && let ReturnType::Type(return_type) = function.return_type(db)
    {
        *return_type
    } else {
        ty
    }
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
    /// Returns whether a value of this kind is assignable to `target`.
    ///
    /// Two different types are compared here, since identical types were
    /// already accepted. So a literal is assignable only to its primitive.
    /// Whether a string matches a template literal type such as `` `a${string}` ``
    /// is not evaluated.
    fn is_assignable_to(self, target: Self) -> Option<bool> {
        if target.is_template || self.is_template && target.is_literal {
            return (self.primitive != target.primitive).then_some(false);
        }
        if target.is_literal {
            return Some(false);
        }
        Some(
            self.primitive == target.primitive
                || self.primitive == Primitive::Undefined && target.primitive == Primitive::Void,
        )
    }
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

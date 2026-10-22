use super::{InferredType, MAX_PROMISE_TYPE_STEPS, MAX_TYPE_RELATION_DEPTH};
use crate::TypeDb;
use crate::generated::global_types::ids::READONLY_ARRAY_ID_GLOBAL_TYPE_ID;
use crate::interned_types::TypeData;
use crate::type_data::TypeOperator;

/// Describes how a type exposes `any` without wrapping it in another generic
/// type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnyTypeKind {
    /// The type is `any`.
    Any,
    /// The type is an array of `any`, such as `any[]`, `readonly any[]`, or
    /// `ReadonlyArray<any>`.
    AnyArray,
    /// The type is a `Promise` that resolves to `any`, possibly through nested
    /// Promises or as one member of a union or intersection.
    PromiseAny,
}

impl<'db> InferredType<'db> {
    /// Classifies whether this type is `any`, an array of `any`, or a Promise
    /// that resolves to `any`.
    ///
    /// Returns `None` for every other type, including types that inference
    /// could not resolve.
    pub fn any_type_kind(self) -> Option<AnyTypeKind> {
        if is_any(self.db, self.data) {
            return Some(AnyTypeKind::Any);
        }
        if is_any_array(self.db, self.data) {
            return Some(AnyTypeKind::AnyArray);
        }
        let mut constituents = match self.data {
            TypeData::Union(union) => union.types(self.db).iter(),
            TypeData::Intersection(intersection) => intersection.types(self.db).iter(),
            _ => std::slice::from_ref(&self.data).iter(),
        };
        constituents
            .any(|ty| {
                promise_argument(self.db, *ty, 0).is_some()
                    && is_any(self.db, awaited(self.db, *ty))
            })
            .then_some(AnyTypeKind::PromiseAny)
    }

    /// Returns whether this type is the explicit `any` or `unknown` keyword.
    pub fn is_any_or_unknown_keyword(self) -> bool {
        matches!(
            self.data.expand_canonical_global(self.db),
            TypeData::AnyKeyword | TypeData::UnknownKeyword
        )
    }

    /// Returns whether this type is an array of the explicit `unknown`
    /// keyword, such as `unknown[]` or `readonly unknown[]`.
    pub fn is_unknown_array(self) -> bool {
        array_element(self.db, self.data).is_some_and(|element| {
            matches!(
                element.expand_canonical_global(self.db),
                TypeData::UnknownKeyword
            )
        })
    }

    /// Returns the type that awaiting a value of this type produces.
    ///
    /// Promises, including nested Promises and instances of types that extend
    /// `Promise`, are unwrapped. Every other type is returned unchanged.
    pub fn awaited_type(self) -> Self {
        Self::new(self.db, awaited(self.db, self.data))
    }

    /// Returns the declared return type of the callable variant of this type,
    /// ignoring `null` and `undefined` variants of a union.
    ///
    /// Returns `None` when there's no callable variant or more than one.
    pub fn non_nullish_callable_return_type(self) -> Option<Self> {
        let mut callables = self
            .data
            .union_iterator(self.db)
            .filter(|ty| !matches!(ty, TypeData::Null | TypeData::Undefined));
        let callable = callables.next()?;
        if callables.next().is_some() {
            return None;
        }
        Self::new(self.db, callable).callable_return_type()
    }

    /// Returns whether this type is an instance of a generic type with type
    /// arguments, such as `Set<string>`.
    pub fn has_type_arguments(self) -> bool {
        matches!(
            self.data,
            TypeData::InstanceOf(instance) if !instance.type_parameters(self.db).is_empty()
        )
    }

    /// Returns whether a value of this type passes `any` into a type argument
    /// of `receiver` that expects a specific type, such as `Set<any>` passed
    /// where `Set<string>` is expected.
    ///
    /// Only instances of the same generic type are compared, argument by
    /// argument. Receiving types that are `any`, `unknown`, or unresolved
    /// accept `any`.
    pub fn has_unsafe_any_type_argument_for(self, receiver: Self) -> bool {
        let (TypeData::InstanceOf(_), TypeData::InstanceOf(_)) = (self.data, receiver.data) else {
            return false;
        };
        is_unsafe_any_assignment(self.db, self.data, receiver.data, 0)
    }
}

fn is_any<'db>(db: &'db dyn TypeDb, ty: TypeData<'db>) -> bool {
    matches!(ty.expand_canonical_global(db), TypeData::AnyKeyword)
}

fn is_any_array<'db>(db: &'db dyn TypeDb, ty: TypeData<'db>) -> bool {
    array_element(db, ty).is_some_and(|element| is_any(db, element))
}

/// Returns the element type of an array, a `readonly` array, or a
/// `ReadonlyArray`.
fn array_element<'db>(db: &'db dyn TypeDb, mut ty: TypeData<'db>) -> Option<TypeData<'db>> {
    if let TypeData::TypeOperator(operator) = ty
        && operator.operator(db) == TypeOperator::Readonly
    {
        ty = operator.ty(db);
    }
    let TypeData::InstanceOf(instance) = ty else {
        return None;
    };
    let target = instance.ty(db);
    let is_readonly_array = target == TypeData::GlobalType(READONLY_ARRAY_ID_GLOBAL_TYPE_ID)
        || matches!(
            target.expand_canonical_global(db),
            TypeData::Interface(interface) if interface.name(db).text() == "ReadonlyArray"
        );
    if !target.is_array_class(db) && !is_readonly_array {
        return None;
    }
    instance.type_parameters(db).first().copied()
}

/// Returns the type a Promise resolves to, for instances of `Promise`,
/// `PromiseLike`, or a class or interface that extends one of them.
///
/// Type arguments of the extending type aren't substituted, so a base such as
/// `Promise<T>` resolves to the generic `T`.
fn promise_argument<'db>(
    db: &'db dyn TypeDb,
    ty: TypeData<'db>,
    depth: usize,
) -> Option<TypeData<'db>> {
    if depth > MAX_PROMISE_TYPE_STEPS {
        return None;
    }
    let TypeData::InstanceOf(instance) = ty else {
        return None;
    };
    let target = instance.ty(db);
    if target.is_promise_class(db) {
        return instance.type_parameters(db).first().copied();
    }
    match target.expand_canonical_global(db) {
        TypeData::Interface(interface) => {
            if interface.name(db).text() == "PromiseLike" {
                return instance.type_parameters(db).first().copied();
            }
            interface
                .extends(db)
                .iter()
                .find_map(|base| promise_argument(db, *base, depth + 1))
        }
        TypeData::Class(class) => class
            .extends(db)
            .and_then(|base| promise_argument(db, base, depth + 1)),
        _ => None,
    }
}

fn awaited<'db>(db: &'db dyn TypeDb, mut ty: TypeData<'db>) -> TypeData<'db> {
    for _ in 0..MAX_PROMISE_TYPE_STEPS {
        match promise_argument(db, ty, 0) {
            Some(argument) => ty = argument,
            None => break,
        }
    }
    ty
}

fn is_unsafe_any_assignment<'db>(
    db: &'db dyn TypeDb,
    sender: TypeData<'db>,
    receiver: TypeData<'db>,
    depth: usize,
) -> bool {
    if depth > MAX_TYPE_RELATION_DEPTH {
        return false;
    }
    if is_any(db, sender) {
        let receiver = receiver.expand_canonical_global(db);
        return !receiver.is_indeterminate() && !matches!(receiver, TypeData::Conditional);
    }
    let (TypeData::InstanceOf(sender), TypeData::InstanceOf(receiver)) = (sender, receiver) else {
        return false;
    };
    if !is_same_generic_type(db, sender.ty(db), receiver.ty(db)) {
        return false;
    }
    sender
        .type_parameters(db)
        .iter()
        .zip(receiver.type_parameters(db))
        .any(|(sender, receiver)| is_unsafe_any_assignment(db, *sender, *receiver, depth + 1))
}

/// Returns whether `a` and `b` refer to the same generic class or interface.
///
/// Instantiating a class, as in `new Set<any>()`, can produce a copy of the
/// class whose members use the type arguments, so classes and interfaces are
/// also compared by name and by their type parameters.
fn is_same_generic_type<'db>(db: &'db dyn TypeDb, a: TypeData<'db>, b: TypeData<'db>) -> bool {
    if a == b {
        return true;
    }
    match (a.expand_canonical_global(db), b.expand_canonical_global(db)) {
        (TypeData::Class(a), TypeData::Class(b)) => {
            a == b
                || (a.is_builtin(db) == b.is_builtin(db)
                    && a.name(db) == b.name(db)
                    && a.type_parameters(db) == b.type_parameters(db))
        }
        (TypeData::Interface(a), TypeData::Interface(b)) => {
            a == b || (a.name(db) == b.name(db) && a.type_parameters(db) == b.type_parameters(db))
        }
        _ => false,
    }
}

//! Conservative branch filtering of normalized types and unions by runtime
//! tests.

use std::borrow::Cow;

use biome_js_syntax::numbers::{
    canonicalize_js_bigint_literal, parse_js_number_with_single_rounding,
};
use biome_rowan::Text;

use crate::builders::IntersectionBuilder;
use crate::generated::global_types::ids::FUNCTION_ID_GLOBAL_TYPE_ID;
use crate::globals::{
    GLOBAL_ANY_KEYWORD_ID, GLOBAL_BIGINT_KEYWORD_ID, GLOBAL_BOOLEAN_KEYWORD_ID,
    GLOBAL_NULL_KEYWORD_ID, GLOBAL_NUMBER_KEYWORD_ID, GLOBAL_OBJECT_KEYWORD_ID,
    GLOBAL_STRING_KEYWORD_ID, GLOBAL_SYMBOL_KEYWORD_ID, GLOBAL_UNDEFINED_ID, GLOBAL_UNKNOWN_ID,
    GLOBAL_UNKNOWN_KEYWORD_ID,
};
use crate::interned_types::{
    InternedGenericTypeParameter, InternedIntersection, InternedLiteral, InternedObject, Literal,
    TypeMember, TypeMemberKind,
};
use crate::resolved::InferredTypeData;
use crate::type_relation::{
    Assignability, InstanceType, PropertyLookup, PropertyPresence, TypeResolver,
    assignment_reduced_members, find_property, instance_types, is_assignable_in_generic_body,
    is_derived_from, is_empty_object_type, is_subtype, primitive_can_satisfy, property_presence,
    referenced_generic, resolve_wrapped_type, widened_literal,
};
use crate::{
    ObjectKind, RawTypeData, RawTypeId, TypeDb, TypeReference, TypeReferenceQualifier,
    TypeofExpression,
};

const MAX_NARROWING_STEPS: usize = 256;
const MAX_LITERAL_BYTES: usize = 1024;

/// A possible result of JavaScript's runtime `typeof` operator.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypeofKind {
    Undefined,
    Object,
    Boolean,
    Number,
    BigInt,
    String,
    Symbol,
    Function,
}

/// A runtime test applied to a value, not a structural type intersection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NarrowingPredicate<'db> {
    Truthy,
    Nullish,
    Typeof(TypeofKind),
    /// Strict equality with a primitive literal, `null`, or `undefined`.
    /// Other operand types leave the input unchanged.
    Literal(InferredTypeData<'db>),
    /// Loose equality (`==`) with a string, number, bigint, or boolean
    /// literal. Loose equality with `null` or `undefined` is [`Self::Nullish`].
    /// Other operand types leave the input unchanged.
    LooseLiteral(InferredTypeData<'db>),
    /// Equality with a value of type `value` that is not a unit type, such
    /// as `string` or `"a" | "b"`, strict or loose. [`Self::new_equality`]
    /// gives the other predicates for a unit type.
    Equal {
        value: InferredTypeData<'db>,
        strict: bool,
    },
}

impl<'db> NarrowingPredicate<'db> {
    /// Returns the predicate of an equality with a value of type `value`,
    /// strict (`===`) when `strict` is true and loose (`==`) otherwise.
    ///
    /// A unit type, `null`, `undefined`, or a string, number, bigint, or
    /// boolean literal that Biome decodes, gives the predicate of a
    /// comparison with that value, so the false branch removes the members
    /// equal to it. Any other type gives [`Self::Equal`], whose false branch
    /// removes nothing, because a value of such a type can differ from each
    /// member of the narrowed type.
    pub fn new_equality(db: &'db dyn TypeDb, value: InferredTypeData<'db>, strict: bool) -> Self {
        match value {
            InferredTypeData::Null | InferredTypeData::Undefined if !strict => Self::Nullish,
            InferredTypeData::Null | InferredTypeData::Undefined => Self::Literal(value),
            InferredTypeData::Literal(_) if literal_value(db, value).is_some() => {
                if strict {
                    Self::Literal(value)
                } else {
                    Self::LooseLiteral(value)
                }
            }
            _ => Self::Equal { value, strict },
        }
    }
}

/// Returns whether `ty` carries no information that a runtime test could
/// filter: the internal `Unknown`, which inference gives a type it couldn't
/// determine.
///
/// `Unknown` absorbs every union it joins, and the narrowing functions
/// return it unchanged for every test and branch. No test can therefore
/// refine it or prove a branch impossible, so callers can skip collecting
/// tests for it. `never` is excluded: it also stays unchanged, but it already
/// proves that a branch is impossible. `any` is excluded too, because a
/// primitive `typeof` test, `instanceof`, and a type predicate narrow it, as
/// [`narrow_type`] describes.
pub fn is_narrowing_invariant(ty: InferredTypeData<'_>) -> bool {
    ty == InferredTypeData::Unknown
}

/// Returns whether a collected type reference already denotes the internal
/// `Unknown`, without resolving it.
///
/// This is the resolution-free counterpart of [`is_narrowing_invariant`];
/// `raw_types` is the module table that local references index. It returns
/// false for references that only resolve to such a type, such as an alias.
pub fn is_raw_narrowing_invariant(reference: &TypeReference, raw_types: &[RawTypeData]) -> bool {
    let TypeReference::Resolved(id) = reference else {
        return false;
    };
    match id {
        RawTypeId::Global(_) => *id == GLOBAL_UNKNOWN_ID,
        RawTypeId::Local(local) => {
            matches!(raw_types.get(local.index()), Some(RawTypeData::Unknown))
        }
    }
}

/// Returns whether a collected type reference already denotes `any`, as the
/// annotation `x: any` does, without resolving it.
///
/// Only some runtime tests narrow `any`, as [`narrow_type`] describes, so
/// collection uses this to skip the other tests of such a variable.
/// `raw_types` is the module table that local references index. It returns
/// false for references that only resolve to `any`, such as an alias or an
/// import.
pub fn is_raw_any(reference: &TypeReference, raw_types: &[RawTypeData]) -> bool {
    let TypeReference::Resolved(id) = reference else {
        return false;
    };
    match id {
        RawTypeId::Global(_) => *id == GLOBAL_ANY_KEYWORD_ID,
        RawTypeId::Local(local) => {
            matches!(raw_types.get(local.index()), Some(RawTypeData::AnyKeyword))
        }
    }
}

/// Returns whether a collected type reference may resolve to a union with at
/// least two members that are object types, without resolving it.
///
/// A runtime test of a property narrows the value that holds it only when
/// the value's declared type is such a union. [`may_narrow_by_property`]
/// checks the same condition on a resolved type.
///
/// Returns false when the raw tables show that the type is one of these:
///
/// - a primitive type or a literal type;
/// - a single object type, interface, class, function, tuple, intersection,
///   or generic written in place;
/// - the inferred result of `new`, taken as one instance;
/// - the inferred result of `+`, unary `-`, `~`, or `typeof`, which is a
///   primitive;
/// - a union of the types above with at most one that is not primitive, as
///   in `o: { v: string | null }` or `x: { a: 1 } | undefined`.
///
/// A type name, an import, a `typeof` type, other inferred types, and a chain
/// of raw references longer than the step limit may stand for a union of
/// object types, so this returns true for them. `raw_types` is the module
/// table that local references index.
pub fn raw_may_be_object_union(reference: &TypeReference, raw_types: &[RawTypeData]) -> bool {
    raw_object_members(reference, raw_types) >= 2
}

/// Returns whether a collected type reference may resolve to a type with an
/// object type or `unknown`, which `instanceof` can narrow, without resolving
/// it. [`may_narrow_by_instanceof`] answers the same question for a resolved
/// type.
///
/// This returns false when the raw tables show that every member of the type
/// is a primitive type, a literal type of a primitive, `null`, or
/// `undefined`, as for `x: string | null`, and for the results of `+`, unary
/// `-`, `~`, and `typeof`. Otherwise, as for an object type, `unknown`, a
/// type name, or an inferred type, it returns true. `raw_types` is the
/// module table that local references index.
pub fn raw_may_narrow_by_instanceof(reference: &TypeReference, raw_types: &[RawTypeData]) -> bool {
    raw_object_members(reference, raw_types) >= 1
}

/// Returns whether a collected type reference may resolve to a type that
/// `null` or `undefined` can have, without resolving it.
///
/// A runtime test that a value is not nullish, such as the one that `?.`
/// applies to its object, narrows only such a type. Returns false when the
/// raw tables show that every member of the type is `string`, `number`,
/// `boolean`, `bigint`, `symbol`, a literal type, an object, interface, class,
/// function, constructor, or tuple type written in place, `object`, `this`, a
/// module or namespace, or the result of `new`, `+`, unary `-`, `~`, or
/// `typeof`, as for the object literal of `const o = { a: 1 }`. Every other
/// type may be nullish, including `null`, `undefined`, `void`, `unknown`,
/// `any`, a type name, an import, a generic, a `typeof` query, a nested union,
/// the result of a call, and a type behind a chain of raw references longer
/// than the step limit. `raw_types` is the module table that local references
/// index.
pub fn raw_may_be_nullish(reference: &TypeReference, raw_types: &[RawTypeData]) -> bool {
    let Some(target) = raw_reference_target(reference, raw_types) else {
        return true;
    };
    match target {
        RawReferenceTarget::Local(RawTypeData::Union(union)) => {
            union
                .types()
                .iter()
                .any(|member| match raw_reference_target(member, raw_types) {
                    // A nested union may be nullish, without looking into it.
                    Some(target) => raw_target_may_be_nullish(&target),
                    None => true,
                })
        }
        target => raw_target_may_be_nullish(&target),
    }
}

/// Returns whether a collected type reference may resolve to a union,
/// without resolving it.
///
/// An assignment narrows a variable or a property only when its declared
/// type is a union; see [`narrow_by_assignment`]. This returns false when the
/// raw tables show that the type is a primitive type, a literal type,
/// `boolean`, an object, interface, class, function, constructor, or tuple
/// type written in place, `object`, `any`, `unknown`, or the result of `new`,
/// `+`, unary `-`, `~`, or `typeof`. `boolean` counts as no union because an
/// assignment keeps it whole. A type name, an import, an intersection, other
/// inferred types, and a chain of raw references longer than the step limit
/// may be unions. Resolution widens the literal type of a mutable location to
/// its primitive type. This follows such a type to its value, because
/// widening keeps a union a union. `raw_types` is the module table that local
/// references index.
pub fn raw_may_be_union(reference: &TypeReference, raw_types: &[RawTypeData]) -> bool {
    let Some(target) = raw_reference_target(reference, raw_types) else {
        return true;
    };
    let data = match target {
        RawReferenceTarget::Global(id) => {
            return !is_primitive_global(id)
                && ![
                    GLOBAL_ANY_KEYWORD_ID,
                    GLOBAL_UNKNOWN_ID,
                    GLOBAL_UNKNOWN_KEYWORD_ID,
                    GLOBAL_OBJECT_KEYWORD_ID,
                ]
                .contains(&id);
        }
        RawReferenceTarget::Local(data) => data,
    };
    match data {
        RawTypeData::AnyKeyword
        | RawTypeData::BigInt
        | RawTypeData::Boolean
        | RawTypeData::Class(_)
        | RawTypeData::Constructor(_)
        | RawTypeData::Function(_)
        | RawTypeData::Interface(_)
        | RawTypeData::Literal(_)
        | RawTypeData::Null
        | RawTypeData::Number
        | RawTypeData::Object(_)
        | RawTypeData::ObjectKeyword
        | RawTypeData::String
        | RawTypeData::Symbol
        | RawTypeData::Tuple(_)
        | RawTypeData::Undefined
        | RawTypeData::Unknown
        | RawTypeData::UnknownKeyword => false,
        RawTypeData::TypeofExpression(expression) => !matches!(
            expression.as_ref(),
            TypeofExpression::New(_)
                | TypeofExpression::Addition(_)
                | TypeofExpression::BitwiseNot(_)
                | TypeofExpression::Typeof(_)
                | TypeofExpression::UnaryMinus(_)
        ),
        _ => true,
    }
}

/// Returns whether a collected type reference may resolve to a type with a
/// literal member, without resolving it.
///
/// A compound assignment, `++`, or `--` replaces each literal member of the
/// type of a variable or a property with its primitive type, so it changes
/// only a type with a literal member; see [`base_literal_type`]. This returns
/// false when the raw tables show that the type has none: a primitive type,
/// `boolean`, an object type written in place, the result of `new`, or the
/// literal value of a mutable location, as for `let i = 0`, because
/// resolution widens it to its primitive type. A type name, an import, a
/// union with a member that may have one, other inferred types, a property
/// read as the value of a mutable location, as `o.n` in `let i = o.n`, whose
/// declared literal types resolution keeps, and a chain of raw references
/// longer than the step limit may have literal members.
/// `raw_types` is the module table that local references index.
pub fn raw_may_have_literal(reference: &TypeReference, raw_types: &[RawTypeData]) -> bool {
    let mut pending = vec![reference];
    let mut remaining = MAX_RAW_REFERENCE_STEPS;
    while let Some(reference) = pending.pop() {
        let Some(left) = remaining.checked_sub(1) else {
            return true;
        };
        remaining = left;
        let data = match reference {
            TypeReference::Qualifier(_) | TypeReference::Import(_) => return true,
            // Global handles stand for keywords and built-in types.
            TypeReference::Resolved(RawTypeId::Global(_)) => continue,
            TypeReference::Resolved(RawTypeId::Local(local)) => {
                let Some(data) = raw_types.get(local.index()) else {
                    return true;
                };
                data
            }
        };
        match data {
            RawTypeData::Reference(next) => pending.push(next),
            RawTypeData::Union(union) => pending.extend(union.types()),
            RawTypeData::Literal(literal) => {
                if literal.is_primitive() {
                    return true;
                }
            }
            RawTypeData::TypeofExpression(expression) => match expression.as_ref() {
                TypeofExpression::WidenedLiteral(widened) => {
                    if is_raw_member_access(&widened.value, raw_types) {
                        return true;
                    }
                }
                TypeofExpression::New(_) => {}
                _ => return true,
            },
            RawTypeData::AnyKeyword
            | RawTypeData::BigInt
            | RawTypeData::Boolean
            | RawTypeData::Class(_)
            | RawTypeData::Constructor(_)
            | RawTypeData::Function(_)
            | RawTypeData::Interface(_)
            | RawTypeData::Null
            | RawTypeData::Number
            | RawTypeData::Object(_)
            | RawTypeData::ObjectKeyword
            | RawTypeData::String
            | RawTypeData::Symbol
            | RawTypeData::Tuple(_)
            | RawTypeData::Undefined
            | RawTypeData::Unknown
            | RawTypeData::UnknownKeyword => {}
            _ => return true,
        }
    }
    false
}

/// Returns whether `reference` is the collected type of a member access, as
/// `o.n` or `o["n"]`.
fn is_raw_member_access(reference: &TypeReference, raw_types: &[RawTypeData]) -> bool {
    matches!(
        reference,
        TypeReference::Resolved(RawTypeId::Local(local))
            if matches!(
                raw_types.get(local.index()),
                Some(RawTypeData::TypeofExpression(expression))
                    if matches!(
                        expression.as_ref(),
                        TypeofExpression::StaticMember(_)
                            | TypeofExpression::OptionalChainStaticMember(_)
                            | TypeofExpression::ComputedMember(_)
                    )
            )
    )
}

/// Returns whether `target` may be nullish. A union may be nullish without
/// looking into its members. [`raw_may_be_nullish`] lists the types that
/// can't be nullish.
fn raw_target_may_be_nullish(target: &RawReferenceTarget) -> bool {
    let data = match target {
        RawReferenceTarget::Global(id) => {
            return ![
                GLOBAL_BIGINT_KEYWORD_ID,
                GLOBAL_BOOLEAN_KEYWORD_ID,
                GLOBAL_NUMBER_KEYWORD_ID,
                GLOBAL_STRING_KEYWORD_ID,
                GLOBAL_SYMBOL_KEYWORD_ID,
            ]
            .contains(id);
        }
        RawReferenceTarget::Local(data) => data,
    };
    match data {
        RawTypeData::BigInt
        | RawTypeData::Boolean
        | RawTypeData::Number
        | RawTypeData::String
        | RawTypeData::Symbol
        | RawTypeData::Literal(_)
        | RawTypeData::Class(_)
        | RawTypeData::Constructor(_)
        | RawTypeData::Function(_)
        | RawTypeData::Interface(_)
        | RawTypeData::Module(_)
        | RawTypeData::Namespace(_)
        | RawTypeData::Object(_)
        | RawTypeData::Tuple(_)
        | RawTypeData::ImportNamespace(_)
        | RawTypeData::ObjectKeyword
        | RawTypeData::ThisKeyword => false,
        RawTypeData::TypeofExpression(expression) => !matches!(
            expression.as_ref(),
            TypeofExpression::New(_)
                | TypeofExpression::Addition(_)
                | TypeofExpression::BitwiseNot(_)
                | TypeofExpression::Typeof(_)
                | TypeofExpression::UnaryMinus(_)
        ),
        _ => true,
    }
}

/// Returns how many object types the type at `reference` may have among its
/// members, as far as raw tables tell, up to two.
///
/// Returns zero for primitive types and one for a single object type. Returns
/// two for a union with two object types and for a type that raw tables can't
/// resolve, such as a type name or an import.
fn raw_object_members(reference: &TypeReference, raw_types: &[RawTypeData]) -> usize {
    let Some(target) = raw_reference_target(reference, raw_types) else {
        return 2;
    };
    match target {
        // Global handles stand for keywords and built-in types.
        RawReferenceTarget::Global(id) => usize::from(!is_primitive_global(id)),
        RawReferenceTarget::Local(RawTypeData::Union(union)) => {
            let mut count = 0;
            for member in union.types() {
                count += match raw_reference_target(member, raw_types) {
                    Some(RawReferenceTarget::Global(id)) => usize::from(!is_primitive_global(id)),
                    // A nested union counts as two, without looking into it.
                    Some(RawReferenceTarget::Local(member)) => raw_local_object_members(member),
                    None => 2,
                };
                if count >= 2 {
                    return 2;
                }
            }
            count
        }
        RawReferenceTarget::Local(data) => raw_local_object_members(data),
    }
}

/// Returns how many object types the local type `data` may have, up to two.
/// A union, a reference, and other types that only resolution can tell count
/// as two.
fn raw_local_object_members(data: &RawTypeData) -> usize {
    match data {
        RawTypeData::BigInt
        | RawTypeData::Boolean
        | RawTypeData::Null
        | RawTypeData::Number
        | RawTypeData::String
        | RawTypeData::Symbol
        | RawTypeData::Undefined => 0,
        RawTypeData::Literal(literal) => usize::from(!literal.is_primitive()),
        RawTypeData::TypeofExpression(expression) => match expression.as_ref() {
            // A construct signature rarely returns a union. When one does,
            // the value keeps its wider declared type.
            TypeofExpression::New(_) => 1,
            TypeofExpression::Addition(_)
            | TypeofExpression::BitwiseNot(_)
            | TypeofExpression::Typeof(_)
            | TypeofExpression::UnaryMinus(_) => 0,
            _ => 2,
        },
        RawTypeData::Conditional
        | RawTypeData::Extends(_)
        | RawTypeData::Global
        | RawTypeData::ImportNamespace(_)
        | RawTypeData::IndexedAccess(_)
        | RawTypeData::InstanceOf(_)
        | RawTypeData::MappedType(_)
        | RawTypeData::MergedReference(_)
        | RawTypeData::Reference(_)
        | RawTypeData::TypeofType(_)
        | RawTypeData::TypeofValue(_)
        | RawTypeData::Union(_)
        | RawTypeData::Unknown => 2,
        _ => 1,
    }
}

/// Returns whether the global type `id` is a primitive keyword, `null`, or
/// `undefined`.
fn is_primitive_global(id: RawTypeId) -> bool {
    [
        GLOBAL_BIGINT_KEYWORD_ID,
        GLOBAL_BOOLEAN_KEYWORD_ID,
        GLOBAL_NULL_KEYWORD_ID,
        GLOBAL_NUMBER_KEYWORD_ID,
        GLOBAL_STRING_KEYWORD_ID,
        GLOBAL_SYMBOL_KEYWORD_ID,
        GLOBAL_UNDEFINED_ID,
    ]
    .contains(&id)
}

/// What a raw reference stands for once its chain of references is followed.
enum RawReferenceTarget<'a> {
    Global(RawTypeId),
    Local(&'a RawTypeData),
}

/// Follows `reference` through raw references to the type it stands for.
///
/// Follows a [`TypeofExpression::WidenedLiteral`] to the value that it
/// widens, because widening a literal type to its primitive type doesn't
/// change whether the type can be nullish or an object.
///
/// Returns `None` for a type name or an import, which raw tables don't
/// resolve, for a local id outside `raw_types`, and when the chain exceeds
/// [`MAX_RAW_REFERENCE_STEPS`].
fn raw_reference_target<'a>(
    reference: &'a TypeReference,
    raw_types: &'a [RawTypeData],
) -> Option<RawReferenceTarget<'a>> {
    let mut reference = reference;
    for _ in 0..MAX_RAW_REFERENCE_STEPS {
        match reference {
            TypeReference::Qualifier(_) | TypeReference::Import(_) => return None,
            TypeReference::Resolved(id @ RawTypeId::Global(_)) => {
                return Some(RawReferenceTarget::Global(*id));
            }
            TypeReference::Resolved(RawTypeId::Local(local)) => {
                let data = raw_types.get(local.index())?;
                match data {
                    RawTypeData::Reference(next) => reference = next,
                    RawTypeData::TypeofExpression(expression) => match expression.as_ref() {
                        TypeofExpression::WidenedLiteral(widened) => reference = &widened.value,
                        _ => return Some(RawReferenceTarget::Local(data)),
                    },
                    _ => return Some(RawReferenceTarget::Local(data)),
                }
            }
        }
    }
    None
}

/// Limits the chain of raw references that [`raw_reference_target`] and
/// [`raw_predicate_target`] follow.
const MAX_RAW_REFERENCE_STEPS: usize = 16;

/// What a collected type reference names as the target of a type predicate,
/// such as `Foo` in `value is Foo`, as far as raw tables tell.
#[derive(Debug)]
pub enum RawPredicateTarget<'a> {
    /// An object type: a type literal, an interface or a class written in
    /// place, a function or constructor type, a tuple, `object`, an object
    /// literal type, or a name with type arguments, such as `Promise<void>`
    /// or `Record<string, number>`. A name with type arguments can also name
    /// a type alias.
    Object,
    /// A type whose values may be primitives: a primitive type, a primitive
    /// literal, `null`, `undefined`, `void`, `never`, a `typeof` query, an
    /// intersection, an import, or any other type that raw tables don't show
    /// to be an object type.
    Other,
    /// `any`, `unknown`, or a type that inference couldn't determine. None of
    /// these narrows anything as a target.
    Top,
    /// A union, whose members are classified separately.
    Union(&'a [TypeReference]),
    /// A name without type arguments. Only the scope that declares the name
    /// tells whether it is an interface, a class, or a type alias.
    Named(&'a TypeReferenceQualifier),
}

/// Classifies the target of a type predicate as an object type, a type whose
/// values may be primitives, a type that narrows nothing, a union, or a name
/// without type arguments, without resolving it.
///
/// `raw_types` is the module table that local references index. A chain of
/// raw references longer than the step limit is [`RawPredicateTarget::Other`].
pub fn raw_predicate_target<'a>(
    reference: &'a TypeReference,
    raw_types: &'a [RawTypeData],
) -> RawPredicateTarget<'a> {
    let mut reference = reference;
    for _ in 0..MAX_RAW_REFERENCE_STEPS {
        let local = match reference {
            TypeReference::Qualifier(qualifier) if qualifier.type_parameters.is_empty() => {
                return RawPredicateTarget::Named(qualifier);
            }
            TypeReference::Qualifier(_) => return RawPredicateTarget::Object,
            TypeReference::Import(_) => return RawPredicateTarget::Other,
            TypeReference::Resolved(id @ RawTypeId::Global(_)) => {
                return if [
                    GLOBAL_ANY_KEYWORD_ID,
                    GLOBAL_UNKNOWN_ID,
                    GLOBAL_UNKNOWN_KEYWORD_ID,
                ]
                .contains(id)
                {
                    RawPredicateTarget::Top
                } else if *id == GLOBAL_OBJECT_KEYWORD_ID {
                    RawPredicateTarget::Object
                } else {
                    RawPredicateTarget::Other
                };
            }
            TypeReference::Resolved(RawTypeId::Local(local)) => local,
        };
        return match raw_types.get(local.index()) {
            Some(RawTypeData::Reference(next)) => {
                reference = next;
                continue;
            }
            // A reference by name to a type collects as an instance of it.
            Some(RawTypeData::InstanceOf(instance)) if instance.type_parameters.is_empty() => {
                reference = &instance.ty;
                continue;
            }
            Some(RawTypeData::Union(union)) => RawPredicateTarget::Union(union.types()),
            Some(RawTypeData::Literal(literal)) if literal.is_primitive() => {
                RawPredicateTarget::Other
            }
            Some(
                RawTypeData::Class(_)
                | RawTypeData::Constructor(_)
                | RawTypeData::Function(_)
                | RawTypeData::InstanceOf(_)
                | RawTypeData::Interface(_)
                | RawTypeData::Literal(_)
                | RawTypeData::Object(_)
                | RawTypeData::ObjectKeyword
                | RawTypeData::Tuple(_),
            ) => RawPredicateTarget::Object,
            Some(RawTypeData::AnyKeyword | RawTypeData::Unknown | RawTypeData::UnknownKeyword)
            | None => RawPredicateTarget::Top,
            Some(_) => RawPredicateTarget::Other,
        };
    }
    RawPredicateTarget::Other
}

/// Narrows `ty` to the values that can pass a runtime test when `positive` is
/// true, or to the values that can fail it when `positive` is false.
///
/// Pass a normalized `ty`. The result is `NeverKeyword` only when no member of
/// `ty` can pass, and it is `ty` itself when the test removes nothing. A member
/// stays unchanged when the test can't tell whether its values pass, which is
/// the case for `Unknown` and unresolved handles. Other members follow these
/// rules:
///
/// - A `void` member becomes `Unknown`, because a `void` value can be any value
///   at runtime.
/// - `any` narrows only in the true branch of `typeof` with a primitive kind
///   or `"undefined"`, to the type of that kind, as in TypeScript. Every other
///   test and branch keeps `any`, including `typeof` with `"object"` or
///   `"function"`. [`narrow_by_instanceof`] and [`narrow_by_type_predicate`]
///   also narrow `any` in their true branches.
/// - `unknown` narrows as the union `{} | null | undefined` for truthiness,
///   nullishness, `typeof`, and strict equality with `null` or `undefined`, as
///   in TypeScript: each of the three narrows by the rules here, and the result
///   is `unknown` again when none of them changes. So a truthy `unknown` is
///   `{}`, `!== null` gives `{} | undefined`, and `typeof` with `"object"`
///   gives `object | null`. Strict equality with another literal gives the
///   literal in the true branch and keeps `unknown` in the false branch.
/// - `{}`, a type literal or an object expression without members, becomes
///   `object` in the true branch of `typeof` with `"object"`, as in
///   TypeScript. An empty interface or class stays as it is.
/// - A type parameter `T` becomes its intersection with the type of the
///   values that pass, as in TypeScript: `T & {}` for truthiness and
///   `!= null`, `T & ({} | undefined)` for `!== null`, `T & string` for
///   `typeof` with `"string"`, and so on, unless its constraint decides the
///   test. An intersection with a type parameter narrows each operand. The
///   type parameter's constraint is read through `resolve`.
/// - The false branch of an equality test keeps a primitive type such as
///   `string`, because a type can't exclude a single literal from it. The false
///   branch of a truthiness test keeps `string`, `number`, and `bigint` for the
///   same reason, as TypeScript does at a read. [`falsy_values`] gives the
///   narrower type of the value that `&&` returns.
/// - An equality test leaves `ty` unchanged when its literal is a template
///   literal, a string with escapes, or longer than 1024 bytes, because Biome
///   doesn't decode those.
/// - Loose equality with a literal ([`NarrowingPredicate::LooseLiteral`])
///   narrows the members of the literal's kind as strict equality does and
///   removes `null` and `undefined`. It keeps primitive types of other kinds
///   and object types, some of whose values convert to the literal, as
///   `"1" == 1` shows, and removes a literal of another kind only when it
///   never loosely equals the literal, as `"a"` for `== 1`. Its false branch
///   is that of strict equality, and it keeps `unknown` in both branches.
/// - Equality with a value whose type is not a unit type
///   ([`NarrowingPredicate::Equal`]) keeps, in the true branch, the members
///   that may equal a value of that type, and replaces a primitive type with
///   the value's literals of its kind, so `string | number` compared with
///   `"a" | 1` becomes `"a" | 1`. A loose comparison also keeps the members
///   of other kinds whose values may convert to a compared value. A strict
///   comparison gives `unknown` the value's type when that is a single
///   primitive or literal type, and `object` when it is a single object
///   type. The false branch keeps every member.
/// - A union with more than 256 types, counting nested unions and their
///   members, is returned unchanged.
///
/// A primitive or a function can have an object type, such as
/// `{ length: number }`, which every string has. So falsy and `typeof` tests
/// compare an object type with the apparent types of the values that pass:
///
/// - The false branch of a truthiness test removes it when no falsy string,
///   number, boolean, or bigint can have it.
/// - The true branch of `typeof` with a primitive kind or `"function"` removes
///   it when no value of that kind can have it. It replaces it with the
///   primitive type or `Function` when every value of that kind has it, as
///   TypeScript does.
/// - The true branch of `typeof` with `"undefined"` removes it.
/// - The false branch of `typeof` with `"object"` removes it when no primitive
///   and no function can have it.
///
/// Other branches keep object types, and so does a comparison that can't
/// decide. An object type is a type literal, an interface, or an instance of a
/// class, an interface, or a type literal.
///
/// `resolve` replaces a local or global handle with the type it stands for.
/// Narrowing calls it only to compare object types.
pub fn narrow_type<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    predicate: NarrowingPredicate<'db>,
    positive: bool,
    mut resolve: impl FnMut(InferredTypeData<'db>) -> InferredTypeData<'db>,
) -> InferredTypeData<'db> {
    if is_narrowing_invariant(ty) {
        return ty;
    }
    let Some(predicate) = PreparedPredicate::new(db, predicate) else {
        return ty;
    };
    narrow_prepared(db, ty, &predicate, positive, &mut resolve)
}

/// A runtime test of a property of a value, as `shape.kind === "circle"`
/// tests the property `kind` of `shape`.
#[derive(Clone, Copy, Debug)]
pub struct PropertyTest<'a, 'db> {
    /// The name of the tested property.
    pub name: &'a str,
    /// The test that the property's value passes when `positive` is true, or
    /// fails when it is false.
    pub predicate: NarrowingPredicate<'db>,
    pub positive: bool,
    /// Whether the test reads the property through an optional chain, as in
    /// `shape?.kind`, which gives `undefined` when `shape` is `null` or
    /// `undefined`.
    pub optional_chain: bool,
}

/// Narrows `ty` by a runtime test of one of its properties, as
/// `if (shape.kind === "circle")` narrows `shape`, following TypeScript's
/// narrowing by a discriminant.
///
/// `declared` is the declared type of the tested value, and it decides
/// whether the property is a discriminant. As in TypeScript, the property is
/// a discriminant when `declared` is a union and the property type of at
/// least one of its members is made of unit types, such as `"circle"`, `1`,
/// `null`, or `boolean`. Members that are primitives or that lack the
/// property don't count. When the property is not a discriminant, `ty` is
/// returned unchanged, so `{ k: string } | { k: number }` is not narrowed by
/// `typeof k`. A property whose type is the same in every member, which
/// TypeScript does not treat as a discriminant either, keeps every member.
/// `ty` is also returned unchanged when `declared` has fewer than two members
/// that may be object types, the condition that [`may_narrow_by_property`]
/// checks. In that case, the only narrowed type TypeScript can give is
/// `never`.
///
/// The test's predicate narrows the union of the property types of the
/// members of `ty`, the same way [`narrow_type`] narrows a value. The result
/// keeps the members whose property type shares a value with what the
/// predicate leaves. So `kind !== "c"` keeps both members of
/// `{ kind: "c" } | { kind: string }`, because the predicate leaves `string`,
/// which shares a value with `"c"`. A test that no property value passes
/// gives `never`, as does a single member whose property the test rules out
/// when `ty` is not a union. Object types are assumed to share a value with
/// every type except `null` and `undefined`.
///
/// A `null` or `undefined` member of `ty` is kept when the test reads the
/// property through an optional chain, because the chain gives the property
/// the value `undefined` for that member. `ty` is returned unchanged in these
/// cases:
///
/// - a member of `ty` is `null` or `undefined`, and the test reads the
///   property without an optional chain;
/// - another member of `ty` is not an object type with the property;
/// - a property type includes a type that this function can't compare, such
///   as a generic, `unknown`, or an intersection;
/// - the union traversal, which visits at most 256 types, runs out.
///
/// `resolve` resolves handles as for [`narrow_type`], and is also used to
/// look up the properties.
pub fn narrow_by_property<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    declared: InferredTypeData<'db>,
    test: PropertyTest<'_, 'db>,
    mut resolve: impl FnMut(InferredTypeData<'db>) -> InferredTypeData<'db>,
) -> InferredTypeData<'db> {
    if is_narrowing_invariant(ty)
        || ty == InferredTypeData::NeverKeyword
        || !may_narrow_by_property(db, declared)
    {
        return ty;
    }
    let Some(predicate) = PreparedPredicate::new(db, test.predicate) else {
        return ty;
    };
    let resolve: &mut TypeResolver<'db, '_> = &mut resolve;
    let mut remaining = MAX_NARROWING_STEPS;
    let Some(members) = union_members(db, ty, &mut remaining) else {
        return ty;
    };
    let mut properties = Vec::with_capacity(members.len());
    let mut has_nullish = false;
    for member in &members {
        match member_property(db, *member, test.name, resolve, &mut remaining) {
            Some(MemberProperty::Nullish) if test.optional_chain => {
                has_nullish = true;
                properties.push(None);
            }
            Some(MemberProperty::Values(values)) => properties.push(Some(values)),
            _ => return ty,
        }
    }
    let is_discriminant = if ty == declared {
        properties.iter().flatten().any(is_unit)
    } else {
        is_discriminant(db, declared, test.name, resolve, &mut remaining)
    };
    if !is_discriminant {
        return ty;
    }
    let mut property_types: Vec<_> = properties
        .iter()
        .flatten()
        .flatten()
        .map(|(ty, _)| *ty)
        .collect();
    if property_types.is_empty() {
        return ty;
    }
    if has_nullish {
        property_types.push(InferredTypeData::Undefined);
    }
    let property = InferredTypeData::union_from_types(db, property_types);
    let narrowed = narrow_prepared(db, property, &predicate, test.positive, resolve);
    if narrowed == InferredTypeData::NeverKeyword {
        return narrowed;
    }
    let Some(remaining_values) = union_members(db, narrowed, &mut remaining).and_then(|types| {
        types
            .into_iter()
            .map(|ty| PropertyValue::new(db, ty, resolve))
            .collect::<Option<Vec<_>>>()
    }) else {
        return ty;
    };
    let kept: Vec<_> = members
        .iter()
        .zip(&properties)
        .filter(|(_, values)| {
            values.as_ref().is_none_or(|values| {
                values.iter().any(|(_, value)| {
                    remaining_values
                        .iter()
                        .any(|remaining| value.overlaps(db, remaining))
                })
            })
        })
        .map(|(member, _)| *member)
        .collect();
    if kept.len() == members.len() {
        ty
    } else {
        InferredTypeData::union_from_types(db, kept)
    }
}

/// Returns whether `declared` is a union with at least two members that are
/// neither `null`, `undefined`, nor a primitive type. Only then can
/// [`narrow_by_property`] narrow a value of that declared type to a type
/// other than `never`.
///
/// A test of a property keeps the `null` and `undefined` members, and no
/// primitive member has a discriminant. With at most one other member, the
/// test either keeps the value or rules out every member.
pub fn may_narrow_by_property(db: &dyn TypeDb, declared: InferredTypeData) -> bool {
    let InferredTypeData::Union(union) = declared else {
        return false;
    };
    union
        .types(db)
        .iter()
        .filter(|member| {
            !matches!(member, InferredTypeData::Null | InferredTypeData::Undefined)
                && !matches!(
                    typeof_kind(db, **member),
                    Some(
                        TypeofKind::BigInt
                            | TypeofKind::Boolean
                            | TypeofKind::Number
                            | TypeofKind::String
                            | TypeofKind::Symbol
                    )
                )
        })
        .nth(1)
        .is_some()
}

/// Returns whether every member of `ty` other than `null` and `undefined`
/// declares the property `name` itself as readonly, so that no assignment
/// through a value of type `ty` can change the property.
///
/// A property is readonly when it is declared `readonly`, when it belongs to
/// an object under an `as const` assertion, when a mapped type such as
/// `Readonly<T>` makes it readonly, and when it is an element of a `readonly`
/// tuple. A getter is never readonly, because Biome doesn't collect setters,
/// so a getter may have one. A property that a member inherits, through an
/// `extends` clause, a prototype, or a module augmentation, doesn't count,
/// because an augmentation in a module that Biome doesn't index can give the
/// member a property of the same name that isn't readonly.
///
/// Returns false when `ty` has no member other than `null` and `undefined`,
/// when a member is a primitive, a function, an array, an intersection, a
/// generic, `unknown`, or another type whose properties can't be looked up,
/// and when the union traversal, which visits at most 256 types, runs out.
/// `resolve` resolves handles as for [`narrow_type`], and is also used to
/// look up the property.
pub fn has_readonly_property<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    name: &str,
    mut resolve: impl FnMut(InferredTypeData<'db>) -> InferredTypeData<'db>,
) -> bool {
    let resolve: &mut TypeResolver<'db, '_> = &mut resolve;
    let mut pending = vec![ty];
    let mut has_member = false;
    for _ in 0..MAX_NARROWING_STEPS {
        let Some(current) = pending.pop() else {
            return has_member;
        };
        match resolve_wrapped_type(db, current, resolve) {
            InferredTypeData::Union(union) => pending.extend(union.types(db).iter().copied()),
            InferredTypeData::Null | InferredTypeData::Undefined => {}
            member => match find_property(db, member, name, resolve) {
                PropertyLookup::Found {
                    is_readonly: true,
                    is_inherited: false,
                    ..
                } => has_member = true,
                PropertyLookup::Found { .. }
                | PropertyLookup::Missing
                | PropertyLookup::Unknown => {
                    return false;
                }
            },
        }
    }
    false
}

/// Narrows `ty` by the runtime test `name in value`, as TypeScript does.
///
/// Like TypeScript, Biome treats an object type as closed, so a member of the
/// union `ty` lacks every property that it doesn't declare.
/// [`property_presence`] looks up the property in each member.
///
/// - When `positive` is true, the members that lack the property are
///   removed. A member stays when it declares the property, even as
///   optional, or when one of its index signatures accepts the name. A member
///   also stays when Biome can't tell whether it has the property, as for a
///   primitive, a function, a tuple, or a class instance.
/// - When `positive` is true and every member lacks the property, the result
///   is the intersection of `ty` with `{ name: unknown }`, as TypeScript
///   gives `ty & Record<name, unknown>`. A member lacks the property when it
///   is an object type that neither declares it nor has an index signature
///   that accepts the name, `object`, `{}`, or a type parameter whose
///   constraint lacks it. The members of `Object`, such as `toString`, count
///   as declared. When no member declares the property but Biome can't tell
///   whether one lacks it, `ty` is returned unchanged.
/// - When `positive` is false, the members that declare the property as
///   required are removed, which can give `never`.
///
/// `null` and `undefined` lack every property. A `name` that starts with `#`
/// leaves `ty` unchanged, because Biome spells a private name such as `#p`
/// with its `#`, while the string `"#p"` never names a private member.
/// `ty` is also returned unchanged when the union traversal, which visits at
/// most 256 types, runs out. `resolve` replaces local and global handles with
/// the types they stand for, both to narrow `ty` and to look up the property.
pub fn narrow_by_in_operator<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    name: &Text,
    positive: bool,
    mut resolve: impl FnMut(InferredTypeData<'db>) -> InferredTypeData<'db>,
) -> InferredTypeData<'db> {
    let text = name.text();
    if is_narrowing_invariant(ty) || ty == InferredTypeData::NeverKeyword || text.starts_with('#') {
        return ty;
    }
    let resolve: &mut TypeResolver<'db, '_> = &mut resolve;
    let mut remaining = MAX_NARROWING_STEPS;
    let Some(members) = union_members(db, ty, &mut remaining) else {
        return ty;
    };
    let mut presences = Vec::with_capacity(members.len());
    for member in &members {
        let Some(left) = remaining.checked_sub(1) else {
            return ty;
        };
        remaining = left;
        let is_primitive = matches!(
            typeof_kind(db, *member),
            Some(
                TypeofKind::BigInt
                    | TypeofKind::Boolean
                    | TypeofKind::Number
                    | TypeofKind::String
                    | TypeofKind::Symbol
            )
        );
        presences.push(if is_primitive {
            PropertyPresence::Unknown
        } else {
            property_presence(db, *member, text, resolve)
        });
    }
    if positive
        && !presences.iter().any(|presence| {
            matches!(
                presence,
                PropertyPresence::Required | PropertyPresence::Optional | PropertyPresence::Indexed
            )
        })
    {
        let lacks_property = members.iter().zip(&presences).all(|(member, presence)| {
            *presence == PropertyPresence::Missing || lacks_property(db, *member, text, resolve)
        });
        return if lacks_property {
            let record = TypeMember {
                kind: TypeMemberKind::Named(name.clone()),
                ty: InferredTypeData::UnknownKeyword,
            };
            let record = InferredTypeData::Object(InternedObject::new(
                db,
                None,
                Box::from([record]),
                false,
                ObjectKind::Shape,
            ));
            intersect_narrowed(db, ty, record)
        } else {
            ty
        };
    }
    let removed = if positive {
        PropertyPresence::Missing
    } else {
        PropertyPresence::Required
    };
    let kept: Vec<_> = members
        .iter()
        .zip(&presences)
        .filter(|(_, presence)| **presence != removed)
        .map(|(member, _)| *member)
        .collect();
    if kept.len() == members.len() {
        ty
    } else if kept.is_empty() {
        InferredTypeData::NeverKeyword
    } else {
        InferredTypeData::union_from_types(db, kept)
    }
}

/// Returns whether `member` is `object` or a type parameter, and no value of
/// it has the property `name`. As TypeScript does for `name in value`, the
/// property is looked up on the type parameter's constraint, and on `Object`
/// for `object` and for a type parameter without a constraint.
///
/// Returns false for any other type, and when Biome can't tell, as for a
/// constraint whose members Biome can't list.
fn lacks_property<'db>(
    db: &'db dyn TypeDb,
    member: InferredTypeData<'db>,
    name: &str,
    resolve: &mut TypeResolver<'db, '_>,
) -> bool {
    let constraint = match referenced_generic(db, member) {
        Some(generic) => generic.constraint(db),
        None if member == InferredTypeData::ObjectKeyword => None,
        None => return false,
    };
    let Some(constraint) = constraint else {
        return property_presence(db, empty_object(db), name, resolve) == PropertyPresence::Missing;
    };
    let mut remaining = MAX_NARROWING_STEPS;
    let Some(constraint) = flattened(db, constraint, resolve) else {
        return false;
    };
    let Some(members) = union_members(db, constraint, &mut remaining) else {
        return false;
    };
    members.into_iter().all(|member| {
        if member == InferredTypeData::ObjectKeyword {
            property_presence(db, empty_object(db), name, resolve) == PropertyPresence::Missing
        } else {
            property_presence(db, member, name, resolve) == PropertyPresence::Missing
        }
    })
}

/// Narrows `ty` by the runtime test `value instanceof C`, where
/// `constructor` is the type of `C`, matching TypeScript except where noted.
///
/// The test compares each member of the union `ty` with the instance types
/// of `constructor`, such as `Date` for `x instanceof Date`, by declaration.
/// A type is derived from a class or an interface when its `extends` clauses
/// lead to that declaration, and [`is_derived_from`] lists the other cases.
///
/// - When `positive` is true, a member derived from an instance type stays,
///   a member that an instance type is derived from becomes that instance
///   type, and the other members are removed. So `A | string` becomes `B`
///   with `class B extends A`. A type parameter is derived from what its
///   constraint is derived from. When no member relates to an instance type,
///   each type parameter whose constraint the instance type is derived from,
///   or that has no constraint, gives its intersection with the instance
///   type, as `T` gives `T & A`. When no member remains, the result follows
///   TypeScript's choice between the instance type, `ty`, and their
///   intersection, as for a type predicate (see
///   [`narrow_by_type_predicate`]), so `Bar` gives `Bar & A`. `unknown` and
///   `any` become the instance type, which has an `any` type argument for
///   each type parameter of a generic class, as `Map<any, any>` for `Map`.
/// - When `positive` is false, the members derived from the instance type
///   are removed, which can give `never`. As in TypeScript, nothing is
///   removed when `constructor` has several instance types, or when the
///   instance type is `{}` or not an object type, and `any` stays `any`.
///
/// A member of a union that an instance of a generic class is derived from
/// stays as it is, because it has its own type arguments.
///
/// Returns `ty` unchanged when Biome can't tell the result:
///
/// - [`instance_types`] can't tell the instance types, as for `Object`,
///   `Function`, a generic construct signature, or a class with a static
///   `[Symbol.hasInstance]` method;
/// - a member of the union is `unknown` when `positive` is true;
/// - [`is_derived_from`] can't decide for a pair of types;
/// - a comparison that chooses among the instance type, `ty`, and their
///   intersection is undetermined;
/// - the union traversal would visit more than 256 types.
///
/// `resolve` resolves handles as for [`narrow_type`].
pub fn narrow_by_instanceof<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    constructor: InferredTypeData<'db>,
    positive: bool,
    mut resolve: impl FnMut(InferredTypeData<'db>) -> InferredTypeData<'db>,
) -> InferredTypeData<'db> {
    if is_narrowing_invariant(ty) || ty == InferredTypeData::NeverKeyword {
        return ty;
    }
    let resolve: &mut TypeResolver<'db, '_> = &mut resolve;
    let Some(instances) = instance_types(db, constructor, resolve) else {
        return ty;
    };
    let narrowed = if positive {
        instances_in(db, ty, &instances, resolve)
    } else if ty == InferredTypeData::AnyKeyword {
        None
    } else {
        instances_excluded(db, ty, &instances, resolve)
    };
    narrowed.unwrap_or(ty)
}

/// Returns the true branch of [`narrow_by_instanceof`], or `None` when it
/// leaves `ty` unchanged.
fn instances_in<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    instances: &[InstanceType<'db>],
    resolve: &mut TypeResolver<'db, '_>,
) -> Option<InferredTypeData<'db>> {
    if matches!(
        ty,
        InferredTypeData::UnknownKeyword | InferredTypeData::AnyKeyword
    ) {
        return instances
            .iter()
            .map(|instance| instance.with_any_arguments(db))
            .collect::<Option<Vec<_>>>()
            .map(|types| InferredTypeData::union_from_types(db, types));
    }
    let mut remaining = MAX_NARROWING_STEPS;
    let members = union_members(db, ty, &mut remaining)?;
    if members.contains(&InferredTypeData::UnknownKeyword) {
        return None;
    }
    let mut narrowed = Vec::new();
    for instance in instances {
        let related = narrowed.len();
        for member in &members {
            remaining = remaining.checked_sub(1)?;
            // A type parameter is derived from what its constraint is
            // derived from, and no instance type is derived from it.
            if let Some(generic) = referenced_generic(db, *member) {
                let is_derived = match generic.constraint(db) {
                    Some(constraint) => is_derived_from(db, constraint, instance.target, resolve)?,
                    None => false,
                };
                if is_derived {
                    narrowed.push(*member);
                }
            } else if is_derived_from(db, *member, instance.target, resolve)? {
                narrowed.push(*member);
            } else if is_derived_from(db, instance.target, *member, resolve)? {
                // A member keeps its own type arguments, which an instance
                // of a generic class lacks.
                narrowed.push(if instance.is_generic() {
                    *member
                } else {
                    instance.target
                });
            }
        }
        // An instance type that no member relates to gives its intersection
        // with each type parameter whose constraint it is derived from. An
        // unconstrained type parameter counts as `{}`, from which every
        // object type is derived.
        if narrowed.len() == related {
            for member in &members {
                let Some(generic) = referenced_generic(db, *member) else {
                    continue;
                };
                let is_related = match generic.constraint(db) {
                    Some(constraint) => is_derived_from(db, instance.target, constraint, resolve)?,
                    None => true,
                };
                if is_related {
                    let instance_type = instance.with_any_arguments(db)?;
                    narrowed.push(intersect_narrowed(db, *member, instance_type));
                }
            }
        }
    }
    if !narrowed.is_empty() {
        return Some(InferredTypeData::union_from_types(db, narrowed));
    }
    let instance_types = instances
        .iter()
        .map(|instance| instance.with_any_arguments(db))
        .collect::<Option<Vec<_>>>()?;
    narrow_unrelated(
        db,
        ty,
        InferredTypeData::union_from_types(db, instance_types),
        resolve,
    )
}

/// Returns the type that TypeScript gives a value of type `ty` that a test
/// shows to have type `candidate`, when no member of `ty` relates to a member
/// of `candidate`, or `None` when Biome can't tell.
///
/// As in TypeScript, the result is `candidate` when it is a subtype of `ty`,
/// `ty` when it is assignable to `candidate`, `candidate` when it is
/// assignable to `ty`, and otherwise the intersection of the two, built by
/// [`intersect_narrowed`]. So `Bar` becomes `Bar & Foo` for `isFoo(x)` with
/// `interface Foo { a: number }` and `interface Bar { c: number }`. A type
/// parameter that the types refer to belongs to the narrowed code, so it
/// relates to another type only through its constraint (see
/// [`is_assignable_in_generic_body`]). A comparison that can't tell gives
/// `None`, so the value keeps its type.
fn narrow_unrelated<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    candidate: InferredTypeData<'db>,
    resolve: &mut TypeResolver<'db, '_>,
) -> Option<InferredTypeData<'db>> {
    match is_subtype(db, candidate, ty, resolve) {
        Assignability::Assignable => return Some(candidate),
        Assignability::Undetermined => return None,
        Assignability::NotAssignable => {}
    }
    match is_assignable_in_generic_body(db, ty, candidate, resolve) {
        Assignability::Assignable => return Some(ty),
        Assignability::Undetermined => return None,
        Assignability::NotAssignable => {}
    }
    match is_assignable_in_generic_body(db, candidate, ty, resolve) {
        Assignability::Assignable => Some(candidate),
        Assignability::Undetermined => None,
        Assignability::NotAssignable => Some(intersect_narrowed(db, ty, candidate)),
    }
}

/// Narrows `ty` by the runtime test `#name in value`, where `class` is the
/// type of the class that declares the private name `#name`, as TypeScript
/// does. Only an object that the class constructed, or the class itself for a
/// static name, has the private name.
///
/// For a name that instances have, the test narrows as `value instanceof C`
/// does (see [`narrow_by_instanceof`]), with the instance type of the class.
/// For a static name, which `is_static` tells, it compares the members of
/// `ty` with the class itself. In the true branch, a member that is the
/// class stays and a member that is another class, an instance, or a
/// primitive is removed. The false branch gives `never` when `ty` is the
/// class and otherwise keeps `ty`, including a member that is the class, as
/// TypeScript does: there it removes only the members derived from the
/// class, and the class itself is not one of them. A member that Biome
/// can't compare leaves `ty` unchanged in either branch, and so does a class
/// whose instance type Biome can't tell. `resolve` resolves handles as for
/// [`narrow_type`].
pub fn narrow_by_private_name<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    class: InferredTypeData<'db>,
    is_static: bool,
    positive: bool,
    mut resolve: impl FnMut(InferredTypeData<'db>) -> InferredTypeData<'db>,
) -> InferredTypeData<'db> {
    if is_narrowing_invariant(ty) || ty == InferredTypeData::NeverKeyword {
        return ty;
    }
    let resolve: &mut TypeResolver<'db, '_> = &mut resolve;
    if !is_static {
        let Some(instances) = instance_types(db, class, resolve) else {
            return ty;
        };
        let narrowed = if positive {
            instances_in(db, ty, &instances, resolve)
        } else if ty == InferredTypeData::AnyKeyword {
            None
        } else {
            instances_excluded(db, ty, &instances, resolve)
        };
        return narrowed.unwrap_or(ty);
    }
    if matches!(
        ty,
        InferredTypeData::AnyKeyword | InferredTypeData::UnknownKeyword
    ) {
        return if positive { class } else { ty };
    }
    let class_instance = InferredTypeData::instance_of(db, class, Box::default());
    if !positive {
        return match is_same_class(db, ty, class_instance, resolve) {
            Some(true) => InferredTypeData::NeverKeyword,
            Some(false) | None => ty,
        };
    }
    let mut remaining = MAX_NARROWING_STEPS;
    let Some(members) = union_members(db, ty, &mut remaining) else {
        return ty;
    };
    let mut kept = Vec::with_capacity(members.len());
    for member in &members {
        let Some(is_class) = is_same_class(db, *member, class_instance, resolve) else {
            return ty;
        };
        if is_class {
            kept.push(*member);
        }
    }
    if kept.is_empty() {
        return narrow_unrelated(db, ty, class, resolve).unwrap_or(ty);
    }
    if kept.len() == members.len() {
        ty
    } else {
        InferredTypeData::union_from_types(db, kept)
    }
}

/// Returns whether `member` is the class whose instance type is
/// `class_instance`, compared by declaration, or `None` when Biome can't
/// tell. A primitive, an instance, an object type, or a function is no
/// class.
fn is_same_class<'db>(
    db: &'db dyn TypeDb,
    member: InferredTypeData<'db>,
    class_instance: InferredTypeData<'db>,
    resolve: &mut TypeResolver<'db, '_>,
) -> Option<bool> {
    if member.is_primitive(db) {
        return Some(false);
    }
    match resolve_wrapped_type(db, member, resolve).expand_canonical_global(db) {
        InferredTypeData::Class(_) => {
            let member_instance = InferredTypeData::instance_of(db, member, Box::default());
            Some(
                is_derived_from(db, member_instance, class_instance, resolve)?
                    && is_derived_from(db, class_instance, member_instance, resolve)?,
            )
        }
        InferredTypeData::InstanceOf(_)
        | InferredTypeData::Object(_)
        | InferredTypeData::Interface(_)
        | InferredTypeData::Function(_)
        | InferredTypeData::Tuple(_)
        | InferredTypeData::ObjectKeyword
        | InferredTypeData::Null
        | InferredTypeData::Undefined => Some(false),
        _ => None,
    }
}

/// Narrows `ty` by the runtime test `value.constructor === C`, where
/// `constructor` is the type of `C`, in the branch where the test holds, as
/// TypeScript does. The other branch keeps `ty`.
///
/// The test narrows only when `C` is a class, or another single constructor
/// whose `prototype` property gives its instances a type other than `any`,
/// `Object`, or `Function`. A member of `ty` stays when Biome can't rule out
/// that `C` constructed it:
///
/// - when `C` is a class that the code declares, or the member is an
///   instance of one, the member must be an instance of `C` itself, not of a
///   class derived from it, because a derived class has its own
///   `constructor`;
/// - otherwise, the member must be assignable to the instance type, so
///   `string` stays for `String` and `Date` for `Date`, whose instance types
///   TypeScript's library declares as interfaces.
///
/// `any` becomes the instance type, with an `any` type argument for each
/// type parameter of a generic class, and `unknown` stays. As in TypeScript,
/// an object that overrides `constructor` can pass the test without being
/// an instance of `C`. `resolve` resolves handles as for [`narrow_type`].
pub fn narrow_by_constructor<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    constructor: InferredTypeData<'db>,
    mut resolve: impl FnMut(InferredTypeData<'db>) -> InferredTypeData<'db>,
) -> InferredTypeData<'db> {
    if is_narrowing_invariant(ty) || ty == InferredTypeData::NeverKeyword {
        return ty;
    }
    let resolve: &mut TypeResolver<'db, '_> = &mut resolve;
    let resolved = resolve_wrapped_type(db, constructor, resolve);
    // TypeScript reads the instances' type from the `prototype` property,
    // which a construct signature without one doesn't give.
    let is_class = matches!(
        resolved.expand_canonical_global(db),
        InferredTypeData::Class(_)
    );
    if !is_class
        && !matches!(
            find_property(db, resolved, "prototype", resolve),
            PropertyLookup::Found { ty, .. } if ty != InferredTypeData::AnyKeyword
        )
    {
        return ty;
    }
    let Some(instances) = instance_types(db, constructor, resolve) else {
        return ty;
    };
    let [instance] = instances.as_slice() else {
        return ty;
    };
    if ty == InferredTypeData::AnyKeyword {
        return instance.with_any_arguments(db).unwrap_or(ty);
    }
    let mut remaining = MAX_NARROWING_STEPS;
    let Some(members) = union_members(db, ty, &mut remaining) else {
        return ty;
    };
    let target_is_class = is_declared_class_instance(db, instance.target, resolve);
    let mut kept = Vec::with_capacity(members.len());
    for member in &members {
        let stays = if *member == InferredTypeData::UnknownKeyword {
            true
        } else if target_is_class || is_declared_class_instance(db, *member, resolve) {
            !matches!(
                (
                    is_derived_from(db, *member, instance.target, resolve),
                    is_derived_from(db, instance.target, *member, resolve),
                ),
                (Some(false), _) | (_, Some(false))
            )
        } else {
            is_assignable_in_generic_body(db, *member, instance.target, resolve)
                != Assignability::NotAssignable
        };
        if stays {
            kept.push(*member);
        }
    }
    if kept.len() == members.len() {
        ty
    } else {
        InferredTypeData::union_from_types(db, kept)
    }
}

/// Returns whether `ty` is an instance of a class that is not built in.
fn is_declared_class_instance<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    resolve: &mut TypeResolver<'db, '_>,
) -> bool {
    matches!(
        ty,
        InferredTypeData::InstanceOf(instance)
            if matches!(
                resolve(instance.ty(db)).expand_canonical_global(db),
                InferredTypeData::Class(class) if !class.is_builtin(db)
            )
    )
}

/// Returns the false branch of [`narrow_by_instanceof`], or `None` when it
/// leaves `ty` unchanged.
fn instances_excluded<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    instances: &[InstanceType<'db>],
    resolve: &mut TypeResolver<'db, '_>,
) -> Option<InferredTypeData<'db>> {
    let [instance] = instances else {
        return None;
    };
    if !is_object_type(db, instance.target, resolve)
        || is_empty_object_type(db, instance.target, resolve)
    {
        return None;
    }
    let mut remaining = MAX_NARROWING_STEPS;
    let members = union_members(db, ty, &mut remaining)?;
    let mut kept = Vec::with_capacity(members.len());
    for member in &members {
        remaining = remaining.checked_sub(1)?;
        // A type parameter is derived from what its constraint is derived
        // from.
        let is_derived = match referenced_generic(db, *member) {
            Some(generic) => match generic.constraint(db) {
                Some(constraint) => is_derived_from(db, constraint, instance.target, resolve)?,
                None => false,
            },
            None => is_derived_from(db, *member, instance.target, resolve)?,
        };
        if !is_derived {
            kept.push(*member);
        }
    }
    Some(if kept.len() == members.len() {
        ty
    } else {
        InferredTypeData::union_from_types(db, kept)
    })
}

/// Returns whether `declared` is `unknown` or `any`, or has a member that is
/// neither `null`, `undefined`, nor a primitive type. Only then can
/// [`narrow_by_instanceof`] narrow a value of that declared type to a type
/// other than `never`.
///
/// No primitive is derived from a class, so `instanceof` can only make a
/// branch impossible for a value of a primitive type, or leave it unchanged.
/// Biome applies a type predicate whose target has only object types only to
/// a declared type that this function accepts. TypeScript also narrows a
/// primitive by such a target when the primitive's wrapper type satisfies
/// it, as it narrows `string` by `{ length: number }`.
pub fn may_narrow_by_instanceof(db: &dyn TypeDb, declared: InferredTypeData) -> bool {
    let is_object = |member: &InferredTypeData| {
        !matches!(
            member,
            InferredTypeData::Null
                | InferredTypeData::Undefined
                | InferredTypeData::VoidKeyword
                | InferredTypeData::NeverKeyword
        ) && !matches!(
            typeof_kind(db, *member),
            Some(
                TypeofKind::BigInt
                    | TypeofKind::Boolean
                    | TypeofKind::Number
                    | TypeofKind::String
                    | TypeofKind::Symbol
            )
        )
    };
    match declared {
        InferredTypeData::Union(union) => union.types(db).iter().any(is_object),
        declared => is_object(&declared),
    }
}

/// Narrows `ty` by a call to a type-guard function whose type predicate
/// targets `target`, as `isFoo(value)` with `value is Foo` narrows `value`.
/// The result matches TypeScript except where noted below. `target` must be
/// flattened, as the declared type of a variable is.
///
/// A primitive type, a primitive literal, `null`, or `undefined` as `target`
/// applies the runtime test that it stands for through [`narrow_type`]:
/// `typeof value === "string"` for `string`, and strict equality for a
/// literal. The false branch of a primitive type keeps the literals of that
/// primitive, as `"a"` for `string`, when the true branch gets the primitive
/// type from an object type, as from `{ length: number }`. TypeScript keeps
/// them too, because its union of such a literal and the primitive type is
/// the primitive type.
///
/// For other targets, Biome compares each member of `ty` with each member of
/// `target` in TypeScript's subtype relation, using [`is_subtype`]:
///
/// - When `positive` is true, a member of `ty` that is a subtype of a member
///   of `target` stays, a member of `target` that is a subtype of a member of
///   `ty` replaces it, and the other members are removed. So `Foo2 | Bar`
///   becomes `Foo2` with `interface Foo2 extends Foo`, and `unknown` becomes
///   `target`. A type parameter relates to another type only through its
///   constraint. A member of `target` that no member relates to gives its
///   intersection with each type parameter whose constraint it is a subtype
///   of, or that has no constraint, so `T | string` becomes `T & Foo`.
/// - When no member remains in the true branch, the result is, as in
///   TypeScript, `target` if it is a subtype of `ty`, `ty` if it is
///   assignable to `target`, `target` if it is assignable to `ty`, and
///   otherwise the intersection of the two, so `Bar` becomes `Bar & Foo`
///   with `interface Bar { c: number }`. The first comparison that Biome
///   can't tell keeps `ty`.
/// - When `positive` is false, the result removes each member that the true
///   branch keeps as it is rather than replacing it with a member of
///   `target`. A type parameter stays unless it is a subtype of `target`. As
///   in TypeScript, this can give `never`.
///
/// When a member and a member of `target` are subtypes of each other,
/// TypeScript's strict subtype relation picks which of the two the true branch
/// keeps. Biome can't always make the same choice: its subtype comparison does
/// not compare properties whose only difference is `readonly`. Unless the two
/// types are equal, Biome keeps the member in both branches.
///
/// For `interface Foo { a: number }`, `interface Same { a: number }`, and
/// `interface Bar { c: number }`, testing a `Same | Bar` value with a type
/// predicate targeting `Foo` gives `Same` in Biome's true branch and
/// `Same | Bar` in its false branch. TypeScript gives `Same` and `Bar`,
/// respectively. The false branch also keeps a member when Biome can tell only
/// that a member of `target` is a subtype of it. It keeps `unknown`, and it
/// keeps every member when the true branch is empty.
///
/// `any` becomes `target` when `positive` is true and stays `any` when it is
/// false, as in TypeScript, whatever the target. TypeScript gives `never` in
/// the false branch when `target` is `any` itself.
///
/// `ty` is returned unchanged in these cases:
///
/// - A comparison of two members can't tell whether either is a subtype of
///   the other, as for classes that can declare private members and literal
///   types that TypeScript may widen. [`is_subtype`] describes these
///   comparisons.
/// - `ty` is a union with `unknown`.
/// - Biome may reach two classes with the same contents as one.
/// - `target` has `any` or an undetermined member. When `ty` is `any`, only
///   an undetermined `target` leaves it unchanged.
/// - The comparisons run out of their budget of 256 visited types and pairs.
///
/// `resolve` resolves handles as for [`narrow_type`].
pub fn narrow_by_type_predicate<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    target: InferredTypeData<'db>,
    positive: bool,
    mut resolve: impl FnMut(InferredTypeData<'db>) -> InferredTypeData<'db>,
) -> InferredTypeData<'db> {
    if is_narrowing_invariant(ty) || ty == InferredTypeData::NeverKeyword {
        return ty;
    }
    if ty == InferredTypeData::AnyKeyword {
        return if positive && !matches!(target, InferredTypeData::Unknown) {
            target
        } else {
            ty
        };
    }
    let resolve: &mut TypeResolver<'db, '_> = &mut resolve;
    let Some(predicate) = primitive_predicate(db, target) else {
        return predicate_branch(db, ty, target, positive, resolve).unwrap_or(ty);
    };
    let narrowed = narrow_type(db, ty, predicate, positive, &mut *resolve);
    let NarrowingPredicate::Typeof(kind) = predicate else {
        return narrowed;
    };
    let Some(primitive) = typeof_type(db, kind).filter(|_| !positive) else {
        return narrowed;
    };
    let mut remaining = MAX_NARROWING_STEPS;
    let true_branch = narrow_type(db, ty, predicate, true, &mut *resolve);
    let (Some(members), Some(true_members)) = (
        union_members(db, ty, &mut remaining),
        union_members(db, true_branch, &mut remaining),
    ) else {
        return narrowed;
    };
    if !true_members.contains(&primitive) {
        return narrowed;
    }
    let literals: Vec<_> = members
        .into_iter()
        .filter(|member| has_primitive_of_literal(db, *member, &true_members))
        .collect();
    if literals.is_empty() {
        return narrowed;
    }
    let mut kept = literals;
    if narrowed != InferredTypeData::NeverKeyword {
        kept.push(narrowed);
    }
    InferredTypeData::union_from_types(db, kept)
}

/// Returns the type of a variable or a property right after a value of type
/// `assigned` is assigned to it, given its `declared` type. TypeScript
/// narrows an assignment the same way, except for `boolean` as described
/// below.
///
/// Only a union narrows. The result keeps the members of `declared` that a
/// value of `assigned` may have, so `string | number` becomes `string` for
/// `"a"`, and `"a" | "b"` becomes `"a"`. Members are filtered, not replaced
/// by `assigned`. The result is `declared` itself when `declared` is not a
/// union, when every member stays, and when `assigned` isn't assignable to
/// the members that stay. The last case covers `any` and an `assigned` type
/// that inference couldn't determine. A `never` value gives `never`.
///
/// `boolean` stays whole, and a boolean value counts as both `true` and
/// `false`, because Biome types some comparisons, such as `1 === 1`, as
/// `true` where TypeScript gives `boolean`. So `boolean | undefined` becomes
/// `boolean` for `true`, where TypeScript gives `true`. A comparison that
/// Biome can't decide keeps the member. The comparisons visit at most 256
/// pairs and types, after which the result is `declared`. `resolve` replaces
/// local and global handles with the types they stand for.
pub fn narrow_by_assignment<'db>(
    db: &'db dyn TypeDb,
    declared: InferredTypeData<'db>,
    assigned: InferredTypeData<'db>,
    mut resolve: impl FnMut(InferredTypeData<'db>) -> InferredTypeData<'db>,
) -> InferredTypeData<'db> {
    if declared == assigned || !matches!(declared, InferredTypeData::Union(_)) {
        return declared;
    }
    if assigned == InferredTypeData::NeverKeyword {
        return assigned;
    }
    let mut remaining = MAX_NARROWING_STEPS;
    let (Some(members), Some(values)) = (
        union_members(db, declared, &mut remaining),
        union_members(db, assigned, &mut remaining),
    ) else {
        return declared;
    };
    let mut expanded = Vec::with_capacity(values.len());
    for value in values {
        let is_boolean = matches!(value, InferredTypeData::Boolean)
            || matches!(value, InferredTypeData::Literal(literal) if matches!(literal.literal(db), Literal::Boolean(_)));
        if is_boolean {
            expanded.extend([boolean_literal(db, true), boolean_literal(db, false)]);
        } else {
            expanded.push(value);
        }
    }
    let resolve: &mut TypeResolver<'db, '_> = &mut resolve;
    assignment_reduced_members(db, &members, &expanded, assigned, resolve).unwrap_or(declared)
}

/// Returns `ty` with each literal member replaced by its primitive type,
/// matching TypeScript. For example, `1 | 2 | "a"` becomes `number | string`,
/// and other members stay as they are.
///
/// This is the type of a variable or a property after a compound assignment
/// such as `x += 1`, `x++`, or `x--`, and the type that an assignment such as
/// `x = x + 1` narrows. Returns `Unknown` when the members exceed the work
/// limit of 256 types, because keeping a literal would be narrower than
/// TypeScript.
pub fn base_literal_type<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
) -> InferredTypeData<'db> {
    match ty {
        InferredTypeData::Literal(_) => widened_literal(db, ty).unwrap_or(ty),
        InferredTypeData::Union(_) => {
            let mut remaining = MAX_NARROWING_STEPS;
            let Some(members) = union_members(db, ty, &mut remaining) else {
                return InferredTypeData::Unknown;
            };
            InferredTypeData::union_from_types(
                db,
                members
                    .into_iter()
                    .map(|member| widened_literal(db, member).unwrap_or(member))
                    .collect(),
            )
        }
        _ => ty,
    }
}

/// Returns the type that a value of type `ty` gives a variable whose type
/// evolves with its writes, or adds to the element type of an array that
/// evolves. The result is `ty` with its literal members replaced by their
/// primitive types, as TypeScript widens them, so `1 | "a"` becomes
/// `number | string`.
///
/// The result replaces the variable's type instead of narrowing a declared
/// one, so a value whose type Biome may infer differently from TypeScript
/// gives `Unknown`. Biome's type matches TypeScript's when it is built from
/// primitive types, `null`, `undefined`, `never`, interfaces, instances of
/// classes and interfaces with such type arguments, as `Array<number>` or
/// `Date`, and object types whose members are all properties of such types,
/// as `{ k: number }`. A literal type inside an object type or a type
/// argument gives `Unknown`, because TypeScript widens both `{ k: 1 }` and
/// `{ k: id(1) }` to `{ k: number }`, while Biome keeps `1` in the second.
/// Functions, methods, getters, tuples, generics, `any`, `unknown`, and
/// `void` give `Unknown` too. `resolve` resolves handles as for
/// [`narrow_type`], and a type with more than 256 parts gives `Unknown`.
pub fn evolving_assigned_type<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    mut resolve: impl FnMut(InferredTypeData<'db>) -> InferredTypeData<'db>,
) -> InferredTypeData<'db> {
    let mut remaining = MAX_NARROWING_STEPS;
    let resolve: &mut TypeResolver<'db, '_> = &mut resolve;
    widened_value(db, ty, resolve, &mut remaining).unwrap_or(InferredTypeData::Unknown)
}

/// Returns `ty` resolved, with its literal members widened, or `None` when
/// Biome may type a value of type `ty` differently from TypeScript.
/// [`evolving_assigned_type`] lists the types that match.
fn widened_value<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    resolve: &mut TypeResolver<'db, '_>,
    remaining: &mut usize,
) -> Option<InferredTypeData<'db>> {
    *remaining = remaining.checked_sub(1)?;
    let ty = resolve(ty);
    match ty {
        InferredTypeData::Literal(_) => widened_literal(db, ty),
        InferredTypeData::InstanceOf(instance)
            if resolve(instance.ty(db)).should_flatten_instance(instance.type_parameters(db)) =>
        {
            widened_value(db, instance.ty(db), resolve, remaining)
        }
        InferredTypeData::Union(union) => {
            let members = union
                .types(db)
                .iter()
                .map(|member| widened_value(db, *member, resolve, remaining))
                .collect::<Option<Vec<_>>>()?;
            Some(InferredTypeData::union_from_types(db, members))
        }
        _ => is_exact_value(db, ty, resolve, remaining).then_some(ty),
    }
}

/// Returns whether Biome types `ty` as TypeScript does, where `ty` is a
/// value's type, a property type, or a type argument. TypeScript widens no
/// literal type in these positions, so `ty` must match exactly.
/// [`evolving_assigned_type`] lists the types that match.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "Every other type may differ from TypeScript's."
)]
fn is_exact_value<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    resolve: &mut TypeResolver<'db, '_>,
    remaining: &mut usize,
) -> bool {
    let Some(left) = remaining.checked_sub(1) else {
        return false;
    };
    *remaining = left;
    match resolve(ty) {
        InferredTypeData::BigInt
        | InferredTypeData::Boolean
        | InferredTypeData::Null
        | InferredTypeData::Number
        | InferredTypeData::String
        | InferredTypeData::Symbol
        | InferredTypeData::Undefined
        | InferredTypeData::NeverKeyword
        | InferredTypeData::Interface(_) => true,
        InferredTypeData::Union(union) => union
            .types(db)
            .iter()
            .all(|member| is_exact_value(db, *member, resolve, remaining)),
        InferredTypeData::InstanceOf(instance) => {
            let target = resolve(instance.ty(db));
            let arguments = instance.type_parameters(db);
            if target.should_flatten_instance(arguments) {
                return is_exact_value(db, target, resolve, remaining);
            }
            matches!(
                target.expand_canonical_global(db),
                InferredTypeData::Class(_) | InferredTypeData::Interface(_)
            ) && arguments
                .iter()
                .all(|argument| is_exact_value(db, *argument, resolve, remaining))
        }
        InferredTypeData::Object(object) => {
            !object.has_unknown_members(db)
                && object.members(db).iter().all(|member| {
                    matches!(
                        member.kind,
                        TypeMemberKind::Named(_)
                            | TypeMemberKind::NamedOptional(_)
                            | TypeMemberKind::ReadonlyNamed(_)
                            | TypeMemberKind::ReadonlyNamedOptional(_)
                    ) && is_exact_value(db, member.ty, resolve, remaining)
                })
        }
        _ => false,
    }
}

/// Returns the element type of `ty`, the type of an array that evolves with
/// its writes. Each member of `ty` must be an instance of `Array` with one
/// type argument, and the element type is the union of those arguments, so
/// the join of `Array<number>` and `Array<string>` gives `number | string`.
/// An array that nothing has been added to is `Array<never>`, whose element
/// type is `never`.
///
/// Returns `None` when a member is something else, as `Unknown` or
/// `undefined`, and when the members exceed the work limit of 256 types.
pub fn evolving_array_element<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
) -> Option<InferredTypeData<'db>> {
    let mut remaining = MAX_NARROWING_STEPS;
    let members = union_members(db, ty, &mut remaining)?;
    let mut elements = Vec::with_capacity(members.len());
    for member in members {
        let InferredTypeData::InstanceOf(instance) = member else {
            return None;
        };
        let [element] = instance.type_parameters(db).as_ref() else {
            return None;
        };
        if !instance.ty(db).is_array_class(db) {
            return None;
        }
        elements.push(*element);
    }
    Some(InferredTypeData::union_from_types(db, elements))
}

/// Returns the runtime test that the flattened target of a type predicate
/// stands for when it is a primitive type, a primitive literal, `null`, or
/// `undefined`, or `None` for other targets.
fn primitive_predicate<'db>(
    db: &'db dyn TypeDb,
    target: InferredTypeData<'db>,
) -> Option<NarrowingPredicate<'db>> {
    Some(match target {
        InferredTypeData::BigInt => NarrowingPredicate::Typeof(TypeofKind::BigInt),
        InferredTypeData::Boolean => NarrowingPredicate::Typeof(TypeofKind::Boolean),
        InferredTypeData::Number => NarrowingPredicate::Typeof(TypeofKind::Number),
        InferredTypeData::String => NarrowingPredicate::Typeof(TypeofKind::String),
        InferredTypeData::Symbol => NarrowingPredicate::Typeof(TypeofKind::Symbol),
        InferredTypeData::Null | InferredTypeData::Undefined => NarrowingPredicate::Literal(target),
        InferredTypeData::Literal(literal)
            if matches!(
                literal.literal(db),
                Literal::BigInt(_) | Literal::Boolean(_) | Literal::Number(_) | Literal::String(_)
            ) =>
        {
            NarrowingPredicate::Literal(target)
        }
        _ => return None,
    })
}

/// Returns the branch of [`narrow_by_type_predicate`] that `positive`
/// selects, or `None` when it leaves `ty` unchanged.
fn predicate_branch<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    target: InferredTypeData<'db>,
    positive: bool,
    resolve: &mut TypeResolver<'db, '_>,
) -> Option<InferredTypeData<'db>> {
    let mut remaining = MAX_NARROWING_STEPS;
    let candidates = union_members(db, target, &mut remaining)?;
    if candidates.iter().any(|candidate| {
        matches!(
            candidate,
            InferredTypeData::AnyKeyword | InferredTypeData::Unknown
        )
    }) {
        return None;
    }
    if ty == InferredTypeData::UnknownKeyword {
        return positive.then_some(target);
    }
    let members = union_members(db, ty, &mut remaining)?;
    if members.contains(&InferredTypeData::UnknownKeyword) {
        return None;
    }
    let mut narrowed = Vec::new();
    let mut kept_as_is = vec![false; members.len()];
    for candidate in &candidates {
        let related = narrowed.len();
        for (member, kept) in members.iter().zip(&mut kept_as_is) {
            remaining = remaining.checked_sub(1)?;
            match predicate_match(db, *member, *candidate, resolve)? {
                PredicateMatch::Member => {
                    narrowed.push(*member);
                    *kept = true;
                }
                PredicateMatch::Either => narrowed.push(*member),
                PredicateMatch::Target => narrowed.push(*candidate),
                PredicateMatch::Unrelated => {}
            }
        }
        // A member of the target that no member relates to gives its
        // intersection with each type parameter whose constraint it is a
        // subtype of, or that has no constraint.
        if narrowed.len() == related {
            for member in &members {
                let Some(generic) = referenced_generic(db, *member) else {
                    continue;
                };
                remaining = remaining.checked_sub(1)?;
                let is_related = match generic.constraint(db) {
                    Some(constraint) => match is_subtype(db, *candidate, constraint, resolve) {
                        Assignability::Assignable => true,
                        Assignability::NotAssignable => false,
                        Assignability::Undetermined => return None,
                    },
                    None => true,
                };
                if is_related {
                    narrowed.push(intersect_narrowed(db, *member, *candidate));
                }
            }
        }
    }
    if positive {
        return if narrowed.is_empty() {
            narrow_unrelated(db, ty, target, resolve)
        } else {
            Some(InferredTypeData::union_from_types(db, narrowed))
        };
    }
    if narrowed.is_empty() {
        return None;
    }
    let kept: Vec<_> = members
        .iter()
        .zip(&kept_as_is)
        .filter(|(member, kept_as_is)| {
            !**kept_as_is || has_primitive_of_literal(db, **member, &narrowed)
        })
        .map(|(member, _)| *member)
        .collect();
    Some(if kept.len() == members.len() {
        ty
    } else if kept.is_empty() {
        InferredTypeData::NeverKeyword
    } else {
        InferredTypeData::union_from_types(db, kept)
    })
}

/// How a member of a narrowed type relates to a member of the target of a
/// type predicate in the subtype relation.
enum PredicateMatch {
    /// The member is a subtype of the target member, and TypeScript keeps it
    /// as it is. This holds when the two are the same type, or when the
    /// target member is not a subtype of the member.
    Member,
    /// The true branch keeps the member, but TypeScript may keep the target
    /// member instead. This holds when the member is a subtype of the target
    /// member and the target member may be a subtype of the member, because
    /// TypeScript's strict subtype relation then picks one. It also holds
    /// when Biome can tell only that the target member is a subtype of the
    /// member. In both cases, the member contains whichever TypeScript keeps.
    Either,
    /// The target member is a subtype of the member and replaces it.
    Target,
    /// Neither is a subtype of the other.
    Unrelated,
}

/// Returns how `member` relates to `candidate`, a member of the target of a
/// type predicate.
///
/// Returns `None` when Biome can't tell whether either is a subtype of the
/// other, and when the two may be instances of distinct classes with the
/// same contents.
fn predicate_match<'db>(
    db: &'db dyn TypeDb,
    member: InferredTypeData<'db>,
    candidate: InferredTypeData<'db>,
    resolve: &mut TypeResolver<'db, '_>,
) -> Option<PredicateMatch> {
    let is_candidate_subtype =
        |resolve: &mut TypeResolver<'db, '_>| is_subtype(db, candidate, member, resolve);
    Some(match is_subtype(db, member, candidate, resolve) {
        // Two equal class instances without a handle may be two declarations,
        // so Biome can't tell how they relate. Biome identifies a class by its
        // contents, including the scope it opens in its module, and by the
        // handle that names it. One class in each of two modules can have the
        // same contents and equal scopes. TypeScript tells such classes apart
        // when they have private members.
        Assignability::Assignable
            if member == candidate && is_class_instance_without_handle(db, member, resolve) =>
        {
            return None;
        }
        Assignability::Assignable if member == candidate => PredicateMatch::Member,
        Assignability::Assignable => match is_candidate_subtype(resolve) {
            Assignability::NotAssignable => PredicateMatch::Member,
            Assignability::Assignable | Assignability::Undetermined => PredicateMatch::Either,
        },
        Assignability::NotAssignable => match is_candidate_subtype(resolve) {
            Assignability::Assignable => PredicateMatch::Target,
            Assignability::NotAssignable => PredicateMatch::Unrelated,
            Assignability::Undetermined => return None,
        },
        Assignability::Undetermined => match is_candidate_subtype(resolve) {
            Assignability::Assignable => PredicateMatch::Either,
            Assignability::NotAssignable | Assignability::Undetermined => return None,
        },
    })
}

/// Returns whether `ty` is an instance of a class that is not built in and
/// that Biome reaches without a handle to a local type of a module.
fn is_class_instance_without_handle<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    resolve: &mut TypeResolver<'db, '_>,
) -> bool {
    let InferredTypeData::InstanceOf(instance) = ty else {
        return false;
    };
    let class = instance.ty(db);
    !matches!(class, InferredTypeData::Local(_))
        && matches!(
            resolve(class).expand_canonical_global(db),
            InferredTypeData::Class(class) if !class.is_builtin(db)
        )
}

/// Returns whether `member` is a literal type, such as `"a"` or `true`, and
/// `types` has its primitive type, such as `string` or `boolean`.
/// TypeScript reduces the union of a literal type and its primitive type to
/// the primitive type.
fn has_primitive_of_literal<'db>(
    db: &'db dyn TypeDb,
    member: InferredTypeData<'db>,
    types: &[InferredTypeData<'db>],
) -> bool {
    widened_literal(db, member).is_some_and(|primitive| types.contains(&primitive))
}

/// Returns whether the property `name` is a discriminant of `declared`: the
/// property type of at least one member of `declared` is made of unit types.
/// Members that are `null`, `undefined`, or primitives, and members that lack
/// the property, don't count.
///
/// Returns false when a member's property can't be looked up or has a type
/// that [`PropertyValue`] does not describe, and when the `remaining` budget
/// runs out.
fn is_discriminant<'db>(
    db: &'db dyn TypeDb,
    declared: InferredTypeData<'db>,
    name: &str,
    resolve: &mut TypeResolver<'db, '_>,
    remaining: &mut usize,
) -> bool {
    let Some(members) = union_members(db, declared, remaining) else {
        return false;
    };
    let mut is_discriminant = false;
    for member in members {
        match member_property(db, member, name, resolve, remaining) {
            Some(MemberProperty::Values(values)) => is_discriminant |= is_unit(&values),
            Some(MemberProperty::Nullish | MemberProperty::Primitive | MemberProperty::Missing) => {
            }
            Some(MemberProperty::Unknown) | None => return false,
        }
    }
    is_discriminant
}

/// Returns whether every value of a property type is a unit type: a literal
/// type, `null`, `undefined`, or `boolean`. A member with such a property
/// type makes the property a discriminant.
fn is_unit(values: &PropertyValues) -> bool {
    values.iter().all(|(_, value)| value.is_unit())
}

/// Returns the members of `ty`, with nested unions flattened, or `ty` itself
/// when it is not a union.
///
/// Returns `None` when the members exceed the `remaining` budget, which each
/// visited type spends.
fn union_members<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    remaining: &mut usize,
) -> Option<Vec<InferredTypeData<'db>>> {
    let mut pending = vec![ty];
    let mut members = Vec::new();
    while let Some(current) = pending.pop() {
        *remaining = remaining.checked_sub(1)?;
        match current {
            InferredTypeData::Union(union) => pending.extend(union.types(db).iter().rev()),
            _ => members.push(current),
        }
    }
    Some(members)
}

/// The resolved values of a property type, each with the type it comes from.
type PropertyValues<'db> = Vec<(InferredTypeData<'db>, PropertyValue<'db>)>;

/// What a member of a union says about one of its properties.
enum MemberProperty<'db> {
    /// The member is `null` or `undefined`, which has no properties.
    Nullish,
    /// The member is a primitive type, whose properties are not looked up.
    Primitive,
    /// The member is an object type without the property.
    Missing,
    /// The property can't be looked up, or its type includes a type that
    /// [`PropertyValue`] does not describe.
    Unknown,
    /// The values of the property. An optional property also has the value
    /// `undefined`.
    Values(PropertyValues<'db>),
}

/// Returns what `member` says about its property `name`, or `None` when the
/// `remaining` budget runs out.
fn member_property<'db>(
    db: &'db dyn TypeDb,
    member: InferredTypeData<'db>,
    name: &str,
    resolve: &mut TypeResolver<'db, '_>,
    remaining: &mut usize,
) -> Option<MemberProperty<'db>> {
    *remaining = remaining.checked_sub(1)?;
    if matches!(member, InferredTypeData::Null | InferredTypeData::Undefined) {
        return Some(MemberProperty::Nullish);
    }
    if matches!(
        typeof_kind(db, member),
        Some(
            TypeofKind::BigInt
                | TypeofKind::Boolean
                | TypeofKind::Number
                | TypeofKind::String
                | TypeofKind::Symbol
        )
    ) {
        return Some(MemberProperty::Primitive);
    }
    let (ty, is_optional) = match find_property(db, member, name, resolve) {
        PropertyLookup::Found {
            ty, is_optional, ..
        } => (ty, is_optional),
        PropertyLookup::Missing => return Some(MemberProperty::Missing),
        PropertyLookup::Unknown => return Some(MemberProperty::Unknown),
    };
    let mut values = Vec::new();
    let mut pending = vec![ty];
    if is_optional {
        pending.push(InferredTypeData::Undefined);
    }
    while let Some(current) = pending.pop() {
        *remaining = remaining.checked_sub(1)?;
        match resolve_wrapped_type(db, current, resolve) {
            InferredTypeData::Union(union) => pending.extend(union.types(db).iter().rev()),
            resolved => match PropertyValue::new(db, resolved, resolve) {
                Some(value) => values.push((resolved, value)),
                None => return Some(MemberProperty::Unknown),
            },
        }
    }
    Some(MemberProperty::Values(values))
}

/// What [`narrow_by_property`] knows about the values of one type in a
/// property type.
#[derive(Clone, Copy)]
enum PropertyValue<'db> {
    Null,
    Undefined,
    /// A string, number, bigint, boolean, or symbol: the literal type
    /// `literal`, or any value of `kind` when `literal` is `None`.
    Primitive {
        kind: TypeofKind,
        literal: Option<InferredTypeData<'db>>,
    },
    /// A value of an object type, a function, or a tuple, which is never
    /// `null` or `undefined`.
    Object,
}

impl<'db> PropertyValue<'db> {
    /// Returns what the resolved type `ty` says about its values, or `None`
    /// for a type it does not describe, such as a generic, `unknown`, `void`,
    /// or an intersection.
    fn new(
        db: &'db dyn TypeDb,
        ty: InferredTypeData<'db>,
        resolve: &mut TypeResolver<'db, '_>,
    ) -> Option<Self> {
        Some(match ty {
            InferredTypeData::Null => Self::Null,
            InferredTypeData::Undefined => Self::Undefined,
            InferredTypeData::Literal(literal) => match literal.literal(db) {
                Literal::Object(_) | Literal::RegExp(_) => Self::Object,
                // A template literal type can have many values.
                Literal::Template(_) => Self::Primitive {
                    kind: TypeofKind::String,
                    literal: None,
                },
                Literal::BigInt(_)
                | Literal::Boolean(_)
                | Literal::Number(_)
                | Literal::String(_) => Self::Primitive {
                    kind: typeof_kind(db, ty)?,
                    literal: Some(ty),
                },
            },
            InferredTypeData::InstanceOf(instance) => {
                match resolve(instance.ty(db)).expand_canonical_global(db) {
                    InferredTypeData::Class(_)
                    | InferredTypeData::Interface(_)
                    | InferredTypeData::Object(_)
                    | InferredTypeData::Tuple(_) => Self::Object,
                    _ => return None,
                }
            }
            InferredTypeData::Object(_) | InferredTypeData::Interface(_) => Self::Object,
            ty if is_non_primitive(db, ty) => Self::Object,
            ty => match typeof_kind(db, ty)? {
                TypeofKind::Object | TypeofKind::Function => Self::Object,
                TypeofKind::Undefined => Self::Undefined,
                kind => Self::Primitive {
                    kind,
                    literal: None,
                },
            },
        })
    }

    /// Returns whether TypeScript counts the type as a unit type when it
    /// looks for a discriminant: a literal type, `null`, `undefined`, or
    /// `boolean`, which is the union of two literal types.
    fn is_unit(&self) -> bool {
        match self {
            Self::Null | Self::Undefined => true,
            Self::Primitive { kind, literal } => literal.is_some() || *kind == TypeofKind::Boolean,
            Self::Object => false,
        }
    }

    /// Returns whether a value can have both this type and `other`.
    ///
    /// Two primitives of the same kind can share a value unless they are
    /// different literals. An object type can share a value with every type
    /// except `null` and `undefined`, because comparing object types with
    /// each other or with primitives isn't modelled.
    fn overlaps(&self, db: &'db dyn TypeDb, other: &Self) -> bool {
        match (self, other) {
            (Self::Null, Self::Null) | (Self::Undefined, Self::Undefined) => true,
            (Self::Null | Self::Undefined, _) | (_, Self::Null | Self::Undefined) => false,
            (Self::Object, _) | (_, Self::Object) => true,
            (
                Self::Primitive {
                    kind,
                    literal: Some(literal),
                },
                Self::Primitive {
                    kind: other_kind,
                    literal: Some(other_literal),
                },
            ) => {
                kind == other_kind
                    && (literal == other_literal
                        || match (
                            literal_value(db, *literal),
                            literal_value(db, *other_literal),
                        ) {
                            (Some(value), Some(other_value)) => value == other_value,
                            // Literals that can't be decoded might be equal.
                            _ => true,
                        })
            }
            (
                Self::Primitive { kind, .. },
                Self::Primitive {
                    kind: other_kind, ..
                },
            ) => kind == other_kind,
        }
    }
}

/// Returns the falsy values of `ty`: the type that a left operand of `&&`
/// contributes to the value of the expression.
///
/// The result is the false branch of a truthiness test in [`narrow_type`],
/// except that `string` and `bigint` become `""` and `0n`, their only falsy
/// values. `number` stays `number` because both zero and `NaN` are falsy.
/// Inputs, limits, object types, `resolve`, and unchanged types are the same
/// as for [`narrow_type`].
pub fn falsy_values<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    mut resolve: impl FnMut(InferredTypeData<'db>) -> InferredTypeData<'db>,
) -> InferredTypeData<'db> {
    if is_narrowing_invariant(ty) {
        return ty;
    }
    narrow_prepared(db, ty, &PreparedPredicate::FalsyValues, false, &mut resolve)
}

fn narrow_prepared<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    predicate: &PreparedPredicate<'db>,
    positive: bool,
    resolve: &mut TypeResolver<'db, '_>,
) -> InferredTypeData<'db> {
    if !matches!(ty, InferredTypeData::Union(_)) {
        return narrow_non_union(db, ty, predicate, positive, Siblings::default(), resolve);
    }

    let mut pending = vec![ty];
    let mut remaining = MAX_NARROWING_STEPS;
    let mut members = Vec::new();
    while let Some(current) = pending.pop() {
        if remaining == 0 {
            return ty;
        }
        remaining -= 1;
        if let InferredTypeData::Union(union) = current {
            let types = union.types(db);
            // Reserve the budget for pending siblings before copying children.
            if types.len() > remaining.saturating_sub(pending.len()) {
                return ty;
            }
            pending.extend(types.iter().rev().copied());
        } else {
            members.push(current);
        }
    }

    let siblings = Siblings {
        has_null: members.contains(&InferredTypeData::Null),
        has_undefined: members.contains(&InferredTypeData::Undefined),
    };
    let mut narrowed = Vec::with_capacity(members.len());
    let mut changed = false;
    for member in members {
        let result = narrow_non_union(db, member, predicate, positive, siblings, resolve);
        changed |= result != member;
        if result != InferredTypeData::NeverKeyword {
            narrowed.push(result);
        }
    }

    if narrowed.is_empty() {
        InferredTypeData::NeverKeyword
    } else if !changed {
        ty
    } else {
        // These are bounded, non-union leaves, so construction cannot expand
        // another union graph outside the traversal budget.
        InferredTypeData::union_from_types(db, narrowed)
    }
}

/// Records which nullish values the other members of a narrowed union are.
/// A type parameter that `!== null`, `!== undefined`, or `typeof` with
/// `"undefined"` narrows intersects with `{}` alone when another member is
/// already the other nullish value.
#[derive(Clone, Copy, Default)]
struct Siblings {
    has_null: bool,
    has_undefined: bool,
}

enum PreparedPredicate<'db> {
    Truthy,
    /// Keeps the values that [`falsy_values`] describes in either branch.
    FalsyValues,
    Nullish,
    Typeof(TypeofKind),
    Literal(InferredTypeData<'db>, LiteralValue<'db>),
    LooseLiteral(InferredTypeData<'db>, LiteralValue<'db>),
    Equal(ComparedValues<'db>),
}

impl<'db> PreparedPredicate<'db> {
    /// Returns `predicate` with its literal decoded, or `None` for a literal
    /// that can't be decoded, or a value whose members exceed the union
    /// budget, which leave every type unchanged.
    fn new(db: &'db dyn TypeDb, predicate: NarrowingPredicate<'db>) -> Option<Self> {
        Some(match predicate {
            NarrowingPredicate::Truthy => Self::Truthy,
            NarrowingPredicate::Nullish => Self::Nullish,
            NarrowingPredicate::Typeof(kind) => Self::Typeof(kind),
            NarrowingPredicate::Literal(literal) => {
                Self::Literal(literal, literal_value(db, literal)?)
            }
            NarrowingPredicate::LooseLiteral(literal) => match literal_value(db, literal)? {
                LiteralValue::Null | LiteralValue::Undefined => Self::Nullish,
                value => Self::LooseLiteral(literal, value),
            },
            NarrowingPredicate::Equal { value, strict } => {
                Self::Equal(ComparedValues::new(db, value, strict)?)
            }
        })
    }
}

/// The members of the type of a value that a narrowed value is compared
/// with, sorted into the kinds that decide whether the two may be equal.
struct ComparedValues<'db> {
    /// The type of the value, which a strict comparison gives `unknown` when
    /// it is a single primitive or literal type.
    value: InferredTypeData<'db>,
    strict: bool,
    members: Vec<ComparedMember<'db>>,
}

/// One member of the type of a compared value; see [`ComparedValues`].
enum ComparedMember<'db> {
    /// `null`, `undefined`, or a string, number, bigint, or boolean literal
    /// that Biome decodes.
    Unit(InferredTypeData<'db>, LiteralValue<'db>),
    /// The primitive type of a kind, or a literal of that kind that Biome
    /// doesn't decode, such as a string with an escape.
    Primitive(TypeofKind),
    /// A type whose values are never primitives, such as `object`, a tuple,
    /// a function, or an instance of a built-in class.
    NonPrimitive,
    /// A type literal, an interface, or an instance of one or of a class,
    /// which every value except `null` and `undefined` may have, because a
    /// primitive has the members of its wrapper type, as every string has
    /// `length`.
    Structural,
    /// A type that Biome can't sort, such as `unknown`, `any`, or a type
    /// parameter, whose values may be anything.
    Any,
}

impl<'db> ComparedValues<'db> {
    /// Sorts the members of `value`, or returns `None` when they exceed the
    /// union budget.
    fn new(db: &'db dyn TypeDb, value: InferredTypeData<'db>, strict: bool) -> Option<Self> {
        let mut remaining = MAX_NARROWING_STEPS;
        let members = union_members(db, value, &mut remaining)?
            .into_iter()
            .map(|member| ComparedMember::new(db, member))
            .collect();
        Some(Self {
            value,
            strict,
            members,
        })
    }

    /// Narrows `ty`, a member of the narrowed type that is not a union, to
    /// the values that may equal one of the compared values, as TypeScript
    /// narrows by a value whose type is not a unit type: the true branch
    /// keeps the members that may equal a member of the value, and replaces
    /// a primitive type with the value's literals of its kind. The false
    /// branch keeps `ty`, because a value of a type that is not a unit type
    /// can differ from every value of `ty`.
    ///
    /// A loose comparison keeps a member of another kind whenever one of its
    /// values may coerce to a compared value, as the rules of
    /// [`loosely_equal`] decide for literals.
    fn narrow(
        &self,
        db: &'db dyn TypeDb,
        ty: InferredTypeData<'db>,
        positive: bool,
    ) -> InferredTypeData<'db> {
        if !positive {
            return ty;
        }
        let keep = |kept: bool| retain_if(ty, Some(kept), true);
        if let Some(actual) = literal_value(db, ty) {
            return keep(
                self.members
                    .iter()
                    .any(|member| member.may_equal_value(&actual, self.strict)),
            );
        }
        if let Some(kind) = primitive_kind(ty) {
            return self.narrow_primitive(db, ty, kind);
        }
        match ty {
            // Every value of a structural type except `null` and `undefined`
            // can have it, as every string has `{ length: number }`.
            ty if is_structural_type(db, ty) && !is_non_primitive(db, ty) => {
                keep(self.members.iter().any(|member| !member.is_nullish()))
            }
            ty if is_non_primitive(db, ty) || is_definite_non_primitive(db, ty) => {
                keep(self.members.iter().any(|member| {
                    if self.strict {
                        matches!(
                            member,
                            ComparedMember::NonPrimitive
                                | ComparedMember::Structural
                                | ComparedMember::Any
                        )
                    } else {
                        // An object can convert to any primitive.
                        !member.is_nullish()
                    }
                }))
            }
            _ => ty,
        }
    }

    /// Narrows `ty`, the primitive type of `kind`, as [`Self::narrow`]
    /// describes: it stays when a compared value may be any value of the
    /// kind, becomes the union of the compared literals of the kind when
    /// only they may equal one of its values, and is removed when no
    /// compared value may.
    fn narrow_primitive(
        &self,
        db: &'db dyn TypeDb,
        ty: InferredTypeData<'db>,
        kind: TypeofKind,
    ) -> InferredTypeData<'db> {
        let mut literals = Vec::new();
        for member in &self.members {
            match member {
                ComparedMember::Unit(literal, value) if value_kind(value) == Some(kind) => {
                    // `NaN` equals no value.
                    if !matches!(value, LiteralValue::Number(value) if value.is_nan()) {
                        literals.push(*literal);
                    }
                }
                ComparedMember::Unit(_, value) => {
                    if !self.strict
                        && !is_nullish_value(value)
                        && !never_loosely_equals_kind(value, kind)
                    {
                        return ty;
                    }
                }
                ComparedMember::Primitive(member_kind) => {
                    if *member_kind == kind || !self.strict {
                        return ty;
                    }
                }
                ComparedMember::NonPrimitive => {
                    if !self.strict {
                        return ty;
                    }
                }
                ComparedMember::Structural | ComparedMember::Any => return ty,
            }
        }
        if literals.is_empty() {
            InferredTypeData::NeverKeyword
        } else {
            InferredTypeData::union_from_types(db, literals)
        }
    }

    /// Returns what a strict comparison gives `unknown` in its true branch:
    /// the value's type when it is a single primitive or literal type,
    /// `object` when it is a single object type, and `unknown` otherwise, as
    /// in TypeScript.
    fn narrow_unknown(&self) -> InferredTypeData<'db> {
        if !self.strict {
            return InferredTypeData::UnknownKeyword;
        }
        match self.members.as_slice() {
            [ComparedMember::Unit(..) | ComparedMember::Primitive(_)] => self.value,
            [ComparedMember::NonPrimitive | ComparedMember::Structural] => {
                InferredTypeData::ObjectKeyword
            }
            _ => InferredTypeData::UnknownKeyword,
        }
    }
}

impl<'db> ComparedMember<'db> {
    fn new(db: &'db dyn TypeDb, member: InferredTypeData<'db>) -> Self {
        if let Some(value) = literal_value(db, member) {
            return Self::Unit(member, value);
        }
        if let Some(kind) = primitive_kind(member) {
            return Self::Primitive(kind);
        }
        match member {
            InferredTypeData::Literal(literal) => match literal.literal(db) {
                Literal::Boolean(_) => Self::Primitive(TypeofKind::Boolean),
                Literal::Number(_) => Self::Primitive(TypeofKind::Number),
                Literal::BigInt(_) => Self::Primitive(TypeofKind::BigInt),
                Literal::String(_) | Literal::Template(_) => Self::Primitive(TypeofKind::String),
                Literal::Object(_) | Literal::RegExp(_) => Self::NonPrimitive,
            },
            member if is_non_primitive(db, member) || is_definite_non_primitive(db, member) => {
                Self::NonPrimitive
            }
            member if is_structural_type(db, member) => Self::Structural,
            _ => Self::Any,
        }
    }

    fn is_nullish(&self) -> bool {
        matches!(self, Self::Unit(_, value) if is_nullish_value(value))
    }

    /// Returns whether a value of this member may equal `actual`, the value
    /// of a literal member of the narrowed type.
    fn may_equal_value(&self, actual: &LiteralValue, strict: bool) -> bool {
        match self {
            Self::Unit(_, value) => {
                if strict {
                    value == actual
                } else {
                    loosely_equal(value, actual)
                }
            }
            Self::Primitive(kind) => {
                if value_kind(actual) == Some(*kind) {
                    true
                } else {
                    !strict
                        && !is_nullish_value(actual)
                        && !never_loosely_equals_kind(actual, *kind)
                }
            }
            Self::NonPrimitive => !strict && !is_nullish_value(actual),
            Self::Structural => !is_nullish_value(actual),
            Self::Any => true,
        }
    }
}

/// Returns the kind of the primitive type `ty`, such as `string`, or `None`
/// for other types, literals included.
fn primitive_kind(ty: InferredTypeData) -> Option<TypeofKind> {
    Some(match ty {
        InferredTypeData::Boolean => TypeofKind::Boolean,
        InferredTypeData::Number => TypeofKind::Number,
        InferredTypeData::BigInt => TypeofKind::BigInt,
        InferredTypeData::String => TypeofKind::String,
        InferredTypeData::Symbol => TypeofKind::Symbol,
        _ => return None,
    })
}

/// Returns the kind of a decoded literal value, or `None` for `null` and
/// `undefined`.
fn value_kind(value: &LiteralValue) -> Option<TypeofKind> {
    Some(match value {
        LiteralValue::Null | LiteralValue::Undefined => return None,
        LiteralValue::Boolean(_) => TypeofKind::Boolean,
        LiteralValue::Number(_) => TypeofKind::Number,
        LiteralValue::BigInt(_) => TypeofKind::BigInt,
        LiteralValue::String(_) => TypeofKind::String,
    })
}

fn is_nullish_value(value: &LiteralValue) -> bool {
    matches!(value, LiteralValue::Null | LiteralValue::Undefined)
}

/// Returns whether `ty` is a type whose values are never primitives and that
/// [`is_non_primitive`] doesn't list: a tuple, a function, a class, or a
/// constructor.
fn is_definite_non_primitive(db: &dyn TypeDb, ty: InferredTypeData) -> bool {
    match ty {
        InferredTypeData::Tuple(_)
        | InferredTypeData::Function(_)
        | InferredTypeData::Class(_)
        | InferredTypeData::Constructor(_) => true,
        InferredTypeData::Literal(literal) => {
            matches!(literal.literal(db), Literal::Object(_) | Literal::RegExp(_))
        }
        _ => false,
    }
}

/// Returns whether JavaScript's loose equality `a == b` may hold for the
/// primitive values `a` and `b`.
///
/// Values of one kind compare as with strict equality, and `null` and
/// `undefined` equal only each other. For values of different kinds, the
/// result is false only where Biome can show it: a boolean converts to `0`
/// or `1` before it is compared with a number, a bigint and a number compare
/// by their mathematical values, and a string without any digit converts to
/// `NaN`, which equals nothing, before it is compared with a number, a
/// boolean, or a bigint. The string must not be empty, white space, or
/// `Infinity` with an optional sign, which convert to numbers. Every other
/// pair may be equal.
fn loosely_equal(a: &LiteralValue, b: &LiteralValue) -> bool {
    use LiteralValue::{BigInt, Boolean, Null, Number, String, Undefined};
    match (a, b) {
        (Null | Undefined, Null | Undefined) => true,
        (Null | Undefined, _) | (_, Null | Undefined) => false,
        (Boolean(boolean), Number(number)) | (Number(number), Boolean(boolean)) => {
            f64::from(u8::from(*boolean)) == *number
        }
        (BigInt(bigint), Number(number)) | (Number(number), BigInt(bigint)) => {
            !bigint_differs_from_number(bigint, *number)
        }
        (String(string), Number(_) | Boolean(_) | BigInt(_))
        | (Number(_) | Boolean(_) | BigInt(_), String(string)) => !converts_to_nan(string),
        (Boolean(_), BigInt(_)) | (BigInt(_), Boolean(_)) => true,
        (Boolean(_), Boolean(_))
        | (Number(_), Number(_))
        | (BigInt(_), BigInt(_))
        | (String(_), String(_)) => a == b,
    }
}

/// Returns whether no value of the primitive kind `kind` loosely equals
/// `value`, a value of another kind, as [`loosely_equal`] decides it for a
/// string without any digit, which equals no number, boolean, or bigint.
fn never_loosely_equals_kind(value: &LiteralValue, kind: TypeofKind) -> bool {
    matches!(value, LiteralValue::String(string) if converts_to_nan(string))
        && matches!(
            kind,
            TypeofKind::Number | TypeofKind::Boolean | TypeofKind::BigInt
        )
}

/// Returns whether `bigint`, the canonical text of a bigint literal such as
/// `12n`, certainly differs from `number`. Returns false when Biome can't
/// compare them, as for a bigint outside the range of `i128`.
fn bigint_differs_from_number(bigint: &str, number: f64) -> bool {
    if !number.is_finite() || number.fract() != 0.0 {
        return true;
    }
    let Some(Ok(bigint)) = bigint.strip_suffix('n').map(str::parse::<i128>) else {
        return false;
    };
    // An integer below this bound converts to `i128` exactly, and one above
    // it is larger than every `i128`.
    if number.abs() >= 1e38 {
        return true;
    }
    number as i128 != bigint
}

/// Returns whether the string `value` converts to `NaN` as a number: it has
/// no decimal digit and isn't empty, white space, or `Infinity` with an
/// optional sign.
///
/// Rust's white space and the byte order mark together include every
/// character that JavaScript trims, so a string that JavaScript trims to
/// nothing or to `Infinity` is never taken as `NaN`.
fn converts_to_nan(value: &str) -> bool {
    if value.bytes().any(|byte| byte.is_ascii_digit()) {
        return false;
    }
    let trimmed = value.trim_matches(|c: char| c.is_whitespace() || c == '\u{feff}');
    !matches!(trimmed, "" | "Infinity" | "+Infinity" | "-Infinity")
}

fn narrow_non_union<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    predicate: &PreparedPredicate<'db>,
    positive: bool,
    siblings: Siblings,
    resolve: &mut TypeResolver<'db, '_>,
) -> InferredTypeData<'db> {
    if let Some(generic) = referenced_generic(db, ty) {
        return narrow_type_parameter(db, ty, generic, predicate, positive, siblings, resolve);
    }
    match ty {
        InferredTypeData::Intersection(intersection) => {
            return narrow_intersection(db, ty, intersection, predicate, positive, resolve);
        }
        InferredTypeData::VoidKeyword => return InferredTypeData::Unknown,
        InferredTypeData::AnyKeyword => {
            return match (predicate, positive) {
                (PreparedPredicate::Typeof(kind), true)
                    if !matches!(kind, TypeofKind::Object | TypeofKind::Function) =>
                {
                    typeof_type(db, *kind).unwrap_or(ty)
                }
                _ => ty,
            };
        }
        InferredTypeData::UnknownKeyword => {
            return narrow_unknown(db, predicate, positive, resolve);
        }
        _ => {}
    }
    match predicate {
        PreparedPredicate::Truthy => match ty {
            InferredTypeData::Boolean => boolean_literal(db, positive),
            _ => match truthiness(db, ty) {
                None if !positive => narrow_object(db, ty, predicate, positive, resolve),
                truthiness => retain_if(ty, truthiness, positive),
            },
        },
        PreparedPredicate::FalsyValues => match ty {
            InferredTypeData::String => InferredTypeData::Literal(InternedLiteral::new(
                db,
                Literal::String(Text::new_static("").into()),
            )),
            InferredTypeData::BigInt => InferredTypeData::Literal(InternedLiteral::new(
                db,
                Literal::BigInt(Text::new_static("0n")),
            )),
            _ => narrow_non_union(
                db,
                ty,
                &PreparedPredicate::Truthy,
                false,
                Siblings::default(),
                resolve,
            ),
        },
        PreparedPredicate::Nullish => {
            let nullish = match ty {
                InferredTypeData::Null | InferredTypeData::Undefined => Some(true),
                ty if is_non_primitive(db, ty) || is_structural_type(db, ty) => Some(false),
                _ => typeof_kind(db, ty).map(|_| false),
            };
            retain_if(ty, nullish, positive)
        }
        PreparedPredicate::Typeof(kind) => {
            let matches = if is_non_primitive(db, ty) {
                // Non-primitive values can still be callable.
                match kind {
                    TypeofKind::Object | TypeofKind::Function => None,
                    _ => Some(false),
                }
            } else {
                typeof_kind(db, ty).map(|actual| actual == *kind)
            };
            match matches {
                None => narrow_object(db, ty, predicate, positive, resolve),
                matches => retain_if(ty, matches, positive),
            }
        }
        PreparedPredicate::Literal(literal, value) => {
            narrow_literal(db, ty, *literal, value, positive)
        }
        PreparedPredicate::LooseLiteral(literal, value) => {
            narrow_loose_literal(db, ty, *literal, value, positive)
        }
        PreparedPredicate::Equal(compared) => compared.narrow(db, ty, positive),
    }
}

/// Narrows `unknown` by `predicate`, as [`narrow_type`] describes.
///
/// For truthiness, nullishness, `typeof`, and strict equality with `null` or
/// `undefined`, `unknown` stands for `{} | null | undefined`, the values that
/// TypeScript gives it for such tests. Each of the three is narrowed on its
/// own, and the result is `unknown` again only when all three stay
/// unchanged, so `!== null` gives `{} | undefined`. Strict equality with
/// another literal gives the literal in the true branch, or `never` for
/// `NaN`, and keeps `unknown` in the false branch.
fn narrow_unknown<'db>(
    db: &'db dyn TypeDb,
    predicate: &PreparedPredicate<'db>,
    positive: bool,
    resolve: &mut TypeResolver<'db, '_>,
) -> InferredTypeData<'db> {
    let unknown = InferredTypeData::UnknownKeyword;
    let nullish_literal = match predicate {
        // TypeScript keeps `unknown` where it is loosely equal to a literal,
        // which many values of different kinds can be.
        PreparedPredicate::LooseLiteral(..) => return unknown,
        PreparedPredicate::Equal(compared) => {
            return if positive {
                compared.narrow_unknown()
            } else {
                unknown
            };
        }
        PreparedPredicate::Literal(_, LiteralValue::Null | LiteralValue::Undefined) => true,
        PreparedPredicate::Literal(_, LiteralValue::Number(value)) if value.is_nan() => {
            return if positive {
                InferredTypeData::NeverKeyword
            } else {
                unknown
            };
        }
        PreparedPredicate::Literal(literal, _) => return if positive { *literal } else { unknown },
        PreparedPredicate::Truthy
        | PreparedPredicate::FalsyValues
        | PreparedPredicate::Nullish
        | PreparedPredicate::Typeof(_) => false,
    };
    let empty = empty_object(db);
    let members = [empty, InferredTypeData::Null, InferredTypeData::Undefined];
    let narrowed = members.map(|member| {
        // `{}` has no value that equals `null` or `undefined`, which the
        // comparison of literals doesn't tell for object types.
        if nullish_literal && member == empty {
            return if positive {
                InferredTypeData::NeverKeyword
            } else {
                member
            };
        }
        narrow_non_union(
            db,
            member,
            predicate,
            positive,
            Siblings::default(),
            resolve,
        )
    });
    if narrowed == members {
        unknown
    } else {
        InferredTypeData::union_from_types(db, narrowed.to_vec())
    }
}

/// Narrows `ty`, a reference to the type parameter `generic`, as TypeScript
/// narrows a value whose type is a type parameter of the code that tests it.
///
/// Such a value can have any value of the type parameter's constraint, or
/// any value at all without one, so a test that keeps some of them gives the
/// intersection of `ty` with the type of the values that pass:
///
/// - truthiness and `!= null` give `T & {}`, TypeScript's `NonNullable<T>`;
/// - `!== null` gives `T & ({} | undefined)`, and `!== undefined` or
///   `typeof value !== "undefined"` gives `T & ({} | null)`, or `T & {}` when
///   another member of the narrowed union is already the other nullish value
///   or the constraint excludes it;
/// - `typeof` gives `T & string`, `T & undefined`, `T & Function`, and so on,
///   and `(T & object) | (T & null)` for `"object"`, keeping only the parts
///   whose values the constraint can have.
///
/// A test that every value of the constraint passes keeps `ty`, as for
/// `x != null` with `T extends {}`, and so does a test that none passes,
/// where TypeScript gives `never`. Every other test and branch keeps `ty`,
/// as equality with another literal and the falsy branch do, and so does a
/// constraint that refers to another type parameter at its top level, as
/// `T extends U | null` does, which Biome doesn't narrow. `siblings` tells
/// which nullish values the other members of the narrowed union are.
fn narrow_type_parameter<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    generic: InternedGenericTypeParameter<'db>,
    predicate: &PreparedPredicate<'db>,
    positive: bool,
    siblings: Siblings,
    resolve: &mut TypeResolver<'db, '_>,
) -> InferredTypeData<'db> {
    let constraint = match generic.constraint(db) {
        Some(constraint) => match flattened(db, constraint, resolve) {
            // Narrowing such a constraint would narrow its type parameter,
            // whose constraint can lead back to this one.
            Some(constraint) if refers_to_type_parameter(db, constraint) => return ty,
            Some(constraint) => Some(constraint),
            None => return ty,
        },
        None => None,
    };
    let null = PreparedPredicate::Literal(InferredTypeData::Null, LiteralValue::Null);
    let undefined =
        PreparedPredicate::Literal(InferredTypeData::Undefined, LiteralValue::Undefined);
    let mut passes = |predicate: &PreparedPredicate<'db>, positive: bool| {
        constraint.map_or(Some(InferredTypeData::UnknownKeyword), |constraint| {
            let narrowed = narrow_prepared(db, constraint, predicate, positive, resolve);
            (narrowed != InferredTypeData::NeverKeyword).then_some(narrowed)
        })
    };
    let removed_nullish = match (predicate, positive) {
        (PreparedPredicate::Truthy | PreparedPredicate::Nullish, _)
            if positive == matches!(predicate, PreparedPredicate::Truthy) =>
        {
            return if passes(&PreparedPredicate::Nullish, true).is_some() {
                intersect_narrowed(db, ty, empty_object(db))
            } else {
                ty
            };
        }
        (PreparedPredicate::Literal(_, LiteralValue::Null), false) => Some(false),
        (
            PreparedPredicate::Literal(_, LiteralValue::Undefined)
            | PreparedPredicate::Typeof(TypeofKind::Undefined),
            false,
        ) => Some(true),
        _ => None,
    };
    if let Some(removes_undefined) = removed_nullish {
        let (removed, other, other_type, other_is_sibling) = if removes_undefined {
            (&undefined, &null, InferredTypeData::Null, siblings.has_null)
        } else {
            (
                &null,
                &undefined,
                InferredTypeData::Undefined,
                siblings.has_undefined,
            )
        };
        if passes(removed, true).is_none() {
            return ty;
        }
        let candidate = if !other_is_sibling && passes(other, true).is_some() {
            InferredTypeData::union_from_types(db, vec![empty_object(db), other_type])
        } else {
            empty_object(db)
        };
        return intersect_narrowed(db, ty, candidate);
    }
    let (PreparedPredicate::Typeof(kind), true) = (predicate, positive) else {
        return ty;
    };
    // A constraint whose every value passes, or whose no value passes,
    // leaves nothing to intersect with.
    match passes(predicate, true) {
        Some(narrowed) if constraint != Some(narrowed) => {}
        Some(_) | None => return ty,
    }
    let Some(kind_type) = typeof_type(db, *kind) else {
        let null_part = passes(&null, true).is_some();
        let objects = match (constraint, passes(&null, false)) {
            (None, _) => true,
            (Some(_), None) => false,
            (Some(_), Some(without_null)) => {
                narrow_prepared(
                    db,
                    without_null,
                    &PreparedPredicate::Typeof(TypeofKind::Object),
                    true,
                    resolve,
                ) != InferredTypeData::NeverKeyword
            }
        };
        let parts: Vec<_> = [
            objects.then_some(InferredTypeData::ObjectKeyword),
            null_part.then_some(InferredTypeData::Null),
        ]
        .into_iter()
        .flatten()
        .map(|part| intersect_narrowed(db, ty, part))
        .collect();
        return if parts.is_empty() {
            ty
        } else {
            InferredTypeData::union_from_types(db, parts)
        };
    };
    intersect_narrowed(db, ty, kind_type)
}

/// Narrows `ty`, the intersection `intersection` with a type parameter as an
/// operand, such as the `T & {}` or `T & string` that narrowing gives, by
/// narrowing each operand and intersecting the results. So
/// `typeof value === "string"` turns `T & {}` into `T & string`, and
/// truthiness keeps `T & string`, because `string & {}` is `string`.
///
/// As TypeScript does, an intersection with a primitive operand keeps its
/// object operands as they are, because they tag the primitive, as in
/// `T & string & { __brand: 1 }`. A test that removes every value of an
/// operand removes the intersection, except for `typeof` with `"function"`
/// and the false branch of `typeof` with `"object"`, which TypeScript
/// applies to an intersection when any operand passes them.
///
/// An equality test, strict or loose, with a literal, `null`, `undefined`, or
/// a typed value removes the intersection when it removes every value of an
/// operand and keeps the intersection as it is otherwise, as TypeScript keeps
/// `T & string` where the value equals `"a"`. A `boolean` operand narrows to
/// `true` or `false` instead, because TypeScript reads `T & boolean` as
/// `(T & true) | (T & false)` and keeps the members that can be equal. A
/// type predicate whose target is a literal, `null`, or `undefined` narrows
/// as strict equality with the target does, so `value is "a"` keeps
/// `T & string` where TypeScript gives `T & "a"`.
///
/// An intersection without a type parameter stays as it is.
fn narrow_intersection<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    intersection: InternedIntersection<'db>,
    predicate: &PreparedPredicate<'db>,
    positive: bool,
    resolve: &mut TypeResolver<'db, '_>,
) -> InferredTypeData<'db> {
    let operands = intersection.types(db);
    if !operands
        .iter()
        .any(|operand| referenced_generic(db, *operand).is_some())
    {
        return ty;
    }
    let has_primitive = operands.iter().any(|operand| operand.is_primitive(db));
    let any_operand_passes = matches!(
        (predicate, positive),
        (PreparedPredicate::Typeof(TypeofKind::Function), true)
            | (PreparedPredicate::Typeof(TypeofKind::Object), false)
    );
    let is_equality = matches!(
        predicate,
        PreparedPredicate::Literal(..)
            | PreparedPredicate::LooseLiteral(..)
            | PreparedPredicate::Equal(_)
    );
    let mut narrowed = Vec::with_capacity(operands.len());
    let mut changed = false;
    for operand in operands {
        let is_tag = has_primitive
            && !operand.is_primitive(db)
            && referenced_generic(db, *operand).is_none();
        let result = if is_tag {
            *operand
        } else {
            narrow_prepared(db, *operand, predicate, positive, resolve)
        };
        if result == InferredTypeData::NeverKeyword {
            if !any_operand_passes {
                return result;
            }
            narrowed.push(*operand);
            continue;
        }
        if is_equality && *operand != InferredTypeData::Boolean {
            narrowed.push(*operand);
            continue;
        }
        changed |= result != *operand;
        narrowed.push(result);
    }
    if !changed {
        return ty;
    }
    IntersectionBuilder::without_merging(db)
        .add_all(narrowed)
        .build()
}

/// Returns whether `ty` or a member of it, when it is a union, is a type
/// parameter or an intersection with one.
fn refers_to_type_parameter<'db>(db: &'db dyn TypeDb, ty: InferredTypeData<'db>) -> bool {
    match ty {
        InferredTypeData::Union(union) => union
            .types(db)
            .iter()
            .any(|member| is_type_parameter_or_intersection(db, *member)),
        ty => is_type_parameter_or_intersection(db, ty),
    }
}

/// Returns whether `ty` is a type parameter, or an intersection with a type
/// parameter among its operands, such as the `T & {}` that narrowing gives.
pub fn is_type_parameter_or_intersection<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
) -> bool {
    referenced_generic(db, ty).is_some()
        || matches!(ty, InferredTypeData::Intersection(intersection)
            if intersection
                .types(db)
                .iter()
                .any(|operand| referenced_generic(db, *operand).is_some()))
}

/// Returns `ty` with its handles resolved, also those of the members of a
/// union, or `None` when the members exceed the budget of 256 visited types.
fn flattened<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    resolve: &mut TypeResolver<'db, '_>,
) -> Option<InferredTypeData<'db>> {
    let mut pending = vec![ty];
    let mut members = Vec::new();
    let mut remaining = MAX_NARROWING_STEPS;
    while let Some(current) = pending.pop() {
        remaining = remaining.checked_sub(1)?;
        match resolve_wrapped_type(db, current, resolve) {
            InferredTypeData::Union(union) => pending.extend(union.types(db).iter().rev()),
            member => members.push(member),
        }
    }
    Some(InferredTypeData::union_from_types(db, members))
}

/// Returns the intersection of `ty`, the type of a value that a runtime test
/// narrows, with `candidate`, a type that the test shows the value to have,
/// as TypeScript gives `T & string` or `Bar & Foo`.
///
/// The intersection is normalized as the intersection builder normalizes
/// one, so it distributes over the members of a union, `string & {}` is
/// `string`, and `null & {}` is `never`. Its object operands are never merged
/// into one type, so each keeps the declaration it names, which a later
/// `instanceof` or type predicate compares, as TypeScript compares the
/// operands of `B & A` with `A`.
pub(crate) fn intersect_narrowed<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    candidate: InferredTypeData<'db>,
) -> InferredTypeData<'db> {
    IntersectionBuilder::without_merging(db)
        .add(ty)
        .add(candidate)
        .build()
}

/// Returns `{}`, the type literal without members, which every value other
/// than `null` and `undefined` has.
fn empty_object(db: &dyn TypeDb) -> InferredTypeData<'_> {
    InferredTypeData::Object(InternedObject::new(
        db,
        None,
        Box::default(),
        false,
        ObjectKind::Shape,
    ))
}

/// Returns whether `ty` is `{}`: a type literal or an object expression
/// without members, also through an alias. An interface or a class without
/// members is not, because TypeScript keeps such a type where it replaces `{}`
/// with `object`.
fn is_empty_object_literal<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    resolve: &mut TypeResolver<'db, '_>,
) -> bool {
    let ty = match ty {
        InferredTypeData::InstanceOf(instance) if instance.type_parameters(db).is_empty() => {
            resolve(instance.ty(db))
        }
        ty => ty,
    };
    matches!(
        ty,
        InferredTypeData::Object(object)
            if object.prototype(db).is_none()
                && !object.has_unknown_members(db)
                && object.members(db).is_empty()
    )
}

/// Narrows the object type `ty` by a falsy or `typeof` test that the type's
/// truthiness or `typeof` result does not decide.
///
/// Returns `NeverKeyword` when no value that passes the test can have `ty`. In
/// the true branch of `typeof`, returns the tested kind's type when every value
/// of that kind has `ty`, and `object` for `"object"` when `ty` is `{}`.
/// Returns `ty` unchanged otherwise, including when `ty` is not an object type
/// or the branch keeps object types. [`narrow_type`] lists the branches that
/// compare object types.
fn narrow_object<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    predicate: &PreparedPredicate<'db>,
    positive: bool,
    resolve: &mut TypeResolver<'db, '_>,
) -> InferredTypeData<'db> {
    if positive
        && matches!(predicate, PreparedPredicate::Typeof(TypeofKind::Object))
        && is_empty_object_literal(db, ty, resolve)
    {
        return InferredTypeData::ObjectKeyword;
    }
    const FALSY_KINDS: [TypeofKind; 4] = [
        TypeofKind::String,
        TypeofKind::Number,
        TypeofKind::Boolean,
        TypeofKind::BigInt,
    ];
    const NOT_OBJECT_KINDS: [TypeofKind; 6] = [
        TypeofKind::String,
        TypeofKind::Number,
        TypeofKind::Boolean,
        TypeofKind::BigInt,
        TypeofKind::Symbol,
        TypeofKind::Function,
    ];
    // The kinds of the values that pass the test, and the type that replaces
    // `ty` when every value of the only kind has it.
    let (kinds, implied) = match (predicate, positive) {
        (PreparedPredicate::Truthy, false) => (&FALSY_KINDS[..], None),
        (PreparedPredicate::Typeof(TypeofKind::Object), false) => (&NOT_OBJECT_KINDS[..], None),
        (PreparedPredicate::Typeof(kind), true) if *kind != TypeofKind::Object => {
            (std::slice::from_ref(kind), typeof_type(db, *kind))
        }
        _ => return ty,
    };
    if !is_object_type(db, ty, resolve) {
        return ty;
    }
    for kind in kinds {
        match primitive_can_satisfy(db, *kind, ty, resolve) {
            Assignability::NotAssignable => {}
            Assignability::Assignable => return implied.unwrap_or(ty),
            Assignability::Undetermined => return ty,
        }
    }
    InferredTypeData::NeverKeyword
}

/// Returns whether `ty` is a type literal, an interface, or an instance of a
/// class, an interface, or a type literal.
fn is_object_type<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    resolve: &mut TypeResolver<'db, '_>,
) -> bool {
    match ty {
        InferredTypeData::Object(_) | InferredTypeData::Interface(_) => true,
        InferredTypeData::InstanceOf(instance) => matches!(
            resolve(instance.ty(db)).expand_canonical_global(db),
            InferredTypeData::Class(_)
                | InferredTypeData::Interface(_)
                | InferredTypeData::Object(_)
        ),
        _ => false,
    }
}

/// Returns the type of the values whose `typeof` is `kind`, or `None` for
/// `"object"`.
fn typeof_type<'db>(db: &'db dyn TypeDb, kind: TypeofKind) -> Option<InferredTypeData<'db>> {
    Some(match kind {
        TypeofKind::Undefined => InferredTypeData::Undefined,
        TypeofKind::Boolean => InferredTypeData::Boolean,
        TypeofKind::Number => InferredTypeData::Number,
        TypeofKind::BigInt => InferredTypeData::BigInt,
        TypeofKind::String => InferredTypeData::String,
        TypeofKind::Symbol => InferredTypeData::Symbol,
        TypeofKind::Function => InferredTypeData::instance_of(
            db,
            InferredTypeData::GlobalType(FUNCTION_ID_GLOBAL_TYPE_ID).expand_canonical_global(db),
            Box::default(),
        ),
        TypeofKind::Object => return None,
    })
}

fn retain_if(
    ty: InferredTypeData<'_>,
    matches: Option<bool>,
    positive: bool,
) -> InferredTypeData<'_> {
    if matches.is_some_and(|matches| matches != positive) {
        InferredTypeData::NeverKeyword
    } else {
        ty
    }
}

fn typeof_kind(db: &dyn TypeDb, ty: InferredTypeData<'_>) -> Option<TypeofKind> {
    Some(match ty {
        InferredTypeData::Undefined => TypeofKind::Undefined,
        InferredTypeData::Null | InferredTypeData::Tuple(_) => TypeofKind::Object,
        InferredTypeData::Boolean => TypeofKind::Boolean,
        InferredTypeData::Number => TypeofKind::Number,
        InferredTypeData::BigInt => TypeofKind::BigInt,
        InferredTypeData::String => TypeofKind::String,
        InferredTypeData::Symbol => TypeofKind::Symbol,
        InferredTypeData::Function(_)
        | InferredTypeData::Class(_)
        | InferredTypeData::Constructor(_) => TypeofKind::Function,
        InferredTypeData::Literal(literal) => match literal.literal(db) {
            Literal::Boolean(_) => TypeofKind::Boolean,
            Literal::Number(_) => TypeofKind::Number,
            Literal::BigInt(_) => TypeofKind::BigInt,
            Literal::String(_) | Literal::Template(_) => TypeofKind::String,
            Literal::Object(_) | Literal::RegExp(_) => TypeofKind::Object,
        },
        _ => return None,
    })
}

fn is_non_primitive(db: &dyn TypeDb, ty: InferredTypeData<'_>) -> bool {
    match ty {
        InferredTypeData::ObjectKeyword => true,
        InferredTypeData::Object(object) => object.kind(db) == ObjectKind::Expression,
        InferredTypeData::InstanceOf(instance) => {
            let target = instance.ty(db);
            target.is_builtin_object_class(db)
        }
        _ => false,
    }
}

// Structural types exclude nullish values but can include falsy primitives:
// for example, a number satisfies an empty interface or `{}`.
fn is_structural_type(db: &dyn TypeDb, ty: InferredTypeData<'_>) -> bool {
    match ty {
        InferredTypeData::Object(_) | InferredTypeData::Interface(_) => true,
        InferredTypeData::InstanceOf(instance) => matches!(
            instance.ty(db).expand_canonical_global(db),
            InferredTypeData::Class(_)
                | InferredTypeData::Interface(_)
                | InferredTypeData::Object(_)
                | InferredTypeData::Tuple(_)
        ),
        _ => false,
    }
}

fn truthiness(db: &dyn TypeDb, ty: InferredTypeData<'_>) -> Option<bool> {
    match ty {
        ty if is_non_primitive(db, ty) => Some(true),
        InferredTypeData::Null | InferredTypeData::Undefined => Some(false),
        InferredTypeData::Symbol
        | InferredTypeData::ObjectKeyword
        | InferredTypeData::Tuple(_)
        | InferredTypeData::Function(_)
        | InferredTypeData::Class(_)
        | InferredTypeData::Constructor(_) => Some(true),
        InferredTypeData::Literal(literal) => match literal.literal(db) {
            Literal::Object(_) | Literal::RegExp(_) => Some(true),
            _ => literal_value(db, ty).map(|value| match value {
                LiteralValue::Null | LiteralValue::Undefined => false,
                LiteralValue::Boolean(value) => value,
                LiteralValue::Number(value) => value != 0.0 && !value.is_nan(),
                LiteralValue::BigInt(value) => value != "0n",
                LiteralValue::String(value) => !value.is_empty(),
            }),
        },
        _ => None,
    }
}

fn boolean_literal(db: &dyn TypeDb, value: bool) -> InferredTypeData<'_> {
    InferredTypeData::Literal(InternedLiteral::new(db, Literal::Boolean(value.into())))
}

// Floating-point equality models strict numeric equality: signed zeros compare
// equal, and NaN compares unequal even to itself.
#[derive(PartialEq)]
enum LiteralValue<'a> {
    Null,
    Undefined,
    Boolean(bool),
    Number(f64),
    BigInt(Cow<'a, str>),
    String(&'a str),
}

fn literal_value<'db>(db: &'db dyn TypeDb, ty: InferredTypeData<'db>) -> Option<LiteralValue<'db>> {
    Some(match ty {
        InferredTypeData::Null => LiteralValue::Null,
        InferredTypeData::Undefined => LiteralValue::Undefined,
        InferredTypeData::Literal(literal) => match literal.literal(db) {
            Literal::Boolean(boolean) => LiteralValue::Boolean(boolean.as_bool()),
            Literal::Number(number) if number.as_str().len() <= MAX_LITERAL_BYTES => {
                let text = number.as_str();
                let (negative, unsigned) = text
                    .strip_prefix('-')
                    .map_or((false, text), |unsigned| (true, unsigned));
                let value = parse_js_number_with_single_rounding(unsigned)?;
                LiteralValue::Number(if negative { -value } else { value })
            }
            Literal::BigInt(text) if text.text().len() <= MAX_LITERAL_BYTES => {
                LiteralValue::BigInt(canonicalize_js_bigint_literal(text.text())?)
            }
            Literal::String(string)
                if string.as_str().len() <= MAX_LITERAL_BYTES
                    && !string.as_str().contains('\\') =>
            {
                LiteralValue::String(string.as_str())
            }
            _ => return None,
        },
        _ => return None,
    })
}

fn narrow_literal<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    literal: InferredTypeData<'db>,
    value: &LiteralValue<'db>,
    positive: bool,
) -> InferredTypeData<'db> {
    if let Some(actual) = literal_value(db, ty) {
        return retain_if(ty, Some(actual == *value), positive);
    }
    // `boolean` is `true | false`, so a value that differs from one of them
    // is the other.
    if let (InferredTypeData::Boolean, LiteralValue::Boolean(value), false) = (ty, value, positive)
    {
        return boolean_literal(db, !value);
    }
    let matches_primitive = match ty {
        InferredTypeData::BigInt => matches!(value, LiteralValue::BigInt(_)),
        InferredTypeData::Boolean => matches!(value, LiteralValue::Boolean(_)),
        InferredTypeData::Number => {
            matches!(value, LiteralValue::Number(value) if !value.is_nan())
        }
        InferredTypeData::String => matches!(value, LiteralValue::String(_)),
        ty if is_non_primitive(db, ty) => false,
        InferredTypeData::Symbol
        | InferredTypeData::ObjectKeyword
        | InferredTypeData::Tuple(_)
        | InferredTypeData::Function(_)
        | InferredTypeData::Class(_)
        | InferredTypeData::Constructor(_) => false,
        InferredTypeData::Literal(literal)
            if matches!(literal.literal(db), Literal::Object(_) | Literal::RegExp(_)) =>
        {
            false
        }
        _ => return ty,
    };
    if matches_primitive {
        if positive { literal } else { ty }
    } else {
        retain_if(ty, Some(false), positive)
    }
}

/// Narrows `ty` by loose equality (`==`) with `literal`, whose value `value`
/// is a string, number, bigint, or boolean.
///
/// The false branch narrows as strict equality does: it removes only the
/// members strictly equal to `literal`, as TypeScript does. In the true
/// branch, a member of the literal's kind narrows as for strict equality, so
/// `string` becomes `"a"` for `== "a"`, and `null` and `undefined` are
/// removed. A literal member of another kind is removed only when
/// [`loosely_equal`] shows that it never equals `value`, as `"a"` for `== 1`.
/// Primitive types of other kinds and object types stay, because some of
/// their values convert to `value`: `"1" == 1` holds, and so does
/// `["a"] == "a"`. TypeScript narrows loose equality as strict equality,
/// which removes them.
fn narrow_loose_literal<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    literal: InferredTypeData<'db>,
    value: &LiteralValue<'db>,
    positive: bool,
) -> InferredTypeData<'db> {
    if !positive {
        return narrow_literal(db, ty, literal, value, false);
    }
    if let Some(actual) = literal_value(db, ty) {
        return retain_if(ty, Some(loosely_equal(&actual, value)), true);
    }
    match (primitive_kind(ty), value_kind(value)) {
        (Some(TypeofKind::Number), _) if matches!(value, LiteralValue::Number(number) if number.is_nan()) => {
            InferredTypeData::NeverKeyword
        }
        (Some(kind), Some(literal_kind)) if kind == literal_kind => literal,
        _ => ty,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generated::global_types::ids::{
        ARRAY_ID_GLOBAL_TYPE_ID, ERROR_ID_GLOBAL_TYPE_ID, OBJECT_ID_GLOBAL_TYPE_ID,
    };
    use crate::interned_types::{
        ClassParents, InternedClass, InternedConstructor, InternedFunction,
        InternedGenericTypeParameter, InternedInterface, InternedIntersection, InternedObject,
        InternedTuple, InternedTypeInstance, InternedTypeOperatorType, InternedTypeofType,
        InternedUnion, LocalTypeHandle, LocalTypeId, ModuleKey, ReturnType, TupleElementType,
        TypeMember, TypeMemberKind,
    };
    use crate::literal::NumberLiteral;
    use salsa::plumbing::AsId;

    #[salsa::db]
    #[derive(Default)]
    struct TestDb {
        storage: salsa::Storage<Self>,
    }

    #[salsa::db]
    impl salsa::Database for TestDb {}

    #[salsa::db]
    impl biome_db::Db for TestDb {
        fn parsed_source_for_path(&self, _path: &camino::Utf8Path) -> Option<biome_db::Source> {
            None
        }
    }

    #[salsa::db]
    impl TypeDb for TestDb {}

    /// Calls [`super::narrow_type`] with a resolver that returns its input, so
    /// local handles stay unresolved.
    fn narrow_type<'db>(
        db: &'db dyn TypeDb,
        ty: InferredTypeData<'db>,
        predicate: NarrowingPredicate<'db>,
        positive: bool,
    ) -> InferredTypeData<'db> {
        super::narrow_type(db, ty, predicate, positive, |ty| ty)
    }

    /// Calls [`super::falsy_values`] with a resolver that returns its input.
    fn falsy_values<'db>(db: &'db dyn TypeDb, ty: InferredTypeData<'db>) -> InferredTypeData<'db> {
        super::falsy_values(db, ty, |ty| ty)
    }

    fn number<'db>(db: &'db TestDb, text: &str) -> InferredTypeData<'db> {
        InferredTypeData::Literal(InternedLiteral::new(
            db,
            Literal::Number(NumberLiteral::new(text.to_owned().into())),
        ))
    }

    fn bigint<'db>(db: &'db TestDb, text: &str) -> InferredTypeData<'db> {
        InferredTypeData::Literal(InternedLiteral::new(
            db,
            Literal::BigInt(text.to_owned().into()),
        ))
    }

    fn string<'db>(db: &'db TestDb, text: &str) -> InferredTypeData<'db> {
        InferredTypeData::Literal(InternedLiteral::new(db, Literal::String(text.into())))
    }

    fn union<'db>(db: &'db TestDb, types: &[InferredTypeData<'db>]) -> InferredTypeData<'db> {
        InferredTypeData::union_from_types(db, types.to_vec())
    }

    #[test]
    fn nullish_filters_both_branches() {
        let db = TestDb::default();
        let nullish = union(&db, &[InferredTypeData::Null, InferredTypeData::Undefined]);
        let ty = union(&db, &[InferredTypeData::String, nullish]);
        assert_eq!(
            narrow_type(&db, ty, NarrowingPredicate::Nullish, true),
            nullish
        );
        assert_eq!(
            narrow_type(&db, ty, NarrowingPredicate::Nullish, false),
            InferredTypeData::String
        );
        assert_eq!(
            narrow_type(
                &db,
                InferredTypeData::VoidKeyword,
                NarrowingPredicate::Nullish,
                true
            ),
            InferredTypeData::Unknown
        );
        assert_eq!(
            narrow_type(
                &db,
                nullish,
                NarrowingPredicate::Literal(InferredTypeData::Null),
                true
            ),
            InferredTypeData::Null
        );
        assert_eq!(
            narrow_type(
                &db,
                nullish,
                NarrowingPredicate::Literal(InferredTypeData::Null),
                false
            ),
            InferredTypeData::Undefined
        );
    }

    #[test]
    fn typeof_uses_runtime_categories() {
        let db = TestDb::default();
        let function = InferredTypeData::Function(InternedFunction::new(
            &db,
            Box::default(),
            Box::default(),
            ReturnType::Type(InferredTypeData::Unknown),
            false,
            None,
        ));
        let cases = [
            (InferredTypeData::Undefined, TypeofKind::Undefined),
            (InferredTypeData::Null, TypeofKind::Object),
            (InferredTypeData::Boolean, TypeofKind::Boolean),
            (InferredTypeData::Number, TypeofKind::Number),
            (InferredTypeData::BigInt, TypeofKind::BigInt),
            (InferredTypeData::String, TypeofKind::String),
            (InferredTypeData::Symbol, TypeofKind::Symbol),
            (function, TypeofKind::Function),
            (number(&db, "0"), TypeofKind::Number),
            (string(&db, "x"), TypeofKind::String),
        ];
        for (ty, actual) in cases {
            for (_, kind) in cases {
                for positive in [false, true] {
                    assert_eq!(
                        narrow_type(&db, ty, NarrowingPredicate::Typeof(kind), positive),
                        if (actual == kind) == positive {
                            ty
                        } else {
                            InferredTypeData::NeverKeyword
                        }
                    );
                }
            }
        }
        let ty = union(&db, &[InferredTypeData::Null, function, string(&db, "x")]);
        assert_eq!(
            narrow_type(
                &db,
                ty,
                NarrowingPredicate::Typeof(TypeofKind::Object),
                true
            ),
            InferredTypeData::Null
        );
        assert_eq!(
            narrow_type(
                &db,
                ty,
                NarrowingPredicate::Typeof(TypeofKind::Object),
                false
            ),
            union(&db, &[function, string(&db, "x")])
        );
        for kind in [TypeofKind::Object, TypeofKind::Function] {
            for positive in [false, true] {
                assert_eq!(
                    narrow_type(
                        &db,
                        InferredTypeData::ObjectKeyword,
                        NarrowingPredicate::Typeof(kind),
                        positive
                    ),
                    InferredTypeData::ObjectKeyword
                );
            }
        }
    }

    #[test]
    fn truthiness_narrows_representable_primitives() {
        let db = TestDb::default();
        for positive in [false, true] {
            assert_eq!(
                narrow_type(
                    &db,
                    InferredTypeData::Boolean,
                    NarrowingPredicate::Truthy,
                    positive
                ),
                boolean_literal(&db, positive)
            );
            for ty in [
                InferredTypeData::Number,
                InferredTypeData::String,
                InferredTypeData::BigInt,
            ] {
                assert_eq!(
                    narrow_type(&db, ty, NarrowingPredicate::Truthy, positive),
                    ty
                );
            }
        }
        let nullable = union(&db, &[InferredTypeData::String, InferredTypeData::Null]);
        assert_eq!(
            narrow_type(&db, nullable, NarrowingPredicate::Truthy, false),
            nullable
        );
        let falsy = [
            boolean_literal(&db, false),
            string(&db, ""),
            InferredTypeData::Null,
        ];
        let ty = union(
            &db,
            &[
                union(&db, &falsy),
                string(&db, "ok"),
                InferredTypeData::Number,
            ],
        );
        assert_eq!(
            narrow_type(&db, ty, NarrowingPredicate::Truthy, true),
            union(&db, &[string(&db, "ok"), InferredTypeData::Number])
        );
        assert_eq!(
            narrow_type(&db, ty, NarrowingPredicate::Truthy, false),
            union(&db, &[union(&db, &falsy), InferredTypeData::Number])
        );
    }

    #[test]
    fn falsy_values_reduce_broad_strings_and_bigints_to_their_falsy_value() {
        let db = TestDb::default();
        for (ty, expected) in [
            (InferredTypeData::String, string(&db, "")),
            (InferredTypeData::BigInt, bigint(&db, "0n")),
            (InferredTypeData::Number, InferredTypeData::Number),
            (InferredTypeData::Boolean, boolean_literal(&db, false)),
            (InferredTypeData::Null, InferredTypeData::Null),
            (InferredTypeData::Symbol, InferredTypeData::NeverKeyword),
            (string(&db, "ok"), InferredTypeData::NeverKeyword),
            (InferredTypeData::VoidKeyword, InferredTypeData::Unknown),
            (
                InferredTypeData::UnknownKeyword,
                InferredTypeData::UnknownKeyword,
            ),
            (InferredTypeData::Unknown, InferredTypeData::Unknown),
        ] {
            assert_eq!(falsy_values(&db, ty), expected, "{ty:?}");
        }
        let ty = union(
            &db,
            &[
                InferredTypeData::String,
                InferredTypeData::BigInt,
                string(&db, "ok"),
                InferredTypeData::Undefined,
            ],
        );
        assert_eq!(
            falsy_values(&db, ty),
            union(
                &db,
                &[
                    string(&db, ""),
                    bigint(&db, "0n"),
                    InferredTypeData::Undefined
                ]
            )
        );
    }

    #[test]
    fn literal_truthiness_uses_values() {
        let db = TestDb::default();
        let cases = [
            (number(&db, "0"), false),
            (number(&db, "-0"), false),
            (number(&db, "NaN"), false),
            (number(&db, "1e-999"), false),
            (number(&db, "0.5"), true),
            (number(&db, "1e400"), true),
            (bigint(&db, "-0x00n"), false),
            (bigint(&db, "0b10n"), true),
            (string(&db, ""), false),
            (string(&db, "text"), true),
            (boolean_literal(&db, true), true),
            (boolean_literal(&db, false), false),
            (InferredTypeData::Symbol, true),
            (InferredTypeData::Undefined, false),
        ];
        for (ty, truthy) in cases {
            for positive in [false, true] {
                assert_eq!(
                    narrow_type(&db, ty, NarrowingPredicate::Truthy, positive),
                    if truthy == positive {
                        ty
                    } else {
                        InferredTypeData::NeverKeyword
                    }
                );
            }
        }
    }

    #[test]
    fn strict_numeric_equality_uses_canonical_values() {
        let db = TestDb::default();
        for (left, right) in [
            ("0x10", "16"),
            ("-0", "0"),
            ("-0x10", "-16"),
            ("1_000", "1e3"),
            ("0.5", ".5"),
            ("0x1000000000000081", "1152921504606847232"),
        ] {
            let left = number(&db, left);
            let predicate = NarrowingPredicate::Literal(number(&db, right));
            assert_eq!(narrow_type(&db, left, predicate, true), left);
            assert_eq!(
                narrow_type(&db, left, predicate, false),
                InferredTypeData::NeverKeyword
            );
        }
        let nan = number(&db, "NaN");
        for ty in [nan, InferredTypeData::Number] {
            assert_eq!(
                narrow_type(&db, ty, NarrowingPredicate::Literal(nan), true),
                InferredTypeData::NeverKeyword
            );
            assert_eq!(
                narrow_type(&db, ty, NarrowingPredicate::Literal(nan), false),
                ty
            );
        }
        let zero_or_one = union(&db, &[number(&db, "0"), number(&db, "1")]);
        assert_eq!(
            narrow_type(
                &db,
                zero_or_one,
                NarrowingPredicate::Literal(number(&db, "-0")),
                false
            ),
            number(&db, "1")
        );
    }

    #[test]
    fn strict_bigint_equality_uses_canonical_values() {
        let db = TestDb::default();
        for (left, right) in [
            ("0x10n", "16n"),
            ("-0n", "0n"),
            ("-0b10n", "-2n"),
            (
                "0xffffffffffffffffffffffffffffffffn",
                "340282366920938463463374607431768211455n",
            ),
        ] {
            let left = bigint(&db, left);
            let predicate = NarrowingPredicate::Literal(bigint(&db, right));
            assert_eq!(narrow_type(&db, left, predicate, true), left);
            assert_eq!(
                narrow_type(&db, left, predicate, false),
                InferredTypeData::NeverKeyword
            );
        }
        assert_eq!(
            narrow_type(
                &db,
                bigint(&db, "1n"),
                NarrowingPredicate::Literal(number(&db, "1")),
                true
            ),
            InferredTypeData::NeverKeyword
        );
    }

    #[test]
    fn strict_literal_equality_does_not_exclude_from_broad_primitives() {
        let db = TestDb::default();
        for (ty, literal) in [
            (InferredTypeData::BigInt, bigint(&db, "1n")),
            (InferredTypeData::Number, number(&db, "1")),
            (InferredTypeData::String, string(&db, "one")),
        ] {
            let predicate = NarrowingPredicate::Literal(literal);
            assert_eq!(narrow_type(&db, ty, predicate, true), literal);
            assert_eq!(narrow_type(&db, ty, predicate, false), ty);
        }
        let ty = union(
            &db,
            &[string(&db, "a"), string(&db, "b"), InferredTypeData::Number],
        );
        assert_eq!(
            narrow_type(&db, ty, NarrowingPredicate::Literal(string(&db, "a")), true),
            string(&db, "a")
        );
        assert_eq!(
            narrow_type(
                &db,
                ty,
                NarrowingPredicate::Literal(string(&db, "a")),
                false
            ),
            union(&db, &[string(&db, "b"), InferredTypeData::Number])
        );
    }

    #[test]
    fn loose_equality_removes_literals_only_when_they_never_equal() {
        let loose = |a: LiteralValue, b: LiteralValue| loosely_equal(&a, &b);
        use LiteralValue::{BigInt, Boolean, Null, Number, String, Undefined};
        // Values of one kind compare strictly; nullish values equal only
        // each other.
        assert!(loose(Number(1.0), Number(1.0)));
        assert!(!loose(Number(1.0), Number(2.0)));
        assert!(loose(Number(0.0), Number(-0.0)));
        assert!(!loose(Number(f64::NAN), Number(f64::NAN)));
        assert!(loose(Null, Undefined));
        assert!(!loose(Null, Boolean(false)));
        assert!(!loose(Undefined, Number(0.0)));
        // A boolean converts to `0` or `1`.
        assert!(loose(Boolean(true), Number(1.0)));
        assert!(loose(Number(0.0), Boolean(false)));
        assert!(!loose(Boolean(true), Number(0.0)));
        assert!(!loose(Boolean(false), Number(2.0)));
        // A bigint and a number compare by value.
        assert!(loose(BigInt("1n".into()), Number(1.0)));
        assert!(loose(Number(-3.0), BigInt("-3n".into())));
        assert!(!loose(BigInt("0n".into()), Number(1.0)));
        assert!(!loose(BigInt("1n".into()), Number(1.5)));
        assert!(!loose(BigInt("1n".into()), Number(f64::INFINITY)));
        assert!(!loose(BigInt("1n".into()), Number(1e40)));
        // A bigint too large to compare may equal the number.
        assert!(loose(
            BigInt("1234567890123456789012345678901234567890n".into()),
            Number(1.0)
        ));
        // A string without digits converts to `NaN`, unless it is empty,
        // white space, or `Infinity` with an optional sign.
        assert!(!loose(String("a"), Number(1.0)));
        assert!(!loose(Boolean(true), String("x")));
        assert!(!loose(String("a"), BigInt("0n".into())));
        assert!(loose(String("1"), Number(1.0)));
        assert!(loose(String("0x"), Number(0.0)));
        assert!(loose(String(""), Number(0.0)));
        assert!(loose(String(" \t\u{feff}\u{2028}"), Boolean(false)));
        assert!(loose(String(" -Infinity "), Number(f64::NEG_INFINITY)));
        assert!(loose(String("+Infinity"), Number(0.0)));
        assert!(!loose(String("infinity"), Number(0.0)));
        // Booleans and bigints, and strings with each other, aren't
        // compared by conversion.
        assert!(loose(Boolean(true), BigInt("2n".into())));
        assert!(!loose(String("a"), String("b")));
    }

    #[test]
    fn loose_literal_equality_keeps_members_of_other_kinds() {
        let db = TestDb::default();
        let one = number(&db, "1");
        let a = string(&db, "a");
        let loose = |literal| NarrowingPredicate::LooseLiteral(literal);
        // `null` and `undefined` go, and so do literals that never equal the
        // value. Primitive types of other kinds and objects stay.
        let ty = union(
            &db,
            &[
                InferredTypeData::String,
                InferredTypeData::Number,
                InferredTypeData::Null,
                InferredTypeData::Undefined,
                a,
                string(&db, "1"),
                boolean_literal(&db, true),
                InferredTypeData::ObjectKeyword,
            ],
        );
        assert_eq!(
            narrow_type(&db, ty, loose(one), true),
            union(
                &db,
                &[
                    InferredTypeData::String,
                    one,
                    string(&db, "1"),
                    boolean_literal(&db, true),
                    InferredTypeData::ObjectKeyword,
                ]
            )
        );
        // The false branch removes only the members strictly equal to the
        // value.
        let ty = union(&db, &[one, string(&db, "1"), InferredTypeData::Null]);
        assert_eq!(
            narrow_type(&db, ty, loose(one), false),
            union(&db, &[string(&db, "1"), InferredTypeData::Null])
        );
        // `boolean` becomes the literal of its kind.
        assert_eq!(
            narrow_type(
                &db,
                InferredTypeData::Boolean,
                loose(boolean_literal(&db, true)),
                true
            ),
            boolean_literal(&db, true)
        );
        // Loose equality with `null` is a nullishness test.
        let ty = union(&db, &[a, InferredTypeData::Undefined]);
        assert_eq!(
            narrow_type(&db, ty, loose(InferredTypeData::Null), true),
            InferredTypeData::Undefined
        );
        // `unknown` stays in both branches.
        for positive in [true, false] {
            assert_eq!(
                narrow_type(&db, InferredTypeData::UnknownKeyword, loose(a), positive),
                InferredTypeData::UnknownKeyword
            );
        }
    }

    #[test]
    fn equality_with_a_typed_value_keeps_the_members_that_may_equal_it() {
        let db = TestDb::default();
        let a = string(&db, "a");
        let one = number(&db, "1");
        let equal = |value, strict| NarrowingPredicate::new_equality(&db, value, strict);
        let ty = union(
            &db,
            &[
                InferredTypeData::String,
                InferredTypeData::Number,
                InferredTypeData::Null,
            ],
        );
        // Primitive types become the value's literals of their kind.
        let value = union(&db, &[a, one]);
        assert_eq!(
            narrow_type(&db, ty, equal(value, true), true),
            union(&db, &[a, one])
        );
        // A primitive type of the value keeps the member, and a type that
        // the value doesn't have goes.
        let value = union(
            &db,
            &[InferredTypeData::String, InferredTypeData::Undefined],
        );
        assert_eq!(
            narrow_type(&db, ty, equal(value, true), true),
            InferredTypeData::String
        );
        // A loose comparison keeps the members of other kinds that may
        // convert to a value, and `null` for `undefined`.
        assert_eq!(narrow_type(&db, ty, equal(value, false), true), ty);
        // The false branch narrows only by a unit type.
        assert_eq!(narrow_type(&db, ty, equal(value, true), false), ty);
        assert_eq!(
            narrow_type(&db, ty, equal(InferredTypeData::Null, true), false),
            union(&db, &[InferredTypeData::String, InferredTypeData::Number])
        );
        // An object type may hold any value other than `null` and
        // `undefined`, as a string has `{ length: number }`.
        let length = object(&db, vec![member("length", InferredTypeData::Number)]);
        assert_eq!(
            narrow_type(&db, ty, equal(length, true), true),
            union(&db, &[InferredTypeData::String, InferredTypeData::Number])
        );
        // `unknown` takes a single primitive type, `object` for an object
        // type, and stays for a union.
        let unknown = InferredTypeData::UnknownKeyword;
        assert_eq!(
            narrow_type(&db, unknown, equal(InferredTypeData::String, true), true),
            InferredTypeData::String
        );
        assert_eq!(
            narrow_type(&db, unknown, equal(length, true), true),
            InferredTypeData::ObjectKeyword
        );
        assert_eq!(narrow_type(&db, unknown, equal(value, true), true), unknown);
        assert_eq!(
            narrow_type(&db, unknown, equal(InferredTypeData::String, false), true),
            unknown
        );
    }

    #[test]
    fn strict_boolean_equality_splits_boolean() {
        let db = TestDb::default();
        for value in [true, false] {
            let predicate = NarrowingPredicate::Literal(boolean_literal(&db, value));
            assert_eq!(
                narrow_type(&db, InferredTypeData::Boolean, predicate, true),
                boolean_literal(&db, value)
            );
            assert_eq!(
                narrow_type(&db, InferredTypeData::Boolean, predicate, false),
                boolean_literal(&db, !value)
            );
        }
        let ty = union(&db, &[InferredTypeData::Boolean, InferredTypeData::Null]);
        assert_eq!(
            narrow_type(
                &db,
                ty,
                NarrowingPredicate::Literal(boolean_literal(&db, true)),
                false
            ),
            union(&db, &[boolean_literal(&db, false), InferredTypeData::Null])
        );
    }

    #[test]
    fn undecoded_and_oversized_literals_remain_conservative() {
        let db = TestDb::default();
        for literal in [
            string(&db, "\\u0041"),
            string(&db, "\\\n"),
            string(&db, &"a".repeat(MAX_LITERAL_BYTES + 1)),
            bigint(&db, &format!("{}n", "1".repeat(MAX_LITERAL_BYTES))),
            number(&db, "0x100000000000000000000000000000000"),
            InferredTypeData::Literal(InternedLiteral::new(
                &db,
                Literal::Template(Text::new_static("text")),
            )),
        ] {
            for positive in [false, true] {
                assert_eq!(
                    narrow_type(
                        &db,
                        literal,
                        NarrowingPredicate::Literal(string(&db, "A")),
                        positive
                    ),
                    literal
                );
                assert_eq!(
                    narrow_type(
                        &db,
                        InferredTypeData::String,
                        NarrowingPredicate::Literal(literal),
                        positive
                    ),
                    InferredTypeData::String
                );
                assert_eq!(
                    narrow_type(&db, literal, NarrowingPredicate::Truthy, positive),
                    literal
                );
            }
        }
    }

    #[test]
    fn uncertainty_and_never_are_preserved() {
        let db = TestDb::default();
        for ty in [InferredTypeData::Unknown, InferredTypeData::NeverKeyword] {
            for predicate in [
                NarrowingPredicate::Truthy,
                NarrowingPredicate::Nullish,
                NarrowingPredicate::Typeof(TypeofKind::Number),
                NarrowingPredicate::Literal(number(&db, "1")),
            ] {
                for positive in [false, true] {
                    assert_eq!(narrow_type(&db, ty, predicate, positive), ty);
                }
            }
        }
        for (unknown, expected) in [
            (InferredTypeData::Unknown, InferredTypeData::Unknown),
            (InferredTypeData::UnknownKeyword, empty_object(&db)),
        ] {
            let ty = InferredTypeData::Union(InternedUnion::new(
                &db,
                Box::from([unknown, InferredTypeData::Null]),
            ));
            assert_eq!(
                narrow_type(&db, ty, NarrowingPredicate::Nullish, false),
                expected
            );
        }
    }

    #[test]
    fn only_inference_failures_are_narrowing_invariant() {
        let db = TestDb::default();
        assert!(is_narrowing_invariant(InferredTypeData::Unknown));
        for ty in [
            InferredTypeData::AnyKeyword,
            InferredTypeData::UnknownKeyword,
            InferredTypeData::NeverKeyword,
            InferredTypeData::String,
            InferredTypeData::Null,
            number(&db, "1"),
        ] {
            assert!(!is_narrowing_invariant(ty), "{ty:?}");
        }
    }

    #[test]
    fn raw_narrowing_invariance_and_any_need_no_resolution() {
        let local = TypeReference::Resolved(RawTypeId::Local(crate::TypeId::new(0)));
        let alias = TypeReference::Resolved(RawTypeId::Local(crate::TypeId::new(1)));
        // Each case gives whether the reference is invariant and whether it is
        // `any`.
        for (reference, raw_types, expected) in [
            (TypeReference::unknown(), vec![], (true, false)),
            (
                TypeReference::Resolved(GLOBAL_ANY_KEYWORD_ID),
                vec![],
                (false, true),
            ),
            (local.clone(), vec![RawTypeData::AnyKeyword], (false, true)),
            (local.clone(), vec![RawTypeData::Unknown], (true, false)),
            (
                local.clone(),
                vec![RawTypeData::UnknownKeyword],
                (false, false),
            ),
            (local.clone(), vec![RawTypeData::String], (false, false)),
            // An alias of `any` is `any` only once resolved.
            (
                local.clone(),
                vec![RawTypeData::Reference(alias), RawTypeData::AnyKeyword],
                (false, false),
            ),
            (local, vec![], (false, false)),
        ] {
            assert_eq!(
                (
                    is_raw_narrowing_invariant(&reference, &raw_types),
                    is_raw_any(&reference, &raw_types),
                ),
                expected,
                "{reference:?} in {raw_types:?}"
            );
        }
    }

    #[test]
    fn discarded_return_values_do_not_prove_undefined() {
        let db = TestDb::default();
        for predicate in [
            NarrowingPredicate::Truthy,
            NarrowingPredicate::Nullish,
            NarrowingPredicate::Typeof(TypeofKind::String),
            NarrowingPredicate::Literal(InferredTypeData::Undefined),
        ] {
            for positive in [false, true] {
                assert_eq!(
                    narrow_type(&db, InferredTypeData::VoidKeyword, predicate, positive),
                    InferredTypeData::Unknown
                );
            }
        }
    }

    #[test]
    fn explicit_unknown_can_be_refined_without_refining_inference_failure() {
        let db = TestDb::default();
        for (predicate, expected) in [
            (
                NarrowingPredicate::Typeof(TypeofKind::String),
                InferredTypeData::String,
            ),
            (
                NarrowingPredicate::Literal(number(&db, "1")),
                number(&db, "1"),
            ),
        ] {
            assert_eq!(
                narrow_type(&db, InferredTypeData::UnknownKeyword, predicate, true),
                expected
            );
            assert_eq!(
                narrow_type(&db, InferredTypeData::UnknownKeyword, predicate, false),
                InferredTypeData::UnknownKeyword
            );
            assert_eq!(
                narrow_type(&db, InferredTypeData::Unknown, predicate, true),
                InferredTypeData::Unknown
            );
        }
    }

    #[test]
    fn unknown_narrows_as_an_empty_object_null_or_undefined() {
        let db = TestDb::default();
        let empty = empty_object(&db);
        let unknown = InferredTypeData::UnknownKeyword;
        let either = |types| union(&db, types);
        let null = InferredTypeData::Null;
        let undefined = InferredTypeData::Undefined;
        for (predicate, positive, expected) in [
            (NarrowingPredicate::Truthy, true, empty),
            (NarrowingPredicate::Truthy, false, unknown),
            (
                NarrowingPredicate::Nullish,
                true,
                either(&[null, undefined]),
            ),
            (NarrowingPredicate::Nullish, false, empty),
            (NarrowingPredicate::Literal(null), true, null),
            (
                NarrowingPredicate::Literal(null),
                false,
                either(&[empty, undefined]),
            ),
            (NarrowingPredicate::Literal(undefined), true, undefined),
            (
                NarrowingPredicate::Literal(undefined),
                false,
                either(&[empty, null]),
            ),
            (
                NarrowingPredicate::Typeof(TypeofKind::Object),
                true,
                either(&[InferredTypeData::ObjectKeyword, null]),
            ),
            (
                NarrowingPredicate::Typeof(TypeofKind::Object),
                false,
                either(&[empty, undefined]),
            ),
            (
                NarrowingPredicate::Typeof(TypeofKind::Function),
                true,
                function_instance(&db),
            ),
            (
                NarrowingPredicate::Typeof(TypeofKind::Function),
                false,
                unknown,
            ),
            (
                NarrowingPredicate::Typeof(TypeofKind::Undefined),
                true,
                undefined,
            ),
            (
                NarrowingPredicate::Typeof(TypeofKind::Undefined),
                false,
                either(&[empty, null]),
            ),
            (
                NarrowingPredicate::Typeof(TypeofKind::String),
                true,
                InferredTypeData::String,
            ),
            (
                NarrowingPredicate::Typeof(TypeofKind::String),
                false,
                unknown,
            ),
            (
                NarrowingPredicate::Literal(string(&db, "a")),
                true,
                string(&db, "a"),
            ),
            (
                NarrowingPredicate::Literal(string(&db, "a")),
                false,
                unknown,
            ),
        ] {
            assert_eq!(
                narrow_type(&db, unknown, predicate, positive),
                expected,
                "{predicate:?} {positive}"
            );
        }
        assert_eq!(falsy_values(&db, unknown), unknown);
    }

    #[test]
    fn any_narrows_only_by_primitive_typeof_in_the_true_branch() {
        let db = TestDb::default();
        let any = InferredTypeData::AnyKeyword;
        for kind in typeof_kinds() {
            let expected = match kind {
                TypeofKind::Object | TypeofKind::Function => any,
                kind => typeof_type(&db, kind).unwrap(),
            };
            assert_eq!(
                narrow_type(&db, any, NarrowingPredicate::Typeof(kind), true),
                expected,
                "{kind:?}"
            );
            assert_eq!(
                narrow_type(&db, any, NarrowingPredicate::Typeof(kind), false),
                any,
                "{kind:?}"
            );
        }
        for predicate in [
            NarrowingPredicate::Truthy,
            NarrowingPredicate::Nullish,
            NarrowingPredicate::Literal(InferredTypeData::Null),
            NarrowingPredicate::Literal(InferredTypeData::Undefined),
            NarrowingPredicate::Literal(string(&db, "a")),
        ] {
            for positive in [false, true] {
                assert_eq!(narrow_type(&db, any, predicate, positive), any);
            }
        }
        assert_eq!(falsy_values(&db, any), any);
    }

    #[test]
    fn unsupported_types_and_literal_predicates_remain_unchanged() {
        let db = TestDb::default();
        let wrapper = InternedTypeofType::new(&db, InferredTypeData::Number);
        let unsupported = [
            InferredTypeData::TypeofType(wrapper),
            InferredTypeData::InstanceOf(InternedTypeInstance::new(
                &db,
                InferredTypeData::Number,
                Box::default(),
            )),
            InferredTypeData::Local(LocalTypeHandle::new(
                &db,
                ModuleKey::new(wrapper.as_id()),
                LocalTypeId::new(0),
            )),
            InferredTypeData::array_class(),
            InferredTypeData::Generic(InternedGenericTypeParameter::new(
                &db,
                false,
                Some(InferredTypeData::Number),
                None,
                Text::new_static("T"),
            )),
            InferredTypeData::Intersection(InternedIntersection::new(
                &db,
                Box::from([InferredTypeData::Number, InferredTypeData::String]),
            )),
        ];
        for ty in unsupported {
            for predicate in [
                NarrowingPredicate::Truthy,
                NarrowingPredicate::Nullish,
                NarrowingPredicate::Typeof(TypeofKind::Number),
                NarrowingPredicate::Literal(number(&db, "1")),
            ] {
                for positive in [false, true] {
                    assert_eq!(narrow_type(&db, ty, predicate, positive), ty);
                    assert_eq!(
                        narrow_type(
                            &db,
                            InferredTypeData::String,
                            NarrowingPredicate::Literal(ty),
                            positive
                        ),
                        InferredTypeData::String
                    );
                }
            }
        }
    }

    fn member<'db>(name: &'static str, ty: InferredTypeData<'db>) -> TypeMember<'db> {
        TypeMember {
            kind: TypeMemberKind::Named(Text::new_static(name)),
            ty,
        }
    }

    fn optional_member<'db>(name: &'static str, ty: InferredTypeData<'db>) -> TypeMember<'db> {
        TypeMember {
            kind: TypeMemberKind::NamedOptional(Text::new_static(name)),
            ty,
        }
    }

    /// Returns an interface named `Shape` with `members`, extending `parents`.
    fn interface<'db>(
        db: &'db TestDb,
        parents: &[InferredTypeData<'db>],
        members: Vec<TypeMember<'db>>,
    ) -> InferredTypeData<'db> {
        InferredTypeData::Interface(InternedInterface::new(
            db,
            Box::default(),
            Box::<[_]>::from(parents),
            members.into_boxed_slice(),
            Text::new_static("Shape"),
            None,
            0,
        ))
    }

    /// Returns an instance of a class without a parent and with `members`.
    fn class_instance<'db>(
        db: &'db TestDb,
        members: Vec<TypeMember<'db>>,
    ) -> InferredTypeData<'db> {
        let class = InferredTypeData::Class(InternedClass::new(
            db,
            Box::default(),
            ClassParents::default(),
            Box::default(),
            members.into_boxed_slice(),
            Some(Text::new_static("K")),
            false,
            None,
            0,
        ));
        InferredTypeData::instance_of(db, class, Box::default())
    }

    /// Returns a method without parameters.
    fn method<'db>(
        db: &'db TestDb,
        name: &'static str,
        returns: InferredTypeData<'db>,
    ) -> InferredTypeData<'db> {
        InferredTypeData::Function(InternedFunction::new(
            db,
            Box::default(),
            Box::default(),
            ReturnType::Type(returns),
            false,
            Some(Text::new_static(name)),
        ))
    }

    fn function_instance(db: &TestDb) -> InferredTypeData<'_> {
        InferredTypeData::instance_of(
            db,
            InferredTypeData::GlobalType(FUNCTION_ID_GLOBAL_TYPE_ID).expand_canonical_global(db),
            Box::default(),
        )
    }

    fn typeof_kinds() -> [TypeofKind; 8] {
        [
            TypeofKind::Undefined,
            TypeofKind::Object,
            TypeofKind::Boolean,
            TypeofKind::Number,
            TypeofKind::BigInt,
            TypeofKind::String,
            TypeofKind::Symbol,
            TypeofKind::Function,
        ]
    }

    #[test]
    fn empty_object_types_become_the_tested_kind() {
        let db = TestDb::default();
        let object = empty_object(&db);
        let expression = InferredTypeData::Object(InternedObject::new(
            &db,
            None,
            Box::default(),
            false,
            ObjectKind::Expression,
        ));
        let empty_interface = interface(&db, &[], Vec::new());
        // An object expression is never a primitive, and `{}` describes it.
        let object_typeof = NarrowingPredicate::Typeof(TypeofKind::Object);
        assert_eq!(
            narrow_type(&db, expression, object_typeof, true),
            InferredTypeData::ObjectKeyword
        );
        assert_eq!(
            narrow_type(&db, expression, object_typeof, false),
            expression
        );
        for ty in [
            object,
            InferredTypeData::instance_of(&db, object, Box::default()),
            empty_interface,
        ] {
            assert_eq!(
                narrow_type(&db, ty, NarrowingPredicate::Nullish, true),
                InferredTypeData::NeverKeyword
            );
            // `""`, `0`, `false`, and `0n` have every member of `{}`.
            assert_eq!(narrow_type(&db, ty, NarrowingPredicate::Truthy, false), ty);
            for kind in typeof_kinds() {
                let expected = match kind {
                    TypeofKind::Undefined => InferredTypeData::NeverKeyword,
                    // As in TypeScript, `{}` becomes `object`, but an empty
                    // interface stays as it is.
                    TypeofKind::Object if ty == empty_interface => ty,
                    TypeofKind::Object => InferredTypeData::ObjectKeyword,
                    TypeofKind::Boolean => InferredTypeData::Boolean,
                    TypeofKind::Number => InferredTypeData::Number,
                    TypeofKind::BigInt => InferredTypeData::BigInt,
                    TypeofKind::String => InferredTypeData::String,
                    TypeofKind::Symbol => InferredTypeData::Symbol,
                    TypeofKind::Function => function_instance(&db),
                };
                assert_eq!(
                    narrow_type(&db, ty, NarrowingPredicate::Typeof(kind), true),
                    expected,
                    "{kind:?}"
                );
                assert_eq!(
                    narrow_type(&db, ty, NarrowingPredicate::Typeof(kind), false),
                    ty,
                    "{kind:?}"
                );
            }
        }
    }

    #[test]
    fn object_types_that_no_primitive_satisfies_are_removed() {
        let db = TestDb::default();
        let literal = InferredTypeData::Object(InternedObject::new(
            &db,
            None,
            Box::from([member("a", InferredTypeData::Number)]),
            false,
            ObjectKind::Shape,
        ));
        for ty in [
            literal,
            interface(&db, &[], vec![member("a", InferredTypeData::Number)]),
            class_instance(&db, vec![member("k", InferredTypeData::Number)]),
            // A missing member is conclusive even when inherited members
            // aren't listed.
            interface(&db, &[literal], vec![member("b", InferredTypeData::String)]),
            // Only optional members, none of which a primitive or a function
            // has.
            interface(
                &db,
                &[],
                vec![optional_member("a", InferredTypeData::Number)],
            ),
        ] {
            assert_eq!(
                narrow_type(&db, ty, NarrowingPredicate::Truthy, false),
                InferredTypeData::NeverKeyword
            );
            assert_eq!(narrow_type(&db, ty, NarrowingPredicate::Truthy, true), ty);
            assert_eq!(
                falsy_values(&db, union(&db, &[ty, InferredTypeData::Undefined])),
                InferredTypeData::Undefined
            );
            for kind in typeof_kinds() {
                let removed = kind != TypeofKind::Object;
                assert_eq!(
                    narrow_type(&db, ty, NarrowingPredicate::Typeof(kind), true),
                    if removed {
                        InferredTypeData::NeverKeyword
                    } else {
                        ty
                    },
                    "{kind:?}"
                );
                assert_eq!(
                    narrow_type(&db, ty, NarrowingPredicate::Typeof(kind), false),
                    if removed {
                        ty
                    } else {
                        InferredTypeData::NeverKeyword
                    },
                    "{kind:?}"
                );
            }
        }
        // No primitive and no function has the members of `Date`, such as
        // `getTime`.
        let date = InferredTypeData::instance_of(
            &db,
            InferredTypeData::GlobalType(
                crate::generated::global_types::ids::DATE_ID_GLOBAL_TYPE_ID,
            ),
            Box::default(),
        );
        assert_eq!(
            narrow_type(
                &db,
                date,
                NarrowingPredicate::Typeof(TypeofKind::Function),
                true
            ),
            InferredTypeData::NeverKeyword
        );
        assert_eq!(
            narrow_type(
                &db,
                date,
                NarrowingPredicate::Typeof(TypeofKind::Object),
                false
            ),
            InferredTypeData::NeverKeyword
        );
        assert_eq!(
            narrow_type(
                &db,
                date,
                NarrowingPredicate::Typeof(TypeofKind::Object),
                true
            ),
            date
        );
    }

    #[test]
    fn object_types_that_a_primitive_satisfies_are_kept_or_replaced() {
        let db = TestDb::default();
        let function = function_instance(&db);
        // Every string and every function has `length`, but no number does.
        let length = interface(&db, &[], vec![member("length", InferredTypeData::Number)]);
        let fixed = interface(
            &db,
            &[],
            vec![member(
                "toFixed",
                method(&db, "toFixed", InferredTypeData::String),
            )],
        );
        let optional_length = interface(
            &db,
            &[],
            vec![optional_member("length", InferredTypeData::Number)],
        );
        let cases = [
            (length, TypeofKind::String, InferredTypeData::String),
            (length, TypeofKind::Number, InferredTypeData::NeverKeyword),
            (length, TypeofKind::Function, function),
            (fixed, TypeofKind::Number, InferredTypeData::Number),
            (fixed, TypeofKind::String, InferredTypeData::NeverKeyword),
            (
                optional_length,
                TypeofKind::String,
                InferredTypeData::String,
            ),
        ];
        for (ty, kind, expected) in cases {
            assert_eq!(
                narrow_type(&db, ty, NarrowingPredicate::Typeof(kind), true),
                expected,
                "{kind:?}"
            );
        }
        for ty in [length, fixed, optional_length] {
            assert_eq!(narrow_type(&db, ty, NarrowingPredicate::Truthy, false), ty);
            assert_eq!(
                narrow_type(
                    &db,
                    ty,
                    NarrowingPredicate::Typeof(TypeofKind::Object),
                    false
                ),
                ty
            );
        }
        // In a union, the true branch of `typeof` with `"function"` removes
        // `string` and replaces `{ length: number }` with `Function`.
        let ty = union(&db, &[length, InferredTypeData::String]);
        assert_eq!(
            narrow_type(
                &db,
                ty,
                NarrowingPredicate::Typeof(TypeofKind::Function),
                true
            ),
            function
        );
    }

    #[test]
    fn undecided_comparisons_keep_object_types() {
        let db = TestDb::default();
        let cases = [
            // `padStart` comes from ES2017 and `description` from ES2019, so
            // their absence from the generated wrappers proves nothing.
            (
                interface(
                    &db,
                    &[],
                    vec![member(
                        "padStart",
                        method(&db, "padStart", InferredTypeData::String),
                    )],
                ),
                TypeofKind::String,
            ),
            (
                interface(
                    &db,
                    &[],
                    vec![optional_member("description", InferredTypeData::String)],
                ),
                TypeofKind::Symbol,
            ),
            // The generated globals have no `BigInt` to compare `toString` with.
            (
                interface(
                    &db,
                    &[],
                    vec![member(
                        "toString",
                        method(&db, "toString", InferredTypeData::String),
                    )],
                ),
                TypeofKind::BigInt,
            ),
            // A mismatched member type is not conclusive.
            (
                interface(&db, &[], vec![member("length", InferredTypeData::String)]),
                TypeofKind::String,
            ),
            // A class can declare private members, which no string has.
            (
                class_instance(&db, vec![member("length", InferredTypeData::Number)]),
                TypeofKind::String,
            ),
        ];
        for (ty, kind) in cases {
            assert_eq!(
                narrow_type(&db, ty, NarrowingPredicate::Typeof(kind), true),
                ty,
                "{kind:?}"
            );
        }
    }

    #[test]
    fn callable_object_types_stay_in_function_branches() {
        let db = TestDb::default();
        let signature = TypeMember {
            kind: TypeMemberKind::CallSignature,
            ty: method(&db, "call", InferredTypeData::VoidKeyword),
        };
        let callable = interface(&db, &[], vec![signature.clone()]);
        for ty in [
            interface(
                &db,
                &[],
                vec![signature, member("foo", InferredTypeData::String)],
            ),
            // The call signature is inherited.
            interface(
                &db,
                &[callable],
                vec![member("foo", InferredTypeData::String)],
            ),
        ] {
            assert_eq!(
                narrow_type(
                    &db,
                    ty,
                    NarrowingPredicate::Typeof(TypeofKind::Function),
                    true
                ),
                ty
            );
            assert_eq!(
                narrow_type(
                    &db,
                    ty,
                    NarrowingPredicate::Typeof(TypeofKind::Object),
                    false
                ),
                ty
            );
            assert_eq!(
                narrow_type(
                    &db,
                    ty,
                    NarrowingPredicate::Typeof(TypeofKind::String),
                    true
                ),
                InferredTypeData::NeverKeyword
            );
        }
        // Only functions have call signatures.
        assert_eq!(
            narrow_type(
                &db,
                callable,
                NarrowingPredicate::Typeof(TypeofKind::String),
                true
            ),
            InferredTypeData::NeverKeyword
        );
    }

    #[test]
    fn object_types_are_resolved_but_generics_stay_unchanged() {
        let db = TestDb::default();
        let wrapper = InternedTypeofType::new(&db, InferredTypeData::Number);
        let handle = InferredTypeData::Local(LocalTypeHandle::new(
            &db,
            ModuleKey::new(wrapper.as_id()),
            LocalTypeId::new(0),
        ));
        let class = InferredTypeData::Class(InternedClass::new(
            &db,
            Box::default(),
            ClassParents::default(),
            Box::default(),
            Box::from([member("k", InferredTypeData::Number)]),
            Some(Text::new_static("K")),
            false,
            None,
            0,
        ));
        let ty = union(
            &db,
            &[
                InferredTypeData::instance_of(&db, handle, Box::default()),
                InferredTypeData::Undefined,
            ],
        );
        let resolve = |ty| if ty == handle { class } else { ty };
        assert_eq!(
            super::narrow_type(&db, ty, NarrowingPredicate::Truthy, false, resolve),
            InferredTypeData::Undefined
        );
        assert_eq!(narrow_type(&db, ty, NarrowingPredicate::Truthy, false), ty);

        let generic = InferredTypeData::Generic(InternedGenericTypeParameter::new(
            &db,
            false,
            Some(interface(
                &db,
                &[],
                vec![member("a", InferredTypeData::Number)],
            )),
            None,
            Text::new_static("T"),
        ));
        for ty in [
            generic,
            InferredTypeData::instance_of(&db, generic, Box::default()),
        ] {
            assert_eq!(narrow_type(&db, ty, NarrowingPredicate::Truthy, false), ty);
            for kind in typeof_kinds() {
                assert_eq!(
                    narrow_type(&db, ty, NarrowingPredicate::Typeof(kind), true),
                    ty
                );
            }
        }
    }

    #[test]
    fn only_object_types_call_the_resolver() {
        let db = TestDb::default();
        let foo = interface(&db, &[], vec![member("a", InferredTypeData::Number)]);
        let primitives = union(
            &db,
            &[
                InferredTypeData::String,
                InferredTypeData::Number,
                InferredTypeData::Boolean,
                InferredTypeData::BigInt,
                InferredTypeData::Symbol,
                InferredTypeData::Null,
                InferredTypeData::Undefined,
                number(&db, "0"),
                string(&db, ""),
                method(&db, "f", InferredTypeData::VoidKeyword),
            ],
        );
        let mut calls = 0;
        let mut predicates = vec![
            NarrowingPredicate::Truthy,
            NarrowingPredicate::Nullish,
            NarrowingPredicate::Literal(number(&db, "1")),
        ];
        predicates.extend(typeof_kinds().map(NarrowingPredicate::Typeof));
        for predicate in predicates {
            for positive in [false, true] {
                super::narrow_type(&db, primitives, predicate, positive, |ty| {
                    calls += 1;
                    ty
                });
            }
        }
        super::falsy_values(&db, primitives, |ty| {
            calls += 1;
            ty
        });
        // Tests that never compare object types with primitives.
        for (predicate, positive) in [
            (NarrowingPredicate::Truthy, true),
            (NarrowingPredicate::Nullish, true),
            (NarrowingPredicate::Nullish, false),
            (NarrowingPredicate::Typeof(TypeofKind::String), false),
            (NarrowingPredicate::Typeof(TypeofKind::Object), true),
            (NarrowingPredicate::Literal(number(&db, "1")), false),
        ] {
            super::narrow_type(&db, foo, predicate, positive, |ty| {
                calls += 1;
                ty
            });
        }
        assert_eq!(calls, 0);

        super::narrow_type(&db, foo, NarrowingPredicate::Truthy, false, |ty| {
            calls += 1;
            ty
        });
        assert_ne!(calls, 0);
    }

    #[test]
    fn objects_from_object_expressions_are_never_primitive() {
        let db = TestDb::default();
        let object = InferredTypeData::Object(InternedObject::new(
            &db,
            None,
            Box::default(),
            false,
            ObjectKind::Expression,
        ));
        for (predicate, positive) in [
            (NarrowingPredicate::Nullish, true),
            (NarrowingPredicate::Truthy, false),
            (NarrowingPredicate::Typeof(TypeofKind::String), true),
        ] {
            assert_eq!(
                narrow_type(&db, object, predicate, positive),
                InferredTypeData::NeverKeyword
            );
            assert_eq!(narrow_type(&db, object, predicate, !positive), object);
        }
    }

    #[test]
    fn union_budget_is_shared_across_depth_and_width() {
        let db = TestDb::default();
        for count in [
            MAX_NARROWING_STEPS - 1,
            MAX_NARROWING_STEPS,
            MAX_NARROWING_STEPS + 1,
        ] {
            let deep = (1..count).fold(InferredTypeData::Null, |ty, _| {
                InferredTypeData::Union(InternedUnion::new(&db, Box::from([ty])))
            });
            let wide = InferredTypeData::Union(InternedUnion::new(
                &db,
                vec![InferredTypeData::Null; count - 1].into_boxed_slice(),
            ));
            for ty in [deep, wide] {
                assert_eq!(
                    narrow_type(&db, ty, NarrowingPredicate::Truthy, true),
                    if count <= MAX_NARROWING_STEPS {
                        InferredTypeData::NeverKeyword
                    } else {
                        ty
                    }
                );
            }
        }
        let repeated = (0..32).fold(InferredTypeData::Null, |ty, _| {
            InferredTypeData::Union(InternedUnion::new(&db, Box::from([ty, ty])))
        });
        assert_eq!(
            narrow_type(&db, repeated, NarrowingPredicate::Truthy, true),
            repeated
        );
    }

    /// Returns an object type literal with `members`.
    fn object<'db>(db: &'db TestDb, members: Vec<TypeMember<'db>>) -> InferredTypeData<'db> {
        InferredTypeData::Object(InternedObject::new(
            db,
            None,
            members.into_boxed_slice(),
            false,
            ObjectKind::Shape,
        ))
    }

    /// Returns `{ kind: kind, value: value }`.
    fn shape<'db>(
        db: &'db TestDb,
        kind: InferredTypeData<'db>,
        value: InferredTypeData<'db>,
    ) -> InferredTypeData<'db> {
        object(db, vec![member("kind", kind), member("value", value)])
    }

    /// Narrows `ty`, whose declared type is `declared`, by `predicate`
    /// applied to its property `name`, with a resolver that returns its input.
    fn narrow_property<'db>(
        db: &'db dyn TypeDb,
        ty: InferredTypeData<'db>,
        declared: InferredTypeData<'db>,
        name: &str,
        predicate: NarrowingPredicate<'db>,
        positive: bool,
    ) -> InferredTypeData<'db> {
        let test = PropertyTest {
            name,
            predicate,
            positive,
            optional_chain: false,
        };
        super::narrow_by_property(db, ty, declared, test, |ty| ty)
    }

    #[test]
    fn discriminants_keep_the_members_whose_property_passes_the_test() {
        let db = TestDb::default();
        let circle = shape(&db, string(&db, "c"), InferredTypeData::Number);
        let square = shape(&db, string(&db, "s"), InferredTypeData::String);
        let shapes = union(&db, &[circle, square]);
        let literal = NarrowingPredicate::Literal(string(&db, "c"));
        for (predicate, positive, expected) in [
            (literal, true, circle),
            (literal, false, square),
            (NarrowingPredicate::Typeof(TypeofKind::String), true, shapes),
            (
                NarrowingPredicate::Typeof(TypeofKind::String),
                false,
                InferredTypeData::NeverKeyword,
            ),
            (NarrowingPredicate::Truthy, true, shapes),
            (
                NarrowingPredicate::Literal(string(&db, "x")),
                true,
                InferredTypeData::NeverKeyword,
            ),
        ] {
            assert_eq!(
                narrow_property(&db, shapes, shapes, "kind", predicate, positive),
                expected,
                "{predicate:?} {positive}"
            );
        }

        // A single member left by earlier tests is checked against the
        // declared union.
        assert_eq!(
            narrow_property(&db, circle, shapes, "kind", literal, true),
            circle
        );
        assert_eq!(
            narrow_property(&db, circle, shapes, "kind", literal, false),
            InferredTypeData::NeverKeyword
        );

        // A broad property type shares a value with every literal of its
        // kind, so `kind !== "c"` keeps both members.
        let broad = shape(&db, InferredTypeData::String, InferredTypeData::Number);
        let mixed = union(&db, &[circle, broad]);
        for positive in [true, false] {
            assert_eq!(
                narrow_property(&db, mixed, mixed, "kind", literal, positive),
                mixed,
                "{positive}"
            );
        }

        // `boolean` is a unit type for discriminants, and `true` shares a
        // value with it.
        let flag = shape(&db, InferredTypeData::Boolean, InferredTypeData::Number);
        let text = shape(&db, InferredTypeData::String, InferredTypeData::Number);
        let flags = union(&db, &[flag, text]);
        assert_eq!(
            narrow_property(
                &db,
                flags,
                flags,
                "kind",
                NarrowingPredicate::Literal(boolean_literal(&db, true)),
                true
            ),
            flag
        );

        // An optional property also has the value `undefined`.
        let optional = object(&db, vec![optional_member("kind", string(&db, "c"))]);
        let optionals = union(&db, &[optional, square]);
        assert_eq!(
            narrow_property(
                &db,
                optionals,
                optionals,
                "kind",
                NarrowingPredicate::Literal(InferredTypeData::Undefined),
                true
            ),
            optional
        );
    }

    #[test]
    fn properties_that_are_not_discriminants_leave_the_type_unchanged() {
        let db = TestDb::default();
        let literal = NarrowingPredicate::Literal(string(&db, "c"));
        let circle = shape(&db, string(&db, "c"), InferredTypeData::Number);
        let square = shape(&db, string(&db, "s"), InferredTypeData::String);
        let generic = InferredTypeData::Generic(InternedGenericTypeParameter::new(
            &db,
            false,
            None,
            None,
            Text::new_static("T"),
        ));
        let index = InferredTypeData::Interface(InternedInterface::new(
            &db,
            Box::default(),
            Box::default(),
            Box::from([TypeMember {
                kind: TypeMemberKind::IndexSignature(InferredTypeData::String),
                ty: string(&db, "c"),
            }]),
            Text::new_static("Index"),
            None,
            0,
        ));
        let cases = [
            // No member has a unit property type.
            union(
                &db,
                &[
                    shape(&db, InferredTypeData::String, InferredTypeData::Number),
                    shape(&db, InferredTypeData::Number, InferredTypeData::Number),
                ],
            ),
            // A generic property type.
            union(
                &db,
                &[circle, shape(&db, generic, InferredTypeData::Number)],
            ),
            // A member without the property.
            union(
                &db,
                &[
                    circle,
                    object(&db, vec![member("r", InferredTypeData::Number)]),
                ],
            ),
            // A member with an index signature.
            union(&db, &[circle, index]),
            // A primitive member.
            union(&db, &[circle, InferredTypeData::String]),
            // A nullish member, without an optional chain.
            union(&db, &[circle, square, InferredTypeData::Undefined]),
        ];
        for ty in cases {
            for positive in [true, false] {
                assert_eq!(
                    narrow_property(&db, ty, ty, "kind", literal, positive),
                    ty,
                    "{ty:?} {positive}"
                );
            }
        }
        // A value whose declared type is not a union is never narrowed.
        assert_eq!(
            narrow_property(&db, circle, circle, "kind", literal, false),
            circle
        );
        // Undecodable literals leave every type unchanged.
        let shapes = union(&db, &[circle, square]);
        assert_eq!(
            narrow_property(
                &db,
                shapes,
                shapes,
                "kind",
                NarrowingPredicate::Literal(string(&db, "\\u0063")),
                true
            ),
            shapes
        );
    }

    #[test]
    fn primitive_and_missing_members_of_the_declared_type_do_not_count() {
        let db = TestDb::default();
        let literal = NarrowingPredicate::Literal(string(&db, "c"));
        let circle = shape(&db, string(&db, "c"), InferredTypeData::Number);
        let square = shape(&db, string(&db, "s"), InferredTypeData::String);
        let other = object(&db, vec![member("r", InferredTypeData::Number)]);
        let shapes = union(&db, &[circle, square]);
        for declared in [
            union(&db, &[circle, square, InferredTypeData::String]),
            union(&db, &[circle, square, other]),
        ] {
            assert_eq!(
                narrow_property(&db, shapes, declared, "kind", literal, true),
                circle,
                "{declared:?}"
            );
        }
    }

    #[test]
    fn nullish_members_are_kept_through_optional_chains() {
        let db = TestDb::default();
        let circle = shape(&db, string(&db, "c"), InferredTypeData::Number);
        let square = shape(&db, string(&db, "s"), InferredTypeData::String);
        let nullable = union(&db, &[circle, square, InferredTypeData::Undefined]);
        let chained = |predicate, positive| {
            let test = PropertyTest {
                name: "kind",
                predicate,
                positive,
                optional_chain: true,
            };
            super::narrow_by_property(&db, nullable, nullable, test, |ty| ty)
        };
        let literal = NarrowingPredicate::Literal(string(&db, "c"));
        assert_eq!(
            chained(literal, true),
            union(&db, &[circle, InferredTypeData::Undefined])
        );
        assert_eq!(
            chained(literal, false),
            union(&db, &[square, InferredTypeData::Undefined])
        );
        // The chain gives `undefined` when the object is nullish.
        assert_eq!(
            chained(
                NarrowingPredicate::Literal(InferredTypeData::Undefined),
                true
            ),
            InferredTypeData::Undefined
        );
    }

    #[test]
    fn tuple_elements_are_properties_named_by_their_index() {
        let db = &TestDb::default();
        /// Returns a tuple of `elements`, each with whether it is optional.
        fn tuple<'db>(
            db: &'db TestDb,
            elements: &[(InferredTypeData<'db>, bool)],
        ) -> InferredTypeData<'db> {
            InferredTypeData::Tuple(InternedTuple::new(
                db,
                elements
                    .iter()
                    .map(|(ty, is_optional)| TupleElementType {
                        ty: *ty,
                        name: None,
                        is_optional: *is_optional,
                        is_rest: false,
                    })
                    .collect::<Box<[_]>>(),
                false,
            ))
        }
        let first = tuple(
            db,
            &[(string(db, "a"), false), (InferredTypeData::Number, false)],
        );
        let second = tuple(
            db,
            &[(string(db, "b"), false), (InferredTypeData::Number, true)],
        );
        let tuples = union(db, &[first, second]);
        let literal = NarrowingPredicate::Literal(string(db, "a"));
        assert_eq!(
            narrow_property(db, tuples, tuples, "0", literal, true),
            first
        );
        assert_eq!(
            narrow_property(db, tuples, tuples, "0", literal, false),
            second
        );
        // `length`, a key past the last element, and a key that is not the
        // canonical spelling of an index leave the union unchanged.
        for name in ["length", "2", "00"] {
            assert_eq!(
                narrow_property(db, tuples, tuples, name, literal, true),
                tuples,
                "{name}"
            );
        }
    }

    /// Returns `is_optional` and `is_readonly` of the property `name` of `ty`,
    /// or `None` when [`find_property`] reports it missing or unknown.
    fn property_modifiers<'db>(
        db: &'db TestDb,
        ty: InferredTypeData<'db>,
        name: &str,
    ) -> Option<(bool, bool)> {
        match find_property(db, ty, name, &mut |ty| ty) {
            PropertyLookup::Found {
                is_optional,
                is_readonly,
                ..
            } => Some((is_optional, is_readonly)),
            PropertyLookup::Missing | PropertyLookup::Unknown => None,
        }
    }

    #[test]
    fn find_property_reports_readonly_properties() {
        let db = &TestDb::default();
        let number = InferredTypeData::Number;
        let kinds = [
            TypeMemberKind::ReadonlyNamed(Text::new_static("readonly")),
            TypeMemberKind::ReadonlyNamedOptional(Text::new_static("readonlyOptional")),
            TypeMemberKind::Named(Text::new_static("mutable")),
            TypeMemberKind::NamedOptional(Text::new_static("mutableOptional")),
            TypeMemberKind::ConstAssertedNamed(Text::new_static("constant")),
            TypeMemberKind::ConstAssertedNamedOptional(Text::new_static("constantOptional")),
            TypeMemberKind::Getter(Text::new_static("getter")),
            TypeMemberKind::ConstAssertedGetter(Text::new_static("constantGetter")),
        ];
        let members = kinds
            .into_iter()
            .map(|kind| TypeMember {
                ty: if matches!(
                    kind,
                    TypeMemberKind::Getter(_) | TypeMemberKind::ConstAssertedGetter(_)
                ) {
                    method(db, "get", number)
                } else {
                    number
                },
                kind,
            })
            .collect::<Vec<_>>();
        let expected = [
            ("readonly", Some((false, true))),
            ("readonlyOptional", Some((true, true))),
            ("mutable", Some((false, false))),
            ("mutableOptional", Some((true, false))),
            ("constant", Some((false, true))),
            ("constantOptional", Some((true, true))),
            // Biome doesn't collect setters, so a getter may have one.
            ("getter", Some((false, false))),
            ("constantGetter", Some((false, false))),
            // A member of `Object`.
            ("toString", Some((false, false))),
            ("missing", None),
        ];
        let shapes = [
            object(db, members.clone()),
            interface(db, &[], members.clone()),
            // Inherited members keep their modifiers.
            interface(db, &[interface(db, &[], members.clone())], Vec::new()),
            class_instance(db, members),
        ];
        for ty in shapes {
            for (name, modifiers) in expected {
                assert_eq!(property_modifiers(db, ty, name), modifiers, "{name}");
            }
        }
    }

    #[test]
    fn find_property_reports_elements_of_readonly_tuples_as_readonly() {
        let db = &TestDb::default();
        let tuple = InferredTypeData::Tuple(InternedTuple::new(
            db,
            Box::from([
                TupleElementType {
                    ty: InferredTypeData::String,
                    name: None,
                    is_optional: false,
                    is_rest: false,
                },
                TupleElementType {
                    ty: InferredTypeData::Number,
                    name: None,
                    is_optional: true,
                    is_rest: false,
                },
            ]),
            false,
        ));
        let readonly = InferredTypeData::TypeOperator(InternedTypeOperatorType::new(
            db,
            tuple,
            crate::TypeOperator::Readonly,
        ));
        assert_eq!(property_modifiers(db, tuple, "0"), Some((false, false)));
        assert_eq!(property_modifiers(db, tuple, "1"), Some((true, false)));
        assert_eq!(property_modifiers(db, readonly, "0"), Some((false, true)));
        assert_eq!(property_modifiers(db, readonly, "1"), Some((true, true)));
        assert_eq!(property_modifiers(db, readonly, "2"), None);
    }

    fn has_readonly_v<'db>(db: &'db TestDb, ty: InferredTypeData<'db>) -> bool {
        super::has_readonly_property(db, ty, "v", |ty| ty)
    }

    #[test]
    fn readonly_properties_must_be_readonly_in_every_member() {
        let db = &TestDb::default();
        let readonly = |kind: fn(Text) -> TypeMemberKind<'static>| TypeMember {
            kind: kind(Text::new_static("v")),
            ty: InferredTypeData::String,
        };
        let declared = object(db, vec![readonly(TypeMemberKind::ReadonlyNamed)]);
        let optional = object(db, vec![readonly(TypeMemberKind::ReadonlyNamedOptional)]);
        let constant = object(db, vec![readonly(TypeMemberKind::ConstAssertedNamed)]);
        let mutable = object(db, vec![member("v", InferredTypeData::String)]);
        let getter = object(
            db,
            vec![TypeMember {
                kind: TypeMemberKind::Getter(Text::new_static("v")),
                ty: method(db, "get", InferredTypeData::String),
            }],
        );
        for (ty, expected) in [
            (declared, true),
            (optional, true),
            (constant, true),
            (
                interface(db, &[], vec![readonly(TypeMemberKind::ReadonlyNamed)]),
                true,
            ),
            (
                class_instance(db, vec![readonly(TypeMemberKind::ReadonlyNamed)]),
                true,
            ),
            // `null` and `undefined` have no property to change.
            (
                union(
                    db,
                    &[
                        declared,
                        InferredTypeData::Null,
                        InferredTypeData::Undefined,
                    ],
                ),
                true,
            ),
            (union(db, &[declared, constant]), true),
            // TypeScript takes the property as readonly when one member of a
            // union declares it `readonly`, but a write through another
            // member can change it.
            (union(db, &[declared, mutable]), false),
            (mutable, false),
            // A setter may change a getter's value.
            (getter, false),
            (
                union(db, &[InferredTypeData::Null, InferredTypeData::Undefined]),
                false,
            ),
            (object(db, Vec::new()), false),
            (union(db, &[declared, InferredTypeData::String]), false),
            (InferredTypeData::Unknown, false),
            (InferredTypeData::UnknownKeyword, false),
        ] {
            assert_eq!(has_readonly_v(db, ty), expected, "{ty:?}");
        }
    }

    #[test]
    fn intersections_have_readonly_properties_only_when_every_operand_does() {
        let db = &TestDb::default();
        let readonly = |kind: fn(Text) -> TypeMemberKind<'static>| TypeMember {
            kind: kind(Text::new_static("v")),
            ty: InferredTypeData::String,
        };
        let declared = object(db, vec![readonly(TypeMemberKind::ReadonlyNamed)]);
        let constant = object(db, vec![readonly(TypeMemberKind::ConstAssertedNamed)]);
        let mutable = object(db, vec![member("v", InferredTypeData::String)]);
        let other = object(db, vec![member("w", InferredTypeData::Number)]);
        for (operands, expected) in [
            (vec![declared, other], true),
            (vec![other, declared], true),
            (vec![declared, constant], true),
            (vec![declared, mutable], false),
            (vec![mutable, declared], false),
            (vec![constant, mutable], false),
        ] {
            let ty = InferredTypeData::intersection_from_types(db, operands);
            assert_eq!(has_readonly_v(db, ty), expected, "{ty:?}");
        }
    }

    #[test]
    fn inherited_readonly_properties_do_not_count() {
        let db = &TestDb::default();
        let parent = interface(
            db,
            &[],
            vec![TypeMember {
                kind: TypeMemberKind::ReadonlyNamed(Text::new_static("v")),
                ty: InferredTypeData::String,
            }],
        );
        // Another declaration of the interface, or a module augmentation, can
        // declare `v` without `readonly` on the child itself.
        assert!(!has_readonly_v(db, interface(db, &[parent], Vec::new())));
        assert!(matches!(
            find_property(db, interface(db, &[parent], Vec::new()), "v", &mut |ty| ty),
            PropertyLookup::Found {
                is_readonly: true,
                is_inherited: true,
                ..
            }
        ));
        assert!(matches!(
            find_property(db, parent, "v", &mut |ty| ty),
            PropertyLookup::Found {
                is_readonly: true,
                is_inherited: false,
                ..
            }
        ));
        // A member of `Object` is inherited too.
        assert!(matches!(
            find_property(db, parent, "toString", &mut |ty| ty),
            PropertyLookup::Found {
                is_inherited: true,
                ..
            }
        ));
    }

    #[test]
    fn readonly_tuples_have_readonly_elements() {
        let db = &TestDb::default();
        let tuple = InferredTypeData::Tuple(InternedTuple::new(
            db,
            Box::from([TupleElementType {
                ty: InferredTypeData::String,
                name: None,
                is_optional: false,
                is_rest: false,
            }]),
            false,
        ));
        let readonly = InferredTypeData::TypeOperator(InternedTypeOperatorType::new(
            db,
            tuple,
            crate::TypeOperator::Readonly,
        ));
        assert!(super::has_readonly_property(db, readonly, "0", |ty| ty));
        assert!(!super::has_readonly_property(db, tuple, "0", |ty| ty));
        assert!(!super::has_readonly_property(db, readonly, "1", |ty| ty));
    }

    /// Narrows `ty` by `name in ty`, with a resolver that returns its input.
    fn narrow_in<'db>(
        db: &'db dyn TypeDb,
        ty: InferredTypeData<'db>,
        name: &str,
        positive: bool,
    ) -> InferredTypeData<'db> {
        super::narrow_by_in_operator(db, ty, &Text::from(name.to_owned()), positive, |ty| ty)
    }

    /// Returns an index signature member whose key has type `key`.
    fn index_signature<'db>(key: InferredTypeData<'db>) -> TypeMember<'db> {
        TypeMember {
            kind: TypeMemberKind::IndexSignature(key),
            ty: InferredTypeData::Number,
        }
    }

    #[test]
    fn in_keeps_the_members_that_declare_the_property() {
        let db = &TestDb::default();
        let a = object(db, vec![member("a", InferredTypeData::Number)]);
        let b = object(db, vec![member("b", InferredTypeData::Number)]);
        let optional = object(db, vec![optional_member("a", InferredTypeData::Number)]);
        let dict = object(db, vec![index_signature(InferredTypeData::String)]);
        let numbers = object(db, vec![index_signature(InferredTypeData::Number)]);
        let inherited = interface(db, &[interface(db, &[], vec![member("a", b)])], vec![]);
        for (ty, name, positive, expected) in [
            (union(db, &[a, b]), "a", true, a),
            (union(db, &[a, b]), "a", false, b),
            (union(db, &[optional, b]), "a", true, optional),
            (
                union(db, &[optional, b]),
                "a",
                false,
                union(db, &[optional, b]),
            ),
            (union(db, &[dict, b]), "a", true, dict),
            (union(db, &[dict, b]), "a", false, union(db, &[dict, b])),
            (union(db, &[numbers, b]), "0", true, numbers),
            (union(db, &[inherited, b]), "a", true, inherited),
            (union(db, &[inherited, b]), "a", false, b),
            // `null` and `undefined` have no property.
            (
                union(db, &[a, b, InferredTypeData::Undefined]),
                "a",
                true,
                a,
            ),
            (
                union(db, &[a, b, InferredTypeData::Null]),
                "a",
                false,
                union(db, &[b, InferredTypeData::Null]),
            ),
            // Every object has the members of `Object`.
            (union(db, &[a, b]), "toString", true, union(db, &[a, b])),
            (
                union(db, &[a, dict]),
                "toString",
                false,
                InferredTypeData::NeverKeyword,
            ),
        ] {
            assert_eq!(
                narrow_in(db, ty, name, positive),
                expected,
                "{name} {positive}"
            );
        }
    }

    #[test]
    fn in_intersects_with_the_property_when_every_member_lacks_it() {
        let db = &TestDb::default();
        let a = object(db, vec![member("a", InferredTypeData::Number)]);
        let b = object(db, vec![member("b", InferredTypeData::Number)]);
        let numbers = object(db, vec![index_signature(InferredTypeData::Number)]);
        let record = object(db, vec![member("c", InferredTypeData::UnknownKeyword)]);
        let generic = InferredTypeData::Generic(InternedGenericTypeParameter::new(
            db,
            false,
            None,
            None,
            Text::new_static("T"),
        ));
        let constrained = InferredTypeData::Generic(InternedGenericTypeParameter::new(
            db,
            false,
            Some(b),
            None,
            Text::new_static("U"),
        ));
        for ty in [
            union(db, &[a, b]),
            // A numeric index signature accepts only numeric names.
            union(db, &[numbers, b]),
            InferredTypeData::ObjectKeyword,
            object(db, Vec::new()),
            generic,
            // The constraint of `U` lacks the property too.
            constrained,
        ] {
            assert_eq!(
                narrow_in(db, ty, "c", true),
                super::intersect_narrowed(db, ty, record),
                "{ty:?}"
            );
            assert_eq!(narrow_in(db, ty, "c", false), ty);
        }
        // `{}` is `Record<"c", unknown>` in the intersection.
        assert_eq!(narrow_in(db, object(db, Vec::new()), "c", true), record);
    }

    #[test]
    fn in_keeps_the_type_when_a_member_may_have_the_property() {
        let db = &TestDb::default();
        let b = object(db, vec![member("b", InferredTypeData::Number)]);
        let class = class_instance(db, vec![member("p", InferredTypeData::Number)]);
        let constrained = InferredTypeData::Generic(InternedGenericTypeParameter::new(
            db,
            false,
            Some(object(db, vec![member("c", InferredTypeData::Number)])),
            None,
            Text::new_static("T"),
        ));
        for (ty, name) in [
            // A class can have a property that Biome doesn't collect.
            (union(db, &[class, b]), "c"),
            // The constraint of `T` declares the property.
            (constrained, "c"),
            // Every object has the members of `Object`.
            (InferredTypeData::ObjectKeyword, "toString"),
        ] {
            assert_eq!(narrow_in(db, ty, name, true), ty, "{name}");
            assert_eq!(narrow_in(db, ty, name, false), ty, "{name}");
        }
        // The `#` of a private name is part of the name Biome gives it.
        let private = object(db, vec![member("#a", InferredTypeData::Number)]);
        assert_eq!(
            narrow_in(db, union(db, &[private, b]), "#a", true),
            union(db, &[private, b])
        );
    }

    #[test]
    fn in_keeps_the_members_whose_property_biome_cannot_tell() {
        let db = &TestDb::default();
        let a = object(db, vec![member("a", InferredTypeData::Number)]);
        let class = class_instance(db, vec![member("p", InferredTypeData::Number)]);
        let error = InferredTypeData::instance_of(
            db,
            InferredTypeData::GlobalType(ERROR_ID_GLOBAL_TYPE_ID),
            Box::default(),
        );
        let callable = interface(
            db,
            &[],
            vec![TypeMember {
                kind: TypeMemberKind::CallSignature,
                ty: method(db, "call", InferredTypeData::VoidKeyword),
            }],
        );
        let computed = object(
            db,
            vec![TypeMember {
                kind: TypeMemberKind::ComputedValue(InferredTypeData::Unknown),
                ty: InferredTypeData::Number,
            }],
        );
        // `{ "\u0061": number }` has the property `a`.
        let escaped_name = object(db, vec![member("\\u0061", InferredTypeData::Number)]);
        // `{ 1.0: number }` has the property `1`.
        let number_name = object(db, vec![member("1.0", InferredTypeData::Number)]);
        let named_a = object(db, vec![member("1", a)]);
        for (other, name) in [
            (class, "a"),
            // Biome's `Error` lacks `cause`, which TypeScript's newer
            // library files add.
            (error, "a"),
            (InferredTypeData::String, "a"),
            (callable, "a"),
            (computed, "a"),
            (escaped_name, "a"),
            (number_name, "1"),
        ] {
            let target = if name == "1" { named_a } else { a };
            let ty = union(db, &[target, other]);
            assert_eq!(narrow_in(db, ty, name, true), ty, "{name}");
            assert_eq!(narrow_in(db, ty, name, false), other, "{name}");
        }
        // A class that declares the property is known to have it.
        let ty = union(db, &[a, class]);
        assert_eq!(narrow_in(db, ty, "p", true), class);
        assert_eq!(narrow_in(db, ty, "p", false), a);
    }

    /// Narrows `ty` by `ty instanceof C`, where `constructor` is the type of
    /// `C`, with a resolver that returns its input.
    fn narrow_instance<'db>(
        db: &'db dyn TypeDb,
        ty: InferredTypeData<'db>,
        constructor: InferredTypeData<'db>,
        positive: bool,
    ) -> InferredTypeData<'db> {
        super::narrow_by_instanceof(db, ty, constructor, positive, |ty| ty)
    }

    /// Returns the class `name` with `members`, extending `parent`.
    fn class<'db>(
        db: &'db TestDb,
        name: &'static str,
        parent: Option<InferredTypeData<'db>>,
        members: Vec<TypeMember<'db>>,
    ) -> InferredTypeData<'db> {
        InferredTypeData::Class(InternedClass::new(
            db,
            Box::default(),
            ClassParents {
                extends: parent,
                interface_bases: Box::default(),
            },
            Box::default(),
            members.into_boxed_slice(),
            Some(Text::new_static(name)),
            false,
            None,
            0,
        ))
    }

    fn instance<'db>(db: &'db TestDb, class: InferredTypeData<'db>) -> InferredTypeData<'db> {
        InferredTypeData::instance_of(db, class, Box::default())
    }

    #[test]
    fn instanceof_keeps_derived_members_and_replaces_their_parents() {
        let db = &TestDb::default();
        let a = class(db, "A", None, vec![member("a", InferredTypeData::Number)]);
        let b = class(
            db,
            "B",
            Some(a),
            vec![member("b", InferredTypeData::Number)],
        );
        let c = class(db, "C", None, vec![member("c", InferredTypeData::Number)]);
        let (ia, ib, ic) = (instance(db, a), instance(db, b), instance(db, c));
        let extending = interface(db, &[ia], vec![member("z", InferredTypeData::Number)]);
        let array = InferredTypeData::GlobalType(ARRAY_ID_GLOBAL_TYPE_ID);
        let tuple = InferredTypeData::Tuple(InternedTuple::new(
            db,
            Box::from([TupleElementType {
                ty: InferredTypeData::Number,
                name: None,
                is_optional: false,
                is_rest: false,
            }]),
            false,
        ));
        for (ty, constructor, positive, expected) in [
            (union(db, &[ib, ic]), a, true, ib),
            (union(db, &[ib, ic]), a, false, ic),
            (union(db, &[ib, ic]), b, true, ib),
            (union(db, &[ia, InferredTypeData::String]), b, true, ib),
            (
                union(db, &[ia, InferredTypeData::String]),
                b,
                false,
                union(db, &[ia, InferredTypeData::String]),
            ),
            (union(db, &[ib, InferredTypeData::Null]), a, true, ib),
            (
                union(db, &[ib, InferredTypeData::Null]),
                a,
                false,
                InferredTypeData::Null,
            ),
            (ib, b, false, InferredTypeData::NeverKeyword),
            (InferredTypeData::UnknownKeyword, a, true, ia),
            (
                InferredTypeData::UnknownKeyword,
                a,
                false,
                InferredTypeData::UnknownKeyword,
            ),
            // An interface derives from the class it extends.
            (union(db, &[extending, ic]), a, true, extending),
            (union(db, &[extending, ic]), a, false, ic),
            // A tuple derives from `Array`.
            (
                union(db, &[tuple, InferredTypeData::String]),
                array,
                true,
                tuple,
            ),
            (
                union(db, &[tuple, InferredTypeData::String]),
                array,
                false,
                InferredTypeData::String,
            ),
        ] {
            assert_eq!(
                narrow_instance(db, ty, constructor, positive),
                expected,
                "{positive}"
            );
        }
    }

    #[test]
    fn instanceof_gives_the_instance_type_when_it_is_a_subtype_of_the_value() {
        let db = &TestDb::default();
        let a = class(db, "A", None, vec![member("a", InferredTypeData::Number)]);
        let ia = instance(db, a);
        let shaped = object(db, vec![member("a", InferredTypeData::Number)]);
        assert_eq!(
            narrow_instance(db, union(db, &[shaped, InferredTypeData::String]), a, true),
            ia
        );
        assert_eq!(
            narrow_instance(db, InferredTypeData::ObjectKeyword, a, true),
            ia
        );
        // `A` is not a subtype of these types, so the value keeps its type.
        // `A` is assignable to the object type, but it lacks the optional
        // property `z`, which TypeScript's subtype relation requires.
        for ty in [
            union(db, &[InferredTypeData::String, InferredTypeData::Number]),
            object(
                db,
                vec![
                    member("a", InferredTypeData::Number),
                    optional_member("z", InferredTypeData::Number),
                ],
            ),
        ] {
            assert_eq!(narrow_instance(db, ty, a, true), ty);
        }
    }

    #[test]
    fn instanceof_leaves_the_type_unchanged_when_biome_cannot_tell() {
        let db = &TestDb::default();
        let a = class(db, "A", None, vec![member("a", InferredTypeData::Number)]);
        let c = class(db, "C", None, vec![member("c", InferredTypeData::Number)]);
        let (ia, ic) = (instance(db, a), instance(db, c));
        let generic = InferredTypeData::Generic(InternedGenericTypeParameter::new(
            db,
            false,
            None,
            None,
            Text::new_static("T"),
        ));
        let has_instance = class(
            db,
            "Checker",
            None,
            vec![TypeMember {
                kind: TypeMemberKind::ComputedStaticNamed(
                    Text::new_static("Symbol.hasInstance"),
                    InferredTypeData::Unknown,
                ),
                ty: method(db, "hasInstance", InferredTypeData::Boolean),
            }],
        );
        let inherits_has_instance = class(db, "SubChecker", Some(has_instance), Vec::new());
        let unresolved_parent = class(db, "X", Some(InferredTypeData::Unknown), Vec::new());
        let generic_constructor = InferredTypeData::Constructor(InternedConstructor::new(
            db,
            Box::from([generic]),
            Box::default(),
            Some(ia),
        ));
        let object_class = InferredTypeData::GlobalType(OBJECT_ID_GLOBAL_TYPE_ID);
        let value = union(db, &[ia, ic]);
        for (ty, constructor) in [
            (InferredTypeData::AnyKeyword, object_class),
            (InferredTypeData::UnknownKeyword, object_class),
            (value, has_instance),
            (value, inherits_has_instance),
            (value, generic_constructor),
            (value, object_class),
            (union(db, &[instance(db, unresolved_parent), ic]), a),
        ] {
            for positive in [true, false] {
                assert_eq!(narrow_instance(db, ty, constructor, positive), ty);
            }
        }
        // Several instance types narrow only the true branch.
        let constructors = union(db, &[a, c]);
        assert_eq!(
            narrow_instance(db, value, constructors, true),
            InferredTypeData::union_from_types(db, vec![ia, ic])
        );
        assert_eq!(narrow_instance(db, value, constructors, false), value);
    }

    #[test]
    fn instanceof_gives_unknown_and_any_the_instance_type_with_any_type_arguments() {
        let db = &TestDb::default();
        let a = class(db, "A", None, vec![member("a", InferredTypeData::Number)]);
        let parameter = |name| {
            InferredTypeData::Generic(InternedGenericTypeParameter::new(
                db,
                false,
                None,
                None,
                Text::new_static(name),
            ))
        };
        let pair = InferredTypeData::Class(InternedClass::new(
            db,
            Box::from([parameter("K"), parameter("V")]),
            ClassParents::default(),
            Box::default(),
            Box::default(),
            Some(Text::new_static("Pair")),
            false,
            None,
            0,
        ));
        let any_pair = InferredTypeData::instance_of(
            db,
            pair,
            Box::from([InferredTypeData::AnyKeyword, InferredTypeData::AnyKeyword]),
        );
        for ty in [
            InferredTypeData::UnknownKeyword,
            InferredTypeData::AnyKeyword,
        ] {
            assert_eq!(narrow_instance(db, ty, a, true), instance(db, a));
            assert_eq!(narrow_instance(db, ty, pair, true), any_pair);
            // As in TypeScript, the false branch removes nothing.
            assert_eq!(narrow_instance(db, ty, a, false), ty);
            assert_eq!(narrow_instance(db, ty, pair, false), ty);
        }
        // A member that a generic class is derived from keeps its own type
        // arguments, and an unrelated member leaves the type unchanged.
        let string_pair = InferredTypeData::instance_of(
            db,
            pair,
            Box::from([InferredTypeData::String, InferredTypeData::Number]),
        );
        let ty = union(db, &[string_pair, InferredTypeData::Null]);
        assert_eq!(narrow_instance(db, ty, pair, true), string_pair);
        assert_eq!(
            narrow_instance(db, InferredTypeData::String, pair, true),
            InferredTypeData::String
        );
    }

    #[test]
    fn instanceof_tells_apart_classes_with_the_same_contents_only_by_handle() {
        let db = &TestDb::default();
        let wrapper = InternedTypeofType::new(db, InferredTypeData::Number);
        let handle = |index| {
            InferredTypeData::Local(LocalTypeHandle::new(
                db,
                ModuleKey::new(wrapper.as_id()),
                LocalTypeId::new(index),
            ))
        };
        let (first, second) = (handle(0), handle(1));
        let k = class(db, "K", None, vec![member("k", InferredTypeData::Number)]);
        let resolve = |ty| if ty == first || ty == second { k } else { ty };
        let ty = union(db, &[instance(db, first), InferredTypeData::String]);
        assert_eq!(
            super::narrow_by_instanceof(db, ty, first, true, resolve),
            instance(db, first)
        );
        assert_eq!(
            super::narrow_by_instanceof(db, ty, first, false, resolve),
            InferredTypeData::String
        );
        // Another handle to the same contents may be another declaration.
        for positive in [true, false] {
            assert_eq!(
                super::narrow_by_instanceof(db, ty, second, positive, resolve),
                ty
            );
        }
    }

    #[test]
    fn only_object_types_unknown_and_any_may_narrow_by_instanceof() {
        let db = &TestDb::default();
        let a = instance(db, class(db, "A", None, Vec::new()));
        for (ty, expected) in [
            (union(db, &[a, InferredTypeData::String]), true),
            (a, true),
            (InferredTypeData::ObjectKeyword, true),
            (InferredTypeData::UnknownKeyword, true),
            (InferredTypeData::AnyKeyword, true),
            (
                union(db, &[InferredTypeData::String, InferredTypeData::Null]),
                false,
            ),
            (InferredTypeData::Number, false),
            (InferredTypeData::Undefined, false),
        ] {
            assert_eq!(may_narrow_by_instanceof(db, ty), expected);
        }
    }

    #[test]
    fn raw_object_unions_need_no_resolution() {
        let local = |index| TypeReference::Resolved(RawTypeId::Local(crate::TypeId::new(index)));
        let union_of = |types: Vec<TypeReference>| {
            RawTypeData::Union(Box::new(crate::Union(types.into_boxed_slice())))
        };
        let primitives = union_of(vec![
            TypeReference::Resolved(GLOBAL_STRING_KEYWORD_ID),
            TypeReference::Resolved(GLOBAL_UNDEFINED_ID),
            local(1),
        ]);
        let with_object = union_of(vec![
            TypeReference::Resolved(GLOBAL_STRING_KEYWORD_ID),
            local(2),
        ]);
        let with_objects = union_of(vec![local(2), local(2)]);
        let with_name = union_of(vec![
            TypeReference::Resolved(GLOBAL_NULL_KEYWORD_ID),
            local(1),
        ]);
        let string_literal = RawTypeData::Literal(Box::new(crate::Literal::String(
            crate::literal::StringLiteral::from(Text::new_static("a")),
        )));
        for (raw_types, expected) in [
            (vec![primitives, string_literal.clone()], false),
            // One object type among primitives can't be told apart by a
            // property.
            (
                vec![
                    with_object,
                    string_literal.clone(),
                    RawTypeData::ObjectKeyword,
                ],
                false,
            ),
            (
                vec![
                    with_objects,
                    string_literal.clone(),
                    RawTypeData::ObjectKeyword,
                ],
                true,
            ),
            // A `typeof` type may stand for a union of object types.
            (
                vec![
                    with_name,
                    RawTypeData::TypeofType(Box::new(TypeReference::unknown())),
                ],
                true,
            ),
            (vec![RawTypeData::ObjectKeyword], false),
            (vec![RawTypeData::String], false),
            (
                vec![
                    RawTypeData::Reference(local(1)),
                    union_of(vec![
                        TypeReference::Resolved(GLOBAL_NULL_KEYWORD_ID),
                        local(2),
                    ]),
                    string_literal.clone(),
                ],
                false,
            ),
            (
                vec![RawTypeData::TypeofType(Box::new(TypeReference::unknown()))],
                true,
            ),
            (vec![RawTypeData::Unknown], true),
            // Inferred types: `new C()` is one instance, `typeof x` is a
            // string, and the result of a call may be anything.
            (
                vec![RawTypeData::from(TypeofExpression::New(
                    crate::TypeofNewExpression {
                        callee: TypeReference::unknown(),
                        arguments: Box::default(),
                    },
                ))],
                false,
            ),
            (
                vec![RawTypeData::from(TypeofExpression::Typeof(
                    crate::TypeofTypeofExpression {
                        argument: TypeReference::unknown(),
                    },
                ))],
                false,
            ),
            (
                vec![RawTypeData::from(TypeofExpression::Call(
                    crate::TypeofCallExpression {
                        callee: TypeReference::unknown(),
                        arguments: Box::default(),
                    },
                ))],
                true,
            ),
        ] {
            assert_eq!(
                raw_may_be_object_union(&local(0), &raw_types),
                expected,
                "{raw_types:?}"
            );
        }
        assert!(!raw_may_be_object_union(
            &TypeReference::Resolved(GLOBAL_STRING_KEYWORD_ID),
            &[]
        ));
        // A local id outside the table may stand for anything.
        assert!(raw_may_be_object_union(&local(3), &[]));
    }

    #[test]
    fn raw_types_without_null_or_undefined_are_never_nullish() {
        let local = |index| TypeReference::Resolved(RawTypeId::Local(crate::TypeId::new(index)));
        let union_of = |types: Vec<TypeReference>| {
            RawTypeData::Union(Box::new(crate::Union(types.into_boxed_slice())))
        };
        let string_literal = RawTypeData::Literal(Box::new(crate::Literal::String(
            crate::literal::StringLiteral::from(Text::new_static("a")),
        )));
        let object = RawTypeData::Object(Box::new(crate::Object {
            prototype: None,
            members: Box::default(),
            has_unknown_members: false,
            kind: ObjectKind::Expression,
        }));
        for (raw_types, expected) in [
            (vec![object.clone()], false),
            (vec![RawTypeData::String], false),
            (vec![string_literal.clone()], false),
            (
                vec![
                    union_of(vec![
                        TypeReference::Resolved(GLOBAL_STRING_KEYWORD_ID),
                        local(1),
                    ]),
                    object.clone(),
                ],
                false,
            ),
            (
                vec![RawTypeData::Reference(local(1)), object.clone()],
                false,
            ),
            (
                vec![RawTypeData::from(TypeofExpression::New(
                    crate::TypeofNewExpression {
                        callee: TypeReference::unknown(),
                        arguments: Box::default(),
                    },
                ))],
                false,
            ),
            (
                vec![union_of(vec![
                    TypeReference::Resolved(GLOBAL_STRING_KEYWORD_ID),
                    TypeReference::Resolved(GLOBAL_UNDEFINED_ID),
                ])],
                true,
            ),
            (
                vec![
                    union_of(vec![
                        TypeReference::Resolved(GLOBAL_NULL_KEYWORD_ID),
                        local(1),
                    ]),
                    object.clone(),
                ],
                true,
            ),
            // A nested union, a `typeof` query, and an inferred call may be
            // nullish.
            (
                vec![
                    union_of(vec![local(1)]),
                    union_of(vec![TypeReference::Resolved(GLOBAL_STRING_KEYWORD_ID)]),
                ],
                true,
            ),
            (
                vec![RawTypeData::TypeofType(Box::new(TypeReference::unknown()))],
                true,
            ),
            (vec![RawTypeData::Null], true),
            (vec![RawTypeData::VoidKeyword], true),
            (vec![RawTypeData::Unknown], true),
            (
                vec![RawTypeData::from(TypeofExpression::Call(
                    crate::TypeofCallExpression {
                        callee: TypeReference::unknown(),
                        arguments: Box::default(),
                    },
                ))],
                true,
            ),
        ] {
            assert_eq!(
                raw_may_be_nullish(&local(0), &raw_types),
                expected,
                "{raw_types:?}"
            );
        }
        assert!(!raw_may_be_nullish(
            &TypeReference::Resolved(GLOBAL_NUMBER_KEYWORD_ID),
            &[]
        ));
        assert!(raw_may_be_nullish(
            &TypeReference::Resolved(GLOBAL_UNDEFINED_ID),
            &[]
        ));
        // A local id outside the table may stand for anything.
        assert!(raw_may_be_nullish(&local(3), &[]));
    }

    #[test]
    fn raw_types_that_may_be_unions_or_have_literals() {
        let local = |index| TypeReference::Resolved(RawTypeId::Local(crate::TypeId::new(index)));
        let union_of = |types: Vec<TypeReference>| {
            RawTypeData::Union(Box::new(crate::Union(types.into_boxed_slice())))
        };
        let number_literal = RawTypeData::Literal(Box::new(crate::Literal::Number(
            crate::literal::NumberLiteral::new(Text::new_static("1")),
        )));
        let widened = |value| {
            RawTypeData::from(TypeofExpression::WidenedLiteral(
                crate::TypeofWidenedLiteralExpression {
                    value,
                    is_return_type: false,
                },
            ))
        };
        let call = RawTypeData::from(TypeofExpression::Call(crate::TypeofCallExpression {
            callee: TypeReference::unknown(),
            arguments: Box::default(),
        }));
        // Each case gives whether the type may be a union, then whether it
        // may have a literal member.
        for (raw_types, union, literal) in [
            (vec![RawTypeData::String], false, false),
            (vec![RawTypeData::Boolean], false, false),
            (vec![number_literal.clone()], false, true),
            (
                vec![union_of(vec![
                    TypeReference::Resolved(GLOBAL_STRING_KEYWORD_ID),
                    TypeReference::Resolved(GLOBAL_UNDEFINED_ID),
                ])],
                true,
                false,
            ),
            (
                vec![
                    union_of(vec![
                        TypeReference::Resolved(GLOBAL_STRING_KEYWORD_ID),
                        local(1),
                    ]),
                    number_literal.clone(),
                ],
                true,
                true,
            ),
            // `let n = 1` widens to `number`, and widening keeps a union a
            // union.
            (
                vec![widened(local(1)), number_literal.clone()],
                false,
                false,
            ),
            (
                vec![
                    widened(local(1)),
                    union_of(vec![local(2), local(2)]),
                    number_literal.clone(),
                ],
                true,
                false,
            ),
            (
                vec![RawTypeData::Reference(local(1)), number_literal],
                false,
                true,
            ),
            (vec![call], true, true),
        ] {
            assert_eq!(
                raw_may_be_union(&local(0), &raw_types),
                union,
                "{raw_types:?}"
            );
            assert_eq!(
                raw_may_have_literal(&local(0), &raw_types),
                literal,
                "{raw_types:?}"
            );
        }
        // Names and ids outside the table may stand for anything.
        assert!(raw_may_be_union(&local(3), &[]));
        assert!(raw_may_have_literal(&local(3), &[]));
        assert!(!raw_may_be_union(
            &TypeReference::Resolved(GLOBAL_NUMBER_KEYWORD_ID),
            &[]
        ));
        assert!(!raw_may_have_literal(
            &TypeReference::Resolved(GLOBAL_NUMBER_KEYWORD_ID),
            &[]
        ));
    }

    #[test]
    fn assignments_keep_the_members_of_the_declared_union_that_the_value_may_have() {
        let db = TestDb::default();
        let db = &db;
        let reduce =
            |declared, assigned| super::narrow_by_assignment(db, declared, assigned, |ty| ty);
        let a = string(db, "a");
        let b = string(db, "b");
        let one = number(db, "1");
        let string_or_number = union(db, &[InferredTypeData::String, InferredTypeData::Number]);
        assert_eq!(reduce(string_or_number, a), InferredTypeData::String);
        assert_eq!(reduce(string_or_number, one), InferredTypeData::Number);
        assert_eq!(reduce(union(db, &[a, b]), a), a);
        // The value of a union keeps each member that one of its values may
        // have.
        assert_eq!(reduce(string_or_number, string_or_number), string_or_number);
        assert_eq!(
            reduce(
                union(db, &[a, b, InferredTypeData::Null]),
                union(db, &[a, InferredTypeData::Null])
            ),
            union(db, &[a, InferredTypeData::Null])
        );
        // A type that is not a union, `any`, and a value that no member
        // accepts keep the declared type, and `never` gives `never`.
        assert_eq!(
            reduce(InferredTypeData::String, a),
            InferredTypeData::String
        );
        assert_eq!(
            reduce(string_or_number, InferredTypeData::AnyKeyword),
            string_or_number
        );
        assert_eq!(
            reduce(union(db, &[a, b]), string(db, "c")),
            union(db, &[a, b])
        );
        assert_eq!(
            reduce(string_or_number, InferredTypeData::NeverKeyword),
            InferredTypeData::NeverKeyword
        );
        // `boolean` stays whole, and a boolean value counts as both values.
        let optional_boolean = union(
            db,
            &[InferredTypeData::Boolean, InferredTypeData::Undefined],
        );
        assert_eq!(
            reduce(optional_boolean, boolean_literal(db, true)),
            InferredTypeData::Boolean
        );
        let literals = union(
            db,
            &[
                boolean_literal(db, true),
                boolean_literal(db, false),
                InferredTypeData::Undefined,
            ],
        );
        assert_eq!(
            reduce(literals, boolean_literal(db, true)),
            union(db, &[boolean_literal(db, true), boolean_literal(db, false)])
        );
    }

    #[test]
    fn base_literal_types_replace_literals_with_their_primitives() {
        let db = TestDb::default();
        let db = &db;
        assert_eq!(
            base_literal_type(
                db,
                union(db, &[number(db, "1"), number(db, "2"), string(db, "a")])
            ),
            union(db, &[InferredTypeData::Number, InferredTypeData::String])
        );
        assert_eq!(
            base_literal_type(db, boolean_literal(db, true)),
            InferredTypeData::Boolean
        );
        assert_eq!(
            base_literal_type(db, bigint(db, "1")),
            InferredTypeData::BigInt
        );
        let nullable = union(db, &[number(db, "1"), InferredTypeData::Null]);
        assert_eq!(
            base_literal_type(db, nullable),
            union(db, &[InferredTypeData::Number, InferredTypeData::Null])
        );
        assert_eq!(
            base_literal_type(db, InferredTypeData::Null),
            InferredTypeData::Null
        );
    }

    /// Returns the type of an object literal with `members`.
    fn object_expression<'db>(
        db: &'db TestDb,
        members: Vec<TypeMember<'db>>,
    ) -> InferredTypeData<'db> {
        InferredTypeData::Object(InternedObject::new(
            db,
            None,
            members.into_boxed_slice(),
            false,
            ObjectKind::Expression,
        ))
    }

    /// Returns `Array<element>`.
    fn array<'db>(db: &'db TestDb, element: InferredTypeData<'db>) -> InferredTypeData<'db> {
        InferredTypeData::instance_of(db, InferredTypeData::array_class(), Box::new([element]))
    }

    #[test]
    fn evolving_values_widen_literals_and_reject_types_biome_may_infer_otherwise() {
        let db = &TestDb::default();
        let evolving = |ty| super::evolving_assigned_type(db, ty, |ty| ty);
        let object = |members| object_expression(db, members);
        let array = |element| array(db, element);
        assert_eq!(
            evolving(union(db, &[number(db, "1"), string(db, "a")])),
            union(db, &[InferredTypeData::Number, InferredTypeData::String])
        );
        assert_eq!(evolving(InferredTypeData::Null), InferredTypeData::Null);
        let widened = object(vec![member("k", InferredTypeData::Number)]);
        assert_eq!(evolving(widened), widened);
        let elements = array(InferredTypeData::Number);
        assert_eq!(evolving(elements), elements);
        let instance = instance(db, class(db, "Date", None, Vec::new()));
        assert_eq!(evolving(instance), instance);
        // A literal inside an object type or a type argument may stay in
        // Biome where TypeScript widens it, and functions, `unknown`, and
        // types that Biome couldn't determine may differ too.
        for ty in [
            object(vec![member("k", number(db, "1"))]),
            array(string(db, "a")),
            array(InferredTypeData::UnknownKeyword),
            object(vec![TypeMember {
                kind: TypeMemberKind::Getter(Text::new_static("k")),
                ty: InferredTypeData::Number,
            }]),
            InferredTypeData::Unknown,
            InferredTypeData::AnyKeyword,
            InferredTypeData::VoidKeyword,
            union(
                db,
                &[InferredTypeData::Number, InferredTypeData::UnknownKeyword],
            ),
        ] {
            assert_eq!(evolving(ty), InferredTypeData::Unknown, "{ty:?}");
        }
    }

    #[test]
    fn evolving_arrays_join_the_elements_of_their_members() {
        let db = &TestDb::default();
        let array = |element| array(db, element);
        assert_eq!(
            super::evolving_array_element(
                db,
                union(
                    db,
                    &[
                        array(InferredTypeData::Number),
                        array(InferredTypeData::String)
                    ]
                )
            ),
            Some(union(
                db,
                &[InferredTypeData::Number, InferredTypeData::String]
            ))
        );
        assert_eq!(
            super::evolving_array_element(db, array(InferredTypeData::NeverKeyword)),
            Some(InferredTypeData::NeverKeyword)
        );
        for ty in [
            InferredTypeData::Unknown,
            union(
                db,
                &[array(InferredTypeData::Number), InferredTypeData::Undefined],
            ),
        ] {
            assert_eq!(super::evolving_array_element(db, ty), None, "{ty:?}");
        }
    }

    #[test]
    fn raw_types_with_an_object_type_may_narrow_by_instanceof() {
        let local = |index| TypeReference::Resolved(RawTypeId::Local(crate::TypeId::new(index)));
        let union_of = |types: Vec<TypeReference>| {
            RawTypeData::Union(Box::new(crate::Union(types.into_boxed_slice())))
        };
        let primitives = union_of(vec![
            TypeReference::Resolved(GLOBAL_STRING_KEYWORD_ID),
            TypeReference::Resolved(GLOBAL_NULL_KEYWORD_ID),
        ]);
        let with_object = union_of(vec![
            TypeReference::Resolved(GLOBAL_STRING_KEYWORD_ID),
            local(1),
        ]);
        for (raw_types, expected) in [
            (vec![primitives], false),
            (vec![RawTypeData::String], false),
            // One object type is enough for `instanceof`.
            (vec![with_object, RawTypeData::ObjectKeyword], true),
            (vec![RawTypeData::ObjectKeyword], true),
            (vec![RawTypeData::Unknown], true),
        ] {
            assert_eq!(
                raw_may_narrow_by_instanceof(&local(0), &raw_types),
                expected,
                "{raw_types:?}"
            );
        }
        assert!(raw_may_narrow_by_instanceof(
            &TypeReference::Resolved(crate::globals::GLOBAL_UNKNOWN_KEYWORD_ID),
            &[]
        ));
        assert!(!raw_may_narrow_by_instanceof(
            &TypeReference::Resolved(GLOBAL_NUMBER_KEYWORD_ID),
            &[]
        ));
    }

    /// Narrows `ty` by a type predicate whose target is `target`, with a
    /// resolver that returns its input.
    fn narrow_predicate<'db>(
        db: &'db dyn TypeDb,
        ty: InferredTypeData<'db>,
        target: InferredTypeData<'db>,
        positive: bool,
    ) -> InferredTypeData<'db> {
        super::narrow_by_type_predicate(db, ty, target, positive, |ty| ty)
    }

    /// Returns the interface `name` with `members`, extending `parents`.
    fn named_interface<'db>(
        db: &'db TestDb,
        name: &'static str,
        parents: &[InferredTypeData<'db>],
        members: Vec<TypeMember<'db>>,
    ) -> InferredTypeData<'db> {
        InferredTypeData::Interface(InternedInterface::new(
            db,
            Box::default(),
            Box::<[_]>::from(parents),
            members.into_boxed_slice(),
            Text::new_static(name),
            None,
            0,
        ))
    }

    #[test]
    fn type_predicates_keep_subtypes_and_replace_supertypes() {
        let db = &TestDb::default();
        let foo = named_interface(db, "Foo", &[], vec![member("a", InferredTypeData::Number)]);
        let foo2 = named_interface(
            db,
            "Foo2",
            &[foo],
            vec![member("b", InferredTypeData::Number)],
        );
        let bar = named_interface(db, "Bar", &[], vec![member("c", InferredTypeData::Number)]);
        let ia = named_interface(db, "IA", &[], vec![member("x", InferredTypeData::Number)]);
        let optional = object(
            db,
            vec![
                member("x", InferredTypeData::Number),
                optional_member("y", InferredTypeData::String),
            ],
        );
        let primitives = union(db, &[InferredTypeData::String, InferredTypeData::Number]);
        for (ty, target, positive, expected) in [
            (union(db, &[foo, InferredTypeData::String]), foo, true, foo),
            (
                union(db, &[foo, InferredTypeData::String]),
                foo,
                false,
                InferredTypeData::String,
            ),
            (union(db, &[foo2, bar]), foo, true, foo2),
            (union(db, &[foo2, bar]), foo, false, bar),
            (InferredTypeData::UnknownKeyword, foo, true, foo),
            (
                InferredTypeData::UnknownKeyword,
                foo,
                false,
                InferredTypeData::UnknownKeyword,
            ),
            // `optional` replaces `IA`, which lacks its optional property, so
            // the `else` branch keeps `IA`.
            (union(db, &[ia, bar]), optional, true, optional),
            (
                union(db, &[ia, bar]),
                optional,
                false,
                union(db, &[ia, bar]),
            ),
            (InferredTypeData::ObjectKeyword, foo, true, foo),
            (
                InferredTypeData::ObjectKeyword,
                foo,
                false,
                InferredTypeData::ObjectKeyword,
            ),
            (
                union(db, &[primitives, InferredTypeData::Boolean]),
                primitives,
                true,
                primitives,
            ),
            (
                union(db, &[primitives, InferredTypeData::Boolean]),
                primitives,
                false,
                InferredTypeData::Boolean,
            ),
            (
                union(db, &[foo, InferredTypeData::Undefined]),
                foo,
                false,
                InferredTypeData::Undefined,
            ),
            (foo, foo, false, InferredTypeData::NeverKeyword),
            (
                union(db, &[foo, bar, InferredTypeData::String]),
                union(db, &[foo, bar]),
                true,
                union(db, &[foo, bar]),
            ),
            // No member is related to `Foo`, and `Foo` is not a subtype of the
            // union, where TypeScript gives `(string | number) & Foo`.
            (primitives, foo, true, primitives),
            (primitives, foo, false, primitives),
            // `never` reaches no value.
            (
                foo,
                InferredTypeData::NeverKeyword,
                true,
                InferredTypeData::NeverKeyword,
            ),
            (foo, InferredTypeData::NeverKeyword, false, foo),
        ] {
            assert_eq!(
                narrow_predicate(db, ty, target, positive),
                expected,
                "{positive}"
            );
        }
    }

    #[test]
    fn type_predicates_compare_primitives_with_the_members_of_their_wrappers() {
        let db = &TestDb::default();
        let foo = named_interface(db, "Foo", &[], vec![member("a", InferredTypeData::Number)]);
        let length = object(db, vec![member("length", InferredTypeData::Number)]);
        let ty = union(
            db,
            &[InferredTypeData::String, InferredTypeData::Number, foo],
        );
        // Every string has `length`, which numbers and `Foo` lack.
        assert_eq!(
            narrow_predicate(db, ty, length, true),
            InferredTypeData::String
        );
        assert_eq!(
            narrow_predicate(db, ty, length, false),
            union(db, &[InferredTypeData::Number, foo])
        );
        // A subtype must have the optional properties of the target too.
        let weak = object(
            db,
            vec![
                member("length", InferredTypeData::Number),
                optional_member("unknownMember", InferredTypeData::Number),
            ],
        );
        assert_eq!(
            narrow_predicate(db, union(db, &[InferredTypeData::String, foo]), weak, false),
            union(db, &[InferredTypeData::String, foo])
        );
    }

    #[test]
    fn type_predicates_keep_literals_whose_primitive_the_true_branch_gets() {
        let db = &TestDb::default();
        let length = object(db, vec![member("length", InferredTypeData::Number)]);
        let ty = union(db, &[string(db, "a"), length]);
        assert_eq!(
            narrow_predicate(db, ty, InferredTypeData::String, true),
            InferredTypeData::String
        );
        // TypeScript's union of `"a"` and `string` is `string`, so the false
        // branch keeps `"a"`.
        assert_eq!(
            narrow_predicate(db, ty, InferredTypeData::String, false),
            ty
        );
        let ty = union(db, &[string(db, "a"), InferredTypeData::Number]);
        assert_eq!(
            narrow_predicate(db, ty, InferredTypeData::String, false),
            InferredTypeData::Number
        );
    }

    #[test]
    fn type_predicates_keep_members_that_may_replace_each_other_in_the_false_branch() {
        let db = &TestDb::default();
        let foo = named_interface(db, "Foo", &[], vec![member("a", InferredTypeData::Number)]);
        let same = named_interface(db, "Same", &[], vec![member("a", InferredTypeData::Number)]);
        let bar = named_interface(db, "Bar", &[], vec![member("c", InferredTypeData::Number)]);
        let ty = union(db, &[same, bar]);
        assert_eq!(narrow_predicate(db, ty, foo, true), same);
        assert_eq!(narrow_predicate(db, ty, foo, false), ty);
    }

    #[test]
    fn type_predicates_leave_the_type_unchanged_when_biome_cannot_tell() {
        let db = &TestDb::default();
        let foo = named_interface(db, "Foo", &[], vec![member("a", InferredTypeData::Number)]);
        // A constraint that inference can't determine can't be compared.
        let constrained = InferredTypeData::Generic(InternedGenericTypeParameter::new(
            db,
            false,
            Some(InferredTypeData::Unknown),
            None,
            Text::new_static("U"),
        ));
        for (ty, target) in [
            (union(db, &[constrained, InferredTypeData::String]), foo),
            (union(db, &[foo, InferredTypeData::UnknownKeyword]), foo),
            (
                union(db, &[foo, InferredTypeData::String]),
                InferredTypeData::AnyKeyword,
            ),
            (
                union(db, &[foo, InferredTypeData::String]),
                union(db, &[foo, InferredTypeData::Unknown]),
            ),
        ] {
            for positive in [true, false] {
                assert_eq!(narrow_predicate(db, ty, target, positive), ty, "{positive}");
            }
        }
    }

    #[test]
    fn type_predicates_give_any_their_target_in_the_true_branch() {
        let db = &TestDb::default();
        let any = InferredTypeData::AnyKeyword;
        let foo = named_interface(db, "Foo", &[], vec![member("a", InferredTypeData::Number)]);
        for target in [
            foo,
            InferredTypeData::String,
            string(db, "a"),
            InferredTypeData::Null,
            union(db, &[foo, InferredTypeData::Number]),
            any,
        ] {
            assert_eq!(narrow_predicate(db, any, target, true), target);
            assert_eq!(narrow_predicate(db, any, target, false), any);
        }
        assert_eq!(
            narrow_predicate(db, any, InferredTypeData::Unknown, true),
            any
        );
    }

    #[test]
    fn type_predicates_compare_literal_types_of_initializers_as_widened() {
        let db = &TestDb::default();
        let one = number(db, "1");
        let literal = InferredTypeData::Object(InternedObject::new(
            db,
            None,
            Box::from([member("a", one)]),
            false,
            ObjectKind::Expression,
        ));
        let shape = object(db, vec![member("a", one)]);
        // An object literal keeps `a: 1` where Biome can't tell that the
        // value is a fresh literal, as in `{ a: id(1) }`. TypeScript types
        // that object as `{ a: number }`, which is not a subtype of
        // `{ a: 1 }`, so the `else` branch keeps it.
        let ty = union(db, &[literal, InferredTypeData::String]);
        assert_eq!(narrow_predicate(db, ty, shape, true), literal);
        assert_eq!(narrow_predicate(db, ty, shape, false), ty);
        // A type literal keeps its literal types.
        let ty = union(db, &[shape, InferredTypeData::String]);
        assert_eq!(
            narrow_predicate(db, ty, shape, false),
            InferredTypeData::String
        );
        // A class field initialized with `id(1)` is `number` in TypeScript,
        // so `{ a: number }` is a subtype of the class instance.
        let class = instance(db, class(db, "A", None, vec![member("a", one)]));
        let wide = object(db, vec![member("a", InferredTypeData::Number)]);
        let ty = union(db, &[wide, InferredTypeData::String]);
        assert_eq!(narrow_predicate(db, ty, class, true), wide);
        assert_eq!(narrow_predicate(db, ty, class, false), ty);
    }

    #[test]
    fn type_predicates_tell_apart_classes_with_the_same_contents_only_by_handle() {
        let db = &TestDb::default();
        let wrapper = InternedTypeofType::new(db, InferredTypeData::Number);
        let handle = |index| {
            InferredTypeData::Local(LocalTypeHandle::new(
                db,
                ModuleKey::new(wrapper.as_id()),
                LocalTypeId::new(index),
            ))
        };
        let (first, second) = (handle(0), handle(1));
        let k = class(db, "K", None, vec![member("k", InferredTypeData::Number)]);
        let resolve = |ty| if ty == first || ty == second { k } else { ty };
        let ty = union(db, &[instance(db, first), InferredTypeData::String]);
        assert_eq!(
            super::narrow_by_type_predicate(db, ty, instance(db, first), true, resolve),
            instance(db, first)
        );
        assert_eq!(
            super::narrow_by_type_predicate(db, ty, instance(db, first), false, resolve),
            InferredTypeData::String
        );
        // Another handle to the same contents may be another declaration, and
        // so may the same contents without a handle.
        let ty_without_handle = union(db, &[instance(db, k), InferredTypeData::String]);
        for (ty, target) in [
            (ty, instance(db, second)),
            (ty_without_handle, instance(db, k)),
        ] {
            for positive in [true, false] {
                assert_eq!(
                    super::narrow_by_type_predicate(db, ty, target, positive, resolve),
                    ty,
                    "{positive}"
                );
            }
        }
    }

    /// Returns an instance of a class called `name` that has `members`, the
    /// `interface_bases` of interfaces merged into it, and no parent class.
    fn merged_class_instance<'db>(
        db: &'db TestDb,
        name: &'static str,
        interface_bases: &[InferredTypeData<'db>],
        members: Vec<TypeMember<'db>>,
    ) -> InferredTypeData<'db> {
        let class = InferredTypeData::Class(InternedClass::new(
            db,
            Box::default(),
            ClassParents {
                extends: None,
                interface_bases: Box::<[_]>::from(interface_bases),
            },
            Box::default(),
            members.into_boxed_slice(),
            Some(Text::new_static(name)),
            false,
            None,
            0,
        ));
        InferredTypeData::instance_of(db, class, Box::default())
    }

    #[test]
    fn class_instances_inherit_from_the_bases_of_merged_interfaces() {
        let db = &TestDb::default();
        let base = named_interface(db, "Base", &[], vec![member("b", InferredTypeData::Number)]);
        let a = class(db, "A", None, vec![member("a", InferredTypeData::Number)]);
        let c = class(db, "C", None, vec![member("c", InferredTypeData::Number)]);
        let (ia, ic) = (instance(db, a), instance(db, c));
        // `class N { k: number }` with `interface N extends Base {}`.
        let n = merged_class_instance(
            db,
            "N",
            &[base],
            vec![member("k", InferredTypeData::Number)],
        );
        // `class M {}` with `interface M extends A {}`.
        let m = merged_class_instance(db, "M", &[ia], vec![]);
        let other = object(db, vec![member("o", InferredTypeData::Number)]);
        let target = object(db, vec![member("b", InferredTypeData::Number)]);
        let both = object(
            db,
            vec![
                member("b", InferredTypeData::Number),
                member("z", InferredTypeData::Number),
            ],
        );

        // `in` finds the property on the interface base.
        assert_eq!(narrow_in(db, union(db, &[n, other]), "b", false), other);
        // `instanceof` counts `M` as derived from its interface base `A`.
        assert_eq!(narrow_instance(db, union(db, &[m, ic]), a, true), m);
        assert_eq!(narrow_instance(db, union(db, &[m, ic]), a, false), ic);
        // A type predicate finds the member on the interface base.
        let ty = union(db, &[n, both, InferredTypeData::String]);
        assert_eq!(
            narrow_predicate(db, ty, target, true),
            union(db, &[n, both])
        );
        assert_eq!(
            narrow_predicate(db, ty, target, false),
            InferredTypeData::String
        );
    }

    #[test]
    fn typeof_function_keeps_classes_that_merged_interfaces_make_callable() {
        let db = &TestDb::default();
        let callable = named_interface(
            db,
            "Callable",
            &[],
            vec![TypeMember {
                kind: TypeMemberKind::CallSignature,
                ty: method(db, "call", InferredTypeData::VoidKeyword),
            }],
        );
        // `class F {}` with `interface F extends Callable {}`.
        let through_base = merged_class_instance(db, "F", &[callable], vec![]);
        // `class G { g: number }` with `interface G { (): void }`.
        let own = merged_class_instance(
            db,
            "G",
            &[],
            vec![
                member("g", InferredTypeData::Number),
                TypeMember {
                    kind: TypeMemberKind::CallSignature,
                    ty: method(db, "call", InferredTypeData::VoidKeyword),
                },
            ],
        );
        // No string has a call signature.
        assert_eq!(
            primitive_can_satisfy(db, TypeofKind::String, through_base, &mut |ty| ty),
            Assignability::NotAssignable
        );
        for instance in [through_base, own] {
            let ty = union(db, &[instance, InferredTypeData::String]);
            assert_eq!(
                narrow_type(
                    db,
                    ty,
                    NarrowingPredicate::Typeof(TypeofKind::Function),
                    true
                ),
                instance
            );
        }
    }

    /// Returns a function with the parameters `(name, type)` that returns a
    /// type predicate about the parameter `tested`, targeting `target`, or
    /// `boolean` when `tested` is `None`.
    fn predicate_function<'db>(
        db: &'db TestDb,
        parameters: &[(&'static str, InferredTypeData<'db>)],
        tested: Option<(&'static str, InferredTypeData<'db>)>,
    ) -> InferredTypeData<'db> {
        let parameters: Box<[_]> = parameters
            .iter()
            .map(|(name, ty)| {
                crate::interned_types::FunctionParameter::Named(
                    crate::interned_types::NamedFunctionParameter {
                        name: Text::new_static(name),
                        ty: *ty,
                        is_optional: false,
                        is_rest: false,
                    },
                )
            })
            .collect();
        let return_type = match tested {
            Some((name, ty)) => ReturnType::Predicate(crate::interned_types::PredicateReturnType {
                parameter_name: Text::new_static(name),
                ty,
            }),
            None => ReturnType::Type(InferredTypeData::Boolean),
        };
        InferredTypeData::Function(InternedFunction::new(
            db,
            Box::default(),
            parameters,
            return_type,
            false,
            None,
        ))
    }

    // TypeScript relates two signatures that return type predicates when the
    // predicates test the parameter at the same position and the source's
    // target is assignable to the target's, as for a type-guard function
    // passed as the callback of `every` or `filter`.
    #[test]
    fn type_predicate_signatures_relate_by_position_and_target() {
        use InferredTypeData::{Number, String, UnknownKeyword};
        let db = TestDb::default();
        let element = union(&db, &[String, Number]);
        let is_string = predicate_function(&db, &[("v", UnknownKeyword)], Some(("v", String)));
        let assignable = |source, target| crate::is_assignable_to(&db, source, target, |ty| ty);
        let callback = predicate_function(
            &db,
            &[("value", element), ("index", Number)],
            Some(("value", String)),
        );
        assert_eq!(assignable(is_string, callback), Some(true));
        let other_target = predicate_function(&db, &[("value", element)], Some(("value", Number)));
        assert_eq!(assignable(is_string, other_target), Some(false));
        let other_position = predicate_function(
            &db,
            &[("index", Number), ("value", element)],
            Some(("value", String)),
        );
        assert_eq!(assignable(is_string, other_position), Some(false));
        // TypeScript infers a type predicate for some functions that return a
        // `boolean` without an annotation.
        let boolean = predicate_function(&db, &[("v", UnknownKeyword)], None);
        assert_eq!(assignable(boolean, callback), None);
    }

    /// Returns a reference to the type parameter `name` with `constraint`, as
    /// an annotation refers to it.
    fn type_parameter<'db>(
        db: &'db TestDb,
        name: &'static str,
        constraint: Option<InferredTypeData<'db>>,
    ) -> InferredTypeData<'db> {
        instance(
            db,
            InferredTypeData::Generic(InternedGenericTypeParameter::new(
                db,
                false,
                constraint,
                None,
                Text::new_static(name),
            )),
        )
    }

    fn intersection<'db>(
        db: &'db TestDb,
        types: &[InferredTypeData<'db>],
    ) -> InferredTypeData<'db> {
        super::intersect_narrowed(db, types[0], types[1])
    }

    #[test]
    fn intersections_of_narrowing_keep_their_object_operands() {
        let db = &TestDb::default();
        let foo = named_interface(db, "Foo", &[], vec![member("a", InferredTypeData::Number)]);
        let bar = named_interface(db, "Bar", &[], vec![member("c", InferredTypeData::Number)]);
        let InferredTypeData::Intersection(narrowed) = super::intersect_narrowed(db, bar, foo)
        else {
            panic!("`Bar & Foo` must keep both operands");
        };
        assert_eq!(narrowed.types(db).as_ref(), [bar, foo]);
        // The intersection is still normalized.
        let t = type_parameter(db, "T", None);
        let empty = object(db, Vec::new());
        assert_eq!(
            super::intersect_narrowed(db, InferredTypeData::String, empty),
            InferredTypeData::String
        );
        assert_eq!(
            super::intersect_narrowed(db, union(db, &[t, InferredTypeData::Null]), empty),
            super::intersect_narrowed(db, t, empty)
        );
    }

    #[test]
    fn type_parameters_narrow_to_intersections() {
        let db = &TestDb::default();
        let t = type_parameter(db, "T", None);
        let empty = object(db, Vec::new());
        let non_nullish = intersection(db, &[t, empty]);
        let null = NarrowingPredicate::Literal(InferredTypeData::Null);
        let undefined = NarrowingPredicate::Literal(InferredTypeData::Undefined);
        let with = |other| union(db, &[non_nullish, intersection(db, &[t, other])]);
        for (predicate, positive, expected) in [
            (NarrowingPredicate::Truthy, true, non_nullish),
            (NarrowingPredicate::Truthy, false, t),
            (NarrowingPredicate::Nullish, false, non_nullish),
            (NarrowingPredicate::Nullish, true, t),
            (null, false, with(InferredTypeData::Undefined)),
            (null, true, t),
            (undefined, false, with(InferredTypeData::Null)),
            (undefined, true, t),
            (NarrowingPredicate::Literal(string(db, "a")), true, t),
            (NarrowingPredicate::Literal(string(db, "a")), false, t),
            (
                NarrowingPredicate::Typeof(TypeofKind::String),
                true,
                intersection(db, &[t, InferredTypeData::String]),
            ),
            (NarrowingPredicate::Typeof(TypeofKind::String), false, t),
            (
                NarrowingPredicate::Typeof(TypeofKind::Undefined),
                true,
                intersection(db, &[t, InferredTypeData::Undefined]),
            ),
            (
                NarrowingPredicate::Typeof(TypeofKind::Undefined),
                false,
                with(InferredTypeData::Null),
            ),
            (
                NarrowingPredicate::Typeof(TypeofKind::Object),
                true,
                union(
                    db,
                    &[
                        intersection(db, &[t, InferredTypeData::ObjectKeyword]),
                        intersection(db, &[t, InferredTypeData::Null]),
                    ],
                ),
            ),
            (NarrowingPredicate::Typeof(TypeofKind::Object), false, t),
        ] {
            assert_eq!(
                narrow_type(db, t, predicate, positive),
                expected,
                "{predicate:?} {positive}"
            );
        }
        // A constraint that refers to another type parameter, here one
        // whose constraint refers back, keeps the type parameter.
        let u = InternedGenericTypeParameter::new(db, false, None, None, Text::new_static("U"));
        let v = type_parameter(
            db,
            "V",
            Some(union(
                db,
                &[
                    instance(db, InferredTypeData::Generic(u)),
                    InferredTypeData::Null,
                ],
            )),
        );
        assert_eq!(narrow_type(db, v, NarrowingPredicate::Truthy, true), v);
        // A nullish value that another member already is needs no `{}`
        // union.
        assert_eq!(
            narrow_type(
                db,
                union(db, &[t, InferredTypeData::Undefined]),
                null,
                false
            ),
            union(db, &[non_nullish, InferredTypeData::Undefined])
        );
    }

    #[test]
    fn type_parameters_narrow_by_the_values_of_their_constraint() {
        let db = &TestDb::default();
        let empty = object(db, Vec::new());
        let strings = type_parameter(db, "S", Some(InferredTypeData::String));
        let objects = type_parameter(db, "O", Some(InferredTypeData::ObjectKeyword));
        let optional = type_parameter(
            db,
            "U",
            Some(union(
                db,
                &[InferredTypeData::String, InferredTypeData::Undefined],
            )),
        );
        let mixed = type_parameter(
            db,
            "M",
            Some(union(
                db,
                &[InferredTypeData::String, InferredTypeData::Number],
            )),
        );
        let null = NarrowingPredicate::Literal(InferredTypeData::Null);
        for (ty, predicate, positive, expected) in [
            // Every value of the constraint passes.
            (strings, NarrowingPredicate::Truthy, true, strings),
            (
                strings,
                NarrowingPredicate::Typeof(TypeofKind::String),
                true,
                strings,
            ),
            (objects, NarrowingPredicate::Nullish, false, objects),
            // No value of the constraint passes, where TypeScript gives
            // `never`.
            (
                strings,
                NarrowingPredicate::Typeof(TypeofKind::Number),
                true,
                strings,
            ),
            (objects, NarrowingPredicate::Truthy, false, objects),
            // No value of the constraint is `null`.
            (optional, null, false, optional),
            (
                optional,
                NarrowingPredicate::Truthy,
                true,
                intersection(db, &[optional, empty]),
            ),
            (
                mixed,
                NarrowingPredicate::Typeof(TypeofKind::String),
                true,
                intersection(db, &[mixed, InferredTypeData::String]),
            ),
            (
                objects,
                NarrowingPredicate::Typeof(TypeofKind::Object),
                true,
                objects,
            ),
        ] {
            assert_eq!(
                narrow_type(db, ty, predicate, positive),
                expected,
                "{ty:?} {predicate:?} {positive}"
            );
        }
    }

    #[test]
    fn intersections_with_a_type_parameter_narrow_each_operand() {
        let db = &TestDb::default();
        let t = type_parameter(db, "T", None);
        let empty = object(db, Vec::new());
        let non_nullish = intersection(db, &[t, empty]);
        let t_string = intersection(db, &[t, InferredTypeData::String]);
        assert_eq!(
            narrow_type(
                db,
                non_nullish,
                NarrowingPredicate::Typeof(TypeofKind::String),
                true
            ),
            t_string
        );
        assert_eq!(
            narrow_type(db, t_string, NarrowingPredicate::Truthy, true),
            t_string
        );
        assert_eq!(
            narrow_type(db, non_nullish, NarrowingPredicate::Truthy, true),
            non_nullish
        );
        // `undefined` has no value that passes, so the intersection goes.
        let t_undefined = intersection(db, &[t, InferredTypeData::Undefined]);
        assert_eq!(
            narrow_type(
                db,
                union(db, &[non_nullish, t_undefined]),
                NarrowingPredicate::Literal(InferredTypeData::Undefined),
                false
            ),
            non_nullish
        );
        // A primitive operand keeps its object tags as they are.
        let tag = object(db, vec![member("brand", InferredTypeData::Number)]);
        let branded = IntersectionBuilder::without_merging(db)
            .add_all([t, InferredTypeData::String, tag])
            .build();
        assert_eq!(
            narrow_type(
                db,
                branded,
                NarrowingPredicate::Typeof(TypeofKind::String),
                true
            ),
            branded
        );
        // An object operand that isn't callable doesn't remove the
        // intersection from the branch of `typeof` with `"function"`.
        let t_foo = intersection(db, &[t, tag]);
        let InferredTypeData::Intersection(narrowed) = narrow_type(
            db,
            t_foo,
            NarrowingPredicate::Typeof(TypeofKind::Function),
            true,
        ) else {
            panic!("a value of `T & {{ brand: number }}` can be a function");
        };
        assert!(narrowed.types(db).contains(&tag));
        // Intersections without a type parameter stay as they are.
        let foo = named_interface(db, "Foo", &[], vec![member("a", InferredTypeData::Number)]);
        let objects = intersection(db, &[foo, tag]);
        assert_eq!(
            narrow_type(
                db,
                objects,
                NarrowingPredicate::Typeof(TypeofKind::String),
                true
            ),
            objects
        );
    }

    #[test]
    fn type_predicates_intersect_type_parameters_and_unrelated_types() {
        let db = &TestDb::default();
        let foo = named_interface(db, "Foo", &[], vec![member("a", InferredTypeData::Number)]);
        let bar = named_interface(db, "Bar", &[], vec![member("c", InferredTypeData::Number)]);
        let t = type_parameter(db, "T", None);
        let bars = type_parameter(db, "B", Some(bar));
        let foos = type_parameter(db, "F", Some(union(db, &[foo, bar])));
        let t_foo = intersection(db, &[t, foo]);
        for (ty, positive, expected) in [
            (t, true, t_foo),
            (t, false, t),
            // `T` is related to `Foo` through its constraint.
            (foos, true, intersection(db, &[foos, foo])),
            // The other member relates to `Foo`, so `T` goes.
            (union(db, &[t, foo]), true, foo),
            (union(db, &[t, foo]), false, t),
            // `string` relates to nothing, and `T` to `Foo` without a
            // constraint.
            (union(db, &[t, InferredTypeData::String]), true, t_foo),
            // No member remains: `Bar & Foo`.
            (bar, true, intersection(db, &[bar, foo])),
            (bar, false, bar),
            // `Foo` is not a subtype of the constraint of `B`, so the
            // intersection comes from the type as a whole.
            (bars, true, intersection(db, &[bars, foo])),
        ] {
            assert_eq!(
                narrow_predicate(db, ty, foo, positive),
                expected,
                "{ty:?} {positive}"
            );
        }
        // A type parameter that the target names is kept as it is, and is
        // not a subtype of `null`.
        assert_eq!(
            narrow_predicate(db, union(db, &[t, InferredTypeData::Null]), t, true),
            t
        );
        // An object type with a missing method relates to no class.
        let ready = object(db, vec![member("ready", InferredTypeData::String)]);
        let r = class(
            db,
            "R",
            None,
            vec![
                optional_member("ready", InferredTypeData::String),
                TypeMember {
                    kind: TypeMemberKind::Named(Text::new_static("check")),
                    ty: method(db, "check", InferredTypeData::Boolean),
                },
            ],
        );
        assert_eq!(
            narrow_predicate(db, instance(db, r), ready, true),
            intersection(db, &[instance(db, r), ready])
        );
    }

    #[test]
    fn instanceof_intersects_type_parameters_and_unrelated_classes() {
        let db = &TestDb::default();
        let a = class(db, "A", None, vec![member("a", InferredTypeData::Number)]);
        let b = class(
            db,
            "B",
            Some(a),
            vec![member("b", InferredTypeData::Number)],
        );
        let c = class(db, "C", None, vec![member("c", InferredTypeData::Number)]);
        let (ia, ib, ic) = (instance(db, a), instance(db, b), instance(db, c));
        let t = type_parameter(db, "T", None);
        let derived = type_parameter(db, "D", Some(ib));
        let parent = type_parameter(db, "P", Some(ia));
        for (ty, constructor, positive, expected) in [
            (t, a, true, intersection(db, &[t, ia])),
            (t, a, false, t),
            (union(db, &[t, ic]), a, true, intersection(db, &[t, ia])),
            (union(db, &[t, ic]), a, false, union(db, &[t, ic])),
            // The constraint of `D` is derived from `A`.
            (derived, a, true, derived),
            (derived, a, false, InferredTypeData::NeverKeyword),
            // `B` is derived from the constraint of `P`.
            (parent, b, true, intersection(db, &[parent, ib])),
            // No member remains: `C & A`.
            (ic, a, true, intersection(db, &[ic, ia])),
            (ic, a, false, ic),
        ] {
            assert_eq!(
                narrow_instance(db, ty, constructor, positive),
                expected,
                "{ty:?} {positive}"
            );
        }
    }

    #[test]
    fn constructor_comparisons_keep_instances_of_the_class_itself() {
        let db = &TestDb::default();
        let a = class(db, "A", None, vec![member("a", InferredTypeData::Number)]);
        let b = class(
            db,
            "B",
            Some(a),
            vec![member("b", InferredTypeData::Number)],
        );
        let c = class(db, "C", None, vec![member("a", InferredTypeData::Number)]);
        let (ia, ib, ic) = (instance(db, a), instance(db, b), instance(db, c));
        let shape = object(db, vec![member("a", InferredTypeData::Number)]);
        let narrow = |ty, constructor| super::narrow_by_constructor(db, ty, constructor, |ty| ty);
        // A derived class and a class with the same members are other
        // constructors, and so is the constructor of an object type.
        assert_eq!(
            narrow(union(db, &[ia, ib, ic, shape, InferredTypeData::Null]), a),
            ia
        );
        assert_eq!(narrow(InferredTypeData::AnyKeyword, a), ia);
        assert_eq!(
            narrow(InferredTypeData::UnknownKeyword, a),
            InferredTypeData::UnknownKeyword
        );
        // A construct signature without a `prototype` property narrows
        // nothing.
        let constructor = InferredTypeData::Constructor(InternedConstructor::new(
            db,
            Box::default(),
            Box::default(),
            Some(ia),
        ));
        let value = union(db, &[ia, ic]);
        assert_eq!(narrow(value, constructor), value);
    }

    #[test]
    fn private_names_narrow_to_the_class_that_declares_them() {
        let db = &TestDb::default();
        let p = class(db, "P", None, vec![member("#p", InferredTypeData::Number)]);
        let q = class(db, "Q", Some(p), Vec::new());
        let c = class(db, "C", None, vec![member("c", InferredTypeData::Number)]);
        let (ip, iq, ic) = (instance(db, p), instance(db, q), instance(db, c));
        let narrow = |ty, is_static, positive| {
            super::narrow_by_private_name(db, ty, p, is_static, positive, |ty| ty)
        };
        assert_eq!(narrow(union(db, &[iq, ic]), false, true), iq);
        assert_eq!(narrow(union(db, &[iq, ic]), false, false), ic);
        assert_eq!(narrow(ic, false, true), intersection(db, &[ic, ip]));
        assert_eq!(narrow(InferredTypeData::UnknownKeyword, false, true), ip);
        // A static name is the class's own. Where the test fails, the class
        // goes only when it is the whole type.
        assert_eq!(narrow(union(db, &[p, ic]), true, true), p);
        let class_or_instance = union(db, &[p, ic]);
        assert_eq!(narrow(class_or_instance, true, false), class_or_instance);
        assert_eq!(narrow(p, true, false), InferredTypeData::NeverKeyword);
    }

    #[test]
    fn type_parameters_relate_only_through_their_constraints() {
        let db = &TestDb::default();
        let foo = named_interface(db, "Foo", &[], vec![member("a", InferredTypeData::Number)]);
        let t = type_parameter(db, "T", None);
        let strings = type_parameter(db, "S", Some(InferredTypeData::String));
        let empty = object(db, Vec::new());
        let resolve = &mut |ty| ty;
        for (source, target, subtype) in [
            (InferredTypeData::String, t, Assignability::NotAssignable),
            (t, InferredTypeData::String, Assignability::NotAssignable),
            (t, foo, Assignability::NotAssignable),
            (t, InferredTypeData::Null, Assignability::NotAssignable),
            (t, t, Assignability::Assignable),
            (
                t,
                union(db, &[t, InferredTypeData::Null]),
                Assignability::Assignable,
            ),
            (
                strings,
                union(db, &[InferredTypeData::String, InferredTypeData::Number]),
                Assignability::Assignable,
            ),
            (
                strings,
                InferredTypeData::Number,
                Assignability::NotAssignable,
            ),
            (
                InferredTypeData::Null,
                intersection(db, &[t, empty]),
                Assignability::NotAssignable,
            ),
            (intersection(db, &[t, empty]), t, Assignability::Assignable),
            (
                t,
                InferredTypeData::UnknownKeyword,
                Assignability::Assignable,
            ),
        ] {
            assert_eq!(
                is_subtype(db, source, target, resolve),
                subtype,
                "{source:?} {target:?}"
            );
            assert_eq!(
                is_assignable_in_generic_body(db, source, target, resolve),
                subtype,
                "{source:?} {target:?}"
            );
        }
        // Elsewhere, a type parameter may still be replaced, so only a match
        // through the constraint is conclusive.
        assert_eq!(
            crate::is_assignable_to(db, InferredTypeData::String, t, |ty| ty),
            None
        );
        assert_eq!(crate::is_assignable_to(db, t, foo, |ty| ty), None);
        assert_eq!(
            crate::is_assignable_to(db, strings, InferredTypeData::String, |ty| ty),
            Some(true)
        );
    }
}

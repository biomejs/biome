//! Conservative branch filtering of normalized types and unions by runtime
//! tests.

use std::borrow::Cow;

use biome_js_syntax::numbers::{
    canonicalize_js_bigint_literal, parse_js_number_with_single_rounding,
};
use biome_rowan::Text;

use crate::generated::global_types::ids::FUNCTION_ID_GLOBAL_TYPE_ID;
use crate::globals::{
    GLOBAL_ANY_KEYWORD_ID, GLOBAL_BIGINT_KEYWORD_ID, GLOBAL_BOOLEAN_KEYWORD_ID,
    GLOBAL_NULL_KEYWORD_ID, GLOBAL_NUMBER_KEYWORD_ID, GLOBAL_OBJECT_KEYWORD_ID,
    GLOBAL_STRING_KEYWORD_ID, GLOBAL_SYMBOL_KEYWORD_ID, GLOBAL_UNDEFINED_ID, GLOBAL_UNKNOWN_ID,
    GLOBAL_UNKNOWN_KEYWORD_ID,
};
use crate::interned_types::{InternedLiteral, Literal};
use crate::resolved::InferredTypeData;
use crate::type_relation::{
    Assignability, InstanceType, PropertyLookup, PropertyPresence, TypeResolver, find_property,
    instance_types, is_derived_from, is_empty_object_type, is_subtype, primitive_can_satisfy,
    property_presence, resolve_wrapped_type, widened_literal,
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
}

/// Returns whether `ty` carries no information that a runtime test could
/// filter: `any` and the internal `Unknown`.
///
/// Either type absorbs every union it joins, and [`narrow_type`] returns it
/// unchanged for every predicate and branch. No test can therefore refine it or
/// prove a branch impossible, so callers can skip collecting tests for it.
/// `never` is excluded: it also stays unchanged, but it already proves that a
/// branch is impossible.
pub fn is_narrowing_invariant(ty: InferredTypeData<'_>) -> bool {
    matches!(ty, InferredTypeData::AnyKeyword | InferredTypeData::Unknown)
}

/// Returns whether a collected type reference already denotes `any` or the
/// internal `Unknown`, without resolving it.
///
/// This is the resolution-free counterpart of [`is_narrowing_invariant`];
/// `raw_types` is the module table that local references index. It returns
/// false for references that only resolve to such a type, such as an alias.
pub fn is_raw_narrowing_invariant(reference: &TypeReference, raw_types: &[RawTypeData]) -> bool {
    let TypeReference::Resolved(id) = reference else {
        return false;
    };
    match id {
        RawTypeId::Global(_) => *id == GLOBAL_UNKNOWN_ID || *id == GLOBAL_ANY_KEYWORD_ID,
        RawTypeId::Local(local) => matches!(
            raw_types.get(local.index()),
            Some(RawTypeData::Unknown | RawTypeData::AnyKeyword)
        ),
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
                match raw_types.get(local.index())? {
                    RawTypeData::Reference(next) => reference = next,
                    data => return Some(RawReferenceTarget::Local(data)),
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
/// the case for `any`, `Unknown`, generic types, and unresolved handles. Other
/// members follow these rules:
///
/// - A `void` member becomes `Unknown`, because a `void` value can be any value
///   at runtime.
/// - `unknown` narrows only in a true branch: to the primitive type of a
///   primitive `typeof` test, or to the literal of an equality test. TypeScript
///   also narrows it for `"object"` and `"function"`, so Biome is wider there.
/// - The false branch of an equality test keeps a primitive type such as
///   `string`, because a type can't exclude a single literal from it. The false
///   branch of a truthiness test keeps `string`, `number`, and `bigint` for the
///   same reason, as TypeScript does at a read. [`falsy_values`] gives the
///   narrower type of the value that `&&` returns.
/// - An equality test leaves `ty` unchanged when its literal is a template
///   literal, a string with escapes, or longer than 1024 bytes, because Biome
///   doesn't decode those.
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
///   primitive, a function, a tuple, or a class instance. When no member
///   declares the property or has such an index signature, `ty` is returned
///   unchanged. TypeScript gives `ty & Record<name, unknown>` instead.
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
    name: &str,
    positive: bool,
    mut resolve: impl FnMut(InferredTypeData<'db>) -> InferredTypeData<'db>,
) -> InferredTypeData<'db> {
    if is_narrowing_invariant(ty) || ty == InferredTypeData::NeverKeyword || name.starts_with('#') {
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
            property_presence(db, *member, name, resolve)
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
        return ty;
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
///   with `class B extends A`. When no member remains, the result is the
///   instance type if it is a subtype of `ty`, as `A` is of
///   `{ a: number } | string` with `class A { a = 1 }`, and `ty` otherwise,
///   where TypeScript gives the intersection of the two. `unknown` becomes
///   the instance type.
/// - When `positive` is false, the members derived from the instance type
///   are removed, which can give `never`. As in TypeScript, nothing is
///   removed when `constructor` has several instance types, or when the
///   instance type is `{}` or not an object type.
///
/// TypeScript gives an instance of a generic class `any` type arguments,
/// such as `Box<any>`. Biome has no such type, so `unknown` stays `unknown`,
/// a member that an instance of a generic class is derived from stays as it
/// is, and the result is `ty` when no member remains.
///
/// Returns `ty` unchanged when Biome can't tell the result:
///
/// - [`instance_types`] can't tell the instance types, as for `Object`,
///   `Function`, a generic construct signature, or a class with a static
///   `[Symbol.hasInstance]` method;
/// - a member of the union is a generic, or `unknown` when `positive` is
///   true;
/// - [`is_derived_from`] can't decide for a pair of types;
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
    // Gives the union of the instance types, or `None` when one is a generic
    // class without type arguments, because a read must never have that type.
    let instance_type = || {
        instances
            .iter()
            .map(|instance| (!instance.is_generic).then_some(instance.target))
            .collect::<Option<Vec<_>>>()
            .map(|types| InferredTypeData::union_from_types(db, types))
    };
    if ty == InferredTypeData::UnknownKeyword {
        return instance_type();
    }
    let mut remaining = MAX_NARROWING_STEPS;
    let members = union_members(db, ty, &mut remaining)?;
    if members.contains(&InferredTypeData::UnknownKeyword) {
        return None;
    }
    let mut narrowed = Vec::new();
    for instance in instances {
        for member in &members {
            remaining = remaining.checked_sub(1)?;
            if is_derived_from(db, *member, instance.target, resolve)? {
                narrowed.push(*member);
            } else if is_derived_from(db, instance.target, *member, resolve)? {
                // A member keeps its own type arguments, which an instance
                // of a generic class lacks.
                narrowed.push(if instance.is_generic {
                    *member
                } else {
                    instance.target
                });
            }
        }
    }
    if !narrowed.is_empty() {
        return Some(InferredTypeData::union_from_types(db, narrowed));
    }
    let instance_type = instance_type()?;
    (is_subtype(db, instance_type, ty, resolve) == Assignability::Assignable)
        .then_some(instance_type)
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
        if !is_derived_from(db, *member, instance.target, resolve)? {
            kept.push(*member);
        }
    }
    Some(if kept.len() == members.len() {
        ty
    } else {
        InferredTypeData::union_from_types(db, kept)
    })
}

/// Returns whether `declared` is `unknown` or has a member that is neither
/// `null`, `undefined`, nor a primitive type. Only then can
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
///   `target`. When no member remains, the result is `target` if it is a
///   subtype of `ty`, and `ty` otherwise, where TypeScript gives an
///   intersection such as `(string | number) & Foo`.
/// - When `positive` is false, the result removes each member that the true
///   branch keeps as it is rather than replacing it with a member of
///   `target`. As in TypeScript, this can give `never`.
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
/// `ty` is returned unchanged in these cases:
///
/// - A comparison of two members can't tell whether either is a subtype of
///   the other, as for generics, classes that can declare private members,
///   and literal types that TypeScript may widen. [`is_subtype`] describes
///   these comparisons.
/// - `ty` is a union with `unknown`.
/// - Biome may reach two classes with the same contents as one.
/// - `target` has `any` or an undetermined member.
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
    if candidates
        .iter()
        .any(|candidate| is_narrowing_invariant(*candidate))
    {
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
    }
    if positive {
        return if narrowed.is_empty() {
            (is_subtype(db, target, ty, resolve) == Assignability::Assignable).then_some(target)
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
        // Biome identifies a class by its contents and by the handle that
        // names it. Two classes with the same contents and no handle, as in
        // two namespaces of one module, may be two declarations, which
        // TypeScript tells apart when they have private members.
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
        return narrow_non_union(db, ty, predicate, positive, resolve);
    }

    let mut pending = vec![ty];
    let mut remaining = MAX_NARROWING_STEPS;
    let mut narrowed = Vec::new();
    let mut changed = false;
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
            let result = narrow_non_union(db, current, predicate, positive, resolve);
            changed |= result != current;
            if result != InferredTypeData::NeverKeyword {
                narrowed.push(result);
            }
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

enum PreparedPredicate<'db> {
    Truthy,
    /// Keeps the values that [`falsy_values`] describes in either branch.
    FalsyValues,
    Nullish,
    Typeof(TypeofKind),
    Literal(InferredTypeData<'db>, LiteralValue<'db>),
}

impl<'db> PreparedPredicate<'db> {
    /// Returns `predicate` with its literal decoded, or `None` for a literal
    /// that can't be decoded, which leaves every type unchanged.
    fn new(db: &'db dyn TypeDb, predicate: NarrowingPredicate<'db>) -> Option<Self> {
        Some(match predicate {
            NarrowingPredicate::Truthy => Self::Truthy,
            NarrowingPredicate::Nullish => Self::Nullish,
            NarrowingPredicate::Typeof(kind) => Self::Typeof(kind),
            NarrowingPredicate::Literal(literal) => {
                Self::Literal(literal, literal_value(db, literal)?)
            }
        })
    }
}

fn narrow_non_union<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    predicate: &PreparedPredicate<'db>,
    positive: bool,
    resolve: &mut TypeResolver<'db, '_>,
) -> InferredTypeData<'db> {
    if ty == InferredTypeData::VoidKeyword {
        return InferredTypeData::Unknown;
    }
    if ty == InferredTypeData::UnknownKeyword && positive {
        return match predicate {
            PreparedPredicate::Typeof(TypeofKind::Undefined) => InferredTypeData::Undefined,
            PreparedPredicate::Typeof(TypeofKind::Boolean) => InferredTypeData::Boolean,
            PreparedPredicate::Typeof(TypeofKind::Number) => InferredTypeData::Number,
            PreparedPredicate::Typeof(TypeofKind::BigInt) => InferredTypeData::BigInt,
            PreparedPredicate::Typeof(TypeofKind::String) => InferredTypeData::String,
            PreparedPredicate::Typeof(TypeofKind::Symbol) => InferredTypeData::Symbol,
            PreparedPredicate::Literal(_, LiteralValue::Number(value)) if value.is_nan() => {
                InferredTypeData::NeverKeyword
            }
            PreparedPredicate::Literal(literal, _) => *literal,
            _ => ty,
        };
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
            _ => narrow_non_union(db, ty, &PreparedPredicate::Truthy, false, resolve),
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
    }
}

/// Narrows the object type `ty` by a falsy or `typeof` test that the type's
/// truthiness or `typeof` result does not decide.
///
/// Returns `NeverKeyword` when no value that passes the test can have `ty`. In
/// the true branch of `typeof`, returns the tested kind's type when every value
/// of that kind has `ty`. Returns `ty` unchanged otherwise, including when `ty`
/// is not an object type or the branch keeps object types. [`narrow_type`]
/// lists the branches that compare object types.
fn narrow_object<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    predicate: &PreparedPredicate<'db>,
    positive: bool,
    resolve: &mut TypeResolver<'db, '_>,
) -> InferredTypeData<'db> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generated::global_types::ids::{
        ARRAY_ID_GLOBAL_TYPE_ID, ERROR_ID_GLOBAL_TYPE_ID, OBJECT_ID_GLOBAL_TYPE_ID,
    };
    use crate::interned_types::{
        InternedClass, InternedConstructor, InternedFunction, InternedGenericTypeParameter,
        InternedInterface, InternedIntersection, InternedObject, InternedTuple,
        InternedTypeInstance, InternedTypeOperatorType, InternedTypeofType, InternedUnion,
        LocalTypeHandle, LocalTypeId, ModuleKey, ReturnType, TupleElementType, TypeMember,
        TypeMemberKind,
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
            (InferredTypeData::Boolean, boolean_literal(&db, true)),
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
        for ty in [
            InferredTypeData::Unknown,
            InferredTypeData::AnyKeyword,
            InferredTypeData::NeverKeyword,
        ] {
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
        for unknown in [InferredTypeData::Unknown, InferredTypeData::UnknownKeyword] {
            let ty = InferredTypeData::Union(InternedUnion::new(
                &db,
                Box::from([unknown, InferredTypeData::Null]),
            ));
            assert_eq!(
                narrow_type(&db, ty, NarrowingPredicate::Nullish, false),
                unknown
            );
        }
    }

    #[test]
    fn only_any_and_inference_failures_are_narrowing_invariant() {
        let db = TestDb::default();
        for ty in [InferredTypeData::AnyKeyword, InferredTypeData::Unknown] {
            assert!(is_narrowing_invariant(ty), "{ty:?}");
        }
        for ty in [
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
    fn raw_narrowing_invariance_needs_no_resolution() {
        let local = TypeReference::Resolved(RawTypeId::Local(crate::TypeId::new(0)));
        for (reference, raw_types, expected) in [
            (TypeReference::unknown(), vec![], true),
            (TypeReference::Resolved(GLOBAL_ANY_KEYWORD_ID), vec![], true),
            (local.clone(), vec![RawTypeData::AnyKeyword], true),
            (local.clone(), vec![RawTypeData::Unknown], true),
            (local.clone(), vec![RawTypeData::UnknownKeyword], false),
            (local.clone(), vec![RawTypeData::String], false),
            (local, vec![], false),
        ] {
            assert_eq!(
                is_raw_narrowing_invariant(&reference, &raw_types),
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
            None,
            Box::default(),
            members.into_boxed_slice(),
            Some(Text::new_static("K")),
            false,
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
        let object = InferredTypeData::Object(InternedObject::new(
            &db,
            None,
            Box::default(),
            false,
            ObjectKind::Shape,
        ));
        for ty in [
            object,
            InferredTypeData::instance_of(&db, object, Box::default()),
            interface(&db, &[], Vec::new()),
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
                    TypeofKind::Object => ty,
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
            None,
            Box::default(),
            Box::from([member("k", InferredTypeData::Number)]),
            Some(Text::new_static("K")),
            false,
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

    /// Narrows `ty` by `name in ty`, with a resolver that returns its input.
    fn narrow_in<'db>(
        db: &'db dyn TypeDb,
        ty: InferredTypeData<'db>,
        name: &str,
        positive: bool,
    ) -> InferredTypeData<'db> {
        super::narrow_by_in_operator(db, ty, name, positive, |ty| ty)
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
    fn in_keeps_the_type_when_no_member_declares_the_property() {
        let db = &TestDb::default();
        let a = object(db, vec![member("a", InferredTypeData::Number)]);
        let b = object(db, vec![member("b", InferredTypeData::Number)]);
        let numbers = object(db, vec![index_signature(InferredTypeData::Number)]);
        let class = class_instance(db, vec![member("p", InferredTypeData::Number)]);
        for ty in [
            union(db, &[a, b]),
            // A numeric index signature accepts only numeric names.
            union(db, &[numbers, b]),
            // A class can have a property that Biome doesn't collect.
            union(db, &[class, b]),
        ] {
            assert_eq!(narrow_in(db, ty, "c", true), ty);
            assert_eq!(narrow_in(db, ty, "c", false), ty);
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
            parent,
            Box::default(),
            members.into_boxed_slice(),
            Some(Text::new_static(name)),
            false,
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
        let generic_class = InferredTypeData::Class(InternedClass::new(
            db,
            Box::from([generic]),
            None,
            Box::default(),
            Box::default(),
            Some(Text::new_static("Box")),
            false,
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
            // TypeScript gives `Box<any>`.
            (InferredTypeData::UnknownKeyword, generic_class),
            (value, has_instance),
            (value, inherits_has_instance),
            (value, generic_constructor),
            (value, object_class),
            (union(db, &[generic, ic]), a),
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
    fn only_object_types_and_unknown_may_narrow_by_instanceof() {
        let db = &TestDb::default();
        let a = instance(db, class(db, "A", None, Vec::new()));
        for (ty, expected) in [
            (union(db, &[a, InferredTypeData::String]), true),
            (a, true),
            (InferredTypeData::ObjectKeyword, true),
            (InferredTypeData::UnknownKeyword, true),
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
        let generic = InferredTypeData::Generic(InternedGenericTypeParameter::new(
            db,
            false,
            None,
            None,
            Text::new_static("T"),
        ));
        for (ty, target) in [
            (union(db, &[generic, InferredTypeData::String]), foo),
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
        // TypeScript widens `{ a: 1 }` to `{ a: number }`, which is not a
        // subtype of `{ a: 1 }`, so the `else` branch keeps it.
        let ty = union(db, &[literal, InferredTypeData::String]);
        assert_eq!(narrow_predicate(db, ty, shape, true), literal);
        assert_eq!(narrow_predicate(db, ty, shape, false), ty);
        // A type literal keeps its literal types.
        let ty = union(db, &[shape, InferredTypeData::String]);
        assert_eq!(
            narrow_predicate(db, ty, shape, false),
            InferredTypeData::String
        );
        // TypeScript types a class field initialized with `1` as `number`, so
        // `{ a: number }` is a subtype of the class.
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
}

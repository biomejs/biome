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
    GLOBAL_NULL_KEYWORD_ID, GLOBAL_NUMBER_KEYWORD_ID, GLOBAL_STRING_KEYWORD_ID,
    GLOBAL_SYMBOL_KEYWORD_ID, GLOBAL_UNKNOWN_ID,
};
use crate::interned_types::{InternedLiteral, Literal};
use crate::resolved::InferredTypeData;
use crate::type_relation::{Assignability, TypeResolver, primitive_can_satisfy};
use crate::{ObjectKind, RawTypeData, RawTypeId, TypeDb, TypeReference, TypeReferenceQualifier};

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

/// Limits the chain of raw references that [`raw_primitive_target`] follows.
const MAX_RAW_REFERENCE_STEPS: usize = 16;

/// What a collected type reference can resolve to, as far as raw tables tell.
#[derive(Debug)]
pub enum RawPrimitiveTarget<'a> {
    /// The reference resolves to a primitive type or a primitive literal.
    Primitive,
    /// The reference cannot resolve to a primitive type or a primitive literal.
    Other,
    /// The reference names a type without type arguments. Only the scope that
    /// declares the name tells whether it is a type alias for a primitive.
    Named(&'a TypeReferenceQualifier),
    /// The reference queries the type of a value, as in `typeof READY`. Only
    /// inference tells whether the value has a primitive type or a primitive
    /// literal type.
    Typeof,
}

/// Classifies whether a collected type reference can resolve to a primitive
/// type or a primitive literal, without resolving it.
///
/// `raw_types` is the module table that local references index. A chain of
/// raw references longer than the step limit is [`RawPrimitiveTarget::Other`].
pub fn raw_primitive_target<'a>(
    reference: &'a TypeReference,
    raw_types: &'a [RawTypeData],
) -> RawPrimitiveTarget<'a> {
    let mut reference = reference;
    for _ in 0..MAX_RAW_REFERENCE_STEPS {
        let local = match reference {
            TypeReference::Qualifier(qualifier) if qualifier.type_parameters.is_empty() => {
                return RawPrimitiveTarget::Named(qualifier);
            }
            TypeReference::Qualifier(_) | TypeReference::Import(_) => {
                return RawPrimitiveTarget::Other;
            }
            TypeReference::Resolved(id @ RawTypeId::Global(_)) => {
                let primitive = [
                    GLOBAL_BIGINT_KEYWORD_ID,
                    GLOBAL_BOOLEAN_KEYWORD_ID,
                    GLOBAL_NULL_KEYWORD_ID,
                    GLOBAL_NUMBER_KEYWORD_ID,
                    GLOBAL_STRING_KEYWORD_ID,
                    GLOBAL_SYMBOL_KEYWORD_ID,
                ]
                .contains(id);
                return if primitive {
                    RawPrimitiveTarget::Primitive
                } else {
                    RawPrimitiveTarget::Other
                };
            }
            TypeReference::Resolved(RawTypeId::Local(local)) => local,
        };
        let primitive = match raw_types.get(local.index()) {
            Some(RawTypeData::Reference(next)) => {
                reference = next;
                continue;
            }
            Some(
                RawTypeData::BigInt
                | RawTypeData::Boolean
                | RawTypeData::Null
                | RawTypeData::Number
                | RawTypeData::String
                | RawTypeData::Symbol
                | RawTypeData::Undefined,
            ) => true,
            Some(RawTypeData::Literal(literal)) => matches!(
                literal.as_ref(),
                crate::Literal::BigInt(_)
                    | crate::Literal::Boolean(_)
                    | crate::Literal::Number(_)
                    | crate::Literal::String(_)
            ),
            // A reference by name to a type collects as an instance of it.
            Some(RawTypeData::InstanceOf(instance)) if instance.type_parameters.is_empty() => {
                reference = &instance.ty;
                continue;
            }
            Some(RawTypeData::TypeofType(_)) => return RawPrimitiveTarget::Typeof,
            _ => false,
        };
        return if primitive {
            RawPrimitiveTarget::Primitive
        } else {
            RawPrimitiveTarget::Other
        };
    }
    RawPrimitiveTarget::Other
}

/// Retains the values of `ty` that can pass a runtime test.
///
/// `positive` selects the test's true branch; `false` selects its complement.
/// Inputs should be normalized. Unresolved handles, wrappers, unsupported
/// structures, and [narrowing-invariant](is_narrowing_invariant) types stay
/// unchanged. A `void` result becomes `Unknown` because its runtime value is
/// unspecified. Explicit TypeScript `unknown` supports positive primitive
/// `typeof` and literal tests. Only proven empty results become `NeverKeyword`.
///
/// Union traversal visits at most 256 types and returns the original input on
/// exhaustion. Literal decoding accepts at most 1024 bytes per operand. Escaped
/// strings and template literals are not decoded. Broad primitive types cannot
/// exclude individual literals, and the false branch of a truthiness test keeps
/// `string`, `number`, and `bigint` unchanged, as TypeScript does at a read.
/// [`falsy_values`] gives the narrower type of the value that `&&` returns.
///
/// An object type that a primitive or a function might satisfy, such as
/// `{ length: number }`, which every string has, is compared with the apparent
/// types of the values that pass a falsy or `typeof` test:
///
/// - The false branch of a truthiness test removes it when no falsy string,
///   number, boolean, or bigint can have it.
/// - The true branch of `typeof` with a primitive kind or `"function"` removes
///   it when no value of that kind can have it, and replaces it with the
///   primitive type or `Function` when every value of that kind has it, as
///   TypeScript does.
/// - The true branch of `typeof` with `"undefined"` removes it.
/// - The false branch of `typeof` with `"object"` removes it when no primitive
///   and no function can have it.
///
/// Other branches keep it, and so does a comparison that cannot decide. Such
/// object types are type literals, interfaces, and instances of classes and
/// interfaces; generics and unresolved handles stay unchanged. `resolve`
/// replaces local and global handles with the types they stand for, as for
/// [`InternedExtendsType::select_branch`]. It is called only to compare
/// object types, so other narrowing never calls it.
///
/// [`InternedExtendsType::select_branch`]: crate::interned_types::InternedExtendsType::select_branch
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
    let predicate = match predicate {
        NarrowingPredicate::Truthy => PreparedPredicate::Truthy,
        NarrowingPredicate::Nullish => PreparedPredicate::Nullish,
        NarrowingPredicate::Typeof(kind) => PreparedPredicate::Typeof(kind),
        NarrowingPredicate::Literal(literal) => {
            let Some(value) = literal_value(db, literal) else {
                return ty;
            };
            PreparedPredicate::Literal(literal, value)
        }
    };
    narrow_prepared(db, ty, &predicate, positive, &mut resolve)
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

/// Narrows `ty` by a falsy or `typeof` test that its kind alone does not
/// decide, when it is an object type, as described by [`narrow_type`].
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

/// Returns the type that every value whose `typeof` is `kind` has, or `None`
/// for `"object"`.
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
    use crate::interned_types::{
        InternedClass, InternedFunction, InternedGenericTypeParameter, InternedInterface,
        InternedIntersection, InternedObject, InternedTypeInstance, InternedTypeofType,
        InternedUnion, LocalTypeHandle, LocalTypeId, ModuleKey, ReturnType, TypeMember,
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

    /// Calls [`super::narrow_type`] with a resolver that returns its input,
    /// because these tests build types without local handles.
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

    /// Returns the interface `name` with `members`, extending `parents`.
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
        // Instances of built-in classes are never primitives, and they are
        // never functions either.
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
        // `Function` replaces an object type only when it is the only member
        // of the branch: other members stay as they are.
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
}

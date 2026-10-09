//! Conservative branch filtering of normalized primitive types and unions.

use std::borrow::Cow;

use biome_js_syntax::numbers::{
    canonicalize_js_bigint_literal, parse_js_number_with_single_rounding,
};
use biome_rowan::Text;

use crate::globals::{GLOBAL_ANY_KEYWORD_ID, GLOBAL_UNKNOWN_ID};
use crate::interned_types::{InternedLiteral, Literal};
use crate::resolved::InferredTypeData;
use crate::{RawTypeData, RawTypeId, TypeDb, TypeReference};

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
/// exclude individual literals; in particular, falsy `number` stays `number`
/// because both zero and `NaN` are possible.
pub fn narrow_type<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    predicate: NarrowingPredicate<'db>,
    positive: bool,
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
    if !matches!(ty, InferredTypeData::Union(_)) {
        return narrow_non_union(db, ty, &predicate, positive);
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
            let result = narrow_non_union(db, current, &predicate, positive);
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
    Nullish,
    Typeof(TypeofKind),
    Literal(InferredTypeData<'db>, LiteralValue<'db>),
}

fn narrow_non_union<'db>(
    db: &'db dyn TypeDb,
    ty: InferredTypeData<'db>,
    predicate: &PreparedPredicate<'db>,
    positive: bool,
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
            InferredTypeData::String if !positive => InferredTypeData::Literal(
                InternedLiteral::new(db, Literal::String(Text::new_static("").into())),
            ),
            InferredTypeData::BigInt if !positive => InferredTypeData::Literal(
                InternedLiteral::new(db, Literal::BigInt(Text::new_static("0n"))),
            ),
            _ => retain_if(ty, truthiness(db, ty), positive),
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
            retain_if(ty, matches, positive)
        }
        PreparedPredicate::Literal(literal, value) => {
            narrow_literal(db, ty, *literal, value, positive)
        }
    }
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
        InternedFunction, InternedGenericTypeParameter, InternedIntersection, InternedObject,
        InternedTypeInstance, InternedTypeofType, InternedUnion, LocalTypeHandle, LocalTypeId,
        ModuleKey, ReturnType,
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
        fn parsed_source_for_path(
            &self,
            _path: &camino::Utf8Path,
        ) -> Option<biome_db::ParsedSource> {
            None
        }
    }

    #[salsa::db]
    impl TypeDb for TestDb {}

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
            assert_eq!(
                narrow_type(
                    &db,
                    InferredTypeData::Number,
                    NarrowingPredicate::Truthy,
                    positive
                ),
                InferredTypeData::Number
            );
        }
        assert_eq!(
            narrow_type(
                &db,
                InferredTypeData::String,
                NarrowingPredicate::Truthy,
                false
            ),
            string(&db, "")
        );
        assert_eq!(
            narrow_type(
                &db,
                InferredTypeData::BigInt,
                NarrowingPredicate::Truthy,
                false
            ),
            bigint(&db, "0n")
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

    #[test]
    fn structural_objects_are_non_nullish_but_may_be_primitive() {
        let db = TestDb::default();
        let object =
            InferredTypeData::Object(InternedObject::new(&db, None, Box::default(), false));
        for ty in [
            object,
            InferredTypeData::instance_of(&db, object, Box::default()),
        ] {
            assert_eq!(
                narrow_type(&db, ty, NarrowingPredicate::Nullish, true),
                InferredTypeData::NeverKeyword
            );
            assert_eq!(narrow_type(&db, ty, NarrowingPredicate::Truthy, false), ty);
            assert_eq!(
                narrow_type(
                    &db,
                    ty,
                    NarrowingPredicate::Typeof(TypeofKind::String),
                    true
                ),
                ty
            );
            for kind in [TypeofKind::Object, TypeofKind::Function] {
                for positive in [false, true] {
                    assert_eq!(
                        narrow_type(&db, ty, NarrowingPredicate::Typeof(kind), positive),
                        ty
                    );
                }
            }
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

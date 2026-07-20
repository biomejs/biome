use crate::TypeDb;
use crate::interned_types::{ConditionalType, Literal, ReturnType, TypeData};
use crate::return_type_relation::{
    ReturnTypeRelation, compare_declared_return_type_owned,
    is_escape_hatch as relation_is_escape_hatch, promise_inner as relation_promise_inner,
};
use crate::type_traversal::{DepthFirstVisitor, TraversalOutcome, VisitContext};
use biome_js_syntax::numbers::canonicalize_js_bigint_literal;
use biome_rowan::Text;
use rustc_hash::FxHashSet;
use std::{borrow::Cow, collections::VecDeque, fmt, ops::ControlFlow};

const MAX_TYPE_VARIANT_STEPS: usize = 1024;
const MAX_TYPE_RELATION_DEPTH: usize = 50;
const MAX_PROMISE_TYPE_STEPS: usize = 64;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum InferredSwitchCase {
    Boolean,
    BooleanLiteral(bool),
    BigInt(Text),
    Number(Text),
    String(Text),
    Null,
    Undefined,
    Symbol,
    UnsupportedLiteral,
}

/// Describes why traversing nested types could not produce a result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypeTraversalError {
    /// A type required for the result could not be resolved.
    UnresolvedType,
    /// A nested type refers back to a type that is still being visited.
    RecursiveType,
    /// The traversal reached its maximum number of visited types.
    LimitExceeded,
}

impl fmt::Display for TypeTraversalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::UnresolvedType => "a type could not be resolved",
            Self::RecursiveType => "a recursive type was encountered",
            Self::LimitExceeded => "the type traversal limit was exceeded",
        })
    }
}

impl std::error::Error for TypeTraversalError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StringificationMode {
    Join,
    ToString,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StringificationUsefulness {
    Always,
    Sometimes,
    Never,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IgnoredPrimitiveTypes {
    pub string: bool,
    pub number: bool,
    pub boolean: bool,
    pub bigint: bool,
}

/// A Salsa-backed type value returned by type inference.
#[derive(Clone, Copy)]
pub struct InferredType<'db> {
    db: &'db dyn TypeDb,
    data: TypeData<'db>,
}

impl<'db> InferredType<'db> {
    pub const fn new(db: &'db dyn TypeDb, data: TypeData<'db>) -> Self {
        Self { db, data }
    }

    pub fn is_number_literal(self, value: f64) -> bool {
        matches!(
            self.data,
            TypeData::Literal(literal)
                if matches!(literal.literal(self.db), Literal::Number(number) if number.to_f64() == Some(value))
        )
    }

    pub fn is_number_or_number_literal(self) -> bool {
        matches!(self.data, TypeData::Number)
            || matches!(
                self.data,
                TypeData::Literal(literal)
                    if matches!(literal.literal(self.db), Literal::Number(_))
            )
    }

    pub fn is_bigint_literal(self, value: i64) -> bool {
        let expected = format!("{value}n");
        matches!(
            self.data,
            TypeData::Literal(literal)
                if matches!(literal.literal(self.db), Literal::BigInt(number) if canonicalize_js_bigint_literal(number.text()).as_deref() == Some(expected.as_str()))
        )
    }

    pub fn is_string_or_string_literal(self) -> bool {
        matches!(self.data, TypeData::String)
            || matches!(
                self.data,
                TypeData::Literal(literal)
                    if matches!(literal.literal(self.db), Literal::String(_))
            )
    }

    pub fn is_all_string_like(self) -> bool {
        self.try_all_variants_match(|data| {
            matches!(data, TypeData::String)
                || matches!(
                    data,
                    TypeData::Literal(literal)
                        if matches!(literal.literal(self.db), Literal::String(_))
                )
        })
        .unwrap_or(false)
    }

    pub fn is_all_number_like(self) -> bool {
        self.try_all_variants_match(|data| {
            matches!(data, TypeData::Number)
                || matches!(
                    data,
                    TypeData::Literal(literal)
                        if matches!(literal.literal(self.db), Literal::Number(_))
                )
        })
        .unwrap_or(false)
    }

    pub fn is_all_boolean_like(self) -> bool {
        self.try_all_variants_match(|data| {
            matches!(data, TypeData::Boolean)
                || matches!(
                    data,
                    TypeData::Literal(literal)
                        if matches!(literal.literal(self.db), Literal::Boolean(_))
                )
        })
        .unwrap_or(false)
    }

    pub fn is_all_bigint_like(self) -> bool {
        self.try_all_variants_match(|data| {
            matches!(data, TypeData::BigInt)
                || matches!(
                    data,
                    TypeData::Literal(literal)
                        if matches!(literal.literal(self.db), Literal::BigInt(_))
                )
        })
        .unwrap_or(false)
    }

    pub fn is_all_integer_like(self) -> bool {
        self.try_all_variants_match(|data| match data {
            TypeData::BigInt => true,
            TypeData::Literal(literal) => match literal.literal(self.db) {
                Literal::BigInt(_) => true,
                Literal::Number(number) => {
                    number.to_f64().is_some_and(|number| number.fract() == 0.0)
                }
                _ => false,
            },
            _ => false,
        })
        .unwrap_or(false)
    }

    pub fn is_all_string_array_or_tuple(self) -> bool {
        self.try_all_variants_match(|data| {
            matches!(data, TypeData::String | TypeData::Tuple(_))
                || matches!(
                    data,
                    TypeData::Literal(literal)
                        if matches!(literal.literal(self.db), Literal::String(_))
                )
                || matches!(
                    data,
                    TypeData::InstanceOf(instance)
                        if instance.ty(self.db).is_array_class(self.db)
                )
        })
        .unwrap_or(false)
    }

    pub fn is_regexp_literal_without_global_flag(self) -> bool {
        matches!(
        self.data,
        TypeData::Literal(literal)
            if matches!(literal.literal(self.db), Literal::RegExp(regexp) if !regexp.flags.contains('g'))
        )
    }

    pub fn regexp_literal(self) -> Option<(Text, Text)> {
        let TypeData::Literal(literal) = self.data else {
            return None;
        };
        let Literal::RegExp(regexp) = literal.literal(self.db) else {
            return None;
        };
        Some((regexp.pattern.clone(), regexp.flags.clone()))
    }

    pub fn is_array(self) -> bool {
        matches!(self.data, TypeData::InstanceOf(instance) if instance.ty(self.db).is_array_class(self.db))
    }

    /// Returns `Some(true)` for arrays whose element type resolves to a
    /// `Promise` or `PromiseLike` instance, and `Some(false)` for conclusive
    /// non-matches.
    ///
    /// Returns `None` when the element type is absent, unresolved, recursive,
    /// or exceeds the promise traversal limit.
    pub fn is_array_of_promise(self) -> Option<bool> {
        let TypeData::InstanceOf(instance) = self.data else {
            return if is_indeterminate_type(self.data) {
                None
            } else {
                Some(false)
            };
        };

        if !instance.ty(self.db).is_array_class(self.db) {
            return Some(false);
        }
        instance
            .type_parameters(self.db)
            .first()
            .and_then(|ty| is_promise_instance(self.db, *ty))
    }

    pub fn is_disposable(self) -> bool {
        self.has_computed_member("Symbol.dispose")
    }

    pub fn is_async_disposable(self) -> bool {
        self.has_computed_member("Symbol.asyncDispose")
    }

    /// Returns `Some(true)` when this type resolves through an instance to
    /// `Promise` or `PromiseLike`, and `Some(false)` for conclusive non-matches.
    ///
    /// Returns `None` when traversal encounters an unresolved or recursive type,
    /// or exceeds the promise traversal limit.
    pub fn is_promise_instance(self) -> Option<bool> {
        is_promise_instance(self.db, self.data)
    }

    pub fn is_function(self) -> bool {
        self.data.callable_function(self.db).is_some()
    }

    /// Returns `Some(true)` when this type or any of its variants is callable,
    /// and `Some(false)` for conclusive non-callable types.
    ///
    /// Returns `None` when traversal encounters an unresolved or recursive type,
    /// or exceeds the type-variant traversal limit.
    pub fn is_callable(self) -> Option<bool> {
        let mut visitor = CallableVisitor {
            db: self.db,
            indeterminate: false,
        };
        match visitor.visit(self.data, MAX_TYPE_VARIANT_STEPS) {
            TraversalOutcome::Break(is_callable) => Some(is_callable),
            TraversalOutcome::Complete { encountered_cycle }
                if !encountered_cycle && !visitor.indeterminate =>
            {
                Some(false)
            }
            TraversalOutcome::Complete { .. } | TraversalOutcome::LimitExceeded => None,
        }
    }

    pub fn is_at_least_as_wide_as_object(self) -> bool {
        is_at_least_as_wide_as_object(self.db, self.data, &mut FxHashSet::default(), 0)
    }

    pub fn promise_inner_type(self) -> Option<Self> {
        relation_promise_inner(self.db, self.data).map(|data| Self::new(self.db, data))
    }

    pub fn is_return_type_relation_escape_hatch(self) -> bool {
        relation_is_escape_hatch(self.data)
    }

    pub fn compare_declared_return_type(self, inferred: &[Self]) -> ReturnTypeRelation<'db> {
        compare_declared_return_type_owned(
            self.db,
            self.data,
            inferred.iter().map(|ty| ty.data).collect(),
        )
    }

    /// Returns `Some(true)` when this callable returns a `Promise` or
    /// `PromiseLike` instance, and `Some(false)` for conclusive non-matches.
    ///
    /// Returns `None` when the callable or its return type is unresolved,
    /// recursive, or exceeds the promise traversal limit.
    pub fn function_returns_promise(self) -> Option<bool> {
        let Some(function) = self.data.callable_function(self.db) else {
            return if is_indeterminate_type(self.data) {
                None
            } else {
                Some(false)
            };
        };
        let ReturnType::Type(return_ty) = function.return_type(self.db) else {
            return Some(false);
        };
        is_promise_instance(self.db, *return_ty)
    }

    pub fn function_returns_conditional(self) -> bool {
        self.function_return_matches(|ty| matches!(ty, TypeData::Conditional))
    }

    pub fn function_returns_void(self) -> bool {
        self.function_return_matches(|ty| matches!(ty, TypeData::VoidKeyword))
    }

    /// Returns `Some(true)` when a union contains a `Promise` or `PromiseLike`
    /// instance, and `Some(false)` for non-unions or conclusive non-matches.
    ///
    /// Returns `None` when union traversal encounters an unresolved or recursive
    /// type, or exceeds the promise traversal limit.
    pub fn has_promise_variant(self) -> Option<bool> {
        match self.data {
            TypeData::Union(_) => is_promise_instance(self.db, self.data),
            _ => Some(false),
        }
    }

    /// Returns whether the top-level type was successfully inferred.
    ///
    /// Internal `Unknown` variants poison unions in `UnionBuilder`, while the
    /// explicit TypeScript `unknown` keyword remains an inferred type.
    pub const fn is_inferred(self) -> bool {
        !matches!(
            self.data,
            TypeData::Unknown
                | TypeData::Divergent(_)
                | TypeData::Local(_)
                | TypeData::TypeofExpression(_)
        )
    }

    pub fn is_always_truthy(self) -> bool {
        self.conditional_type().is_truthy()
    }

    pub fn is_always_falsy(self) -> bool {
        self.conditional_type().is_falsy()
    }

    pub fn is_non_nullish(self) -> bool {
        self.conditional_type().is_non_nullish()
    }

    pub fn is_nullish(self) -> bool {
        self.conditional_type().is_nullish()
    }

    /// Returns `Some(true)` when any variant is `null`, `undefined`, or `void`,
    /// and `Some(false)` when every variant is conclusively non-nullish.
    ///
    /// Returns `None` when no match is found and variant traversal encounters an
    /// unresolved or recursive type, or exceeds the variant traversal limit.
    pub fn has_nullish_variant(self) -> Option<bool> {
        self.try_any_variant_matches(|data| {
            matches!(
                data,
                TypeData::Null | TypeData::Undefined | TypeData::VoidKeyword
            )
        })
        .ok()
    }

    /// Returns whether every variant can be replaced safely in a nullish
    /// coalescing expression.
    ///
    /// Returns `None` when variant traversal cannot establish a result because
    /// it encounters an unresolved or recursive type, or exceeds its limit.
    pub fn is_safe_for_nullish_coalescing(self) -> Option<bool> {
        self.try_all_variants_match(|data| {
            if matches!(data, TypeData::InstanceOf(_)) {
                return true;
            }
            matches!(
                data.conditional_type_shallow(self.db),
                Some(ConditionalType::Truthy | ConditionalType::Nullish)
            )
        })
        .ok()
    }

    /// Returns whether every variant of a nullish union is nullish or an ignored
    /// primitive. Non-union types return `Some(false)`.
    ///
    /// Returns `None` when variant traversal cannot establish a result because
    /// it encounters an unresolved or recursive type, or exceeds its limit.
    pub fn nullish_union_matches_ignored_primitives(
        self,
        ignored: IgnoredPrimitiveTypes,
    ) -> Option<bool> {
        let TypeData::Union(_) = self.data else {
            return Some(false);
        };

        self.try_all_variants_match(|data| match data {
            TypeData::Null | TypeData::Undefined | TypeData::VoidKeyword => true,
            TypeData::String => ignored.string,
            TypeData::Number => ignored.number,
            TypeData::Boolean => ignored.boolean,
            TypeData::BigInt => ignored.bigint,
            TypeData::Literal(literal) => match literal.literal(self.db) {
                Literal::String(_) => ignored.string,
                Literal::Number(_) => ignored.number,
                Literal::Boolean(_) => ignored.boolean,
                Literal::BigInt(_) => ignored.bigint,
                _ => false,
            },
            _ => false,
        })
        .ok()
    }

    /// Returns `Some(true)` when any variant is `null`, and `Some(false)` when
    /// every variant is conclusively non-null.
    ///
    /// Returns `None` when no match is found and variant traversal encounters an
    /// unresolved or recursive type, or exceeds its limit.
    pub fn has_null_variant(self) -> Option<bool> {
        self.try_any_variant_matches(|data| matches!(data, TypeData::Null))
            .ok()
    }

    /// Returns `Some(true)` when any variant is `undefined` or `void`, and
    /// `Some(false)` when every variant conclusively excludes both.
    ///
    /// Returns `None` when no match is found and variant traversal encounters an
    /// unresolved or recursive type, or exceeds its limit.
    pub fn has_undefined_variant(self) -> Option<bool> {
        self.try_any_variant_matches(|data| {
            matches!(data, TypeData::Undefined | TypeData::VoidKeyword)
        })
        .ok()
    }

    pub fn has_invalid_plus_operand_variant(self) -> bool {
        self.try_any_variant_matches(|data| match data {
            TypeData::NeverKeyword | TypeData::Symbol | TypeData::UnknownKeyword => true,
            TypeData::Literal(literal) => {
                matches!(literal.literal(self.db), Literal::Object(_))
            }
            TypeData::Intersection(intersection) => intersection
                .types(self.db)
                .iter()
                .all(|ty| is_object_like(self.db, *ty)),
            data => is_object_like(self.db, data),
        })
        .unwrap_or(false)
    }

    pub fn has_number_like_variant(self) -> bool {
        self.try_any_variant_matches(|data| {
            matches!(data, TypeData::Number)
                || matches!(
                    data,
                    TypeData::Literal(literal)
                        if matches!(literal.literal(self.db), Literal::Number(_))
                )
        })
        .unwrap_or(false)
    }

    pub fn has_bigint_like_variant(self) -> bool {
        self.try_any_variant_matches(|data| {
            matches!(data, TypeData::BigInt)
                || matches!(
                    data,
                    TypeData::Literal(literal)
                        if matches!(literal.literal(self.db), Literal::BigInt(_))
                )
        })
        .unwrap_or(false)
    }

    pub fn plus_operand_description(self) -> String {
        type_description(self.db, self.data)
    }

    /// Returns the deduplicated switch-case variants represented by this type.
    ///
    /// Returns [`TypeTraversalError::LimitExceeded`] when traversal exceeds the
    /// type-variant limit. Literals that cannot form supported cases are represented by
    /// [`InferredSwitchCase::UnsupportedLiteral`].
    pub fn try_switch_case_variants(self) -> Result<Vec<InferredSwitchCase>, TypeTraversalError> {
        let mut cases = Vec::new();
        let mut seen = FxHashSet::default();
        let mut pending = vec![self.data];
        let mut remaining_steps = MAX_TYPE_VARIANT_STEPS;

        while let Some(data) = pending.pop() {
            if !seen.insert(data) {
                continue;
            }
            if remaining_steps == 0 {
                return Err(TypeTraversalError::LimitExceeded);
            }
            remaining_steps -= 1;

            match data {
                TypeData::Boolean => cases.push(InferredSwitchCase::Boolean),
                TypeData::Literal(literal) => cases.push(match literal.literal(self.db) {
                    Literal::Boolean(boolean) => {
                        InferredSwitchCase::BooleanLiteral(boolean.as_bool())
                    }
                    Literal::Number(number) => InferredSwitchCase::Number(number.text().clone()),
                    Literal::String(string) => {
                        InferredSwitchCase::String(Text::new_owned(string.as_str().into()))
                    }
                    Literal::BigInt(bigint) => {
                        match canonicalize_js_bigint_literal(bigint.text()) {
                            Some(Cow::Borrowed(_)) => InferredSwitchCase::BigInt(bigint.clone()),
                            Some(Cow::Owned(bigint)) => {
                                InferredSwitchCase::BigInt(Text::new_owned(bigint.into()))
                            }
                            None => InferredSwitchCase::UnsupportedLiteral,
                        }
                    }
                    Literal::Object(_) | Literal::RegExp(_) | Literal::Template(_) => {
                        InferredSwitchCase::UnsupportedLiteral
                    }
                }),
                TypeData::Null => cases.push(InferredSwitchCase::Null),
                TypeData::Undefined => cases.push(InferredSwitchCase::Undefined),
                TypeData::Symbol => cases.push(InferredSwitchCase::Symbol),
                TypeData::InstanceOf(instance) => pending.push(instance.ty(self.db)),
                TypeData::Intersection(intersection) => pending.extend(
                    intersection
                        .types(self.db)
                        .iter()
                        .filter(|ty| ty.is_primitive(self.db))
                        .rev()
                        .copied(),
                ),
                TypeData::TypeofType(typeof_type) => pending.push(typeof_type.ty(self.db)),
                TypeData::TypeofValue(typeof_value) => pending.push(typeof_value.ty(self.db)),
                TypeData::Union(union) => {
                    pending.extend(union.types(self.db).iter().rev().copied());
                }
                _ => {}
            }
        }

        let mut unique_cases = FxHashSet::default();
        cases.retain(|case| unique_cases.insert(case.clone()));
        Ok(cases)
    }

    pub fn stringification_usefulness(
        self,
        mode: StringificationMode,
        ignored_type_names: &[&str],
    ) -> StringificationUsefulness {
        let mut active = FxHashSet::default();
        stringification_usefulness(self.db, self.data, mode, ignored_type_names, &mut active, 0)
    }

    pub fn could_equal_string_literal(self, value: &str) -> bool {
        self.could_equal_literal(|data| match data {
            TypeData::String => Some(true),
            TypeData::Literal(literal) => Some(matches!(
                literal.literal(self.db),
                Literal::String(string) if string.as_str() == value
            )),
            _ => Some(false),
        })
    }

    pub fn could_equal_number_literal(self, value: f64) -> bool {
        self.could_equal_literal(|data| match data {
            TypeData::Number => Some(true),
            TypeData::Literal(literal) => Some(matches!(
                literal.literal(self.db),
                Literal::Number(number) if number.to_f64() == Some(value)
            )),
            _ => Some(false),
        })
    }

    pub fn could_equal_boolean_literal(self, value: bool) -> bool {
        self.could_equal_literal(|data| match data {
            TypeData::Boolean => Some(true),
            TypeData::Literal(literal) => Some(matches!(
                literal.literal(self.db),
                Literal::Boolean(boolean) if boolean.as_bool() == value
            )),
            _ => Some(false),
        })
    }

    pub fn could_equal_null(self) -> bool {
        self.could_equal_literal(|data| {
            Some(matches!(
                data,
                TypeData::Null | TypeData::Undefined | TypeData::VoidKeyword
            ))
        })
    }

    pub const fn is_null(self) -> bool {
        matches!(self.data, TypeData::Null)
    }

    pub const fn is_undefined(self) -> bool {
        matches!(self.data, TypeData::Undefined)
    }

    fn try_all_variants_match(
        self,
        predicate: impl FnMut(TypeData<'db>) -> bool,
    ) -> Result<bool, TypeTraversalError> {
        let mut visitor = AllVariantsVisitor {
            db: self.db,
            predicate,
            saw_variant: false,
            indeterminate: false,
        };
        match visitor.visit(self.data, MAX_TYPE_VARIANT_STEPS) {
            TraversalOutcome::Break(result) => Ok(result),
            TraversalOutcome::Complete { encountered_cycle }
                if !encountered_cycle && !visitor.indeterminate =>
            {
                Ok(visitor.saw_variant)
            }
            TraversalOutcome::Complete {
                encountered_cycle: true,
            } => Err(TypeTraversalError::RecursiveType),
            TraversalOutcome::Complete { .. } => Err(TypeTraversalError::UnresolvedType),
            TraversalOutcome::LimitExceeded => Err(TypeTraversalError::LimitExceeded),
        }
    }

    fn conditional_type(self) -> ConditionalType {
        let mut conditional = ConditionalType::Unknown;
        let mut seen = FxHashSet::default();
        let mut pending = vec![self.data];

        for _ in 0..MAX_TYPE_VARIANT_STEPS {
            let Some(data) = pending.pop() else {
                return conditional;
            };
            if !seen.insert(data) {
                continue;
            }

            if let Some(next) = data.conditional_type_shallow(self.db) {
                conditional = if conditional == ConditionalType::Unknown {
                    next
                } else {
                    conditional.merged_with(next)
                };
            } else {
                match data {
                    TypeData::Generic(generic) => {
                        let Some(constraint) = generic.constraint(self.db) else {
                            return ConditionalType::Unknown;
                        };
                        pending.push(constraint);
                    }
                    TypeData::InstanceOf(instance) => pending.push(instance.ty(self.db)),
                    TypeData::Intersection(intersection) => {
                        pending.extend(intersection.types(self.db).iter().copied());
                    }
                    TypeData::MergedReference(reference) => pending.extend(
                        [
                            reference.ty(self.db),
                            reference.value_ty(self.db),
                            reference.namespace_ty(self.db),
                        ]
                        .into_iter()
                        .flatten(),
                    ),
                    TypeData::TypeofType(typeof_type) => pending.push(typeof_type.ty(self.db)),
                    TypeData::TypeofValue(typeof_value) => pending.push(typeof_value.ty(self.db)),
                    TypeData::Union(union) => {
                        pending.extend(union.types(self.db).iter().copied());
                    }
                    _ => return ConditionalType::Unknown,
                }
            }

            if conditional != ConditionalType::Unknown && !conditional.is_mergeable() {
                return conditional;
            }
        }

        ConditionalType::Unknown
    }

    fn could_equal_literal(self, mut predicate: impl FnMut(TypeData<'db>) -> Option<bool>) -> bool {
        let mut seen = FxHashSet::default();
        let mut pending = vec![self.data];

        for _ in 0..MAX_TYPE_VARIANT_STEPS {
            let Some(data) = pending.pop() else {
                return false;
            };
            if !seen.insert(data) {
                continue;
            }

            match data {
                TypeData::Unknown
                | TypeData::Divergent(_)
                | TypeData::AnyKeyword
                | TypeData::UnknownKeyword
                | TypeData::Local(_)
                | TypeData::TypeofExpression(_) => return true,
                TypeData::Generic(generic) => {
                    let Some(constraint) = generic.constraint(self.db) else {
                        return true;
                    };
                    pending.push(constraint);
                }
                TypeData::InstanceOf(instance) => pending.push(instance.ty(self.db)),
                TypeData::TypeofType(typeof_type) => pending.push(typeof_type.ty(self.db)),
                TypeData::TypeofValue(typeof_value) => pending.push(typeof_value.ty(self.db)),
                TypeData::Union(union) => {
                    pending.extend(union.types(self.db).iter().copied());
                }
                data => match predicate(data) {
                    Some(true) => return true,
                    Some(false) => {}
                    None => return true,
                },
            }
        }

        true
    }

    fn try_any_variant_matches(
        self,
        predicate: impl FnMut(TypeData<'db>) -> bool,
    ) -> Result<bool, TypeTraversalError> {
        let mut visitor = AnyVariantVisitor {
            db: self.db,
            predicate,
            indeterminate: false,
        };
        match visitor.visit(self.data, MAX_TYPE_VARIANT_STEPS) {
            TraversalOutcome::Break(result) => Ok(result),
            TraversalOutcome::Complete { encountered_cycle }
                if !encountered_cycle && !visitor.indeterminate =>
            {
                Ok(false)
            }
            TraversalOutcome::Complete {
                encountered_cycle: true,
            } => Err(TypeTraversalError::RecursiveType),
            TraversalOutcome::Complete { .. } => Err(TypeTraversalError::UnresolvedType),
            TraversalOutcome::LimitExceeded => Err(TypeTraversalError::LimitExceeded),
        }
    }

    fn function_return_matches(self, predicate: impl Fn(TypeData<'db>) -> bool) -> bool {
        let Some(function) = self.data.callable_function(self.db) else {
            return false;
        };
        let ReturnType::Type(return_ty) = function.return_type(self.db) else {
            return false;
        };
        predicate(*return_ty)
    }

    fn has_computed_member(self, name: &str) -> bool {
        let mut seen = FxHashSet::default();
        let mut pending = vec![self.data];

        for _ in 0..MAX_TYPE_VARIANT_STEPS {
            let Some(data) = pending.pop() else {
                return false;
            };
            if !seen.insert(data) {
                continue;
            }

            match data {
                TypeData::Class(class) => {
                    if class.members(self.db).iter().any(|member| {
                        !member.kind.is_static() && member.kind.computed_name() == Some(name)
                    }) {
                        return true;
                    }
                    pending.extend(class.extends(self.db));
                    pending.extend(class.implements(self.db).iter().copied());
                }
                TypeData::Interface(interface) => {
                    if interface
                        .members(self.db)
                        .iter()
                        .any(|member| member.kind.computed_name() == Some(name))
                    {
                        return true;
                    }
                    pending.extend(interface.extends(self.db).iter().copied());
                }
                TypeData::Literal(literal) => {
                    if let Literal::Object(members) = literal.literal(self.db)
                        && members
                            .iter()
                            .any(|member| member.kind.computed_name() == Some(name))
                    {
                        return true;
                    }
                }
                TypeData::Object(object) => {
                    if object
                        .members(self.db)
                        .iter()
                        .any(|member| member.kind.computed_name() == Some(name))
                    {
                        return true;
                    }
                    pending.extend(object.prototype(self.db));
                }
                TypeData::Generic(generic) => pending.extend(generic.constraint(self.db)),
                TypeData::InstanceOf(instance) => pending.push(instance.ty(self.db)),
                TypeData::Intersection(intersection) => {
                    pending.extend(intersection.types(self.db).iter().copied());
                }
                TypeData::MergedReference(reference) => pending.extend(reference.targets(self.db)),
                TypeData::TypeofType(typeof_type) => pending.push(typeof_type.ty(self.db)),
                TypeData::TypeofValue(typeof_value) => pending.push(typeof_value.ty(self.db)),
                TypeData::Union(union) => {
                    pending.extend(union.types(self.db).iter().copied());
                }
                _ => {}
            }
        }

        false
    }
}

/// Checks whether a type or any of its nested types is callable.
///
/// `indeterminate` records whether the visitor encountered a type it could not
/// resolve. In that case, not finding a callable type cannot be reported as
/// `false`. The visitor stops as soon as it finds a callable type.
struct CallableVisitor<'db> {
    db: &'db dyn TypeDb,
    indeterminate: bool,
}

impl<'db> DepthFirstVisitor<TypeData<'db>> for CallableVisitor<'db> {
    type Break = bool;

    fn enter(
        &mut self,
        data: TypeData<'db>,
        context: &mut VisitContext<'_, TypeData<'db>>,
    ) -> ControlFlow<Self::Break> {
        let has_call_signature = match data {
            TypeData::Interface(interface) => interface
                .members(self.db)
                .iter()
                .any(|member| member.kind.is_call_signature()),
            TypeData::Object(object) => object
                .members(self.db)
                .iter()
                .any(|member| member.kind.is_call_signature()),
            _ => false,
        };
        if data.callable_function(self.db).is_some() || has_call_signature {
            return ControlFlow::Break(true);
        }

        match data {
            TypeData::Unknown
            | TypeData::Divergent(_)
            | TypeData::Local(_)
            | TypeData::TypeofExpression(_)
            | TypeData::AnyKeyword
            | TypeData::UnknownKeyword => self.indeterminate = true,
            TypeData::Generic(generic) => {
                if let Some(constraint) = generic.constraint(self.db) {
                    context.push(constraint);
                } else {
                    self.indeterminate = true;
                }
            }
            TypeData::InstanceOf(instance) => context.push(instance.ty(self.db)),
            TypeData::Interface(interface) => {
                context.extend(interface.extends(self.db).iter().copied());
            }
            TypeData::Intersection(intersection) => {
                context.extend(intersection.types(self.db).iter().copied());
            }
            TypeData::MergedReference(reference) => {
                context.extend(reference.targets(self.db));
            }
            TypeData::Object(object) => context.extend(object.prototype(self.db)),
            TypeData::TypeOperator(operator) => context.push(operator.ty(self.db)),
            TypeData::TypeofType(typeof_type) => context.push(typeof_type.ty(self.db)),
            TypeData::TypeofValue(typeof_value) => context.push(typeof_value.ty(self.db)),
            TypeData::Union(union) => context.extend(union.types(self.db).iter().copied()),
            _ => {}
        }

        ControlFlow::Continue(())
    }
}

/// Checks whether every nested type satisfies a predicate.
///
/// `saw_variant` records whether the visitor found a type it could check.
/// `indeterminate` records whether it encountered a type it could not resolve.
/// The visitor stops as soon as a type does not satisfy the predicate.
struct AllVariantsVisitor<'db, P> {
    db: &'db dyn TypeDb,
    predicate: P,
    saw_variant: bool,
    indeterminate: bool,
}

impl<'db, P> DepthFirstVisitor<TypeData<'db>> for AllVariantsVisitor<'db, P>
where
    P: FnMut(TypeData<'db>) -> bool,
{
    type Break = bool;

    fn enter(
        &mut self,
        data: TypeData<'db>,
        context: &mut VisitContext<'_, TypeData<'db>>,
    ) -> ControlFlow<Self::Break> {
        match data {
            TypeData::Union(union) => {
                if union.types(self.db).is_empty() {
                    return ControlFlow::Break(false);
                }
                context.extend(union.types(self.db).iter().copied());
            }
            TypeData::Intersection(intersection) => {
                let primitive_types = intersection
                    .types(self.db)
                    .iter()
                    .copied()
                    .filter(|ty| ty.is_primitive(self.db))
                    .collect::<Vec<_>>();
                if primitive_types.is_empty() {
                    if (self.predicate)(data) {
                        self.saw_variant = true;
                    } else {
                        return ControlFlow::Break(false);
                    }
                } else {
                    context.extend(primitive_types);
                }
            }
            TypeData::Generic(generic) => {
                let Some(constraint) = generic.constraint(self.db) else {
                    self.indeterminate = true;
                    return ControlFlow::Continue(());
                };
                context.push(constraint);
            }
            data if is_indeterminate_type(data) => self.indeterminate = true,
            _ if (self.predicate)(data) => self.saw_variant = true,
            _ => return ControlFlow::Break(false),
        }

        ControlFlow::Continue(())
    }
}

/// Checks whether any nested type satisfies a predicate.
///
/// `indeterminate` records whether the visitor encountered a type it could not
/// resolve. In that case, not finding a match cannot be reported as `false`.
/// The visitor stops as soon as a type satisfies the predicate.
struct AnyVariantVisitor<'db, P> {
    db: &'db dyn TypeDb,
    predicate: P,
    indeterminate: bool,
}

impl<'db, P> DepthFirstVisitor<TypeData<'db>> for AnyVariantVisitor<'db, P>
where
    P: FnMut(TypeData<'db>) -> bool,
{
    type Break = bool;

    fn enter(
        &mut self,
        data: TypeData<'db>,
        context: &mut VisitContext<'_, TypeData<'db>>,
    ) -> ControlFlow<Self::Break> {
        match data {
            TypeData::Intersection(intersection) => {
                if (self.predicate)(data) {
                    return ControlFlow::Break(true);
                }
                context.extend(
                    intersection
                        .types(self.db)
                        .iter()
                        .copied()
                        .filter(|ty| ty.is_primitive(self.db)),
                );
            }
            TypeData::Generic(generic) => {
                if let Some(constraint) = generic.constraint(self.db) {
                    context.push(constraint);
                } else {
                    self.indeterminate = true;
                }
            }
            TypeData::InstanceOf(instance) => context.push(instance.ty(self.db)),
            TypeData::TypeofType(typeof_type) => context.push(typeof_type.ty(self.db)),
            TypeData::TypeofValue(typeof_value) => context.push(typeof_value.ty(self.db)),
            TypeData::Union(union) => context.extend(union.types(self.db).iter().copied()),
            data if is_indeterminate_type(data) => self.indeterminate = true,
            data if (self.predicate)(data) => return ControlFlow::Break(true),
            _ => {}
        }

        ControlFlow::Continue(())
    }
}

fn is_at_least_as_wide_as_object<'db>(
    db: &'db dyn TypeDb,
    ty: TypeData<'db>,
    seen: &mut FxHashSet<TypeData<'db>>,
    depth: usize,
) -> bool {
    if depth >= MAX_TYPE_RELATION_DEPTH || !seen.insert(ty) {
        return true;
    }
    let result = match ty {
        TypeData::AnyKeyword
        | TypeData::Unknown
        | TypeData::UnknownKeyword
        | TypeData::ObjectKeyword
        | TypeData::Conditional
        | TypeData::TypeofExpression(_)
        | TypeData::TypeofType(_)
        | TypeData::TypeofValue(_) => true,
        TypeData::Object(object) => object.members(db).is_empty(),
        TypeData::Interface(interface) => interface.members(db).is_empty(),
        TypeData::Class(class) => class
            .members(db)
            .iter()
            .all(|member| member.kind.is_static()),
        TypeData::Generic(generic) => generic
            .constraint(db)
            .is_none_or(|ty| is_at_least_as_wide_as_object(db, ty, seen, depth + 1)),
        TypeData::InstanceOf(instance) => {
            is_at_least_as_wide_as_object(db, instance.ty(db), seen, depth + 1)
        }
        TypeData::Intersection(intersection) => {
            intersection
                .types(db)
                .iter()
                .any(|ty| matches!(ty, TypeData::AnyKeyword))
                || intersection
                    .types(db)
                    .iter()
                    .all(|ty| is_at_least_as_wide_as_object(db, *ty, seen, depth + 1))
        }
        TypeData::Union(union) => union
            .types(db)
            .iter()
            .any(|ty| is_at_least_as_wide_as_object(db, *ty, seen, depth + 1)),
        TypeData::MergedReference(reference) => reference
            .targets(db)
            .any(|ty| is_at_least_as_wide_as_object(db, ty, seen, depth + 1)),
        _ => false,
    };
    seen.remove(&ty);
    result
}

fn stringification_usefulness<'db>(
    db: &'db dyn TypeDb,
    data: TypeData<'db>,
    mode: StringificationMode,
    ignored_type_names: &[&str],
    active: &mut FxHashSet<TypeData<'db>>,
    depth: usize,
) -> StringificationUsefulness {
    use StringificationUsefulness::Always;

    if depth >= MAX_TYPE_VARIANT_STEPS || !active.insert(data) {
        return Always;
    }

    let result = if let TypeData::Generic(generic) = data {
        generic.constraint(db).map_or(Always, |constraint| {
            stringification_usefulness(db, constraint, mode, ignored_type_names, active, depth + 1)
        })
    } else if matches!(mode, StringificationMode::ToString) {
        match is_ignored_stringification_type(db, data, ignored_type_names) {
            None | Some(true) => Always,
            Some(false) if is_safe_stringification_type(db, data) => Always,
            Some(false) => stringification_usefulness_unignored(
                db,
                data,
                mode,
                ignored_type_names,
                active,
                depth,
            ),
        }
    } else {
        stringification_usefulness_unignored(db, data, mode, ignored_type_names, active, depth)
    };

    active.remove(&data);
    result
}

fn stringification_usefulness_unignored<'db>(
    db: &'db dyn TypeDb,
    data: TypeData<'db>,
    mode: StringificationMode,
    ignored_type_names: &[&str],
    active: &mut FxHashSet<TypeData<'db>>,
    depth: usize,
) -> StringificationUsefulness {
    use StringificationUsefulness::{Always, Never};

    match data {
        TypeData::Union(union) => combine_stringification_union(union.types(db).iter().map(|ty| {
            stringification_usefulness(db, *ty, mode, ignored_type_names, active, depth + 1)
        })),
        TypeData::Intersection(intersection) => {
            combine_stringification_intersection(intersection.types(db).iter().map(|ty| {
                stringification_usefulness(db, *ty, mode, ignored_type_names, active, depth + 1)
            }))
        }
        TypeData::Tuple(tuple) => {
            combine_stringification_tuple(tuple.elements(db).iter().map(|element| {
                stringification_usefulness(
                    db,
                    element.ty,
                    StringificationMode::ToString,
                    ignored_type_names,
                    active,
                    depth + 1,
                )
            }))
        }
        TypeData::InstanceOf(instance) if instance.ty(db).is_array_class(db) => instance
            .type_parameters(db)
            .first()
            .map_or(Always, |element| {
                stringification_usefulness(
                    db,
                    *element,
                    StringificationMode::ToString,
                    ignored_type_names,
                    active,
                    depth + 1,
                )
            }),
        TypeData::InstanceOf(instance) => stringification_usefulness(
            db,
            instance.ty(db),
            mode,
            ignored_type_names,
            active,
            depth + 1,
        ),
        _ if matches!(mode, StringificationMode::Join) => Always,
        _ => match uses_base_object_stringification(db, data, &mut FxHashSet::default(), depth + 1)
        {
            Some(true) => Never,
            Some(false) | None => Always,
        },
    }
}

fn combine_stringification_union(
    usefulness: impl Iterator<Item = StringificationUsefulness>,
) -> StringificationUsefulness {
    let mut combined = None;
    for usefulness in usefulness {
        match combined {
            None => combined = Some(usefulness),
            Some(existing) if existing == usefulness => {}
            Some(_) => return StringificationUsefulness::Sometimes,
        }
    }
    combined.unwrap_or(StringificationUsefulness::Always)
}

fn combine_stringification_intersection(
    usefulness: impl Iterator<Item = StringificationUsefulness>,
) -> StringificationUsefulness {
    if usefulness
        .into_iter()
        .any(|usefulness| usefulness == StringificationUsefulness::Always)
    {
        StringificationUsefulness::Always
    } else {
        StringificationUsefulness::Never
    }
}

fn combine_stringification_tuple(
    usefulness: impl Iterator<Item = StringificationUsefulness>,
) -> StringificationUsefulness {
    let mut saw_sometimes = false;
    for usefulness in usefulness {
        match usefulness {
            StringificationUsefulness::Always => {}
            StringificationUsefulness::Sometimes => saw_sometimes = true,
            StringificationUsefulness::Never => return StringificationUsefulness::Never,
        }
    }
    if saw_sometimes {
        StringificationUsefulness::Sometimes
    } else {
        StringificationUsefulness::Always
    }
}

fn is_safe_stringification_type<'db>(db: &'db dyn TypeDb, data: TypeData<'db>) -> bool {
    match data {
        TypeData::AnyKeyword
        | TypeData::BigInt
        | TypeData::Boolean
        | TypeData::Function(_)
        | TypeData::Null
        | TypeData::Number
        | TypeData::String
        | TypeData::Symbol
        | TypeData::Undefined
        | TypeData::Unknown
        | TypeData::UnknownKeyword
        | TypeData::NeverKeyword
        | TypeData::VoidKeyword => true,
        TypeData::Literal(literal) => matches!(
            literal.literal(db),
            Literal::BigInt(_)
                | Literal::Boolean(_)
                | Literal::Number(_)
                | Literal::String(_)
                | Literal::Template(_)
        ),
        _ => false,
    }
}

fn is_ignored_stringification_type<'db>(
    db: &'db dyn TypeDb,
    data: TypeData<'db>,
    ignored_type_names: &[&str],
) -> Option<bool> {
    let mut seen = FxHashSet::default();
    let mut pending = vec![data];
    let mut remaining_steps = MAX_TYPE_VARIANT_STEPS;

    while let Some(data) = pending.pop() {
        if !seen.insert(data) {
            continue;
        }
        if remaining_steps == 0 {
            return None;
        }
        remaining_steps -= 1;

        let name = match data {
            TypeData::Class(class) => class.name(db).as_ref().map(Text::text),
            TypeData::Generic(generic) => Some(generic.name(db).text()),
            TypeData::Interface(interface) => Some(interface.name(db).text()),
            TypeData::Literal(literal) if matches!(literal.literal(db), Literal::RegExp(_)) => {
                Some("RegExp")
            }
            TypeData::TypeofValue(value) => Some(value.identifier(db).text()),
            _ => None,
        };
        if name.is_some_and(|name| ignored_type_names.contains(&name)) {
            return Some(true);
        }

        match data {
            TypeData::Class(class) => pending.extend(class.extends(db)),
            TypeData::Generic(generic) => pending.extend(generic.constraint(db)),
            TypeData::InstanceOf(instance) => pending.push(instance.ty(db)),
            TypeData::Interface(interface) => {
                pending.extend(interface.extends(db).iter().copied());
            }
            TypeData::MergedReference(reference) => pending.extend(reference.targets(db)),
            TypeData::TypeOperator(operator) => pending.push(operator.ty(db)),
            TypeData::TypeofType(typeof_type) => pending.push(typeof_type.ty(db)),
            TypeData::TypeofValue(typeof_value) => pending.push(typeof_value.ty(db)),
            _ => {}
        }
    }

    Some(false)
}

fn uses_base_object_stringification<'db>(
    db: &'db dyn TypeDb,
    data: TypeData<'db>,
    active: &mut FxHashSet<TypeData<'db>>,
    depth: usize,
) -> Option<bool> {
    if depth >= MAX_TYPE_VARIANT_STEPS || !active.insert(data) {
        return Some(false);
    }

    let result = match data {
        TypeData::Class(class) => {
            if has_custom_stringification_member(class.members(db)) {
                Some(false)
            } else if let Some(base) = class.extends(db) {
                uses_base_object_stringification(db, base, active, depth + 1)
            } else {
                Some(true)
            }
        }
        TypeData::InstanceOf(instance) => {
            uses_base_object_stringification(db, instance.ty(db), active, depth + 1)
        }
        TypeData::Interface(interface) => {
            if has_custom_stringification_member(interface.members(db)) {
                Some(false)
            } else if interface.extends(db).is_empty() {
                Some(true)
            } else {
                combine_base_stringification(
                    interface
                        .extends(db)
                        .iter()
                        .map(|base| uses_base_object_stringification(db, *base, active, depth + 1)),
                )
            }
        }
        TypeData::Literal(literal) => match literal.literal(db) {
            Literal::Object(members) => Some(!has_custom_stringification_member(members)),
            Literal::RegExp(_) => Some(true),
            _ => Some(false),
        },
        TypeData::MergedReference(reference) => combine_base_stringification(
            reference
                .targets(db)
                .map(|target| uses_base_object_stringification(db, target, active, depth + 1)),
        ),
        TypeData::Object(object) => Some(!has_custom_stringification_member(object.members(db))),
        TypeData::ObjectKeyword => Some(true),
        TypeData::TypeofValue(value) => {
            uses_base_object_stringification(db, value.ty(db), active, depth + 1)
        }
        _ => None,
    };

    active.remove(&data);
    result
}

fn combine_base_stringification(results: impl Iterator<Item = Option<bool>>) -> Option<bool> {
    let mut saw_true = false;
    for result in results {
        match result {
            Some(false) => return Some(false),
            Some(true) => saw_true = true,
            None => {}
        }
    }
    saw_true.then_some(true)
}

fn has_custom_stringification_member(members: &[crate::interned_types::TypeMember<'_>]) -> bool {
    members.iter().any(|member| {
        ["toLocaleString", "toString", "valueOf"]
            .iter()
            .any(|name| member.kind.has_name(name))
    })
}

fn is_promise_instance<'db>(db: &'db dyn TypeDb, data: TypeData<'db>) -> Option<bool> {
    let mut completed = FxHashSet::default();
    let mut pending = VecDeque::from([(data, Vec::new(), false, false)]);
    let mut processed = 0;
    let mut indeterminate = false;

    while let Some((data, path, is_instance_target, is_promise_like_target)) = pending.pop_front() {
        if path.contains(&data) {
            indeterminate = true;
            continue;
        }
        if !completed.insert((data, is_instance_target, is_promise_like_target)) {
            continue;
        }
        if processed == MAX_PROMISE_TYPE_STEPS {
            indeterminate = true;
            continue;
        }
        processed += 1;

        if is_instance_target && data.is_promise_class(db) {
            return Some(true);
        }
        let is_named_promise_like = is_instance_target
            && match data {
                TypeData::Class(class) => class
                    .name(db)
                    .as_ref()
                    .is_some_and(|name| name.text() == "PromiseLike"),
                TypeData::Interface(interface) => interface.name(db).text() == "PromiseLike",
                _ => false,
            };
        let is_promise_like_target = is_promise_like_target || is_named_promise_like;
        if is_promise_like_target {
            let members = match data {
                TypeData::Class(class) => Some(class.members(db)),
                TypeData::Interface(interface) => Some(interface.members(db)),
                TypeData::Object(object) => Some(object.members(db)),
                _ => None,
            };
            if let Some(members) = members {
                for member in members
                    .iter()
                    .filter(|member| !member.kind.is_static() && member.kind.has_name("then"))
                {
                    match InferredType::new(db, member.ty).is_callable() {
                        Some(true) => return Some(true),
                        Some(false) => {}
                        None => indeterminate = true,
                    }
                }
            }
        }
        if is_indeterminate_type(data) {
            indeterminate = true;
            continue;
        }

        let mut child_path = path;
        child_path.push(data);
        let remaining_steps = MAX_PROMISE_TYPE_STEPS - processed;
        let available_frontier = remaining_steps.saturating_sub(pending.len());
        match data {
            TypeData::Class(class) => {
                if let Some(base) = class.extends(db) {
                    if available_frontier == 0 {
                        return None;
                    }
                    pending.push_back((
                        base,
                        child_path,
                        is_instance_target,
                        is_promise_like_target,
                    ));
                }
            }
            TypeData::Generic(generic) => {
                if let Some(constraint) = generic.constraint(db) {
                    if available_frontier == 0 {
                        return None;
                    }
                    pending.push_back((
                        constraint,
                        child_path,
                        is_instance_target,
                        is_promise_like_target,
                    ));
                } else {
                    indeterminate = true;
                }
            }
            TypeData::InstanceOf(instance) => {
                if available_frontier == 0 {
                    return None;
                }
                pending.push_back((instance.ty(db), child_path, true, is_promise_like_target));
            }
            TypeData::Interface(interface) => {
                if interface.extends(db).len() > available_frontier {
                    return None;
                }
                pending.extend(interface.extends(db).iter().copied().map(|child| {
                    (
                        child,
                        child_path.clone(),
                        is_instance_target,
                        is_promise_like_target,
                    )
                }));
            }
            TypeData::Intersection(intersection) => {
                if intersection.types(db).len() > available_frontier {
                    return None;
                }
                pending.extend(intersection.types(db).iter().copied().map(|child| {
                    (
                        child,
                        child_path.clone(),
                        is_instance_target,
                        is_promise_like_target,
                    )
                }));
            }
            TypeData::MergedReference(reference) => {
                if reference.targets(db).count() > available_frontier {
                    return None;
                }
                pending.extend(reference.targets(db).map(|child| {
                    (
                        child,
                        child_path.clone(),
                        is_instance_target,
                        is_promise_like_target,
                    )
                }));
            }
            TypeData::TypeOperator(operator) => {
                if available_frontier == 0 {
                    return None;
                }
                pending.push_back((
                    operator.ty(db),
                    child_path,
                    is_instance_target,
                    is_promise_like_target,
                ));
            }
            TypeData::TypeofType(typeof_type) => {
                if available_frontier == 0 {
                    return None;
                }
                pending.push_back((
                    typeof_type.ty(db),
                    child_path,
                    is_instance_target,
                    is_promise_like_target,
                ));
            }
            TypeData::TypeofValue(typeof_value) => {
                if available_frontier == 0 {
                    return None;
                }
                pending.push_back((
                    typeof_value.ty(db),
                    child_path,
                    is_instance_target,
                    is_promise_like_target,
                ));
            }
            TypeData::Union(union) => {
                if union.types(db).len() > available_frontier {
                    return None;
                }
                pending.extend(union.types(db).iter().copied().map(|child| {
                    (
                        child,
                        child_path.clone(),
                        is_instance_target,
                        is_promise_like_target,
                    )
                }));
            }
            TypeData::Global
            | TypeData::GlobalType(_)
            | TypeData::BigInt
            | TypeData::Boolean
            | TypeData::Null
            | TypeData::Number
            | TypeData::String
            | TypeData::Symbol
            | TypeData::Undefined
            | TypeData::Conditional
            | TypeData::Constructor(_)
            | TypeData::Function(_)
            | TypeData::Module(_)
            | TypeData::Namespace(_)
            | TypeData::Object(_)
            | TypeData::Tuple(_)
            | TypeData::Literal(_)
            | TypeData::NeverKeyword
            | TypeData::ObjectKeyword
            | TypeData::ThisKeyword
            | TypeData::VoidKeyword => {}
            TypeData::Unknown
            | TypeData::Divergent(_)
            | TypeData::Local(_)
            | TypeData::TypeofExpression(_)
            | TypeData::AnyKeyword
            | TypeData::UnknownKeyword => {
                indeterminate = true;
            }
        }
    }

    if indeterminate { None } else { Some(false) }
}

const fn is_indeterminate_type(data: TypeData<'_>) -> bool {
    matches!(
        data,
        TypeData::Unknown
            | TypeData::Divergent(_)
            | TypeData::Local(_)
            | TypeData::TypeofExpression(_)
            | TypeData::AnyKeyword
            | TypeData::UnknownKeyword
    )
}

fn is_object_like<'db>(db: &'db dyn TypeDb, data: TypeData<'db>) -> bool {
    match data {
        TypeData::Class(_)
        | TypeData::Constructor(_)
        | TypeData::Function(_)
        | TypeData::Interface(_)
        | TypeData::Module(_)
        | TypeData::Namespace(_)
        | TypeData::Object(_)
        | TypeData::ObjectKeyword
        | TypeData::Tuple(_) => true,
        TypeData::InstanceOf(instance) => is_object_like(db, instance.ty(db)),
        _ => false,
    }
}

fn type_description<'db>(db: &'db dyn TypeDb, data: TypeData<'db>) -> String {
    match data {
        TypeData::Unknown | TypeData::UnknownKeyword | TypeData::Divergent(_) => "unknown".into(),
        TypeData::AnyKeyword => "any".into(),
        TypeData::NeverKeyword => "never".into(),
        TypeData::Null => "null".into(),
        TypeData::Undefined | TypeData::VoidKeyword => "undefined".into(),
        TypeData::Boolean => "boolean".into(),
        TypeData::Number => "number".into(),
        TypeData::String => "string".into(),
        TypeData::BigInt => "bigint".into(),
        TypeData::Symbol => "symbol".into(),
        TypeData::ObjectKeyword | TypeData::Object(_) => "object".into(),
        TypeData::Interface(_) => "interface".into(),
        TypeData::Class(_) => "class".into(),
        TypeData::Function(_) => "function".into(),
        TypeData::Tuple(_) => "tuple".into(),
        TypeData::Module(_) | TypeData::Namespace(_) => "namespace".into(),
        TypeData::Constructor(_) => "constructor".into(),
        TypeData::InstanceOf(instance) => type_description(db, instance.ty(db)),
        TypeData::Intersection(intersection) => intersection
            .types(db)
            .iter()
            .map(|ty| type_description(db, *ty))
            .collect::<Vec<_>>()
            .join(" & "),
        TypeData::Union(union) => union
            .types(db)
            .iter()
            .map(|ty| type_description(db, *ty))
            .collect::<Vec<_>>()
            .join(" | "),
        TypeData::Literal(literal) => match literal.literal(db) {
            Literal::BigInt(_) => "bigint".into(),
            Literal::Boolean(_) => "boolean".into(),
            Literal::Number(_) => "number".into(),
            Literal::String(_) | Literal::Template(_) => "string".into(),
            Literal::Object(_) => "object".into(),
            Literal::RegExp(_) => "RegExp".into(),
        },
        TypeData::Generic(_)
        | TypeData::Local(_)
        | TypeData::MergedReference(_)
        | TypeData::TypeOperator(_)
        | TypeData::TypeofExpression(_)
        | TypeData::TypeofType(_)
        | TypeData::TypeofValue(_)
        | TypeData::Conditional
        | TypeData::Global
        | TypeData::GlobalType(_)
        | TypeData::ThisKeyword => format!("{data:?}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interned_types::{InternedLiteral, InternedUnion};

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

    fn bigint<'db>(db: &'db TestDb, text: &'static str) -> InferredType<'db> {
        InferredType::new(
            db,
            TypeData::Literal(InternedLiteral::new(
                db,
                Literal::BigInt(Text::new_static(text)),
            )),
        )
    }

    #[test]
    fn bigint_semantics_use_canonical_values() {
        let db = TestDb::default();

        for text in ["0n", "-0n", "0b0n", "0o0n", "0x0n"] {
            let bigint = bigint(&db, text);
            assert!(bigint.is_bigint_literal(0), "{text}");
            assert!(bigint.is_always_falsy(), "{text}");
            assert_eq!(
                bigint.try_switch_case_variants(),
                Ok(vec![InferredSwitchCase::BigInt(Text::new_static("0n"))]),
                "{text}"
            );
        }

        let hexadecimal = bigint(&db, "0x10n");
        assert!(hexadecimal.is_bigint_literal(16));
        assert!(hexadecimal.is_always_truthy());
        assert_eq!(
            hexadecimal.try_switch_case_variants(),
            Ok(vec![InferredSwitchCase::BigInt(Text::new_static("16n"))])
        );
    }

    #[test]
    fn incomplete_predicates_are_indeterminate() {
        let db = TestDb::default();
        let unknown = InferredType::new(&db, TypeData::Unknown);

        assert_eq!(
            unknown.try_any_variant_matches(|_| false),
            Err(TypeTraversalError::UnresolvedType)
        );
        assert_eq!(
            unknown.try_all_variants_match(|_| true),
            Err(TypeTraversalError::UnresolvedType)
        );

        assert_eq!(unknown.is_promise_instance(), None);
        assert_eq!(unknown.has_nullish_variant(), None);
        assert_eq!(unknown.has_null_variant(), None);
        assert_eq!(unknown.has_undefined_variant(), None);
        assert_eq!(unknown.is_safe_for_nullish_coalescing(), None);

        let null_or_unknown = TypeData::Union(InternedUnion::new(
            &db,
            Vec::from([TypeData::Null, TypeData::Unknown]).into_boxed_slice(),
        ));
        let union = InferredType::new(&db, null_or_unknown);
        assert_eq!(union.has_nullish_variant(), Some(true));
        assert_eq!(union.has_undefined_variant(), None);
        assert_eq!(union.is_safe_for_nullish_coalescing(), None);
    }
}

use crate::TypeDb;
use crate::interned_types::{ConditionalType, Literal, ReturnType, TypeData, TypeMember};
use crate::misleading_return::{
    MisleadingReturnType, ReturnTypeEvidence, check_misleading_return_type,
};
use biome_rowan::Text;
use rustc_hash::FxHashSet;
use std::collections::VecDeque;

const MAX_TYPE_VARIANT_STEPS: usize = 1024;
const MAX_TYPE_RELATION_STEPS: usize = 50;
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

/// A database-backed type value returned by type inference.
#[derive(Clone, Copy)]
pub struct InferredType<'db> {
    db: &'db dyn TypeDb,
    data: TypeData<'db>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IgnoredPrimitiveTypes {
    pub string: bool,
    pub number: bool,
    pub boolean: bool,
    pub bigint: bool,
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
        matches!(
            self.data,
            TypeData::Literal(literal)
                if matches!(literal.literal(self.db), Literal::BigInt(number) if number.text().trim_end_matches('n').parse() == Ok(value))
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
        // Exhaustion is conservative: every consumer treats `false` as "do not
        // diagnose" (and therefore cannot offer a fix).
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

    pub fn is_promise_instance(self) -> Option<bool> {
        is_promise_instance(self.db, self.data)
    }

    pub fn is_function(self) -> bool {
        self.data.callable_function(self.db).is_some()
    }

    pub fn is_at_least_as_wide_as_object(self) -> bool {
        is_at_least_as_wide_as_object(self.db, self.data, &mut FxHashSet::default(), 0)
    }

    pub fn check_misleading_return_type(
        self,
        returns: &[Self],
        evidence: ReturnTypeEvidence,
        is_async: bool,
    ) -> Option<MisleadingReturnType> {
        check_misleading_return_type(
            self.db,
            self.data,
            returns.iter().map(|ty| ty.data).collect::<Vec<_>>(),
            evidence,
            is_async,
        )
    }

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

    pub fn has_nullish_variant(self) -> Option<bool> {
        self.try_any_variant_matches(|data| {
            matches!(
                data,
                TypeData::Null | TypeData::Undefined | TypeData::VoidKeyword
            )
        })
    }

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
    }

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
                Literal::Object(_) | Literal::RegExp(_) | Literal::Template(_) => false,
            },
            _ => false,
        })
    }

    pub fn has_null_variant(self) -> Option<bool> {
        self.try_any_variant_matches(|data| matches!(data, TypeData::Null))
    }

    pub fn has_undefined_variant(self) -> Option<bool> {
        self.try_any_variant_matches(|data| {
            matches!(data, TypeData::Undefined | TypeData::VoidKeyword)
        })
    }

    pub fn has_invalid_plus_operand_variant(self) -> bool {
        // Exhaustion returns `false`, which can only suppress this diagnostic.
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

    pub fn try_switch_case_variants(self) -> Option<Vec<InferredSwitchCase>> {
        let mut cases = Vec::new();
        let mut seen = FxHashSet::default();
        let mut pending = vec![self.data];
        let mut remaining_steps = MAX_TYPE_VARIANT_STEPS;

        while let Some(data) = pending.pop() {
            if !seen.insert(data) {
                continue;
            }
            if remaining_steps == 0 {
                return None;
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
                        InferredSwitchCase::BigInt(canonical_bigint_text(bigint))
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
        Some(cases)
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
        mut predicate: impl FnMut(TypeData<'db>) -> bool,
    ) -> Option<bool> {
        enum Visit<'db> {
            Enter(TypeData<'db>),
            Exit(TypeData<'db>),
        }

        let mut saw_variant = false;
        let mut indeterminate = false;
        let mut active = FxHashSet::default();
        let mut completed = FxHashSet::default();
        let mut pending = vec![Visit::Enter(self.data)];
        let mut remaining_steps = MAX_TYPE_VARIANT_STEPS;

        while let Some(visit) = pending.pop() {
            let data = match visit {
                Visit::Exit(data) => {
                    active.remove(&data);
                    completed.insert(data);
                    continue;
                }
                Visit::Enter(data) => data,
            };
            if completed.contains(&data) {
                continue;
            }
            if !active.insert(data) {
                indeterminate = true;
                continue;
            }
            if remaining_steps == 0 {
                return None;
            }
            remaining_steps -= 1;
            pending.push(Visit::Exit(data));

            match data {
                TypeData::Union(union) => {
                    if union.types(self.db).is_empty() {
                        return Some(false);
                    }
                    pending.extend(union.types(self.db).iter().copied().map(Visit::Enter));
                }
                TypeData::Generic(generic) => {
                    let Some(constraint) = generic.constraint(self.db) else {
                        indeterminate = true;
                        continue;
                    };
                    pending.push(Visit::Enter(constraint));
                }
                data if is_indeterminate_type(data) => indeterminate = true,
                _ if predicate(data) => saw_variant = true,
                _ => return Some(false),
            }
        }

        if indeterminate {
            None
        } else {
            Some(saw_variant)
        }
    }

    fn conditional_type(self) -> ConditionalType {
        let mut conditional = ConditionalType::Unknown;
        let mut seen = FxHashSet::default();
        let mut pending = vec![self.data];
        let mut remaining_steps = MAX_TYPE_VARIANT_STEPS;

        while let Some(data) = pending.pop() {
            if !seen.insert(data) {
                continue;
            }
            if remaining_steps == 0 {
                return ConditionalType::Unknown;
            }
            remaining_steps -= 1;

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
                    TypeData::InstanceOf(instance) => {
                        let target = instance.ty(self.db);
                        if target.is_array_class(self.db) {
                            conditional = if conditional == ConditionalType::Unknown {
                                ConditionalType::Truthy
                            } else {
                                conditional.merged_with(ConditionalType::Truthy)
                            };
                        } else {
                            pending.push(target);
                        }
                    }
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

        conditional
    }

    fn could_equal_literal(self, mut predicate: impl FnMut(TypeData<'db>) -> Option<bool>) -> bool {
        let mut seen = FxHashSet::default();
        let mut pending = vec![self.data];
        let mut remaining_steps = MAX_TYPE_VARIANT_STEPS;

        while let Some(data) = pending.pop() {
            if !seen.insert(data) {
                continue;
            }
            if remaining_steps == 0 {
                return true;
            }
            remaining_steps -= 1;

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

        false
    }

    fn try_any_variant_matches(
        self,
        mut predicate: impl FnMut(TypeData<'db>) -> bool,
    ) -> Option<bool> {
        enum Visit<'db> {
            Enter(TypeData<'db>),
            Exit(TypeData<'db>),
        }

        let mut indeterminate = false;
        let mut active = FxHashSet::default();
        let mut completed = FxHashSet::default();
        let mut pending = vec![Visit::Enter(self.data)];
        let mut remaining_steps = MAX_TYPE_VARIANT_STEPS;

        while let Some(visit) = pending.pop() {
            let data = match visit {
                Visit::Exit(data) => {
                    active.remove(&data);
                    completed.insert(data);
                    continue;
                }
                Visit::Enter(data) => data,
            };
            if completed.contains(&data) {
                continue;
            }
            if !active.insert(data) {
                indeterminate = true;
                continue;
            }
            if remaining_steps == 0 {
                return None;
            }
            remaining_steps -= 1;
            pending.push(Visit::Exit(data));

            match data {
                TypeData::Generic(generic) => {
                    if let Some(constraint) = generic.constraint(self.db) {
                        pending.push(Visit::Enter(constraint));
                    } else {
                        indeterminate = true;
                    }
                }
                TypeData::InstanceOf(instance) => {
                    pending.push(Visit::Enter(instance.ty(self.db)));
                }
                TypeData::TypeofType(typeof_type) => {
                    pending.push(Visit::Enter(typeof_type.ty(self.db)));
                }
                TypeData::TypeofValue(typeof_value) => {
                    pending.push(Visit::Enter(typeof_value.ty(self.db)));
                }
                TypeData::Union(union) => {
                    pending.extend(union.types(self.db).iter().copied().map(Visit::Enter));
                }
                data if is_indeterminate_type(data) => indeterminate = true,
                data if predicate(data) => return Some(true),
                _ => {}
            }
        }

        if indeterminate { None } else { Some(false) }
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
        let mut remaining_steps = MAX_TYPE_VARIANT_STEPS;

        while let Some(data) = pending.pop() {
            if !seen.insert(data) {
                continue;
            }
            if remaining_steps == 0 {
                return false;
            }
            remaining_steps -= 1;

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

        // `useDisposables` only diagnoses and fixes on `true`, so exhaustion
        // cannot create a false positive or an unsafe fix.
        false
    }
}

fn canonical_bigint_text(text: &Text) -> Text {
    let raw = text.text().strip_suffix('n').unwrap_or(text.text());
    let cleaned = raw.replace('_', "");
    let (negative, unsigned) = cleaned
        .strip_prefix('-')
        .map_or((false, cleaned.as_str()), |value| (true, value));
    let (radix, digits) = if let Some(value) = unsigned.strip_prefix("0x") {
        (16, value)
    } else if let Some(value) = unsigned.strip_prefix("0o") {
        (8, value)
    } else if let Some(value) = unsigned.strip_prefix("0b") {
        (2, value)
    } else {
        (10, unsigned)
    };

    let mut decimal = vec![0_u8];
    for digit in digits.chars() {
        let Some(digit) = digit.to_digit(radix) else {
            return text.clone();
        };
        let mut carry = digit;
        for decimal_digit in &mut decimal {
            let value = u32::from(*decimal_digit) * radix + carry;
            *decimal_digit = (value % 10) as u8;
            carry = value / 10;
        }
        while carry != 0 {
            decimal.push((carry % 10) as u8);
            carry /= 10;
        }
    }
    while decimal.len() > 1 && decimal.last() == Some(&0) {
        decimal.pop();
    }

    let is_zero = decimal == [0];
    let mut result = String::with_capacity(decimal.len() + 2);
    if negative && !is_zero {
        result.push('-');
    }
    result.extend(
        decimal
            .into_iter()
            .rev()
            .map(|digit| char::from(b'0' + digit)),
    );
    result.push('n');
    Text::new_owned(result.into())
}

fn is_at_least_as_wide_as_object<'db>(
    db: &'db dyn TypeDb,
    ty: TypeData<'db>,
    seen: &mut FxHashSet<TypeData<'db>>,
    depth: usize,
) -> bool {
    // `true` treats the cast target as wide and suppresses
    // `noMisleadingReturnType`; it cannot create a diagnostic.
    if depth >= MAX_TYPE_RELATION_STEPS || !seen.insert(ty) {
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
        // `Always` suppresses `noBaseToString`; unknown ancestry must never be
        // interpreted as proof that base object stringification is used.
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

fn has_custom_stringification_member(members: &[TypeMember<'_>]) -> bool {
    members.iter().any(|member| {
        ["toLocaleString", "toString", "valueOf"]
            .iter()
            .any(|name| member.kind.has_name(name))
    })
}

pub(crate) fn is_promise_instance<'db>(db: &'db dyn TypeDb, data: TypeData<'db>) -> Option<bool> {
    let mut completed = FxHashSet::default();
    let mut pending = VecDeque::from([(data, Vec::new())]);
    let mut processed = 0;
    let mut indeterminate = false;

    while let Some((data, path)) = pending.pop_front() {
        if path.contains(&data) {
            indeterminate = true;
            continue;
        }
        if !completed.insert(data) {
            continue;
        }
        if processed == MAX_PROMISE_TYPE_STEPS {
            indeterminate = true;
            continue;
        }
        processed += 1;

        if is_promise_target(db, data) {
            return Some(true);
        }
        if is_indeterminate_type(data) {
            indeterminate = true;
            continue;
        }

        let mut child_path = path;
        child_path.push(data);
        let mut push = |child| pending.push_back((child, child_path.clone()));
        match data {
            TypeData::Class(class) => class.extends(db).into_iter().for_each(&mut push),
            TypeData::Generic(generic) => generic.constraint(db).into_iter().for_each(&mut push),
            TypeData::InstanceOf(instance) => push(instance.ty(db)),
            TypeData::Interface(interface) => {
                interface.extends(db).iter().copied().for_each(&mut push);
            }
            TypeData::Intersection(intersection) => {
                intersection.types(db).iter().copied().for_each(&mut push);
            }
            TypeData::MergedReference(reference) => reference.targets(db).for_each(&mut push),
            TypeData::TypeOperator(operator) => push(operator.ty(db)),
            TypeData::TypeofType(typeof_type) => push(typeof_type.ty(db)),
            TypeData::TypeofValue(typeof_value) => push(typeof_value.ty(db)),
            TypeData::Union(union) => {
                union.types(db).iter().copied().for_each(&mut push);
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
            | TypeData::UnknownKeyword => unreachable!(),
        }
    }

    if indeterminate { None } else { Some(false) }
}

fn is_promise_target<'db>(db: &'db dyn TypeDb, data: TypeData<'db>) -> bool {
    data.is_promise_class(db)
        || matches!(data, TypeData::Class(class) if class.name(db).as_ref().is_some_and(|name| name.text() == "PromiseLike"))
        || matches!(data, TypeData::Interface(interface) if interface.name(db).text() == "PromiseLike")
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
        | TypeData::GlobalType(_)
        | TypeData::Local(_)
        | TypeData::MergedReference(_)
        | TypeData::TypeOperator(_)
        | TypeData::TypeofExpression(_)
        | TypeData::TypeofType(_)
        | TypeData::TypeofValue(_)
        | TypeData::Conditional
        | TypeData::Global
        | TypeData::ThisKeyword => "unknown".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        globals_ids::PROMISE_ID_GLOBAL_TYPE_ID,
        interned_types::{
            InternedClass, InternedFunction, InternedGenericTypeParameter, InternedInterface,
            InternedTypeInstance, InternedTypeofType, InternedUnion, TypeMemberKind,
        },
    };
    use salsa::plumbing::FromId;

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

    #[salsa::input]
    struct PromiseChainSteps {
        steps: usize,
    }

    #[salsa::tracked]
    fn promise_chain_status(db: &dyn TypeDb, input: PromiseChainSteps) -> Option<bool> {
        is_promise_instance(db, promise_class_chain(db, input.steps(db)))
    }

    fn promise_class_chain<'db>(db: &'db dyn TypeDb, steps: usize) -> TypeData<'db> {
        assert!(steps >= 2);
        let target = (2..steps).fold(
            TypeData::GlobalType(PROMISE_ID_GLOBAL_TYPE_ID),
            |extends, _| {
                TypeData::Class(InternedClass::new(
                    db,
                    Box::default(),
                    Some(extends),
                    Box::default(),
                    Box::default(),
                    None,
                ))
            },
        );
        TypeData::InstanceOf(InternedTypeInstance::new(db, target, Box::default()))
    }

    fn promise_like_interface_chain<'db>(db: &'db dyn TypeDb, steps: usize) -> TypeData<'db> {
        assert!(steps >= 2);
        let target = (2..steps).fold(
            TypeData::Interface(InternedInterface::new(
                db,
                Box::default(),
                Box::default(),
                Box::default(),
                Text::new_static("PromiseLike"),
            )),
            |extends, _| {
                TypeData::Interface(InternedInterface::new(
                    db,
                    Box::default(),
                    Vec::from([extends]).into_boxed_slice(),
                    Box::default(),
                    Text::new_static("Derived"),
                ))
            },
        );
        TypeData::InstanceOf(InternedTypeInstance::new(db, target, Box::default()))
    }

    fn typeof_chain<'db>(db: &'db TestDb, distinct_types: usize) -> TypeData<'db> {
        assert!(distinct_types > 0);
        (1..distinct_types).fold(TypeData::String, |ty, _| {
            TypeData::TypeofType(InternedTypeofType::new(db, ty))
        })
    }

    fn generic_chain<'db>(
        db: &'db TestDb,
        distinct_types: usize,
        leaf: TypeData<'db>,
    ) -> TypeData<'db> {
        assert!(distinct_types > 0);
        (1..distinct_types).fold(leaf, |constraint, index| {
            TypeData::Generic(InternedGenericTypeParameter::new(
                db,
                Some(constraint),
                None,
                Text::new_owned(format!("T{index}").into_boxed_str()),
            ))
        })
    }

    fn class_chain<'db>(
        db: &'db TestDb,
        distinct_types: usize,
        leaf_name: &'static str,
        leaf_members: Box<[TypeMember<'db>]>,
    ) -> TypeData<'db> {
        assert!(distinct_types > 0);
        (1..distinct_types).fold(
            TypeData::Class(InternedClass::new(
                db,
                Box::default(),
                None,
                Box::default(),
                leaf_members,
                Some(Text::new_static(leaf_name)),
            )),
            |extends, _| {
                TypeData::Class(InternedClass::new(
                    db,
                    Box::default(),
                    Some(extends),
                    Box::default(),
                    Box::default(),
                    None,
                ))
            },
        )
    }

    #[test]
    fn variant_predicates_report_exhaustion() {
        let db = TestDb::default();

        for distinct_types in [
            MAX_TYPE_VARIANT_STEPS - 1,
            MAX_TYPE_VARIANT_STEPS,
            MAX_TYPE_VARIANT_STEPS + 1,
        ] {
            let expected = (distinct_types <= MAX_TYPE_VARIANT_STEPS).then_some(true);
            let nullish =
                InferredType::new(&db, generic_chain(&db, distinct_types, TypeData::Null));
            assert_eq!(nullish.has_nullish_variant(), expected);
            assert_eq!(nullish.has_null_variant(), expected);
            assert_eq!(nullish.is_safe_for_nullish_coalescing(), expected);
        }
    }

    #[test]
    fn ignored_names_and_disposable_members_observe_variant_limit() {
        let db = TestDb::default();
        let dispose_member = TypeMember {
            kind: TypeMemberKind::ComputedValueNamed(
                Text::new_static("Symbol.dispose"),
                TypeData::Symbol,
            ),
            ty: TypeData::Unknown,
        };

        for distinct_types in [
            MAX_TYPE_VARIANT_STEPS - 1,
            MAX_TYPE_VARIANT_STEPS,
            MAX_TYPE_VARIANT_STEPS + 1,
        ] {
            let ignored = class_chain(&db, distinct_types, "Ignored", Box::default());
            assert_eq!(
                is_ignored_stringification_type(&db, ignored, &["Ignored"]),
                (distinct_types <= MAX_TYPE_VARIANT_STEPS).then_some(true)
            );
            assert_eq!(
                InferredType::new(&db, ignored)
                    .stringification_usefulness(StringificationMode::ToString, &["Ignored"]),
                StringificationUsefulness::Always
            );

            let disposable = class_chain(
                &db,
                distinct_types,
                "DisposableLeaf",
                Box::new([dispose_member.clone()]),
            );
            assert_eq!(
                InferredType::new(&db, disposable).is_disposable(),
                distinct_types <= MAX_TYPE_VARIANT_STEPS
            );
        }
    }

    #[test]
    fn capped_predicates_terminate_on_cycles() {
        let db = TestDb::default();
        let reference = InternedGenericTypeParameter::from_id(unsafe { salsa::Id::from_index(0) });
        let cycle = TypeData::Generic(InternedGenericTypeParameter::new(
            &db,
            Some(TypeData::Generic(reference)),
            None,
            Text::new_static("Cycle"),
        ));

        assert_eq!(InferredType::new(&db, cycle).has_null_variant(), None);
        assert_eq!(
            is_ignored_stringification_type(&db, cycle, &["Missing"]),
            Some(false)
        );
        assert!(!InferredType::new(&db, cycle).is_disposable());
    }

    #[test]
    fn nullish_predicates_preserve_indeterminate_variants() {
        let db = TestDb::default();
        let unconstrained = TypeData::Generic(InternedGenericTypeParameter::new(
            &db,
            None,
            None,
            Text::new_static("T"),
        ));

        for data in [
            TypeData::Unknown,
            TypeData::UnknownKeyword,
            TypeData::AnyKeyword,
            unconstrained,
        ] {
            let ty = InferredType::new(&db, data);
            assert_eq!(ty.has_null_variant(), None);
            assert_eq!(ty.has_undefined_variant(), None);
            assert_eq!(ty.is_safe_for_nullish_coalescing(), None);
        }

        let null_or_unknown = TypeData::Union(InternedUnion::new(
            &db,
            Vec::from([TypeData::Null, TypeData::Unknown]).into_boxed_slice(),
        ));
        let ty = InferredType::new(&db, null_or_unknown);
        assert_eq!(ty.has_null_variant(), Some(true));
        assert_eq!(ty.has_undefined_variant(), None);
        assert_eq!(ty.is_safe_for_nullish_coalescing(), None);
    }

    #[test]
    fn switch_case_variants_observe_distinct_type_limit() {
        let db = TestDb::default();

        for distinct_types in [
            MAX_TYPE_VARIANT_STEPS - 1,
            MAX_TYPE_VARIANT_STEPS,
            MAX_TYPE_VARIANT_STEPS + 1,
        ] {
            let variants = InferredType::new(&db, typeof_chain(&db, distinct_types))
                .try_switch_case_variants();
            if distinct_types <= MAX_TYPE_VARIANT_STEPS {
                assert_eq!(variants, Some(Vec::new()));
            } else {
                assert_eq!(variants, None);
            }
        }
    }

    #[test]
    fn switch_case_variants_do_not_charge_repeated_types() {
        let db = TestDb::default();
        let chain = typeof_chain(&db, MAX_TYPE_VARIANT_STEPS - 1);
        let repeated = TypeData::Union(InternedUnion::new(
            &db,
            vec![chain; MAX_TYPE_VARIANT_STEPS + 1].into_boxed_slice(),
        ));

        assert_eq!(
            InferredType::new(&db, repeated).try_switch_case_variants(),
            Some(Vec::new())
        );
    }

    #[test]
    fn switch_case_variants_terminate_on_cycles() {
        let db = TestDb::default();
        // The first interned value receives index zero, so its child points back to itself.
        let self_reference = InternedTypeofType::from_id(unsafe { salsa::Id::from_index(0) });
        let cycle = TypeData::TypeofType(InternedTypeofType::new(
            &db,
            TypeData::TypeofType(self_reference),
        ));

        assert_eq!(
            InferredType::new(&db, cycle).try_switch_case_variants(),
            Some(Vec::new())
        );
    }

    #[test]
    fn promise_inheritance_boundaries_are_tristate() {
        let db = TestDb::default();

        for steps in [63, 64, 65] {
            let expected = (steps <= 64).then_some(true);
            assert_eq!(
                is_promise_instance(&db, promise_class_chain(&db, steps)),
                expected,
                "Promise inheritance steps {steps}"
            );
            assert_eq!(
                is_promise_instance(&db, promise_like_interface_chain(&db, steps)),
                expected,
                "PromiseLike inheritance steps {steps}"
            );
        }
    }

    #[test]
    fn promise_status_distinguishes_completion_cycles_and_union_poisoning() {
        let db = TestDb::default();
        assert_eq!(is_promise_instance(&db, TypeData::Number), Some(false));

        let self_reference = InternedClass::from_id(unsafe { salsa::Id::from_index(0) });
        let cycle = TypeData::InstanceOf(InternedTypeInstance::new(
            &db,
            TypeData::Class(InternedClass::new(
                &db,
                Box::default(),
                Some(TypeData::Class(self_reference)),
                Box::default(),
                Box::default(),
                None,
            )),
            Box::default(),
        ));
        assert_eq!(is_promise_instance(&db, cycle), None);

        let promise = TypeData::promise_instance(&db, Box::new([TypeData::Number]));
        for types in [
            Vec::from([TypeData::Number, TypeData::Unknown]),
            Vec::from([TypeData::Unknown, TypeData::Number]),
        ] {
            let union = TypeData::Union(InternedUnion::new(&db, types.into_boxed_slice()));
            assert_eq!(is_promise_instance(&db, union), None);
        }
        for types in [
            Vec::from([promise, TypeData::Unknown]),
            Vec::from([TypeData::Unknown, promise]),
        ] {
            let union = TypeData::Union(InternedUnion::new(&db, types.into_boxed_slice()));
            assert_eq!(is_promise_instance(&db, union), Some(true));
        }

        let function = TypeData::Function(InternedFunction::new(
            &db,
            Box::default(),
            Box::default(),
            ReturnType::Type(TypeData::Unknown),
            false,
            None,
        ));
        assert_eq!(
            InferredType::new(&db, function).function_returns_promise(),
            None
        );
        let array = TypeData::instance_of(
            &db,
            TypeData::GlobalType(crate::globals_ids::ARRAY_ID_GLOBAL_TYPE_ID),
            Box::new([TypeData::Unknown]),
        );
        assert_eq!(InferredType::new(&db, array).is_array_of_promise(), None);
    }

    #[test]
    fn promise_status_is_repeatable_and_invalidates() {
        let mut db = TestDb::default();
        let input = PromiseChainSteps::new(&db, 65);
        assert_eq!(promise_chain_status(&db, input), None);
        assert_eq!(promise_chain_status(&db, input), None);

        salsa::Setter::to(input.set_steps(&mut db), 64);
        assert_eq!(promise_chain_status(&db, input), Some(true));
        assert_eq!(promise_chain_status(&db, input), Some(true));
    }
}

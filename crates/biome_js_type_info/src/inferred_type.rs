use crate::TypeDb;
use crate::interned_types::{
    ConditionalType, InternedClass, Literal, ReturnType, TypeData, TypeMember, TypeMemberKind,
};
use biome_rowan::Text;
use rustc_hash::FxHashSet;

const MAX_TYPE_VARIANT_STEPS: usize = 1024;
const MAX_GENERIC_CONSTRAINT_HOPS: usize = 6;

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

#[derive(Clone, Copy, Debug, Default)]
pub struct ReturnTypeEvidence {
    pub has_any_const: bool,
    pub object_wide_casts: usize,
    pub has_narrower_than_object: bool,
    pub has_pinning_assertion: bool,
    pub prefer_inferred_suggestion: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MisleadingReturnType {
    pub suggestion: Option<String>,
}

/// A Salsa-backed type value returned by type inference.
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
        self.all_variants_match(|data| {
            matches!(data, TypeData::String)
                || matches!(
                    data,
                    TypeData::Literal(literal)
                        if matches!(literal.literal(self.db), Literal::String(_))
                )
        })
    }

    pub fn is_all_number_like(self) -> bool {
        self.all_variants_match(|data| {
            matches!(data, TypeData::Number)
                || matches!(
                    data,
                    TypeData::Literal(literal)
                        if matches!(literal.literal(self.db), Literal::Number(_))
                )
        })
    }

    pub fn is_all_boolean_like(self) -> bool {
        self.all_variants_match(|data| {
            matches!(data, TypeData::Boolean)
                || matches!(
                    data,
                    TypeData::Literal(literal)
                        if matches!(literal.literal(self.db), Literal::Boolean(_))
                )
        })
    }

    pub fn is_all_bigint_like(self) -> bool {
        self.all_variants_match(|data| {
            matches!(data, TypeData::BigInt)
                || matches!(
                    data,
                    TypeData::Literal(literal)
                        if matches!(literal.literal(self.db), Literal::BigInt(_))
                )
        })
    }

    pub fn is_all_integer_like(self) -> bool {
        self.all_variants_match(|data| match data {
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
    }

    pub fn is_all_string_array_or_tuple(self) -> bool {
        self.all_variants_match(|data| {
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

    pub fn is_array_of_promise(self) -> bool {
        let TypeData::InstanceOf(instance) = self.data else {
            return false;
        };

        instance.ty(self.db).is_array_class(self.db)
            && instance
                .type_parameters(self.db)
                .first()
                .is_some_and(|ty| is_promise_instance(self.db, *ty))
    }

    pub fn is_disposable(self) -> bool {
        self.has_computed_member("Symbol.dispose")
    }

    pub fn is_async_disposable(self) -> bool {
        self.has_computed_member("Symbol.asyncDispose")
    }

    pub fn is_promise_instance(self) -> bool {
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
        let mut annotation = self.data;
        if is_escape_hatch(annotation) {
            return None;
        }
        if is_async {
            annotation = promise_inner(self.db, annotation).unwrap_or(annotation);
        }
        annotation =
            collapse_union_absorbed_by_primitive(self.db, annotation).unwrap_or(annotation);

        let return_types = normalize_boolean_return_types(
            self.db,
            returns.iter().map(|ty| ty.data).collect::<Vec<_>>(),
        );
        if return_types.is_empty() {
            return None;
        }
        if return_types.len() == 1
            && !evidence.has_any_const
            && !evidence.has_pinning_assertion
            && is_literal_of_primitive(self.db, return_types[0])
            && !matches!(annotation, TypeData::Union(_))
        {
            return None;
        }
        if !evidence.has_any_const
            && evidence.object_wide_casts == return_types.len()
            && matches!(annotation, TypeData::ObjectKeyword)
        {
            return None;
        }
        if matches!(annotation, TypeData::Boolean)
            && return_types
                .iter()
                .any(|ty| ty.is_boolean_literal(self.db, true))
            && return_types
                .iter()
                .any(|ty| ty.is_boolean_literal(self.db, false))
        {
            return None;
        }
        if return_types
            .iter()
            .any(|ty| is_any_contaminated(self.db, *ty))
        {
            return None;
        }
        if matches!(annotation, TypeData::Union(_))
            && union_variants(self.db, annotation)
                .iter()
                .any(|ty| matches!(ty, TypeData::UnknownKeyword | TypeData::Unknown))
        {
            return None;
        }
        if includes_undefined(self.db, annotation)
            && !return_types
                .iter()
                .any(|ty| includes_undefined(self.db, *ty))
        {
            return None;
        }
        if return_types
            .iter()
            .any(|ty| is_intersection_with_type_param(self.db, *ty))
        {
            return None;
        }
        if !evidence.has_any_const
            && is_only_property_literal_widening(self.db, annotation, &return_types)
        {
            return None;
        }

        let is_misleading = if matches!(annotation, TypeData::Union(_)) {
            is_union_wider_than_returns(self.db, annotation, &return_types)
        } else if matches!(annotation, TypeData::ObjectKeyword) {
            !return_types
                .iter()
                .any(|ty| includes_object_keyword(self.db, *ty))
                && evidence.object_wide_casts == 0
                && (evidence.has_narrower_than_object
                    || return_types
                        .iter()
                        .any(|ty| is_wider_than(self.db, annotation, *ty)))
        } else {
            return_types
                .iter()
                .all(|ty| is_wider_than(self.db, annotation, *ty))
        };
        if !is_misleading {
            return None;
        }

        let suggestion = if evidence.has_any_const || evidence.prefer_inferred_suggestion {
            render_inferred(self.db, &return_types)
        } else {
            render_narrowed(self.db, annotation, &return_types)
                .or_else(|| render_inferred(self.db, &return_types))
        };
        Some(MisleadingReturnType { suggestion })
    }

    pub fn function_returns_promise(self) -> bool {
        let Some(function) = self.data.callable_function(self.db) else {
            return false;
        };
        let ReturnType::Type(return_ty) = function.return_type(self.db) else {
            return false;
        };
        contains_promise(self.db, *return_ty)
    }

    pub fn function_returns_conditional(self) -> bool {
        self.function_return_matches(|ty| matches!(ty, TypeData::Conditional))
    }

    pub fn function_returns_void(self) -> bool {
        self.function_return_matches(|ty| matches!(ty, TypeData::VoidKeyword))
    }

    pub fn has_promise_variant(self) -> bool {
        match self.data {
            TypeData::Union(union) => union
                .types(self.db)
                .iter()
                .any(|ty| is_promise_instance(self.db, *ty)),
            _ => false,
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

    pub fn has_nullish_variant(self) -> bool {
        self.any_variant_matches(|data| {
            matches!(
                data,
                TypeData::Null | TypeData::Undefined | TypeData::VoidKeyword
            )
        })
    }

    pub fn is_safe_for_nullish_coalescing(self) -> bool {
        self.all_variants_match(|data| {
            if matches!(data, TypeData::InstanceOf(_)) {
                return true;
            }
            matches!(
                data.conditional_type_shallow(self.db),
                Some(ConditionalType::Truthy | ConditionalType::Nullish)
            )
        })
    }

    pub fn nullish_union_matches_ignored_primitives(self, ignored: IgnoredPrimitiveTypes) -> bool {
        let TypeData::Union(_) = self.data else {
            return false;
        };

        self.all_variants_match(|data| match data {
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

    pub fn has_null_variant(self) -> bool {
        self.any_variant_matches(|data| matches!(data, TypeData::Null))
    }

    pub fn has_undefined_variant(self) -> bool {
        self.any_variant_matches(|data| matches!(data, TypeData::Undefined | TypeData::VoidKeyword))
    }

    pub fn has_invalid_plus_operand_variant(self) -> bool {
        self.any_variant_matches(|data| match data {
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
    }

    pub fn has_number_like_variant(self) -> bool {
        self.any_variant_matches(|data| {
            matches!(data, TypeData::Number)
                || matches!(
                    data,
                    TypeData::Literal(literal)
                        if matches!(literal.literal(self.db), Literal::Number(_))
                )
        })
    }

    pub fn has_bigint_like_variant(self) -> bool {
        self.any_variant_matches(|data| {
            matches!(data, TypeData::BigInt)
                || matches!(
                    data,
                    TypeData::Literal(literal)
                        if matches!(literal.literal(self.db), Literal::BigInt(_))
                )
        })
    }

    pub fn plus_operand_description(self) -> String {
        type_description(self.db, self.data)
    }

    pub fn switch_case_variants(self) -> Vec<InferredSwitchCase> {
        let mut cases = Vec::new();
        let mut seen = FxHashSet::default();
        let mut pending = vec![self.data];

        for _ in 0..MAX_TYPE_VARIANT_STEPS {
            let Some(data) = pending.pop() else {
                break;
            };
            if !seen.insert(data) {
                continue;
            }

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
        cases
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

    fn all_variants_match(self, mut predicate: impl FnMut(TypeData<'db>) -> bool) -> bool {
        let mut saw_variant = false;
        let mut seen = FxHashSet::default();
        let mut pending = vec![self.data];

        for _ in 0..MAX_TYPE_VARIANT_STEPS {
            let Some(data) = pending.pop() else {
                return saw_variant;
            };
            if !seen.insert(data) {
                continue;
            }

            match data {
                TypeData::Union(union) => {
                    if union.types(self.db).is_empty() {
                        return false;
                    }
                    pending.extend(union.types(self.db).iter().copied());
                }
                TypeData::Generic(generic) => {
                    let Some(constraint) = generic.constraint(self.db) else {
                        return false;
                    };
                    pending.push(constraint);
                }
                _ if predicate(data) => saw_variant = true,
                _ => return false,
            }
        }

        false
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

    fn any_variant_matches(self, mut predicate: impl FnMut(TypeData<'db>) -> bool) -> bool {
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
                TypeData::Generic(generic) => {
                    if let Some(constraint) = generic.constraint(self.db) {
                        pending.push(constraint);
                    }
                }
                TypeData::InstanceOf(instance) => pending.push(instance.ty(self.db)),
                TypeData::TypeofType(typeof_type) => pending.push(typeof_type.ty(self.db)),
                TypeData::TypeofValue(typeof_value) => pending.push(typeof_value.ty(self.db)),
                TypeData::Union(union) => {
                    pending.extend(union.types(self.db).iter().copied());
                }
                data if predicate(data) => return true,
                _ => {}
            }
        }

        false
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

const MAX_RETURN_TYPE_STEPS: usize = 50;
const MAX_RETURN_TYPE_DESCRIPTION_LENGTH: usize = 80;
const RETURN_TYPE_SEPARATOR: &str = " | ";

fn is_escape_hatch(ty: TypeData<'_>) -> bool {
    matches!(
        ty,
        TypeData::AnyKeyword
            | TypeData::VoidKeyword
            | TypeData::UnknownKeyword
            | TypeData::NeverKeyword
            | TypeData::Unknown
            | TypeData::ThisKeyword
    )
}

fn normalize_boolean_return_types<'db>(
    db: &'db dyn TypeDb,
    mut types: Vec<TypeData<'db>>,
) -> Vec<TypeData<'db>> {
    let has_boolean = types.contains(&TypeData::Boolean);
    let has_true = types.iter().any(|ty| ty.is_boolean_literal(db, true));
    let has_false = types.iter().any(|ty| ty.is_boolean_literal(db, false));
    if !(has_boolean || has_true && has_false) {
        return types;
    }

    let mut seen_boolean = false;
    types.retain(|ty| {
        if matches!(ty, TypeData::Boolean) {
            if seen_boolean {
                return false;
            }
            seen_boolean = true;
            return true;
        }
        !ty.is_boolean_literal(db, true) && !ty.is_boolean_literal(db, false)
    });
    if !seen_boolean {
        types.push(TypeData::Boolean);
    }
    types
}

fn promise_inner<'db>(db: &'db dyn TypeDb, ty: TypeData<'db>) -> Option<TypeData<'db>> {
    let TypeData::InstanceOf(instance) = ty else {
        return None;
    };
    if !instance.ty(db).is_promise_class(db) {
        return None;
    }
    instance
        .type_parameters(db)
        .first()
        .copied()
        .filter(|ty| !is_escape_hatch(*ty))
}

fn union_variants<'db>(db: &'db dyn TypeDb, ty: TypeData<'db>) -> Vec<TypeData<'db>> {
    match ty {
        TypeData::Union(union) => union.types(db).to_vec(),
        ty => Vec::from([ty]),
    }
}

fn collapse_union_absorbed_by_primitive<'db>(
    db: &'db dyn TypeDb,
    ty: TypeData<'db>,
) -> Option<TypeData<'db>> {
    let TypeData::Union(_) = ty else {
        return None;
    };
    let variants = union_variants(db, ty);
    let mut primitive = None;
    for variant in &variants {
        if matches!(
            variant,
            TypeData::String | TypeData::Number | TypeData::Boolean | TypeData::BigInt
        ) {
            if primitive.is_some() {
                return None;
            }
            primitive = Some(*variant);
        }
    }
    let primitive = primitive?;
    variants
        .iter()
        .all(|variant| {
            types_match(db, *variant, primitive) || is_nonunion_wider(db, primitive, *variant)
        })
        .then_some(primitive)
}

fn includes_object_keyword(db: &dyn TypeDb, ty: TypeData<'_>) -> bool {
    matches!(ty, TypeData::ObjectKeyword)
        || matches!(ty, TypeData::Union(_))
            && union_variants(db, ty)
                .iter()
                .any(|variant| matches!(variant, TypeData::ObjectKeyword))
}

fn includes_undefined(db: &dyn TypeDb, ty: TypeData<'_>) -> bool {
    matches!(ty, TypeData::Undefined | TypeData::VoidKeyword)
        || matches!(ty, TypeData::Union(_))
            && union_variants(db, ty)
                .iter()
                .any(|variant| matches!(variant, TypeData::Undefined | TypeData::VoidKeyword))
}

fn is_any_contaminated(db: &dyn TypeDb, ty: TypeData<'_>) -> bool {
    match ty {
        TypeData::AnyKeyword => true,
        TypeData::Union(union) => union
            .types(db)
            .iter()
            .any(|ty| matches!(ty, TypeData::AnyKeyword)),
        TypeData::Intersection(intersection) => intersection
            .types(db)
            .iter()
            .any(|ty| matches!(ty, TypeData::AnyKeyword)),
        _ => false,
    }
}

fn is_intersection_with_type_param(db: &dyn TypeDb, ty: TypeData<'_>) -> bool {
    matches!(ty, TypeData::Intersection(intersection) if intersection.types(db).iter().any(|ty| matches!(ty, TypeData::Generic(_))))
}

fn is_literal_of_primitive(db: &dyn TypeDb, ty: TypeData<'_>) -> bool {
    match ty {
        TypeData::Literal(literal) => matches!(
            literal.literal(db),
            Literal::BigInt(_)
                | Literal::Boolean(_)
                | Literal::Number(_)
                | Literal::String(_)
                | Literal::Template(_)
        ),
        TypeData::Union(union) if union.types(db).len() == 1 => {
            is_literal_of_primitive(db, union.types(db)[0])
        }
        _ => false,
    }
}

fn is_base_type_of_literal(db: &dyn TypeDb, base: TypeData<'_>, literal: TypeData<'_>) -> bool {
    matches!(literal.literal_base_type(db), Some(literal_base) if literal_base == base)
}

fn is_only_property_literal_widening(
    db: &dyn TypeDb,
    annotation: TypeData<'_>,
    returns: &[TypeData<'_>],
) -> bool {
    returns.iter().all(|inferred| {
        let mut stack = vec![(annotation, *inferred)];
        let mut has_widening = false;
        let mut iterations = 0;

        while let Some((annotated, inferred)) = stack.pop() {
            iterations += 1;
            if iterations > MAX_RETURN_TYPE_STEPS {
                return false;
            }

            if let TypeData::Tuple(annotated_tuple) = annotated {
                let TypeData::Tuple(inferred_tuple) = inferred else {
                    return false;
                };
                let annotated_elements = annotated_tuple.elements(db);
                let inferred_elements = inferred_tuple.elements(db);
                if annotated_elements.len() != inferred_elements.len()
                    || annotated_elements.is_empty()
                {
                    return false;
                }
                for (annotated_element, inferred_element) in
                    annotated_elements.iter().zip(inferred_elements)
                {
                    if types_match(db, annotated_element.ty, inferred_element.ty) {
                        continue;
                    }
                    if is_base_type_of_literal(db, annotated_element.ty, inferred_element.ty) {
                        has_widening = true;
                    } else {
                        stack.push((annotated_element.ty, inferred_element.ty));
                    }
                }
                continue;
            }

            let TypeData::Object(annotated_object) = annotated else {
                return false;
            };
            let annotated_members = annotated_object.members(db);
            if annotated_members.is_empty() {
                return false;
            }
            let inferred_members = match inferred {
                TypeData::Object(object) => object.members(db),
                TypeData::Literal(literal) => match literal.literal(db) {
                    Literal::Object(members) => members,
                    _ => return false,
                },
                _ => return false,
            };
            if inferred_members.is_empty() {
                return false;
            }

            if let Some(index_signature) = annotated_members
                .iter()
                .find_map(|member| member.kind.index_signature_type().map(|_| member))
            {
                let mut index_has_widening = false;
                let all_covered = inferred_members.iter().all(|member| {
                    if member.kind.is_const_asserted() {
                        return false;
                    }
                    if types_match(db, index_signature.ty, member.ty) {
                        return true;
                    }
                    if is_base_type_of_literal(db, index_signature.ty, member.ty) {
                        index_has_widening = true;
                        return true;
                    }
                    false
                });
                if !(all_covered && index_has_widening) {
                    return false;
                }
                has_widening = true;
                continue;
            }

            for annotated_member in annotated_members {
                let Some(name) = annotated_member.kind.name() else {
                    continue;
                };
                let Some(inferred_member) = inferred_members
                    .iter()
                    .find(|member| member.kind.has_name(name.text()))
                else {
                    return false;
                };
                if inferred_member.kind.is_const_asserted() {
                    return false;
                }
                if types_match(db, annotated_member.ty, inferred_member.ty) {
                    continue;
                }
                if is_base_type_of_literal(db, annotated_member.ty, inferred_member.ty) {
                    has_widening = true;
                } else {
                    stack.push((annotated_member.ty, inferred_member.ty));
                }
            }
        }

        has_widening
    })
}

fn resolve_generic_chain<'db>(db: &'db dyn TypeDb, mut ty: TypeData<'db>) -> TypeData<'db> {
    for _ in 0..MAX_GENERIC_CONSTRAINT_HOPS {
        let TypeData::Generic(generic) = ty else {
            break;
        };
        let Some(constraint) = generic.constraint(db) else {
            break;
        };
        ty = constraint;
    }
    ty
}

fn type_members<'db>(db: &'db dyn TypeDb, ty: TypeData<'db>) -> Option<&'db [TypeMember<'db>]> {
    match ty {
        TypeData::Object(object) => Some(object.members(db)),
        TypeData::Literal(literal) => match literal.literal(db) {
            Literal::Object(members) => Some(members),
            _ => None,
        },
        _ => None,
    }
}

fn is_strictly_narrower_than_object_keyword(db: &dyn TypeDb, ty: TypeData<'_>) -> bool {
    match ty {
        TypeData::Object(object) => !object.members(db).is_empty(),
        TypeData::InstanceOf(instance) => match instance.ty(db) {
            TypeData::Class(class) => {
                class_has_instance_shape(db, class, &mut FxHashSet::default(), 0)
            }
            _ => true,
        },
        TypeData::Tuple(_) | TypeData::Function(_) => true,
        TypeData::Literal(literal) => match literal.literal(db) {
            Literal::RegExp(_) => true,
            Literal::Object(members) => !members.is_empty(),
            _ => false,
        },
        _ => false,
    }
}

fn class_has_instance_shape<'db>(
    db: &'db dyn TypeDb,
    class: InternedClass<'db>,
    seen: &mut FxHashSet<TypeData<'db>>,
    depth: usize,
) -> bool {
    let ty = TypeData::Class(class);
    if depth >= MAX_RETURN_TYPE_STEPS || !seen.insert(ty) {
        return false;
    }
    if class
        .members(db)
        .iter()
        .any(|member| !member.kind.is_static())
    {
        return true;
    }
    class.extends(db).is_some_and(|base| match base {
        TypeData::Class(base) => class_has_instance_shape(db, base, seen, depth + 1),
        TypeData::InstanceOf(instance) => match instance.ty(db) {
            TypeData::Class(base) => class_has_instance_shape(db, base, seen, depth + 1),
            _ => true,
        },
        _ => true,
    })
}

fn is_nonunion_wider<'db>(
    db: &'db dyn TypeDb,
    annotated: TypeData<'db>,
    inferred: TypeData<'db>,
) -> bool {
    let mut stack = vec![(annotated, resolve_generic_chain(db, inferred))];
    let mut found_wider = false;
    let mut iterations = 0;

    while let Some((annotated, inferred)) = stack.pop() {
        iterations += 1;
        if iterations > MAX_RETURN_TYPE_STEPS {
            return false;
        }
        if is_base_type_of_literal(db, annotated, inferred) {
            found_wider = true;
            continue;
        }
        if types_match(db, annotated, inferred) {
            continue;
        }

        match (annotated, inferred) {
            (TypeData::ObjectKeyword, TypeData::InstanceOf(_)) => found_wider = true,
            (TypeData::InstanceOf(annotated), TypeData::InstanceOf(inferred)) => {
                if !types_match(db, annotated.ty(db), inferred.ty(db)) {
                    return false;
                }
                let annotated_parameters = annotated.type_parameters(db);
                let inferred_parameters = inferred.type_parameters(db);
                if annotated_parameters.len() != inferred_parameters.len()
                    || annotated_parameters.is_empty()
                {
                    return false;
                }
                stack.extend(annotated_parameters.iter().zip(inferred_parameters).map(
                    |(annotated, inferred)| (*annotated, resolve_generic_chain(db, *inferred)),
                ));
            }
            (TypeData::Object(_), TypeData::Object(_) | TypeData::Literal(_)) => {
                if !push_object_pairs(db, annotated, inferred, &mut stack) {
                    return false;
                }
            }
            (TypeData::ObjectKeyword, inferred)
                if is_strictly_narrower_than_object_keyword(db, inferred) =>
            {
                found_wider = true;
            }
            (TypeData::Tuple(annotated), TypeData::Tuple(inferred)) => {
                let annotated_elements = annotated.elements(db);
                let inferred_elements = inferred.elements(db);
                if annotated_elements.len() != inferred_elements.len()
                    || annotated_elements.is_empty()
                {
                    return false;
                }
                stack.extend(annotated_elements.iter().zip(inferred_elements).map(
                    |(annotated, inferred)| (annotated.ty, resolve_generic_chain(db, inferred.ty)),
                ));
            }
            _ => return false,
        }
    }

    found_wider
}

fn push_object_pairs<'db>(
    db: &'db dyn TypeDb,
    annotated: TypeData<'db>,
    inferred: TypeData<'db>,
    stack: &mut Vec<(TypeData<'db>, TypeData<'db>)>,
) -> bool {
    let Some(annotated_members) = type_members(db, annotated) else {
        return false;
    };
    let Some(inferred_members) = type_members(db, inferred) else {
        return false;
    };
    if annotated_members.is_empty() || inferred_members.is_empty() {
        return false;
    }

    if let Some(index_signature) = annotated_members.iter().find(|member| {
        matches!(
            member.kind,
            TypeMemberKind::IndexSignature(_) | TypeMemberKind::ConstAssertedIndexSignature(_)
        )
    }) {
        stack.extend(
            inferred_members
                .iter()
                .map(|member| (index_signature.ty, resolve_generic_chain(db, member.ty))),
        );
        return true;
    }

    for annotated_member in annotated_members {
        let Some(name) = annotated_member.kind.name() else {
            continue;
        };
        let Some(inferred_member) = inferred_members
            .iter()
            .find(|member| member.kind.has_name(name.text()))
        else {
            return false;
        };
        stack.push((
            annotated_member.ty,
            resolve_generic_chain(db, inferred_member.ty),
        ));
    }
    true
}

fn is_wider_than<'db>(
    db: &'db dyn TypeDb,
    annotated: TypeData<'db>,
    inferred: TypeData<'db>,
) -> bool {
    let inferred = resolve_generic_chain(db, inferred);
    match (annotated, inferred) {
        (TypeData::String, TypeData::String)
        | (TypeData::Number, TypeData::Number)
        | (TypeData::Boolean, TypeData::Boolean)
        | (TypeData::BigInt, TypeData::BigInt) => false,
        (TypeData::Union(_), _) => is_union_wider(db, annotated, inferred),
        (_, TypeData::Union(_)) => {
            let variants = union_variants(db, inferred);
            let has_base = variants
                .iter()
                .any(|variant| types_match(db, annotated, *variant));
            let all_subsumed = variants.iter().all(|variant| {
                types_match(db, annotated, *variant)
                    || is_base_type_of_literal(db, annotated, *variant)
            });
            if has_base && all_subsumed {
                return false;
            }
            variants.iter().all(|variant| {
                types_match(db, annotated, *variant) || is_nonunion_wider(db, annotated, *variant)
            }) && variants
                .iter()
                .any(|variant| is_nonunion_wider(db, annotated, *variant))
        }
        _ => is_nonunion_wider(db, annotated, inferred),
    }
}

fn is_union_wider_than_returns<'db>(
    db: &'db dyn TypeDb,
    annotated: TypeData<'db>,
    returns: &[TypeData<'db>],
) -> bool {
    let variants = union_variants(db, annotated);
    if !returns.iter().all(|return_ty| {
        variants.iter().any(|variant| {
            types_match(db, *variant, *return_ty) || is_nonunion_wider(db, *variant, *return_ty)
        })
    }) {
        return false;
    }
    let has_extra = variants.iter().any(|variant| {
        !returns.iter().any(|return_ty| {
            types_match(db, *variant, *return_ty) || is_nonunion_wider(db, *variant, *return_ty)
        })
    });
    let has_wider = returns.iter().any(|return_ty| {
        !variants
            .iter()
            .any(|variant| types_match(db, *variant, *return_ty))
            && variants
                .iter()
                .any(|variant| is_nonunion_wider(db, *variant, *return_ty))
    });
    has_extra || has_wider
}

fn is_union_wider<'db>(
    db: &'db dyn TypeDb,
    annotated: TypeData<'db>,
    inferred: TypeData<'db>,
) -> bool {
    let annotated_variants = union_variants(db, annotated);
    let inferred_variants = union_variants(db, inferred);
    if !inferred_variants.iter().all(|inferred| {
        annotated_variants.iter().any(|annotated| {
            types_match(db, *annotated, *inferred) || is_nonunion_wider(db, *annotated, *inferred)
        })
    }) {
        return false;
    }

    annotated_variants
        .iter()
        .filter(|variant| {
            let TypeData::Generic(generic) = variant else {
                return true;
            };
            let Some(constraint) = generic.constraint(db) else {
                return true;
            };
            !annotated_variants.iter().any(|other| {
                other != *variant
                    && (types_match(db, *other, constraint)
                        || is_nonunion_wider(db, *other, constraint))
            })
        })
        .any(|annotated| {
            !inferred_variants.iter().any(|inferred| {
                types_match(db, *annotated, *inferred)
                    || is_nonunion_wider(db, *annotated, *inferred)
            })
        })
}

fn types_match<'db>(
    db: &'db dyn TypeDb,
    mut left: TypeData<'db>,
    mut right: TypeData<'db>,
) -> bool {
    for _ in 0..MAX_RETURN_TYPE_STEPS {
        if left == right {
            return true;
        }
        match (left, right) {
            (TypeData::Generic(left_generic), TypeData::Generic(right_generic)) => {
                return left_generic.name(db) == right_generic.name(db);
            }
            (TypeData::InstanceOf(left_instance), TypeData::InstanceOf(right_instance))
                if left_instance.type_parameters(db).is_empty()
                    && right_instance.type_parameters(db).is_empty() =>
            {
                left = left_instance.ty(db);
                right = right_instance.ty(db);
            }
            (TypeData::Generic(generic), TypeData::InstanceOf(instance))
                if instance.type_parameters(db).is_empty() =>
            {
                return matches!(instance.ty(db), TypeData::Generic(other) if generic.name(db) == other.name(db));
            }
            (TypeData::InstanceOf(instance), TypeData::Generic(generic))
                if instance.type_parameters(db).is_empty() =>
            {
                return matches!(instance.ty(db), TypeData::Generic(other) if generic.name(db) == other.name(db));
            }
            _ => return false,
        }
    }
    false
}

fn is_at_least_as_wide_as_object<'db>(
    db: &'db dyn TypeDb,
    ty: TypeData<'db>,
    seen: &mut FxHashSet<TypeData<'db>>,
    depth: usize,
) -> bool {
    if depth >= MAX_RETURN_TYPE_STEPS || !seen.insert(ty) {
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

fn literal_text(db: &dyn TypeDb, ty: TypeData<'_>) -> Option<String> {
    let TypeData::Literal(literal) = ty else {
        return None;
    };
    match literal.literal(db) {
        Literal::String(value) => Some(format!("\"{}\"", value.as_str())),
        Literal::Number(value) => Some(value.as_str().to_string()),
        Literal::Boolean(value) => Some(value.as_bool().to_string()),
        _ => None,
    }
}

fn renderable_variant(db: &dyn TypeDb, ty: TypeData<'_>) -> Option<String> {
    match ty {
        TypeData::String => Some("string".into()),
        TypeData::Number => Some("number".into()),
        TypeData::Boolean => Some("boolean".into()),
        TypeData::BigInt => Some("bigint".into()),
        _ => literal_text(db, ty),
    }
}

fn clean_literal_text(text: &str) -> bool {
    !text.contains("...") && !text.contains("__internal") && !text.contains("typeof import(")
}

fn join_description(parts: Vec<String>) -> Option<String> {
    if parts.is_empty() || parts.iter().any(|part| !clean_literal_text(part)) {
        return None;
    }
    let description = parts.join(RETURN_TYPE_SEPARATOR);
    (description.len() <= MAX_RETURN_TYPE_DESCRIPTION_LENGTH).then_some(description)
}

fn render_inferred(db: &dyn TypeDb, returns: &[TypeData<'_>]) -> Option<String> {
    join_description(
        returns
            .iter()
            .map(|ty| literal_text(db, *ty))
            .collect::<Option<Vec<_>>>()?,
    )
}

fn render_narrowed<'db>(
    db: &'db dyn TypeDb,
    annotation: TypeData<'db>,
    returns: &[TypeData<'db>],
) -> Option<String> {
    let variants = union_variants(db, annotation);
    let covered = variants
        .iter()
        .filter(|variant| {
            returns.iter().any(|return_ty| {
                types_match(db, **variant, *return_ty)
                    || is_nonunion_wider(db, **variant, *return_ty)
            })
        })
        .copied()
        .collect::<Vec<_>>();
    if covered.is_empty() || covered.len() == variants.len() {
        return None;
    }
    let has_widening = covered.iter().any(|variant| {
        returns.iter().any(|return_ty| {
            !types_match(db, *variant, *return_ty) && is_nonunion_wider(db, *variant, *return_ty)
        })
    });
    if has_widening
        && !(covered.len() == 1 && returns.len() == 1 && is_literal_of_primitive(db, returns[0]))
    {
        return None;
    }
    join_description(
        covered
            .iter()
            .map(|ty| renderable_variant(db, *ty))
            .collect::<Option<Vec<_>>>()?,
    )
}

fn stringification_usefulness<'db>(
    db: &'db dyn TypeDb,
    data: TypeData<'db>,
    mode: StringificationMode,
    ignored_type_names: &[&str],
    active: &mut FxHashSet<TypeData<'db>>,
    depth: usize,
) -> StringificationUsefulness {
    use StringificationUsefulness::{Always, Never};

    if depth >= MAX_TYPE_VARIANT_STEPS || !active.insert(data) {
        return Always;
    }

    let result = if let TypeData::Generic(generic) = data {
        generic.constraint(db).map_or(Always, |constraint| {
            stringification_usefulness(db, constraint, mode, ignored_type_names, active, depth + 1)
        })
    } else if matches!(mode, StringificationMode::ToString)
        && (is_ignored_stringification_type(db, data, ignored_type_names)
            || is_safe_stringification_type(db, data))
    {
        Always
    } else {
        match data {
            TypeData::Union(union) => {
                combine_stringification_union(union.types(db).iter().map(|ty| {
                    stringification_usefulness(db, *ty, mode, ignored_type_names, active, depth + 1)
                }))
            }
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
            _ => match uses_base_object_stringification(
                db,
                data,
                &mut FxHashSet::default(),
                depth + 1,
            ) {
                Some(true) => Never,
                Some(false) | None => Always,
            },
        }
    };

    active.remove(&data);
    result
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
) -> bool {
    let mut seen = FxHashSet::default();
    let mut pending = vec![data];

    for _ in 0..MAX_TYPE_VARIANT_STEPS {
        let Some(data) = pending.pop() else {
            return false;
        };
        if !seen.insert(data) {
            continue;
        }

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
            return true;
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

    false
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

fn has_custom_stringification_member(members: &[TypeMember<'_>]) -> bool {
    members.iter().any(|member| {
        ["toLocaleString", "toString", "valueOf"]
            .iter()
            .any(|name| member.kind.has_name(name))
    })
}

fn is_promise_instance<'db>(db: &'db dyn TypeDb, mut data: TypeData<'db>) -> bool {
    while let TypeData::InstanceOf(instance) = data {
        data = instance.ty(db);
        if data.is_promise_class(db) {
            return true;
        }
    }

    false
}

fn contains_promise<'db>(db: &'db dyn TypeDb, data: TypeData<'db>) -> bool {
    match data {
        TypeData::Union(union) => union.types(db).iter().any(|ty| contains_promise(db, *ty)),
        data => is_promise_instance(db, data),
    }
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
        | TypeData::ThisKeyword => "unknown".into(),
    }
}

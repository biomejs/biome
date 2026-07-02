use std::borrow::Cow;
use std::hash::{Hash, Hasher};

use biome_rowan::Text;
use hashbrown::{HashTable, hash_table::Entry};
use rustc_hash::FxHasher;

use crate::{
    CallArgumentType, ConstructorParameter, DestructureField, Function, FunctionParameter, Literal,
    MAX_FLATTEN_DEPTH, Resolvable, ResolvedTypeData, ResolvedTypeId, ResolvedTypeMember,
    ResolverId, TypeData, TypeMember, TypeReference, TypeResolver, TypeofCallExpression,
    TypeofDestructureExpression, TypeofExpression,
    conditionals::{
        ConditionalType, reference_to_falsy_subset_of, reference_to_non_nullish_subset_of,
        reference_to_truthy_subset_of,
    },
    globals::{
        GLOBAL_ARRAY_ID, GLOBAL_BIGINT_STRING_LITERAL_ID, GLOBAL_BOOLEAN_STRING_LITERAL_ID,
        GLOBAL_FUNCTION_STRING_LITERAL_ID, GLOBAL_NUMBER_STRING_LITERAL_ID,
        GLOBAL_OBJECT_STRING_LITERAL_ID, GLOBAL_STRING_STRING_LITERAL_ID,
        GLOBAL_SYMBOL_STRING_LITERAL_ID, GLOBAL_TYPEOF_OPERATOR_RETURN_UNION_ID,
        GLOBAL_UNDEFINED_ID, GLOBAL_UNDEFINED_STRING_LITERAL_ID,
    },
};

pub(super) fn flattened_expression(
    expr: &TypeofExpression,
    resolver: &mut dyn TypeResolver,
) -> Option<TypeData> {
    match expr {
        TypeofExpression::Addition(expr) => {
            let left = resolver.resolve_and_get(&expr.left)?;
            let right = resolver.resolve_and_get(&expr.right)?;

            let coerced_ty = |ty: &TypeData| -> Option<TypeData> {
                match ty {
                    TypeData::BigInt => Some(TypeData::BigInt),
                    TypeData::Boolean | TypeData::Null | TypeData::Number | TypeData::Undefined => {
                        Some(TypeData::Number)
                    }
                    TypeData::Class(_)
                    | TypeData::InstanceOf(_)
                    | TypeData::Interface(_)
                    | TypeData::Object(_)
                    | TypeData::ObjectKeyword
                    | TypeData::String => Some(TypeData::String),
                    TypeData::Literal(literal) => match literal.as_ref() {
                        Literal::BigInt(_) => Some(TypeData::BigInt),
                        Literal::Boolean(_) | Literal::Number(_) => Some(TypeData::Number),
                        Literal::Object(_)
                        | Literal::RegExp(_)
                        | Literal::String(_)
                        | Literal::Template(_) => Some(TypeData::String),
                    },
                    TypeData::Unknown => Some(TypeData::Unknown),
                    _ => None,
                }
            };

            match (
                coerced_ty(left.as_raw_data()),
                coerced_ty(right.as_raw_data()),
            ) {
                (Some(TypeData::BigInt), Some(TypeData::BigInt)) => Some(TypeData::BigInt),
                (Some(TypeData::Number), Some(TypeData::Number)) => Some(TypeData::number()),
                (Some(TypeData::String), _) | (_, Some(TypeData::String)) => {
                    Some(TypeData::string())
                }
                (Some(TypeData::Unknown), Some(TypeData::Unknown)) => Some(TypeData::unknown()),
                _ => None,
            }
        }
        TypeofExpression::Await(expr) => {
            let arg = resolver.resolve_and_get(&expr.argument)?;
            let flattened = match arg.as_raw_data() {
                TypeData::BigInt => TypeData::BigInt,
                TypeData::Boolean => TypeData::Boolean,
                TypeData::Class(class) => {
                    arg.apply_module_id_to_data(TypeData::Class(class.clone()))
                }
                TypeData::Function(function) => {
                    arg.apply_module_id_to_data(TypeData::Function(function.clone()))
                }
                TypeData::Literal(literal) => TypeData::Literal(literal.clone()),
                TypeData::Null => TypeData::Null,
                TypeData::Number => TypeData::Number,
                TypeData::Object(object) => {
                    arg.apply_module_id_to_data(TypeData::Object(object.clone()))
                }
                TypeData::String => TypeData::String,
                _ => arg.find_promise_type(resolver)?.to_data(),
            };
            Some(flattened)
        }
        TypeofExpression::BitwiseNot(expr) => {
            resolver
                .resolve_and_get(&expr.argument)
                .map(|resolved| match resolved.is_big_int() {
                    true => TypeData::BigInt,
                    false => TypeData::Number,
                })
        }
        TypeofExpression::Call(expr) => match resolver.resolve_and_get(&expr.callee) {
            Some(callee) => flattened_call(expr, callee.to_data(), resolver),
            None => None,
        },
        TypeofExpression::Conditional(expr) => {
            let test = resolver.resolve_and_get(&expr.test)?;
            let conditional = ConditionalType::from_resolved_data(test, resolver);
            if conditional.is_truthy() {
                Some(TypeData::reference(expr.consequent.clone()))
            } else if conditional.is_falsy() {
                Some(TypeData::reference(expr.alternate.clone()))
            } else {
                conditional.is_inferred().then(|| {
                    TypeData::union_of(
                        resolver,
                        [expr.consequent.clone(), expr.alternate.clone()].into(),
                    )
                })
            }
        }
        TypeofExpression::Destructure(expr) => flattened_destructure(expr, resolver),
        TypeofExpression::Index(expr) => {
            let object = resolver.resolve_and_get(&expr.object)?;
            object
                .find_element_type_at_index(resolver, expr.index)
                .map(|element_reference| element_reference.into_reference(resolver))
                .and_then(|reference| resolver.resolve_and_get(&reference))
                .map(ResolvedTypeData::to_data)
        }
        TypeofExpression::IterableValueOf(expr) => {
            let ty = resolver.resolve_and_get(&expr.ty)?;
            match ty.as_raw_data() {
                TypeData::InstanceOf(instance)
                    if instance.ty == GLOBAL_ARRAY_ID.into()
                        && instance.has_known_type_parameters() =>
                {
                    instance
                        .type_parameters
                        .first()
                        .map(|param| ty.apply_module_id_to_reference(param))
                        .and_then(|param| resolver.resolve_and_get(&param))
                        .map(ResolvedTypeData::to_data)
                }
                _ => {
                    // TODO: Handle other iterable types
                    None
                }
            }
        }
        TypeofExpression::LogicalAnd(expr) => {
            let left = resolver.resolve_and_get(&expr.left)?;
            let conditional = ConditionalType::from_resolved_data(left, resolver);
            if conditional.is_falsy() {
                Some(left.to_data())
            } else if conditional.is_truthy() {
                Some(TypeData::reference(expr.right.clone()))
            } else if conditional.is_inferred() {
                let left = reference_to_falsy_subset_of(&left.to_data(), resolver)
                    .unwrap_or_else(|| expr.left.clone());
                Some(TypeData::union_of(
                    resolver,
                    [left, expr.right.clone()].into(),
                ))
            } else {
                None
            }
        }
        TypeofExpression::LogicalOr(expr) => {
            let left = resolver.resolve_and_get(&expr.left)?;
            let conditional = ConditionalType::from_resolved_data(left, resolver);
            if conditional.is_truthy() {
                Some(left.to_data())
            } else if conditional.is_falsy() {
                Some(TypeData::reference(expr.right.clone()))
            } else if conditional.is_inferred() {
                let left = reference_to_truthy_subset_of(&left.to_data(), resolver)
                    .unwrap_or_else(|| expr.left.clone());
                Some(TypeData::union_of(
                    resolver,
                    [left, expr.right.clone()].into(),
                ))
            } else {
                None
            }
        }
        TypeofExpression::New(expr) => flattened_new(expr, resolver),
        TypeofExpression::NullishCoalescing(expr) => {
            let left = resolver.resolve_and_get(&expr.left)?;
            let conditional = ConditionalType::from_resolved_data(left, resolver);
            if conditional.is_non_nullish() {
                Some(left.to_data())
            } else if conditional.is_nullish() {
                Some(TypeData::reference(expr.right.clone()))
            } else if conditional.is_inferred() {
                let left = reference_to_non_nullish_subset_of(&left.to_data(), resolver)
                    .unwrap_or_else(|| expr.left.clone());
                Some(TypeData::union_of(
                    resolver,
                    [left, expr.right.clone()].into(),
                ))
            } else {
                None
            }
        }
        TypeofExpression::StaticMember(expr) => {
            let object = resolver.resolve_and_get(&expr.object)?;
            if let TypeData::TypeofExpression(object_expr) = object.as_raw_data()
                && should_flatten_static_member_object(object_expr)
            {
                let object = flattened_static_member_object_expression(
                    object_expr.as_ref().clone(),
                    resolver,
                )?;
                let object = resolved_type_data_from_data(&object, resolver)?;
                flattened_static_member(object, &expr.member, resolver)
            } else {
                flattened_static_member(object, &expr.member, resolver)
            }
        }
        TypeofExpression::Super(expr) => {
            resolver
                .resolve_and_get(&expr.parent)
                .map(|resolved| match resolved.as_raw_data() {
                    TypeData::Class(class) => match class.extends.as_ref() {
                        Some(super_class) => TypeData::instance_of(
                            resolved
                                .apply_module_id_to_reference(super_class)
                                .into_owned(),
                        ),
                        None => TypeData::unknown(),
                    },
                    _ => TypeData::unknown(),
                })
        }
        TypeofExpression::This(expr) => resolver
            .resolve_reference(&expr.parent)
            .map(|class_id| TypeData::instance_of(TypeReference::from(class_id))),
        TypeofExpression::Typeof(expr) => resolver
            .resolve_and_get(&expr.argument)
            .map(flattened_typeof_data),
        TypeofExpression::UnaryMinus(expr) => {
            resolver
                .resolve_and_get(&expr.argument)
                .map(|resolved| match resolved.as_raw_data() {
                    TypeData::BigInt => TypeData::BigInt,
                    _ => TypeData::Number,
                })
        }
    }
}

/// Keeps scoped expressions lazy while allowing call/new member chains.
fn should_flatten_static_member_object(expr: &TypeofExpression) -> bool {
    matches!(
        expr,
        TypeofExpression::Call(_) | TypeofExpression::New(_) | TypeofExpression::StaticMember(_)
    )
}

struct FixedStack<T, const CAPACITY: usize> {
    items: [Option<T>; CAPACITY],
    len: usize,
}

impl<T, const CAPACITY: usize> FixedStack<T, CAPACITY> {
    fn new() -> Self {
        Self {
            items: std::array::from_fn(|_| None),
            len: 0,
        }
    }

    fn push(&mut self, item: T) -> Option<()> {
        if self.len == CAPACITY {
            return None;
        }

        self.items[self.len] = Some(item);
        self.len += 1;
        Some(())
    }

    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;
        self.items[self.len].take()
    }

    fn last(&self) -> Option<&T> {
        self.len
            .checked_sub(1)
            .and_then(|index| self.items[index].as_ref())
    }
}

/// Flattens the base of a lazy call/new/member chain before member lookup.
fn flattened_static_member_object_expression(
    mut expr: TypeofExpression,
    resolver: &mut dyn TypeResolver,
) -> Option<TypeData> {
    let mut pending_members = FixedStack::<Text, MAX_FLATTEN_DEPTH>::new();
    let mut remaining_depth = MAX_FLATTEN_DEPTH;

    while remaining_depth > 0 {
        remaining_depth -= 1;

        match expr {
            TypeofExpression::Call(expr) => {
                let callee = resolver.resolve_and_get(&expr.callee)?;
                let object = flattened_call(&expr, callee.to_data(), resolver)?;
                return apply_static_member_chain(object, pending_members, resolver);
            }
            TypeofExpression::New(expr) => {
                let object = flattened_new(&expr, resolver)?;
                return apply_static_member_chain(object, pending_members, resolver);
            }
            TypeofExpression::StaticMember(member_expr) => {
                let object = resolver.resolve_and_get(&member_expr.object)?;
                if let TypeData::TypeofExpression(object_expr) = object.as_raw_data()
                    && should_flatten_static_member_object(object_expr)
                {
                    pending_members.push(member_expr.member)?;
                    expr = object_expr.as_ref().clone();
                    continue;
                }

                let object = flattened_static_member(object, &member_expr.member, resolver)?;
                return apply_static_member_chain(object, pending_members, resolver);
            }
            _ => return None,
        }
    }

    None
}

/// Applies deferred member accesses after the chain base has been flattened.
fn apply_static_member_chain(
    mut object: TypeData,
    mut pending_members: FixedStack<Text, MAX_FLATTEN_DEPTH>,
    resolver: &mut dyn TypeResolver,
) -> Option<TypeData> {
    while let Some(member) = pending_members.pop() {
        let resolved = resolved_type_data_from_data(&object, resolver)?;
        object = flattened_static_member(resolved, &member, resolver)?;
    }

    Some(object)
}

/// Resolves references without registering already-owned type data.
fn resolved_type_data_from_data<'a>(
    data: &'a TypeData,
    resolver: &'a dyn TypeResolver,
) -> Option<ResolvedTypeData<'a>> {
    match data {
        TypeData::Reference(reference) => resolver.resolve_and_get(reference),
        _ => Some(ResolvedTypeData::from((
            ResolverId::from_level(resolver.level()),
            data,
        ))),
    }
}

/// Looks up a static member on already-resolved object data.
fn flattened_static_member(
    object: ResolvedTypeData,
    member_name: &Text,
    resolver: &dyn TypeResolver,
) -> Option<TypeData> {
    match object.as_raw_data() {
        class @ TypeData::Class(_) => {
            let member = class.own_members().find(|member| {
                member.has_name(member_name) && member.is_static() && !member.is_constructor()
            })?;
            Some(type_data_from_resolved_member(
                ResolvedTypeMember::from((object.resolver_id(), member)),
                resolver,
            ))
        }

        TypeData::ImportNamespace(module_id) => {
            let resolved_id = resolver.resolve_import_namespace_member(*module_id, member_name)?;
            resolver
                .get_by_resolved_id(resolved_id)
                .map(ResolvedTypeData::to_data)
        }

        TypeData::Tuple(_tuple) => {
            // Tuples are just fancy arrays, so make sure methods on
            // them can be looked up as such:
            let array = resolver.get_by_resolved_id(GLOBAL_ARRAY_ID)?;
            let member = array.find_member(resolver, |member| {
                member.has_name(member_name) && !member.is_static()
            })?;
            Some(type_data_from_resolved_member(member, resolver))
        }

        TypeData::Union(_) => {
            // Keep nested inferred expressions lazy; otherwise nested member
            // chains can grow without adding useful type information.
            let mut types = None;
            for variant in object.flattened_union_variants(resolver) {
                if variant == GLOBAL_UNDEFINED_ID.into() {
                    continue;
                }

                let Some(resolved) = resolver.resolve_and_get(&variant) else {
                    types
                        .get_or_insert_with(Vec::new)
                        .push(TypeReference::unknown());
                    continue;
                };

                if matches!(resolved.as_raw_data(), TypeData::TypeofExpression(_)) {
                    return None;
                }

                if matches!(resolved.as_raw_data(), TypeData::Unknown) {
                    types
                        .get_or_insert_with(Vec::new)
                        .push(TypeReference::unknown());
                    continue;
                }

                let Some(member) = resolved
                    .find_member(resolver, |member| {
                        static_member_matches(resolved, member, member_name)
                    })
                    .or_else(|| {
                        resolved.find_index_signature_with_ty(resolver, |data| data.is_string())
                    })
                else {
                    continue;
                };
                let is_optional = member.kind().is_optional();
                let member_reference = member.deref_ty(resolver).into_owned();
                push_member_reference(
                    types.get_or_insert_with(Vec::new),
                    member_reference,
                    is_optional,
                );
            }

            if let Some(types) = types {
                Some(TypeData::union_of(resolver, types.into_boxed_slice()))
            } else {
                Some(TypeData::unknown())
            }
        }

        _ => {
            let member = object
                .find_member(resolver, |member| {
                    static_member_matches(object, member, member_name)
                })
                .or_else(|| {
                    object.find_index_signature_with_ty(resolver, |data| data.is_string())
                })?;
            Some(type_data_from_resolved_member(member, resolver))
        }
    }
}

/// Matches static properties on class values and instance members elsewhere.
fn static_member_matches(
    object: ResolvedTypeData,
    member: &ResolvedTypeMember,
    name: &str,
) -> bool {
    member.has_name(name)
        && match object.as_raw_data() {
            TypeData::Class(_) => member.is_static() && !member.kind().is_constructor(),
            _ => !member.is_static(),
        }
}

/// Resolves the instance type produced by a `new` expression.
fn flattened_new(
    expr: &crate::TypeofNewExpression,
    resolver: &mut dyn TypeResolver,
) -> Option<TypeData> {
    let resolved = resolver.resolve_and_get(&expr.callee)?;
    if let TypeData::Class(class) = resolved.as_raw_data() {
        let argument_count = expr.arguments.len();
        let constructed_reference = class
            .members
            .iter()
            .filter(|member| member.kind.is_constructor())
            .find_map(|member| {
                let constructor =
                    resolver.resolve_and_get(&resolved.apply_module_id_to_reference(&member.ty))?;
                match constructor.to_data() {
                    TypeData::Constructor(constructor) => {
                        constructor_accepts_argument_count(&constructor.parameters, argument_count)
                            .then_some(constructor.return_type)
                            .flatten()
                    }
                    _ => None,
                }
            })
            .unwrap_or_else(|| expr.callee.clone());
        Some(TypeData::instance_of(constructed_reference))
    } else {
        None
    }
}

/// Builds the member access result from resolved member metadata.
fn type_data_from_resolved_member(
    member: ResolvedTypeMember<'_>,
    resolver: &dyn TypeResolver,
) -> TypeData {
    let is_optional = member.kind().is_optional();
    let member_reference = member.deref_ty(resolver).into_owned();
    type_data_from_member_reference(member_reference, is_optional, resolver)
}

/// Builds the destructured binding type while preserving optional members.
fn type_data_from_destructured_member(
    member: ResolvedTypeMember<'_>,
    resolver: &dyn TypeResolver,
) -> Option<TypeData> {
    let member_reference = member.deref_ty(resolver).into_owned();
    if member.kind().is_optional() {
        Some(type_data_from_member_reference(
            member_reference,
            true,
            resolver,
        ))
    } else {
        resolver
            .resolve_and_get(&member_reference)
            .map(ResolvedTypeData::to_data)
    }
}

/// Builds the member access result, including `undefined` for optional members.
fn type_data_from_member_reference(
    reference: TypeReference,
    is_optional: bool,
    resolver: &dyn TypeResolver,
) -> TypeData {
    if is_optional {
        TypeData::union_of(resolver, [reference, GLOBAL_UNDEFINED_ID.into()].into())
    } else {
        TypeData::reference(reference)
    }
}

/// Adds a member reference to a union result.
fn push_member_reference(
    types: &mut Vec<TypeReference>,
    reference: TypeReference,
    is_optional: bool,
) {
    types.push(reference);
    if is_optional {
        types.push(GLOBAL_UNDEFINED_ID.into());
    }
}

enum CalleeContinuation {
    Call(TypeofCallExpression),
    Member(Text),
}

/// Resolves a callee through lazy expression wrappers and callable object
/// indirections without recursive call flattening. Every wrapper transition
/// consumes one shared depth step.
fn resolve_callee_to_function(
    callee: TypeData,
    resolver: &mut dyn TypeResolver,
) -> Option<Box<Function>> {
    let mut callee = callee;
    let mut continuations = FixedStack::<CalleeContinuation, MAX_FLATTEN_DEPTH>::new();
    let mut remaining_steps = MAX_FLATTEN_DEPTH;

    loop {
        if let TypeData::TypeofExpression(expr) = callee {
            remaining_steps = remaining_steps.checked_sub(1)?;

            match *expr {
                TypeofExpression::Call(expr) => {
                    callee = resolver.resolve_and_get(&expr.callee)?.to_data();
                    continuations.push(CalleeContinuation::Call(expr))?;
                    continue;
                }
                TypeofExpression::New(expr) => {
                    callee = flattened_new(&expr, resolver)?;
                    continue;
                }
                TypeofExpression::StaticMember(expr) => {
                    callee = resolver.resolve_and_get(&expr.object)?.to_data();
                    continuations.push(CalleeContinuation::Member(expr.member))?;
                    continue;
                }
                expression => {
                    callee = flattened_expression(&expression, resolver)?;
                    continue;
                }
            }
        }

        if matches!(continuations.last(), Some(CalleeContinuation::Member(_))) {
            let member = match continuations.pop()? {
                CalleeContinuation::Member(member) => member,
                CalleeContinuation::Call(_) => return None,
            };
            let resolved = resolved_type_data_from_data(&callee, resolver)?;
            callee = flattened_static_member(resolved, &member, resolver)?;
            continue;
        }

        match callee {
            TypeData::Function(function) => {
                if let Some(CalleeContinuation::Call(expr)) = continuations.pop() {
                    callee = flattened_function_call(&expr, &function, resolver)?;
                    continue;
                }

                return Some(function);
            }
            TypeData::InstanceOf(instance) => {
                remaining_steps = remaining_steps.checked_sub(1)?;

                let instance_callee = resolver.resolve_and_get(&instance.ty)?;
                callee = if instance_callee.is_function() {
                    instance_callee.to_data()
                } else {
                    instance_callee
                        .find_member(resolver, |member| member.kind().is_call_signature())
                        .map(ResolvedTypeMember::to_member)
                        .and_then(|member| resolver.resolve_and_get(&member.deref_ty(resolver)))?
                        .to_data()
                };
            }
            TypeData::Class(_) | TypeData::Interface(_) | TypeData::Object(_) => {
                remaining_steps = remaining_steps.checked_sub(1)?;

                if let Some(CalleeContinuation::Call(expr)) = continuations.last() {
                    match select_overload(&callee, &expr.arguments, resolver) {
                        OverloadSelection::Selected(function) => {
                            callee = TypeData::Function(function);
                            continue;
                        }
                        OverloadSelection::NoMatch => return None,
                        OverloadSelection::NotOverloaded => {}
                    }
                }

                callee =
                    ResolvedTypeData::from((ResolverId::from_level(resolver.level()), &callee))
                        .find_member(resolver, |member| member.kind().is_call_signature())
                        .map(ResolvedTypeMember::to_member)
                        .and_then(|member| resolver.resolve_and_get(&member.deref_ty(resolver)))?
                        .to_data();
            }
            _ => return None,
        }
    }
}

/// Matches constructor overloads by arity, including optional and rest params.
fn constructor_accepts_argument_count(
    parameters: &[ConstructorParameter],
    argument_count: usize,
) -> bool {
    let required_count = parameters
        .iter()
        .filter(|parameter| {
            !function_parameter_is_optional(&parameter.parameter)
                && !function_parameter_is_rest(&parameter.parameter)
        })
        .count();
    let has_rest = parameters
        .iter()
        .any(|parameter| function_parameter_is_rest(&parameter.parameter));

    required_count <= argument_count && (has_rest || argument_count <= parameters.len())
}

/// Reads optionality from either supported parameter shape.
fn function_parameter_is_optional(parameter: &FunctionParameter) -> bool {
    match parameter {
        FunctionParameter::Named(parameter) => parameter.is_optional,
        FunctionParameter::Pattern(parameter) => parameter.is_optional,
    }
}

/// Reads rest-parameter status from either supported parameter shape.
fn function_parameter_is_rest(parameter: &FunctionParameter) -> bool {
    match parameter {
        FunctionParameter::Named(parameter) => parameter.is_rest,
        FunctionParameter::Pattern(parameter) => parameter.is_rest,
    }
}

fn flattened_call(
    expr: &TypeofCallExpression,
    callee: TypeData,
    resolver: &mut dyn TypeResolver,
) -> Option<TypeData> {
    match callee {
        TypeData::Union(union) => {
            // When calling a union-typed callee (e.g. `(() => Promise<void>) | undefined`),
            // resolve the call on each callable variant and collect return types.
            let mut return_types = Vec::new();
            for variant in union.types() {
                let Some(resolved) = resolver.resolve_and_get(variant) else {
                    continue;
                };
                match resolved.as_raw_data() {
                    // Optional call (`?.`): an `undefined` or `null` callee
                    // short-circuits to `undefined`, so preserve it in the
                    // result union instead of silently dropping it.
                    TypeData::Undefined | TypeData::Null => {
                        return_types.push(TypeData::Undefined);
                        continue;
                    }
                    TypeData::Unknown => {
                        continue;
                    }
                    _ => {}
                }
                if let Some(function) =
                    resolve_callee_for_call(resolved.to_data(), &expr.arguments, resolver)
                    && let Some(result) = flattened_function_call(expr, &function, resolver)
                {
                    return_types.push(result);
                }
            }
            match return_types.len() {
                0 => None,
                1 => Some(return_types.into_iter().next().unwrap()),
                _ => {
                    let references = return_types
                        .into_iter()
                        .map(|return_type| {
                            let type_id = resolver.register_type(Cow::Owned(return_type));
                            TypeReference::from(ResolvedTypeId::new(resolver.level(), type_id))
                        })
                        .collect::<Box<[_]>>();
                    Some(TypeData::union_of(resolver, references))
                }
            }
        }
        callee => {
            let function = resolve_callee_for_call(callee, &expr.arguments, resolver)?;
            flattened_function_call(expr, &function, resolver)
        }
    }
}

/// Resolves the callable target for a call expression, selecting overloads first.
fn resolve_callee_for_call(
    callee: TypeData,
    arguments: &[CallArgumentType],
    resolver: &mut dyn TypeResolver,
) -> Option<Box<Function>> {
    match select_overload(&callee, arguments, resolver) {
        OverloadSelection::Selected(function) => Some(function),
        // The callee is an overload set, but no signature accepts the arguments.
        // Its result type is unknown, so do not fall back to the first signature.
        OverloadSelection::NoMatch => None,
        OverloadSelection::NotOverloaded => resolve_callee_to_function(callee, resolver),
    }
}

/// Outcome of [`select_overload`].
enum OverloadSelection {
    /// The callee is not an overload set (it carries fewer than two call
    /// signatures); resolve it through the regular single-signature path.
    NotOverloaded,
    /// The callee is an overload set, but no signature accepts the arguments,
    /// so the call's result type is unknown — notably *not* the first
    /// signature's return type.
    NoMatch,
    /// A signature was selected for the given arguments.
    Selected(Box<Function>),
}

/// Selects an overload when the callee is an object or interface carrying more
/// than one call signature. This is the shape produced for a set of same-name
/// function declarations, but it also applies to a hand-written interface with
/// multiple call signatures.
///
/// Mirroring TypeScript, the first signature whose parameters accept the
/// arguments wins, in declaration order. The three-way result lets the caller
/// tell a non-overloaded callee (fall back to single-signature resolution)
/// apart from an overloaded callee with no matching signature (the result is
/// unknown, so the caller must *not* fall back to the first signature).
fn select_overload(
    callee: &TypeData,
    arguments: &[CallArgumentType],
    resolver: &dyn TypeResolver,
) -> OverloadSelection {
    let resolved = ResolvedTypeData::from((ResolverId::from_level(resolver.level()), callee));
    let mut call_signature_count = 0;
    let mut selected_function = None;

    for member in resolved
        .all_members(resolver)
        .filter(|member| member.kind().is_call_signature())
    {
        call_signature_count += 1;

        if selected_function.is_none() {
            let member = member.to_member();
            let signature = member.deref_ty(resolver);
            if let Some(TypeData::Function(function)) = resolver
                .resolve_and_get(&signature)
                .map(ResolvedTypeData::to_data)
                && signature_accepts_arguments(&function, arguments, resolver)
            {
                selected_function = Some(function);
            }
        }

        if call_signature_count > 1 && selected_function.is_some() {
            break;
        }
    }

    if call_signature_count < 2 {
        OverloadSelection::NotOverloaded
    } else {
        selected_function.map_or(OverloadSelection::NoMatch, OverloadSelection::Selected)
    }
}

/// Narrow applicability heuristic used for overload selection.
///
/// This is deliberately **not** a general assignability check: beyond an arity
/// check, it only disambiguates overloads that differ by whether a callback
/// argument returns a promise, which is the case that matters for
/// promise-related rules. Any argument whose shape it does not understand (e.g.
/// a non-function parameter/argument pair) places no constraint, so the first
/// arity-compatible signature in declaration order wins.
///
/// Callers must therefore treat a `true` result as "this signature is *not
/// ruled out*", not as "the arguments are assignable". When real assignability
/// infrastructure exists, replace this body with `is_assignable_to` per
/// parameter; the [`select_overload`] entry point does not need to change.
fn signature_accepts_arguments(
    function: &Function,
    arguments: &[CallArgumentType],
    resolver: &dyn TypeResolver,
) -> bool {
    let parameters = &function.parameters;
    let accepts_rest = parameters.last().is_some_and(FunctionParameter::is_rest);
    let required = parameters
        .iter()
        .filter(|parameter| !parameter.is_optional() && !parameter.is_rest())
        .count();
    if arguments.len() < required || (!accepts_rest && arguments.len() > parameters.len()) {
        return false;
    }

    parameters
        .iter()
        .zip(arguments)
        .all(|(parameter, argument)| {
            let (CallArgumentType::Argument(argument) | CallArgumentType::Spread(argument)) =
                argument;
            match (
                resolve_function(parameter.ty(), resolver),
                resolve_function(argument, resolver),
            ) {
                (Some(parameter_fn), Some(argument_fn)) => {
                    function_returns_promise(&parameter_fn, resolver)
                        == function_returns_promise(&argument_fn, resolver)
                }
                _ => true,
            }
        })
}

fn resolve_function(ty: &TypeReference, resolver: &dyn TypeResolver) -> Option<Box<Function>> {
    match resolver.resolve_and_get(ty)?.to_data() {
        TypeData::Function(function) => Some(function),
        // Callable interfaces/objects: `interface Cb { (): Promise<void> }`
        TypeData::Interface(interface) => interface
            .members
            .iter()
            .find(|m| m.kind.is_call_signature())
            .and_then(|m| resolve_function(&m.ty, resolver)),
        TypeData::Object(object) => object
            .members
            .iter()
            .find(|m| m.kind.is_call_signature())
            .and_then(|m| resolve_function(&m.ty, resolver)),
        _ => None,
    }
}

fn function_returns_promise(function: &Function, resolver: &dyn TypeResolver) -> bool {
    function
        .return_type
        .as_type()
        .and_then(|return_ty| resolver.resolve_and_get(return_ty))
        .is_some_and(|resolved| resolved.is_promise_instance(resolver))
}

fn flattened_destructure(
    expr: &TypeofDestructureExpression,
    resolver: &mut dyn TypeResolver,
) -> Option<TypeData> {
    let resolved = resolver.resolve_and_get(&expr.ty)?;
    match (resolved.as_raw_data(), &expr.destructure_field) {
        (_subject, DestructureField::Index(index)) => resolved
            .find_element_type_at_index(resolver, *index)
            .map(|element_reference| element_reference.into_reference(resolver))
            .and_then(|reference| resolver.resolve_and_get(&reference))
            .map(ResolvedTypeData::to_data),
        (_subject, DestructureField::RestFrom(index)) => {
            resolved.find_type_of_elements_from_index(resolver, *index)
        }
        (TypeData::InstanceOf(subject_instance), DestructureField::Name(name)) => resolver
            .resolve_and_get(&resolved.apply_module_id_to_reference(&subject_instance.ty))
            .and_then(|subject| {
                subject
                    .find_member(resolver, |member| {
                        !member.is_static() && member.has_name(name.text())
                    })
                    .or_else(|| {
                        subject.find_index_signature_with_ty(resolver, |data| data.is_string())
                    })
            })
            .and_then(|member| type_data_from_destructured_member(member, resolver)),
        (TypeData::InstanceOf(subject_instance), DestructureField::RestExcept(names)) => resolver
            .resolve_and_get(&resolved.apply_module_id_to_reference(&subject_instance.ty))
            .map(|subject| flattened_rest_object(resolver, subject, names)),
        (subject @ TypeData::Class(_), DestructureField::Name(name)) => {
            let member = subject
                .own_members()
                .find(|own_member| {
                    own_member.is_static()
                        && !own_member.is_constructor()
                        && own_member.has_name(name.text())
                })
                .map(|member| ResolvedTypeMember::from((resolved.resolver_id(), member)))?;
            type_data_from_destructured_member(member, resolver)
        }
        (subject @ TypeData::Class(_), DestructureField::RestExcept(names)) => {
            let members = subject
                .own_members()
                .filter(|own_member| {
                    own_member.is_static() && !names.iter().any(|name| own_member.has_name(name))
                })
                .map(|member| {
                    ResolvedTypeMember::from((resolved.resolver_id(), member)).to_member()
                })
                .collect();
            Some(TypeData::object_with_members(members))
        }
        (_, DestructureField::Name(name)) => {
            let member = resolved
                .find_member(resolver, |member| member.has_name(name.text()))
                .or_else(|| {
                    resolved.find_index_signature_with_ty(resolver, |data| data.is_string())
                })?;
            type_data_from_destructured_member(member, resolver)
        }
        (_, DestructureField::RestExcept(excluded_names)) => {
            Some(flattened_rest_object(resolver, resolved, excluded_names))
        }
    }
}

fn flattened_function_call(
    expr: &TypeofCallExpression,
    function: &Function,
    resolver: &mut dyn TypeResolver,
) -> Option<TypeData> {
    let return_type_reference = function.return_type.as_type()?;
    let mut return_type = resolver.resolve_and_get(return_type_reference)?.to_data();

    let generic_reference = match &return_type {
        TypeData::InstanceOf(instance)
            if resolver
                .resolve_and_get(&instance.ty)
                .is_some_and(|resolved| resolved.is_generic()) =>
        {
            Some(instance.ty.clone())
        }
        _ => None,
    };
    if let Some(generic_reference) = generic_reference {
        infer_generic_arguments(
            resolver,
            &mut return_type,
            return_type_reference,
            &generic_reference,
            function,
            expr,
        );
    }

    let mut type_parameter_index = 0;
    while let TypeData::InstanceOf(instance) = &return_type {
        let generic_reference = {
            let Some(type_parameter) = instance.type_parameters.get(type_parameter_index) else {
                break;
            };
            type_parameter_index += 1;
            resolver
                .resolve_and_get(type_parameter)
                .is_some_and(|resolved| resolved.is_generic())
                .then(|| type_parameter.clone())
        };

        if let Some(generic_reference) = generic_reference {
            infer_generic_arguments(
                resolver,
                &mut return_type,
                return_type_reference,
                &generic_reference,
                function,
                expr,
            );
        }
    }

    Some(return_type)
}

fn infer_generic_arguments(
    resolver: &dyn TypeResolver,
    target: &mut TypeData,
    target_reference: &TypeReference,
    generic_reference: &TypeReference,
    function: &Function,
    expr: &TypeofCallExpression,
) {
    // The time complexity is not great on this, but fortunately most functions
    // have very few generics and not too many arguments either.
    for (index, parameter) in function.parameters.iter().enumerate() {
        if let Some(argument) = expr.arguments.get(index) {
            infer_generic_arg(
                resolver,
                target,
                target_reference,
                generic_reference,
                parameter,
                argument,
            );
        }
    }
}

fn infer_generic_arg(
    resolver: &dyn TypeResolver,
    target: &mut TypeData,
    target_reference: &TypeReference,
    generic_reference: &TypeReference,
    param: &FunctionParameter,
    arg: &CallArgumentType,
) -> Option<()> {
    let CallArgumentType::Argument(concrete_reference) = arg else {
        return None; // TODO: Handle spread arguments
    };

    // If the parameter's type directly references the target, we replace all
    // the target's references to the generic with references to the concrete
    // argument type.
    if param.ty() == target_reference {
        target.update_all_references(|reference| {
            if reference == generic_reference {
                *reference = concrete_reference.clone();
            }
        });
        return Some(());
    }

    // Otherwise, we proceed by looking into the parameter type itself...
    let resolved_param = resolver.resolve_and_get(param.ty())?;

    // If the parameter type is a function, ie. callback, we try to infer from
    // the callback's return type.
    let callback_return_ty = resolved_param
        .as_raw_data()
        .as_function()
        .and_then(|callback| callback.return_type.as_type())?;

    // If the callback's return type references the target, we replace all the
    // target's references to the generic with references to the concrete return
    // type.
    if callback_return_ty == generic_reference {
        let concrete_ty = resolver.resolve_and_get(concrete_reference)?.to_data();
        let concrete_callback = concrete_ty.as_function()?;
        let concrete_return_ty = concrete_callback.return_type.as_type()?;
        target.update_all_references(|reference| {
            if reference == generic_reference {
                *reference = concrete_return_ty.clone();
            }
        });
        return Some(());
    }

    None
}

/// Creates a new object with all the non-static properties of `object`, except
/// the given `excluded_names`.
fn flattened_rest_object(
    resolver: &dyn TypeResolver,
    object: ResolvedTypeData,
    excluded_names: &[Text],
) -> TypeData {
    // We need to look up the prototype chain, which may yield duplicate member
    // names. We deduplicate using a hash table so that we can maintain the
    // original order in a vector.
    let mut table: HashTable<usize> = HashTable::default();
    let mut members: Vec<TypeMember> = Vec::new();
    for member in object.all_members(resolver) {
        let Some(name) = &member.name() else {
            continue;
        };
        if member.is_static() || excluded_names.contains(name) {
            continue;
        }

        let entry = table.entry(
            hash_text(name),
            |i| members[*i].name().as_ref().is_some_and(|n| n == name),
            |i| {
                let name = members[*i].name();
                let name = name.as_ref().expect("only named members may be added");
                hash_text(name)
            },
        );
        if let Entry::Vacant(entry) = entry {
            let index = members.len();
            members.push(member.to_member());
            entry.insert(index);
        }
    }
    TypeData::object_with_members(members.into())
}

#[inline(always)]
fn hash_text(text: &Text) -> u64 {
    let mut hash = FxHasher::default();
    text.hash(&mut hash);
    hash.finish()
}

#[inline]
fn flattened_typeof_data(resolved: ResolvedTypeData) -> TypeData {
    match resolved.as_raw_data() {
        TypeData::BigInt => TypeData::reference(GLOBAL_BIGINT_STRING_LITERAL_ID),
        TypeData::Boolean => TypeData::reference(GLOBAL_BOOLEAN_STRING_LITERAL_ID),
        TypeData::Function(_) => TypeData::reference(GLOBAL_FUNCTION_STRING_LITERAL_ID),
        TypeData::Literal(literal) => match literal.as_ref() {
            Literal::BigInt(_) => TypeData::reference(GLOBAL_BIGINT_STRING_LITERAL_ID),
            Literal::Boolean(_) => TypeData::reference(GLOBAL_BOOLEAN_STRING_LITERAL_ID),
            Literal::Object(_) | Literal::RegExp(_) => {
                TypeData::reference(GLOBAL_OBJECT_STRING_LITERAL_ID)
            }
            Literal::Number(_) => TypeData::reference(GLOBAL_NUMBER_STRING_LITERAL_ID),
            Literal::String(_) | Literal::Template(_) => {
                TypeData::reference(GLOBAL_STRING_STRING_LITERAL_ID)
            }
        },
        TypeData::Null => TypeData::reference(GLOBAL_OBJECT_STRING_LITERAL_ID),
        TypeData::Number => TypeData::reference(GLOBAL_NUMBER_STRING_LITERAL_ID),
        TypeData::Object(_) | TypeData::Tuple(_) => {
            TypeData::reference(GLOBAL_OBJECT_STRING_LITERAL_ID)
        }
        TypeData::String => TypeData::reference(GLOBAL_STRING_STRING_LITERAL_ID),
        TypeData::Symbol => TypeData::reference(GLOBAL_SYMBOL_STRING_LITERAL_ID),
        TypeData::Undefined => TypeData::reference(GLOBAL_UNDEFINED_STRING_LITERAL_ID),
        _ => TypeData::reference(GLOBAL_TYPEOF_OPERATOR_RETURN_UNION_ID),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NamedFunctionParameter;

    fn constructor_parameter(is_optional: bool, is_rest: bool) -> ConstructorParameter {
        ConstructorParameter {
            parameter: FunctionParameter::Named(NamedFunctionParameter {
                name: Text::new_static("args"),
                ty: TypeReference::unknown(),
                is_optional,
                is_rest,
            }),
            accessibility: None,
        }
    }

    #[test]
    fn rest_constructor_accepts_zero_arguments() {
        let parameters = [constructor_parameter(false, true)];

        assert!(constructor_accepts_argument_count(&parameters, 0));
    }

    #[test]
    fn required_constructor_parameter_still_requires_an_argument() {
        let parameters = [
            constructor_parameter(false, false),
            constructor_parameter(false, true),
        ];

        assert!(!constructor_accepts_argument_count(&parameters, 0));
        assert!(constructor_accepts_argument_count(&parameters, 1));
    }
}

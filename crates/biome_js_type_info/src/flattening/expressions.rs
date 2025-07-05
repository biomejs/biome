use std::hash::{Hash, Hasher};

use biome_rowan::Text;
use hashbrown::{HashTable, hash_table::Entry};
use rustc_hash::FxHasher;

use crate::{
    CallArgumentType, DestructureField, Function, FunctionParameter, Literal, MAX_FLATTEN_DEPTH,
    Resolvable, ResolvedTypeData, ResolvedTypeMember, ResolverId, TypeData, TypeMember,
    TypeReference, TypeResolver, TypeofCallExpression, TypeofDestructureExpression,
    TypeofExpression, TypeofStaticMemberExpression,
    flattening::conditionals::{
        ConditionalType, reference_to_falsy_subset_of, reference_to_non_nullish_subset_of,
        reference_to_truthy_subset_of,
    },
    globals::{
        GLOBAL_ARRAY_ID, GLOBAL_BIGINT_STRING_LITERAL_ID, GLOBAL_BOOLEAN_STRING_LITERAL_ID,
        GLOBAL_FUNCTION_STRING_LITERAL_ID, GLOBAL_NUMBER_STRING_LITERAL_ID,
        GLOBAL_OBJECT_STRING_LITERAL_ID, GLOBAL_STRING_STRING_LITERAL_ID,
        GLOBAL_SYMBOL_STRING_LITERAL_ID, GLOBAL_TYPEOF_OPERATOR_RETURN_UNION_ID,
        GLOBAL_UNDEFINED_STRING_LITERAL_ID,
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
        TypeofExpression::Destructure(expr) => flattened_destructure(expr, resolver),
        TypeofExpression::New(expr) => {
            let resolved = resolver.resolve_and_get(&expr.callee)?;
            if let TypeData::Class(class) = resolved.as_raw_data() {
                let num_args = expr.arguments.len();
                let constructed_ty = class
                    .members
                    .iter()
                    .find(|member| member.kind.is_constructor())
                    .and_then(|member| {
                        let constructor = resolver
                            .resolve_and_get(&resolved.apply_module_id_to_reference(&member.ty))?;
                        match constructor.to_data() {
                            TypeData::Constructor(constructor) => {
                                // TODO: We might need to make an attempt to match
                                //       type signatures too.
                                (constructor.parameters.len() == num_args)
                                    .then_some(constructor.return_type)
                                    .flatten()
                            }
                            _ => None,
                        }
                    })
                    .unwrap_or_else(|| expr.callee.clone());
                Some(TypeData::instance_of(constructed_ty))
            } else {
                None
            }
        }
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
            match object.as_raw_data() {
                class @ TypeData::Class(_) => {
                    let member = class
                        .own_members()
                        .find(|member| member.has_name(&expr.member) && member.is_static())?;
                    let member = TypeMember {
                        kind: member.kind.clone(),
                        ty: object.apply_module_id_to_reference(&member.ty).into_owned(),
                    };
                    Some(TypeData::reference(member.deref_ty(resolver).into_owned()))
                }

                TypeData::ImportNamespace(module_id) => {
                    let resolved_id =
                        resolver.resolve_import_namespace_member(*module_id, &expr.member)?;
                    resolver
                        .get_by_resolved_id(resolved_id)
                        .map(ResolvedTypeData::to_data)
                }

                TypeData::Tuple(_tuple) => {
                    // Tuples are just fancy arrays, so make sure methods on
                    // them can be looked up as such:
                    let array = resolver
                        .get_by_resolved_id(GLOBAL_ARRAY_ID)
                        .expect("Array type must be registered");
                    let member = array.find_member(resolver, |member| {
                        member.has_name(&expr.member) && !member.is_static()
                    })?;
                    Some(TypeData::reference(member.ty().into_owned()))
                }

                TypeData::Union(union) => {
                    let types: Vec<_> = union
                        .types()
                        .iter()
                        .map(|reference| object.apply_module_id_to_reference(reference))
                        .map(|reference| reference.into_owned())
                        .collect();
                    let types = types
                        .into_iter()
                        .map(|variant| {
                            // Resolve and flatten the type member for each variant.
                            let variant = TypeData::TypeofExpression(Box::new(
                                TypeofExpression::StaticMember(TypeofStaticMemberExpression {
                                    object: variant,
                                    member: expr.member.clone(),
                                }),
                            ));

                            resolver.reference_to_owned_data(variant)
                        })
                        .collect();

                    Some(TypeData::union_of(resolver, types))
                }

                _ => {
                    let member = object
                        .find_member(resolver, |member| member.has_name(&expr.member))
                        .or_else(|| {
                            object.find_index_signature_with_ty(resolver, |ty| ty.is_string())
                        })?;
                    Some(TypeData::reference(member.deref_ty(resolver).into_owned()))
                }
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

fn flattened_call(
    expr: &TypeofCallExpression,
    callee: TypeData,
    resolver: &mut dyn TypeResolver,
) -> Option<TypeData> {
    let mut callee = callee;
    for _ in 0..MAX_FLATTEN_DEPTH {
        match callee {
            TypeData::Function(function) => {
                return flattened_function_call(expr, &function, resolver);
            }
            TypeData::InstanceOf(instance) => {
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
            TypeData::Interface(_) | TypeData::Object(_) => {
                callee =
                    ResolvedTypeData::from((ResolverId::from_level(resolver.level()), &callee))
                        .find_member(resolver, |member| member.kind().is_call_signature())
                        .map(ResolvedTypeMember::to_member)
                        .and_then(|member| resolver.resolve_and_get(&member.deref_ty(resolver)))?
                        .to_data();
            }
            _ => break,
        }
    }

    None
}

fn flattened_destructure(
    expr: &TypeofDestructureExpression,
    resolver: &mut dyn TypeResolver,
) -> Option<TypeData> {
    let resolved = resolver.resolve_and_get(&expr.ty)?;
    match (resolved.as_raw_data(), &expr.destructure_field) {
        (_subject, DestructureField::Index(index)) => resolved
            .find_element_type_at_index(resolver, *index)
            .and_then(|element_reference| {
                let reference = element_reference.into_reference(resolver);
                resolver
                    .resolve_and_get(&reference)
                    .map(ResolvedTypeData::to_data)
            }),
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
                    .or_else(|| subject.find_index_signature_with_ty(resolver, |ty| ty.is_string()))
            })
            .and_then(|member| resolver.resolve_and_get(&member.deref_ty(resolver)))
            .map(ResolvedTypeData::to_data),
        (TypeData::InstanceOf(subject_instance), DestructureField::RestExcept(names)) => resolver
            .resolve_and_get(&resolved.apply_module_id_to_reference(&subject_instance.ty))
            .map(|subject| flattened_rest_object(resolver, subject, names)),
        (subject @ TypeData::Class(_), DestructureField::Name(name)) => {
            let member_ty = subject
                .own_members()
                .find(|own_member| own_member.is_static() && own_member.has_name(name.text()))
                .map(|member| resolved.apply_module_id_to_reference(&member.ty))?;
            resolver
                .resolve_and_get(&member_ty)
                .map(ResolvedTypeData::to_data)
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
                .or_else(|| resolved.find_index_signature_with_ty(resolver, |ty| ty.is_string()))?;
            resolver
                .resolve_and_get(&member.deref_ty(resolver))
                .map(ResolvedTypeData::to_data)
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
    let return_ty_reference = function.return_type.as_type()?;
    let mut return_ty = resolver.resolve_and_get(return_ty_reference)?.to_data();

    let generic_references = match &return_ty {
        TypeData::InstanceOf(instance) => {
            let mut generic_references = match resolver.resolve_and_get(&instance.ty) {
                Some(resolved) if resolved.is_generic() => vec![instance.ty.clone()],
                _ => Vec::new(),
            };
            for param in &instance.type_parameters {
                match resolver.resolve_and_get(param) {
                    Some(resolved) if resolved.is_generic() => {
                        generic_references.push(param.clone())
                    }
                    _ => {}
                }
            }
            generic_references
        }
        _ => Vec::new(),
    };

    // The time complexity is not great on this, but fortunately most functions
    // have very few generics and not too many arguments either.
    for generic_reference in generic_references {
        for (index, param) in function.parameters.iter().enumerate() {
            if let Some(arg) = expr.arguments.get(index) {
                infer_generic_arg(
                    resolver,
                    &mut return_ty,
                    return_ty_reference,
                    &generic_reference,
                    param,
                    arg,
                );
            }
        }
    }

    Some(return_ty)
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
    if param.ty == *target_reference {
        target.update_all_references(|reference| {
            if reference == generic_reference {
                *reference = concrete_reference.clone();
            }
        });
        return Some(());
    }

    // Otherwise, we proceed by looking into the parameter type itself...
    let resolved_param = resolver.resolve_and_get(&param.ty)?;

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

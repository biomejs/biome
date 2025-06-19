use std::collections::{BTreeMap, btree_map::Entry};

use biome_rowan::Text;

use crate::{
    CallArgumentType, DestructureField, Literal, ResolvedTypeData, ResolvedTypeMember, ResolverId,
    TypeData, TypeReference, TypeResolver, TypeofCallExpression, TypeofExpression,
    TypeofStaticMemberExpression,
    globals::{
        GLOBAL_ARRAY_ID, GLOBAL_BIGINT_STRING_LITERAL_ID, GLOBAL_BOOLEAN_STRING_LITERAL_ID,
        GLOBAL_FUNCTION_STRING_LITERAL_ID, GLOBAL_NUMBER_STRING_LITERAL_ID,
        GLOBAL_OBJECT_STRING_LITERAL_ID, GLOBAL_STRING_STRING_LITERAL_ID,
        GLOBAL_SYMBOL_STRING_LITERAL_ID, GLOBAL_TYPEOF_OPERATOR_RETURN_UNION_ID,
        GLOBAL_UNDEFINED_STRING_LITERAL_ID,
    },
};

use super::flattened;

pub(super) fn flattened_expression(
    expr: &TypeofExpression,
    resolver: &mut dyn TypeResolver,
    depth: usize,
) -> Option<TypeData> {
    match expr {
        TypeofExpression::Addition(_expr) => {
            // TODO
            None
        }
        TypeofExpression::Await(expr) => {
            resolver
                .resolve_and_get(&expr.argument)
                .and_then(|resolved| {
                    let flattened = match resolved.as_raw_data() {
                        TypeData::BigInt => TypeData::BigInt,
                        TypeData::Boolean => TypeData::Boolean,
                        TypeData::Class(class) => {
                            resolved.apply_module_id_to_data(TypeData::Class(class.clone()))
                        }
                        TypeData::Function(function) => {
                            resolved.apply_module_id_to_data(TypeData::Function(function.clone()))
                        }
                        TypeData::Literal(literal) => TypeData::Literal(literal.clone()),
                        TypeData::Null => TypeData::Null,
                        TypeData::Number => TypeData::Number,
                        TypeData::Object(object) => {
                            resolved.apply_module_id_to_data(TypeData::Object(object.clone()))
                        }
                        TypeData::String => TypeData::String,
                        _ => resolved.find_promise_type(resolver)?.to_data(),
                    };
                    Some(flattened)
                })
        }
        TypeofExpression::BitwiseNot(expr) => {
            resolver
                .resolve_and_get(&expr.argument)
                .map(|resolved| match resolved.as_raw_data() {
                    TypeData::BigInt => TypeData::BigInt,
                    _ => TypeData::Number,
                })
        }
        TypeofExpression::Call(expr) => match resolver.resolve_and_get(&expr.callee) {
            Some(callee) => {
                let callee = flattened(callee.to_data(), resolver, depth);
                flattened_function_call(expr, callee, resolver, depth).map(
                    |(is_instance, mut ty)| {
                        if is_instance {
                            ty = ty.into_instance(resolver);
                        }

                        flattened(ty, resolver, depth)
                    },
                )
            }
            None => None,
        },
        TypeofExpression::Destructure(expr) => {
            match resolver.resolve_and_get(&expr.ty) {
                Some(resolved) => match (resolved.as_raw_data(), &expr.destructure_field) {
                    (subject, DestructureField::Index(index)) => Some(
                        subject
                            .clone()
                            .find_element_type_at_index(resolved.resolver_id(), resolver, *index)
                            .map_or_else(TypeData::unknown, ResolvedTypeData::to_data),
                    ),
                    (subject, DestructureField::RestFrom(index)) => Some(
                        subject
                            .clone()
                            .find_type_of_elements_from_index(
                                resolved.resolver_id(),
                                resolver,
                                *index,
                            )
                            .map_or_else(TypeData::unknown, ResolvedTypeData::to_data),
                    ),
                    (TypeData::InstanceOf(subject_instance), DestructureField::Name(name)) => {
                        resolver
                            .resolve_and_get(&subject_instance.ty)
                            .map(ResolvedTypeData::to_data)
                            .map(|type_data| flattened(type_data, resolver, depth))
                            .and_then(|subject| {
                                let member = ResolvedTypeData::from((
                                    ResolverId::from_level(resolver.level()),
                                    &subject,
                                ))
                                .all_members(resolver)
                                .find(|member| {
                                    !member.is_static() && member.has_name(name.text())
                                })?;
                                Some(
                                    resolver
                                        .resolve_and_get(&member.ty())
                                        .map_or_else(TypeData::unknown, ResolvedTypeData::to_data),
                                )
                            })
                    }
                    (
                        TypeData::InstanceOf(subject_instance),
                        DestructureField::RestExcept(names),
                    ) => {
                        resolver
                            .resolve_and_get(&subject_instance.ty)
                            .map(|subject| {
                                // We need to look up the prototype chain, which may
                                // yield duplicate member names. We deduplicate
                                // using a map before constructing a new object.
                                let members: BTreeMap<Text, ResolvedTypeMember> = subject
                                    .all_members(resolver)
                                    .filter(|member| {
                                        !member.is_static()
                                            && !names.iter().any(|name| member.has_name(name))
                                    })
                                    .fold(BTreeMap::new(), |mut map, member| {
                                        if let Some(name) = member.name() {
                                            if let Entry::Vacant(entry) = map.entry(name) {
                                                entry.insert(member);
                                            }
                                        }
                                        map
                                    });
                                TypeData::object_with_members(
                                    members
                                        .into_values()
                                        .map(ResolvedTypeMember::to_member)
                                        .collect(),
                                )
                            })
                    }
                    (TypeData::Object(_), DestructureField::Name(name)) => {
                        let member = resolved
                            .all_members(resolver)
                            .find(|member| !member.is_static() && member.has_name(name.text()))?;
                        Some(
                            resolver
                                .resolve_and_get(&member.ty())
                                .map_or_else(TypeData::unknown, ResolvedTypeData::to_data),
                        )
                    }
                    (TypeData::Object(_), DestructureField::RestExcept(names)) => {
                        // We need to look up the prototype chain, which may
                        // yield duplicate member names. We deduplicate
                        // using a map before constructing a new object.
                        let members: BTreeMap<Text, ResolvedTypeMember> = resolved
                            .all_members(resolver)
                            .filter(|member| {
                                !member.is_static()
                                    && !names.iter().any(|name| member.has_name(name))
                            })
                            .fold(BTreeMap::new(), |mut map, member| {
                                if let Some(name) = member.name() {
                                    if let Entry::Vacant(entry) = map.entry(name) {
                                        entry.insert(member);
                                    }
                                }
                                map
                            });
                        Some(TypeData::object_with_members(
                            members
                                .into_values()
                                .map(ResolvedTypeMember::to_member)
                                .collect(),
                        ))
                    }
                    (subject, DestructureField::Name(name)) => Some({
                        let member_ty = subject
                            .own_members()
                            .find(|own_member| {
                                own_member.is_static() && own_member.has_name(name.text())
                            })
                            .map(|member| resolved.apply_module_id_to_reference(&member.ty))?;
                        flattened(
                            resolver
                                .resolve_and_get(&member_ty)
                                .map_or_else(TypeData::unknown, ResolvedTypeData::to_data),
                            resolver,
                            depth,
                        )
                    }),
                    (subject, DestructureField::RestExcept(names)) => Some({
                        let members = subject
                            .own_members()
                            .filter(|own_member| {
                                own_member.is_static()
                                    && !names.iter().any(|name| own_member.has_name(name))
                            })
                            .map(|member| {
                                ResolvedTypeMember::from((resolved.resolver_id(), member))
                                    .to_member()
                            })
                            .collect();
                        TypeData::object_with_members(members)
                    }),
                },
                None => None,
            }
        }
        TypeofExpression::New(expr) => {
            match resolver
                .resolve_and_get(&expr.callee)
                .map(ResolvedTypeData::to_data)
                .map(|type_data| flattened(type_data, resolver, depth))
            {
                Some(TypeData::Class(class)) => Some({
                    let num_args = expr.arguments.len();
                    let constructed_ty = class
                        .members
                        .iter()
                        .find(|member| member.kind.is_constructor())
                        .and_then(|member| {
                            let constructor = resolver.resolve_and_get(&member.ty)?;
                            match constructor.as_raw_data() {
                                TypeData::Constructor(constructor) => {
                                    // TODO: We might need to make an attempt to match
                                    //       type signatures too.
                                    (constructor.parameters.len() == num_args)
                                        .then(|| constructor.return_type.clone())
                                        .flatten()
                                }
                                _ => None,
                            }
                        })
                        .unwrap_or_else(|| expr.callee.clone());
                    TypeData::instance_of(constructed_ty)
                }),
                // TODO: Handle objects with call signatures.
                _ => None,
            }
        }
        TypeofExpression::StaticMember(expr) => {
            if let Some(object) = resolver.resolve_and_get(&expr.object) {
                match object.as_raw_data() {
                    TypeData::InstanceOf(instance) => {
                        let instance_ty = object.apply_module_id_to_reference(&instance.ty);
                        let resolved_instance_ty = resolver.resolve_and_get(&instance_ty);
                        if resolved_instance_ty.is_some_and(|resolved| {
                            matches!(
                                resolved.as_raw_data(),
                                TypeData::Intersection(_) | TypeData::Union(_)
                            )
                        }) {
                            return Some(TypeData::TypeofExpression(Box::new(
                                TypeofExpression::StaticMember(TypeofStaticMemberExpression {
                                    object: instance_ty.into_owned(),
                                    member: expr.member.clone(),
                                }),
                            )));
                        } else if !resolved_instance_ty.is_some_and(ResolvedTypeData::has_members) {
                            return None;
                        }
                    }

                    TypeData::ImportNamespace(module_id) => {
                        let resolved_id =
                            resolver.resolve_import_namespace_member(*module_id, &expr.member)?;
                        return resolver
                            .get_by_resolved_id(resolved_id)
                            .map(ResolvedTypeData::to_data);
                    }

                    TypeData::Tuple(_tuple) => {
                        // Tuples are just fancy arrays, so make sure methods on
                        // them can be looked up as such:
                        let array = resolver
                            .get_by_resolved_id(GLOBAL_ARRAY_ID)
                            .expect("Array type must be registered");
                        let member = array
                            .all_members(resolver)
                            .find(|member| member.has_name(&expr.member) && !member.is_static());
                        return member.map(|member| TypeData::reference(member.ty().into_owned()));
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
                                let variant = flattened(
                                    TypeData::TypeofExpression(Box::new(
                                        TypeofExpression::StaticMember(
                                            TypeofStaticMemberExpression {
                                                object: variant,
                                                member: expr.member.clone(),
                                            },
                                        ),
                                    )),
                                    resolver,
                                    depth,
                                );

                                resolver.reference_to_owned_data(variant)
                            })
                            .collect();

                        return Some(TypeData::union_of(types));
                    }

                    _ => {}
                }

                let is_class = object.is_class();
                let member = object.all_members(resolver).find(|member| {
                    member.has_name(&expr.member)
                        && if is_class {
                            member.is_static()
                        } else {
                            !member.is_static()
                        }
                });
                member.map(|member| TypeData::reference(member.ty().into_owned()))
            } else {
                None
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

fn flattened_function_call(
    expr: &TypeofCallExpression,
    callee: TypeData,
    resolver: &mut dyn TypeResolver,
    depth: usize,
) -> Option<(bool, TypeData)> {
    match callee {
        TypeData::Function(function) => function.return_type.as_type().and_then(|return_ty| {
            let resolved_return_ty = resolver.resolve_and_get(return_ty)?;

            let (is_generic_instance, mut resolved_return_ty) = match resolved_return_ty
                .as_raw_data()
            {
                TypeData::InstanceOf(instance) if instance.type_parameters.is_empty() => resolver
                    .resolve_and_get(&resolved_return_ty.apply_module_id_to_reference(&instance.ty))
                    .filter(|resolved| resolved.is_generic())
                    .map_or((false, resolved_return_ty), |resolved| (true, resolved)),
                _ => (false, resolved_return_ty),
            };

            if is_generic_instance {
                // See if we can infer the return type by looking for the
                // generic in the input arguments.
                let arg_index = function
                    .parameters
                    .iter()
                    .position(|param| (param.ty == *return_ty))?;
                let arg = expr.arguments.get(arg_index)?;
                let reference = match arg {
                    CallArgumentType::Argument(reference) => reference,
                    CallArgumentType::Spread(_) => {
                        return None; // TODO: Handle spread arguments
                    }
                };
                resolved_return_ty = resolver.resolve_and_get(reference)?;
            }

            Some((is_generic_instance, resolved_return_ty.to_data()))
        }),
        TypeData::InstanceOf(instance) => resolver
            .resolve_and_get(&instance.ty)
            .map(ResolvedTypeData::to_data)
            .map(|type_data| flattened(type_data, resolver, depth))
            .and_then(|callee| {
                ResolvedTypeData::from((ResolverId::from_level(resolver.level()), &callee))
                    .all_members(resolver)
                    .find(|member| member.kind().is_call_signature())
                    .map(ResolvedTypeMember::to_member)
                    .and_then(|member| resolver.resolve_and_get(&member.ty))
                    .map(ResolvedTypeData::to_data)
                    .and_then(|callee| flattened_function_call(expr, callee, resolver, depth))
            }),
        TypeData::Object(_) => {
            ResolvedTypeData::from((ResolverId::from_level(resolver.level()), &callee))
                .all_members(resolver)
                .find(|member| member.kind().is_call_signature())
                .map(ResolvedTypeMember::to_member)
                .and_then(|member| resolver.resolve_and_get(&member.ty))
                .map(ResolvedTypeData::to_data)
                .and_then(|callee| flattened_function_call(expr, callee, resolver, depth))
        }
        _ => None,
    }
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
            Literal::Null | Literal::Object(_) | Literal::RegExp(_) => {
                TypeData::reference(GLOBAL_OBJECT_STRING_LITERAL_ID)
            }
            Literal::Number(_) => TypeData::reference(GLOBAL_NUMBER_STRING_LITERAL_ID),
            Literal::String(_) | Literal::Template(_) => {
                TypeData::reference(GLOBAL_STRING_STRING_LITERAL_ID)
            }
        },
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

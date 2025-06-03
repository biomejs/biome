use std::collections::{BTreeMap, btree_map::Entry};

use biome_rowan::Text;

use crate::{
    CallArgumentType, DestructureField, GLOBAL_UNKNOWN_ID, Literal, ResolvedTypeData,
    ResolvedTypeMember, TypeData, TypeInstance, TypeMemberKind, TypeReference, TypeResolver,
    TypeofCallExpression, TypeofExpression, TypeofStaticMemberExpression,
    globals::{
        GLOBAL_BIGINT_STRING_LITERAL_ID, GLOBAL_BOOLEAN_STRING_LITERAL_ID,
        GLOBAL_FUNCTION_STRING_LITERAL_ID, GLOBAL_NUMBER_STRING_LITERAL_ID,
        GLOBAL_OBJECT_STRING_LITERAL_ID, GLOBAL_STRING_STRING_LITERAL_ID,
        GLOBAL_SYMBOL_STRING_LITERAL_ID, GLOBAL_TYPEOF_OPERATOR_RETURN_UNION_ID,
        GLOBAL_UNDEFINED_STRING_LITERAL_ID,
    },
};

impl TypeData {
    /// Flattens the given type.
    ///
    /// Flattening is both an optimisation as well as an aid to make our
    /// reasoning about types easier. It removes unnecessary indirections from
    /// our type structures, and should be performed every time after we perform
    /// type resolution.
    ///
    /// ## Example
    ///
    /// Consider the following example:
    ///
    /// ```ts
    /// const c = 1;
    ///
    /// type A = typeof c;
    /// ```
    ///
    /// After local inference, the inferred type of `A` is:
    ///
    /// ```no_test
    /// TypeData::TypeofValue {
    ///     identifier: "c",
    ///     ty: TypeReference::Unknown
    /// })
    /// ```
    ///
    /// Once we've performed thin type resolution, this becomes:
    ///
    /// ```no_test
    /// TypeData::TypeofValue {
    ///     identifier: "c",
    ///     ty: TypeReference::Module(<type ID of literal>)
    /// })
    /// ```
    ///
    /// With flattening, we can reduce this to:
    ///
    /// ```no_test
    /// TypeData::Literal(Literal::Number(1)))
    /// ```
    pub fn flattened(self, resolver: &mut dyn TypeResolver) -> Self {
        flattened(self, resolver, 0)
    }
}

fn flattened(mut ty: TypeData, resolver: &mut dyn TypeResolver, depth: usize) -> TypeData {
    const MAX_FLATTEN_DEPTH: usize = 10; // Arbitrary depth, may require tweaking.

    for depth in depth + 1..=MAX_FLATTEN_DEPTH {
        match &ty {
            TypeData::MergedReference(merged) => {
                match (&merged.ty, &merged.value_ty, &merged.namespace_ty) {
                    (Some(ty1), Some(ty2), Some(ty3)) if ty1 == ty2 && ty1 == ty3 => {
                        ty = TypeData::Reference(ty1.clone());
                    }
                    (Some(ty1), Some(ty2), None)
                    | (Some(ty1), None, Some(ty2))
                    | (None, Some(ty1), Some(ty2))
                        if ty1 == ty2 =>
                    {
                        ty = TypeData::Reference(ty1.clone());
                    }
                    _ => return ty,
                }
            }
            TypeData::InstanceOf(instance_of) => match resolver.resolve_and_get(&instance_of.ty) {
                Some(resolved) => match resolved.as_raw_data() {
                    TypeData::InstanceOf(resolved_instance) => {
                        return resolved.apply_module_id_to_data(TypeData::instance_of(
                            TypeInstance {
                                ty: resolved_instance.ty.clone(),
                                type_parameters: TypeReference::merge_parameters(
                                    &resolved_instance.type_parameters,
                                    &instance_of.type_parameters,
                                ),
                            },
                        ));
                    }
                    TypeData::Global
                    | TypeData::Function(_)
                    | TypeData::Literal(_)
                    | TypeData::Object(_) => ty = resolved.to_data(),
                    _ => return ty,
                },
                None => return ty,
            },
            TypeData::Reference(reference) => match resolver.resolve_and_get(reference) {
                Some(reference) => ty = reference.to_data(),
                None => return ty,
            },
            TypeData::TypeofExpression(expr) => match expr.as_ref() {
                TypeofExpression::Addition(_expr) => {
                    // TODO
                    return ty;
                }
                TypeofExpression::Await(expr) => match resolver.resolve_and_get(&expr.argument) {
                    Some(resolved) => {
                        return match resolved.as_raw_data() {
                            TypeData::BigInt => TypeData::BigInt,
                            TypeData::Boolean => TypeData::Boolean,
                            TypeData::Class(class) => {
                                resolved.apply_module_id_to_data(TypeData::Class(class.clone()))
                            }
                            TypeData::Function(function) => resolved
                                .apply_module_id_to_data(TypeData::Function(function.clone())),
                            TypeData::Literal(literal) => TypeData::Literal(literal.clone()),
                            TypeData::Null => TypeData::Null,
                            TypeData::Number => TypeData::Number,
                            TypeData::Object(object) => {
                                resolved.apply_module_id_to_data(TypeData::Object(object.clone()))
                            }
                            TypeData::String => TypeData::String,
                            _ => match resolved.find_promise_type(resolver) {
                                Some(promised_ty) => {
                                    ty = promised_ty.to_data();
                                    continue;
                                }
                                None => ty,
                            },
                        };
                    }
                    None => return ty,
                },
                TypeofExpression::BitwiseNot(expr) => {
                    return match resolver.resolve_and_get(&expr.argument) {
                        Some(resolved) => match resolved.as_raw_data() {
                            TypeData::BigInt => TypeData::BigInt,
                            _ => TypeData::Number,
                        },
                        None => ty,
                    };
                }
                TypeofExpression::Call(expr) => match resolver.resolve_and_get(&expr.callee) {
                    Some(callee) => {
                        return flattened_function_call(expr, callee, resolver)
                            .map(|(is_instance, mut ty)| {
                                if is_instance {
                                    ty = ty.into_instance(resolver);
                                }

                                flattened(ty, resolver, depth)
                            })
                            .unwrap_or(ty);
                    }
                    None => return ty,
                },
                TypeofExpression::Destructure(expr) => {
                    match resolver.resolve_and_get(&expr.ty) {
                        Some(resolved) => match (resolved.as_raw_data(), &expr.destructure_field) {
                            (TypeData::Class(class), DestructureField::Name(name)) => {
                                match class.members.iter().find(|own_member| {
                                    own_member.is_static() && own_member.has_name(name.text())
                                }) {
                                    Some(member) => {
                                        ty = flattened(
                                            resolver
                                                .resolve_and_get(
                                                    &resolved
                                                        .apply_module_id_to_reference(&member.ty),
                                                )
                                                .map(ResolvedTypeData::to_data)
                                                .unwrap_or_default(),
                                            resolver,
                                            depth,
                                        );
                                    }
                                    None => return TypeData::reference(GLOBAL_UNKNOWN_ID),
                                }
                            }
                            (TypeData::Class(class), DestructureField::RestExcept(names)) => {
                                return TypeData::object_with_members(
                                    class
                                        .members
                                        .iter()
                                        .filter(|own_member| {
                                            own_member.is_static()
                                                && !names
                                                    .iter()
                                                    .any(|name| own_member.has_name(name))
                                        })
                                        .map(|member| {
                                            ResolvedTypeMember::from((
                                                resolved.resolver_id(),
                                                member,
                                            ))
                                            .to_member()
                                        })
                                        .collect(),
                                );
                            }
                            (TypeData::Interface(interface), DestructureField::Name(name)) => {
                                match interface.members.iter().find(|own_member| {
                                    own_member.is_static() && own_member.has_name(name.text())
                                }) {
                                    Some(member) => {
                                        ty = flattened(
                                            resolver
                                                .resolve_and_get(
                                                    &resolved
                                                        .apply_module_id_to_reference(&member.ty),
                                                )
                                                .map(ResolvedTypeData::to_data)
                                                .unwrap_or_default(),
                                            resolver,
                                            depth,
                                        );
                                    }
                                    None => return TypeData::reference(GLOBAL_UNKNOWN_ID),
                                }
                            }
                            (
                                TypeData::Interface(interface),
                                DestructureField::RestExcept(names),
                            ) => {
                                return TypeData::object_with_members(
                                    interface
                                        .members
                                        .iter()
                                        .filter(|own_member| {
                                            own_member.is_static()
                                                && !names
                                                    .iter()
                                                    .any(|name| own_member.has_name(name))
                                        })
                                        .map(|member| {
                                            ResolvedTypeMember::from((
                                                resolved.resolver_id(),
                                                member,
                                            ))
                                            .to_member()
                                        })
                                        .collect(),
                                );
                            }
                            (subject, DestructureField::Index(index)) => {
                                return subject
                                    .clone()
                                    .find_element_type_at_index(
                                        resolved.resolver_id(),
                                        resolver,
                                        *index,
                                    )
                                    .map(ResolvedTypeData::to_data)
                                    .unwrap_or_default();
                            }
                            (subject, DestructureField::RestFrom(index)) => {
                                return subject
                                    .clone()
                                    .find_type_of_elements_from_index(
                                        resolved.resolver_id(),
                                        resolver,
                                        *index,
                                    )
                                    .map(ResolvedTypeData::to_data)
                                    .unwrap_or_default();
                            }
                            (_, DestructureField::Name(name)) => {
                                let member = resolved.all_members(resolver).find(|member| {
                                    !member.is_static() && member.has_name(name.text())
                                });
                                return match member {
                                    Some(member) => resolver
                                        .resolve_and_get(&member.ty())
                                        .map(ResolvedTypeData::to_data)
                                        .unwrap_or_default(),
                                    None => TypeData::unknown(),
                                };
                            }
                            (_, DestructureField::RestExcept(names)) => {
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
                                return TypeData::object_with_members(
                                    members
                                        .into_values()
                                        .map(ResolvedTypeMember::to_member)
                                        .collect(),
                                );
                            }
                        },
                        None => return ty,
                    }
                }
                TypeofExpression::New(expr) => {
                    match resolver
                        .resolve_and_get(&expr.callee)
                        .map(ResolvedTypeData::to_data)
                        .map(|type_data| flattened(type_data, resolver, depth))
                    {
                        Some(TypeData::Class(class)) => {
                            let num_args = expr.arguments.len();
                            let constructed_ty = class
                                .members
                                .iter()
                                .find_map(|member| match member.kind {
                                    TypeMemberKind::Constructor => {
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
                                    }
                                    _ => None,
                                })
                                .unwrap_or_else(|| expr.callee.clone());
                            ty = TypeData::instance_of(constructed_ty);
                        }
                        // TODO: Handle objects with call signatures.
                        _ => return ty,
                    }
                }
                TypeofExpression::StaticMember(expr) => {
                    if let Some(object) = resolver.resolve_and_get(&expr.object) {
                        // FIXME: Flattening intersections and unions for members should be done in
                        //        `TypeMemberIterator`.

                        if let TypeData::InstanceOf(instance) = object.as_raw_data() {
                            let instance_ty = object.apply_module_id_to_reference(&instance.ty);
                            if resolver
                                .resolve_and_get(&instance_ty)
                                .is_some_and(|object| {
                                    matches!(
                                        object.as_raw_data(),
                                        TypeData::Intersection(_) | TypeData::Union(_)
                                    )
                                })
                            {
                                ty = TypeData::TypeofExpression(Box::new(
                                    TypeofExpression::StaticMember(TypeofStaticMemberExpression {
                                        object: instance_ty.into_owned(),
                                        member: expr.member.clone(),
                                    }),
                                ));
                                continue;
                            }
                        };

                        if let TypeData::Intersection(intersection) = object.as_raw_data() {
                            let types: Vec<_> = intersection
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

                                    resolver.reference_to_registered_data(variant)
                                })
                                .collect();

                            return TypeData::intersection_of(types);
                        }

                        if let TypeData::Union(union) = object.as_raw_data() {
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

                                    resolver.reference_to_registered_data(variant)
                                })
                                .collect();

                            return TypeData::union_of(types);
                        }

                        let is_class = matches!(object.as_raw_data(), TypeData::Class(_));
                        let member = object.all_members(resolver).find(|member| {
                            member.has_name(&expr.member)
                                && if is_class {
                                    member.is_static()
                                } else {
                                    !member.is_static()
                                }
                        });
                        match member {
                            Some(member) => {
                                ty = TypeData::reference(member.ty().into_owned());
                            }
                            None => return TypeData::unknown(),
                        }
                    } else {
                        return ty;
                    }
                }
                TypeofExpression::Super(expr) => match resolver.resolve_and_get(&expr.parent) {
                    Some(resolved) => match resolved.as_raw_data() {
                        TypeData::Class(class) => match class.extends.as_ref() {
                            Some(super_class) => {
                                ty = TypeData::instance_of(
                                    resolved
                                        .apply_module_id_to_reference(super_class)
                                        .into_owned(),
                                );
                            }
                            None => return TypeData::unknown(),
                        },
                        _ => return TypeData::unknown(),
                    },
                    None => return ty,
                },
                TypeofExpression::This(expr) => match resolver.resolve_reference(&expr.parent) {
                    Some(class_id) => {
                        ty = TypeData::instance_of(TypeReference::from(class_id));
                    }
                    None => return ty,
                },
                TypeofExpression::Typeof(expr) => {
                    return match resolver.resolve_and_get(&expr.argument) {
                        Some(resolved) => flattened_typeof_data(resolved),
                        None => ty,
                    };
                }
                TypeofExpression::UnaryMinus(expr) => {
                    return match resolver.resolve_and_get(&expr.argument) {
                        Some(resolved) => match resolved.as_raw_data() {
                            TypeData::BigInt => TypeData::BigInt,
                            _ => TypeData::Number,
                        },
                        None => ty,
                    };
                }
            },
            TypeData::TypeofType(reference) => {
                match resolver.resolve_reference(reference.as_ref()) {
                    Some(resolved) => ty = TypeData::reference(resolved),
                    None => return ty,
                }
            }
            TypeData::TypeofValue(value) if value.ty.is_known() => {
                match resolver.resolve_reference(&value.ty) {
                    Some(resolved) => ty = TypeData::reference(resolved),
                    None => return ty,
                }
            }
            _ => return ty,
        }
    }

    debug_assert!(false, "max flattening depth reached");
    TypeData::Unknown
}

fn flattened_function_call(
    expr: &TypeofCallExpression,
    callee: ResolvedTypeData,
    resolver: &dyn TypeResolver,
) -> Option<(bool, TypeData)> {
    match callee.as_raw_data() {
        TypeData::Function(function) => function.return_type.as_type().and_then(|return_ty| {
            let resolved_return_ty =
                resolver.resolve_and_get(&callee.apply_module_id_to_reference(return_ty))?;

            let (is_generic_instance, mut resolved_return_ty) = match resolved_return_ty
                .as_raw_data()
            {
                TypeData::InstanceOf(instance) if instance.type_parameters.is_empty() => resolver
                    .resolve_and_get(&callee.apply_module_id_to_reference(&instance.ty))
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
        TypeData::Object(_) => callee
            .all_members(resolver)
            .find(|member| member.has_name("constructor"))
            .map(ResolvedTypeMember::to_member)
            .and_then(|member| resolver.resolve_and_get(&member.ty))
            .map(ResolvedTypeData::to_data)
            .map(|data| (false, data)),
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
        TypeData::Object(_) => TypeData::reference(GLOBAL_OBJECT_STRING_LITERAL_ID),
        TypeData::String => TypeData::reference(GLOBAL_STRING_STRING_LITERAL_ID),
        TypeData::Symbol => TypeData::reference(GLOBAL_SYMBOL_STRING_LITERAL_ID),
        TypeData::Undefined => TypeData::reference(GLOBAL_UNDEFINED_STRING_LITERAL_ID),
        _ => TypeData::reference(GLOBAL_TYPEOF_OPERATOR_RETURN_UNION_ID),
    }
}

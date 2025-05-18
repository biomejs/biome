use std::collections::{BTreeMap, btree_map::Entry};

use biome_rowan::Text;

use crate::{
    DestructureField, GenericTypeParameter, ResolvedTypeData, ResolvedTypeMember, TypeData,
    TypeInstance, TypeMember, TypeReference, TypeResolver, TypeofExpression,
    TypeofStaticMemberExpression,
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
            TypeData::InstanceOf(instance_of) => match resolver.resolve_and_get(&instance_of.ty) {
                Some(resolved) => match resolved.as_raw_data() {
                    TypeData::InstanceOf(resolved_instance) => {
                        return resolved.apply_module_id_to_data(TypeData::instance_of(
                            TypeInstance {
                                ty: resolved_instance.ty.clone(),
                                type_parameters: GenericTypeParameter::merge_parameters(
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
                TypeofExpression::Call(expr) => match resolver.resolve_and_get(&expr.callee) {
                    Some(resolved) => {
                        return match resolved.as_raw_data() {
                            TypeData::Function(function) => match function.return_type.as_type() {
                                Some(return_ty) => resolver
                                    .resolve_and_get(
                                        &resolved.apply_module_id_to_reference(return_ty),
                                    )
                                    .map(ResolvedTypeData::to_data)
                                    .map(|data| flattened(data, resolver, depth))
                                    .unwrap_or_default(),
                                None => ty,
                            },
                            TypeData::Object(_) => {
                                let member = resolved
                                    .all_members(resolver)
                                    .find(|member| member.has_name("constructor"))
                                    .map(ResolvedTypeMember::to_member);
                                match member {
                                    Some(member) => {
                                        ty = resolver
                                            .type_from_member(resolved.to_data(), member)
                                            .to_data();
                                        continue;
                                    }
                                    None => TypeData::unknown(),
                                }
                            }
                            _ => ty,
                        };
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
                                                .type_from_member(
                                                    resolved.to_data(),
                                                    ResolvedTypeMember::from((
                                                        resolved.resolver_id(),
                                                        member,
                                                    ))
                                                    .to_member(),
                                                )
                                                .to_data(),
                                            resolver,
                                            depth,
                                        );
                                    }
                                    None => return TypeData::Unknown,
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
                                        .type_from_member(resolved.to_data(), member.to_member())
                                        .to_data(),
                                    None => TypeData::Unknown,
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
                                .find_map(|member| match member {
                                    TypeMember::Constructor(constructor) => {
                                        // TODO: We might need to make an attempt to match
                                        //       type signatures too.
                                        (constructor.parameters.len() == num_args)
                                            .then(|| constructor.return_type.clone())
                                            .flatten()
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
                                let member = member.to_member();
                                ty = TypeData::reference(
                                    resolver.register_type_from_member(object.to_data(), member),
                                );
                            }
                            None => return TypeData::Unknown,
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
                            None => return TypeData::Unknown,
                        },
                        _ => return TypeData::Unknown,
                    },
                    None => return ty,
                },
                TypeofExpression::This(expr) => match resolver.resolve_reference(&expr.parent) {
                    Some(class_id) => {
                        ty = TypeData::instance_of(TypeReference::from(class_id));
                    }
                    None => return ty,
                },
            },
            TypeData::TypeofValue(value) => match resolver.resolve_reference(&value.ty) {
                Some(type_id) => ty = TypeData::reference(type_id),
                None => return ty,
            },
            _ => return ty,
        }
    }

    debug_assert!(false, "max flattening depth reached");
    TypeData::Unknown
}

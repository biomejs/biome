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
        match &self {
            Self::InstanceOf(instance_of) => match resolver.resolve_and_get(&instance_of.ty) {
                Some(resolved) => match resolved.as_raw_data() {
                    Self::InstanceOf(resolved_instance) => {
                        resolved.apply_module_id_to_data(Self::instance_of(TypeInstance {
                            ty: resolved_instance.ty.clone(),
                            type_parameters: GenericTypeParameter::merge_parameters(
                                &resolved_instance.type_parameters,
                                &instance_of.type_parameters,
                            ),
                        }))
                    }
                    Self::Global | Self::Function(_) | Self::Literal(_) | Self::Object(_) => {
                        resolved.to_data().flattened(resolver)
                    }
                    _ => self,
                },
                None => self,
            },
            Self::Reference(reference) => match resolver.resolve_and_get(reference) {
                Some(ty) => ty.to_data().flattened(resolver),
                None => self,
            },
            Self::TypeofExpression(expr) => match expr.as_ref() {
                TypeofExpression::Addition(_expr) => {
                    // TODO
                    self
                }
                TypeofExpression::Await(expr) => match resolver.resolve_and_get(&expr.argument) {
                    Some(resolved) => match resolved.as_raw_data() {
                        Self::BigInt => Self::BigInt,
                        Self::Boolean => Self::Boolean,
                        Self::Class(class) => {
                            resolved.apply_module_id_to_data(Self::Class(class.clone()))
                        }
                        Self::Function(function) => {
                            resolved.apply_module_id_to_data(Self::Function(function.clone()))
                        }
                        Self::Literal(literal) => Self::Literal(literal.clone()),
                        Self::Null => Self::Null,
                        Self::Number => Self::Number,
                        Self::Object(object) => {
                            resolved.apply_module_id_to_data(Self::Object(object.clone()))
                        }
                        Self::String => Self::String,
                        _ => match resolved.find_promise_type(resolver) {
                            Some(ty) => ty.to_data().flattened(resolver),
                            None => self,
                        },
                    },
                    None => self,
                },
                TypeofExpression::Call(expr) => match resolver.resolve_and_get(&expr.callee) {
                    Some(resolved) => match resolved.as_raw_data() {
                        Self::Function(function) => match function.return_type.as_type() {
                            Some(ty) => resolver
                                .resolve_and_get(&resolved.apply_module_id_to_reference(ty))
                                .map(ResolvedTypeData::to_data)
                                .map(|data| data.flattened(resolver))
                                .unwrap_or_default(),
                            None => self,
                        },
                        Self::Object(_) => {
                            let member = resolved
                                .all_members(resolver)
                                .find(|member| member.has_name("constructor"))
                                .map(ResolvedTypeMember::to_member);
                            match member {
                                Some(member) => resolver
                                    .type_from_member(resolved.to_data(), member)
                                    .to_data()
                                    .flattened(resolver),
                                None => Self::unknown(),
                            }
                        }
                        _ => self,
                    },
                    None => self,
                },
                TypeofExpression::Destructure(expr) => {
                    match resolver.resolve_and_get(&expr.ty) {
                        Some(resolved) => match (resolved.as_raw_data(), &expr.destructure_field) {
                            (Self::Class(class), DestructureField::Name(name)) => match class
                                .members
                                .iter()
                                .find(|own_member| {
                                    own_member.is_static() && own_member.has_name(name.text())
                                }) {
                                Some(member) => resolver
                                    .type_from_member(
                                        resolved.to_data(),
                                        ResolvedTypeMember::from((resolved.resolver_id(), member))
                                            .to_member(),
                                    )
                                    .to_data()
                                    .flattened(resolver),
                                None => Self::unknown(),
                            },
                            (Self::Class(class), DestructureField::RestExcept(names)) => {
                                Self::object_with_members(
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
                                )
                            }
                            (ty, DestructureField::Index(index)) => ty
                                .clone()
                                .find_element_type_at_index(
                                    resolved.resolver_id(),
                                    resolver,
                                    *index,
                                )
                                .map(ResolvedTypeData::to_data)
                                .unwrap_or_default(),
                            (ty, DestructureField::RestFrom(index)) => ty
                                .clone()
                                .find_type_of_elements_from_index(
                                    resolved.resolver_id(),
                                    resolver,
                                    *index,
                                )
                                .map(ResolvedTypeData::to_data)
                                .unwrap_or_default(),
                            (_, DestructureField::Name(name)) => {
                                let member = resolved.all_members(resolver).find(|member| {
                                    !member.is_static() && member.has_name(name.text())
                                });
                                match member {
                                    Some(member) => resolver
                                        .type_from_member(resolved.to_data(), member.to_member())
                                        .to_data(),
                                    None => Self::unknown(),
                                }
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
                                Self::object_with_members(
                                    members
                                        .into_values()
                                        .map(ResolvedTypeMember::to_member)
                                        .collect(),
                                )
                            }
                        },
                        None => self,
                    }
                }
                TypeofExpression::New(expr) => {
                    match resolver
                        .resolve_and_get(&expr.callee)
                        .map(ResolvedTypeData::to_data)
                        .map(|type_data| type_data.flattened(resolver))
                    {
                        Some(Self::Class(class)) => {
                            let num_args = expr.arguments.len();
                            let ty = class
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
                            Self::instance_of(ty).flattened(resolver)
                        }
                        // TODO: Handle objects with call signatures.
                        _ => self,
                    }
                }
                TypeofExpression::StaticMember(expr) => {
                    if let Some(object) = resolver.resolve_and_get(&expr.object) {
                        // FIXME: Flattening intersections and unions for members should be done in
                        //        `TypeMemberIterator`.

                        if let Self::InstanceOf(instance) = object.as_raw_data() {
                            let instance_ty = object.apply_module_id_to_reference(&instance.ty);
                            if resolver
                                .resolve_and_get(&instance_ty)
                                .is_some_and(|object| {
                                    matches!(
                                        object.as_raw_data(),
                                        Self::Intersection(_) | Self::Union(_)
                                    )
                                })
                            {
                                return Self::TypeofExpression(Box::new(
                                    TypeofExpression::StaticMember(TypeofStaticMemberExpression {
                                        object: instance_ty.into_owned(),
                                        member: expr.member.clone(),
                                    }),
                                ))
                                .flattened(resolver);
                            }
                        };

                        if let Self::Intersection(intersection) = object.as_raw_data() {
                            let types: Vec<_> = intersection
                                .types()
                                .iter()
                                .map(|reference| object.apply_module_id_to_reference(reference))
                                .map(|reference| reference.into_owned())
                                .collect();
                            let types = types
                                .into_iter()
                                .map(|ty| {
                                    // Resolve and flatten the type member for each variant.
                                    let ty = Self::TypeofExpression(Box::new(
                                        TypeofExpression::StaticMember(
                                            TypeofStaticMemberExpression {
                                                object: ty,
                                                member: expr.member.clone(),
                                            },
                                        ),
                                    ))
                                    .flattened(resolver);

                                    resolver.reference_to_registered_data(ty)
                                })
                                .collect();

                            return Self::intersection_of(types);
                        }

                        if let Self::Union(union) = object.as_raw_data() {
                            let types: Vec<_> = union
                                .types()
                                .iter()
                                .map(|reference| object.apply_module_id_to_reference(reference))
                                .map(|reference| reference.into_owned())
                                .collect();
                            let types = types
                                .into_iter()
                                .map(|ty| {
                                    // Resolve and flatten the type member for each variant.
                                    let ty = Self::TypeofExpression(Box::new(
                                        TypeofExpression::StaticMember(
                                            TypeofStaticMemberExpression {
                                                object: ty,
                                                member: expr.member.clone(),
                                            },
                                        ),
                                    ))
                                    .flattened(resolver);

                                    resolver.reference_to_registered_data(ty)
                                })
                                .collect();

                            return Self::union_of(types);
                        }

                        let is_class = matches!(object.as_raw_data(), Self::Class(_));
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
                                Self::reference(
                                    resolver.register_type_from_member(object.to_data(), member),
                                )
                                .flattened(resolver)
                            }
                            None => Self::unknown(),
                        }
                    } else {
                        self
                    }
                }
                TypeofExpression::Super(expr) => match resolver.resolve_and_get(&expr.parent) {
                    Some(resolved) => match resolved.as_raw_data() {
                        Self::Class(class) => match class.extends.as_ref() {
                            Some(super_class) => Self::instance_of(
                                resolved
                                    .apply_module_id_to_reference(super_class)
                                    .into_owned(),
                            )
                            .flattened(resolver),
                            None => Self::unknown(),
                        },
                        _ => Self::unknown(),
                    },
                    None => self,
                },
                TypeofExpression::This(expr) => match resolver.resolve_reference(&expr.parent) {
                    Some(class_id) => {
                        Self::instance_of(TypeReference::from(class_id)).flattened(resolver)
                    }
                    None => self,
                },
            },
            Self::TypeofValue(value) => match resolver.resolve_reference(&value.ty) {
                Some(type_id) => Self::reference(type_id).flattened(resolver),
                None => self,
            },
            _ => self,
        }
    }
}

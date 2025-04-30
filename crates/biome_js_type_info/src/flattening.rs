use std::collections::{BTreeMap, btree_map::Entry};

use biome_rowan::Text;

use crate::{
    DestructureField, GenericTypeParameter, TypeData, TypeInstance, TypeMember, TypeReference,
    TypeResolver, TypeofExpression,
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
    pub fn flattened(&self, resolver: &mut dyn TypeResolver) -> Self {
        match self {
            Self::InstanceOf(instance_of) => match resolver.resolve_and_get(&instance_of.ty) {
                Some(Self::InstanceOf(resolved_instance)) => Self::instance_of(TypeInstance {
                    ty: resolved_instance.ty.clone(),
                    type_parameters: GenericTypeParameter::merge_parameters(
                        &resolved_instance.type_parameters,
                        &instance_of.type_parameters,
                    ),
                }),
                Some(
                    ty @ (Self::Global | Self::Function(_) | Self::Literal(_) | Self::Object(_)),
                ) => ty.clone().flattened(resolver),
                _ => self.clone(),
            },
            Self::Reference(reference) => match resolver.resolve_and_get(reference) {
                Some(ty) => ty.clone().flattened(resolver),
                None => self.clone(),
            },
            Self::TypeofExpression(expr) => match expr.as_ref() {
                TypeofExpression::Addition(_expr) => {
                    // TODO
                    self.clone()
                }
                TypeofExpression::Await(expr) => match resolver.resolve_and_get(&expr.argument) {
                    Some(Self::Literal(literal)) => Self::Literal(literal.clone()),
                    Some(ty) => match ty.find_promise_type(resolver) {
                        Some(ty) => ty.clone().flattened(resolver),
                        None => self.clone(),
                    },
                    _ => self.clone(),
                },
                TypeofExpression::Call(expr) => match resolver.resolve_and_get(&expr.callee) {
                    Some(Self::Function(function)) => match function.return_type.as_type() {
                        Some(ty) => Self::reference(ty.clone()).flattened(resolver),
                        None => self.clone(),
                    },
                    Some(ty @ Self::Object(object)) => {
                        match object
                            .members
                            .iter()
                            .find(|member| member.has_name("constructor"))
                            .cloned()
                        {
                            Some(member) => resolver
                                .type_from_member(&ty.clone(), &member)
                                .flattened(resolver),
                            None => Self::unknown(),
                        }
                    }
                    _ => self.clone(),
                },
                TypeofExpression::Destructure(expr) => {
                    match (resolver.resolve_and_get(&expr.ty), &expr.destructure_field) {
                        (Some(ty @ Self::Class(class)), DestructureField::Name(name)) => {
                            match class
                                .members
                                .iter()
                                .find(|own_member| {
                                    own_member.is_static() && own_member.has_name(name.text())
                                })
                                .cloned()
                            {
                                Some(member) => resolver
                                    .type_from_member(&ty.clone(), &member)
                                    .flattened(resolver),
                                None => Self::unknown(),
                            }
                        }
                        (Some(Self::Class(class)), DestructureField::RestExcept(names)) => {
                            Self::object_with_members(
                                class
                                    .members
                                    .iter()
                                    .filter(|own_member| {
                                        own_member.is_static()
                                            && !names.iter().any(|name| own_member.has_name(name))
                                    })
                                    .cloned()
                                    .collect(),
                            )
                        }
                        (Some(ty), DestructureField::Index(index)) => ty
                            .clone()
                            .find_element_type_at_index(resolver, *index)
                            .cloned()
                            .unwrap_or_default(),
                        (Some(ty), DestructureField::RestFrom(index)) => ty
                            .clone()
                            .find_type_of_elements_from_index(resolver, *index)
                            .unwrap_or_default(),
                        (Some(ty), DestructureField::Name(name)) => {
                            let member = ty
                                .all_members(resolver)
                                .find(|member| !member.is_static() && member.has_name(name.text()))
                                .cloned();
                            match member {
                                Some(member) => resolver.type_from_member(&ty.clone(), &member),
                                None => Self::unknown(),
                            }
                        }
                        (Some(ty), DestructureField::RestExcept(names)) => {
                            // We need to look up the prototype chain, which may
                            // yield duplicate member names. We deduplicate
                            // using a map before constructing a new object.
                            let members: BTreeMap<Text, TypeMember> = ty
                                .all_members(resolver)
                                .filter(|member| {
                                    !member.is_static()
                                        && !names.iter().any(|name| member.has_name(name))
                                })
                                .fold(BTreeMap::new(), |mut map, member| {
                                    if let Some(name) = member.name() {
                                        if let Entry::Vacant(entry) = map.entry(name) {
                                            entry.insert(member.clone());
                                        }
                                    }
                                    map
                                });
                            Self::object_with_members(members.into_values().collect())
                        }
                        _ => Self::unknown(),
                    }
                }
                TypeofExpression::New(expr) => {
                    match resolver
                        .resolve_and_get(&expr.callee)
                        .cloned()
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
                        _ => self.clone(),
                    }
                }
                TypeofExpression::StaticMember(expr) => {
                    match resolver
                        .resolve_and_get(&expr.object)
                        .cloned()
                        .map(|type_data| type_data.flattened(resolver))
                    {
                        Some(object @ Self::Class(_)) => {
                            let member = object
                                .all_members(resolver)
                                .find(|member| member.is_static() && member.has_name(&expr.member))
                                .cloned();
                            match member {
                                Some(member) => Self::reference(
                                    resolver.register_type_from_member(&object.clone(), &member),
                                ),
                                None => Self::unknown(),
                            }
                        }
                        Some(object) => {
                            let member = object
                                .all_members(resolver)
                                .find(|member| !member.is_static() && member.has_name(&expr.member))
                                .cloned();
                            match member {
                                Some(member) => Self::reference(
                                    resolver.register_type_from_member(&object.clone(), &member),
                                )
                                .flattened(resolver),
                                None => Self::unknown(),
                            }
                        }
                        _ => self.clone(),
                    }
                }
                TypeofExpression::Super(expr) => match resolver.resolve_and_get(&expr.parent) {
                    Some(Self::Class(class)) => match class.extends.as_ref() {
                        Some(super_class) => {
                            Self::instance_of(super_class.clone()).flattened(resolver)
                        }
                        None => Self::unknown(),
                    },
                    _ => self.clone(),
                },
                TypeofExpression::This(expr) => match resolver.resolve_reference(&expr.parent) {
                    Some(class_id) => {
                        Self::instance_of(TypeReference::from(class_id)).flattened(resolver)
                    }
                    None => self.clone(),
                },
            },
            Self::TypeofValue(value) => match resolver.resolve_reference(&value.ty) {
                Some(type_id) => Self::reference(type_id).flattened(resolver),
                None => self.clone(),
            },
            _ => self.clone(),
        }
    }
}

use std::{
    collections::{BTreeMap, btree_map::Entry},
    ops::Deref,
};

use biome_rowan::Text;

use crate::{
    DestructureField, Resolvable, Type, TypeInner, TypeMember, TypeResolver, TypeofExpression,
    globals::ARRAY,
};

impl Type {
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
    /// Type(TypeInner::TypeofValue {
    ///     identifier: "c",
    ///     ty: Type::unknown()
    /// })
    /// ```
    ///
    /// Once we've performed thin type resolution, this becomes:
    ///
    /// ```no_test
    /// Type(TypeInner::TypeofValue {
    ///     identifier: "c",
    ///     ty: Type(TypeInner::Literal(Literal::Number(1)))
    /// })
    /// ```
    ///
    /// With flattening, we can reduce this to:
    ///
    /// ```no_test
    /// Type(TypeInner::Literal(Literal::Number(1)))
    /// ```
    pub fn flattened(&self, resolver: &dyn TypeResolver, stack: &[&TypeInner]) -> Self {
        match self.deref() {
            TypeInner::InstanceOf(ty) => match ty.inner_type() {
                TypeInner::Class(_class) => Self::instance_of(ty.as_ref().clone()),
                TypeInner::Reference(reference) if reference.ty.is_inferred() => {
                    Self::instance_of(reference.ty.clone())
                }
                _ => self.clone(),
            },
            TypeInner::Reference(reference) if reference.ty.is_inferred() => reference
                .ty
                .with_type_parameters(&reference.type_parameters)
                .resolved(resolver, stack),
            TypeInner::TypeofExpression(expr) => match expr.as_ref() {
                TypeofExpression::Addition(_expr) => {
                    // TODO
                    self.clone()
                }
                TypeofExpression::Await(expr) => match expr.argument.deref() {
                    TypeInner::Literal(literal) => TypeInner::Literal(literal.clone()).into(),
                    TypeInner::Object(object) => match object.find_promise_type() {
                        Some(ty) => ty.resolved(resolver, stack),
                        None => self.clone(),
                    },
                    _ => self.clone(),
                },
                TypeofExpression::Call(expr) => match expr.callee.inner_type() {
                    TypeInner::Function(function) => match function.return_type.as_type() {
                        Some(ty) => Self::instance_of(ty.clone()).resolved(resolver, stack),
                        None => self.clone(),
                    },
                    TypeInner::Object(object) => {
                        match object
                            .members
                            .iter()
                            .find(|member| member.has_name("constructor"))
                        {
                            Some(member) => member.to_type(&expr.callee),
                            None => Self::unknown(),
                        }
                    }
                    _ => self.clone(),
                },
                TypeofExpression::Destructure(expr) => {
                    let ty = expr.ty.resolved(resolver, stack);
                    match (ty.inner_type(), &expr.destructure_field) {
                        (TypeInner::Class(class), DestructureField::Name(name)) => class
                            .members
                            .iter()
                            .find(|own_member| {
                                own_member.is_static() && own_member.has_name(name.text())
                            })
                            .map(|member| member.to_type(&ty))
                            .unwrap_or_default(),
                        (TypeInner::Class(class), DestructureField::RestExcept(names)) => {
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
                        (TypeInner::Object(object), DestructureField::Index(_index)) => {
                            if let Some(array) = object.find_parent_class(&ARRAY) {
                                array
                                    .type_parameters
                                    .first()
                                    .map(|param| param.ty.union_with(Self::undefined()))
                                    .unwrap_or_default()
                            } else {
                                Self::unknown()
                            }
                        }
                        (TypeInner::Object(object), DestructureField::Name(name)) => object
                            .all_members()
                            .find(|member| !member.is_static() && member.has_name(name.text()))
                            .map(|member| member.to_type(&ty))
                            .unwrap_or_default(),
                        (TypeInner::Object(object), DestructureField::RestExcept(names)) => {
                            // We need to look up the prototype chain, which may
                            // yield duplicate member names. We deduplicate
                            // using a map before constructing a new object.
                            let members: BTreeMap<Text, TypeMember> = object
                                .all_members()
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
                        (TypeInner::Object(object), DestructureField::RestFrom(_index)) => {
                            if let Some(_array) = object.find_parent_class(&ARRAY) {
                                expr.ty.clone()
                            } else {
                                Self::unknown()
                            }
                        }
                        (TypeInner::Tuple(tuple), DestructureField::Index(index)) => {
                            tuple.get_ty(*index)
                        }
                        (TypeInner::Tuple(tuple), DestructureField::RestFrom(index)) => {
                            TypeInner::Tuple(Box::new(tuple.slice_from(*index))).into()
                        }
                        _ => Self::unknown(),
                    }
                }
                TypeofExpression::New(expr) => {
                    let callee = expr.callee.resolved(resolver, stack);
                    match callee.inner_type() {
                        TypeInner::Class(class) => {
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
                                .map_or_else(
                                    || callee.owned_inner_type(),
                                    |ty| ty.resolved(resolver, stack),
                                );
                            Self::instance_of(ty)
                        }
                        // TODO: Handle objects with call signatures.
                        _ => self.clone(),
                    }
                }
                TypeofExpression::StaticMember(expr) => {
                    let object = expr.object.resolved(resolver, stack);
                    match object.inner_type() {
                        TypeInner::Class(class) => {
                            let member = class
                                .all_members()
                                .find(|member| member.is_static() && member.has_name(&expr.member));
                            match member {
                                Some(member) => Self::instance_of(
                                    member.to_type(&object).resolved(resolver, stack),
                                ),
                                None => Self::unknown(),
                            }
                        }
                        TypeInner::Object(inner) => {
                            let member = inner.all_members().find(|member| {
                                !member.is_static() && member.has_name(&expr.member)
                            });
                            match member {
                                Some(member) => Self::instance_of(
                                    member.to_type(&object).resolved(resolver, stack),
                                ),
                                None => Self::unknown(),
                            }
                        }
                        _ => self.clone(),
                    }
                }
                TypeofExpression::Super(expr) => {
                    let class = expr.parent.resolved(resolver, stack);
                    match class.inner_type() {
                        TypeInner::Class(class) => match class.extends.as_ref() {
                            Some(super_class) => {
                                Self::instance_of(super_class.resolved(resolver, stack))
                            }
                            None => Self::unknown(),
                        },
                        _ => Self::unknown(),
                    }
                }
                TypeofExpression::This(expr) => {
                    let class = expr.parent.resolved(resolver, stack);
                    Self::instance_of(class)
                }
            },
            TypeInner::TypeofValue(value) if value.ty.is_inferred() => {
                value.ty.resolved(resolver, stack)
            }
            _ => self.clone(),
        }
    }
}

use std::ops::Deref;

use crate::{Resolvable, Type, TypeInner, TypeMember, TypeResolver, TypeofExpression};

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

#[cfg(test)]
#[path = "flattening.tests.rs"]
mod tests;

use std::sync::Arc;

use crate::{TypeData, TypeInstance, TypeReference, TypeResolver};

mod expressions;
mod intersections;

use expressions::flattened_expression;
use intersections::flattened_intersection;

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
    /// }
    /// ```
    ///
    /// Once we've performed thin type resolution, this becomes:
    ///
    /// ```no_test
    /// TypeData::TypeofValue {
    ///     identifier: "c",
    ///     ty: TypeReference::Resolved(<type ID of literal>)
    /// }
    /// ```
    ///
    /// With flattening, we can reduce this to:
    ///
    /// ```no_test
    /// TypeData::Literal(Literal::Number(1)))
    /// ```
    pub fn flattened(self: Arc<Self>, resolver: &mut dyn TypeResolver) -> Arc<Self> {
        flattened(self, resolver, 0)
    }
}

fn flattened(
    mut ty: Arc<TypeData>,
    resolver: &mut dyn TypeResolver,
    depth: usize,
) -> Arc<TypeData> {
    const MAX_FLATTEN_DEPTH: usize = 10; // Arbitrary depth, may require tweaking.

    for depth in depth + 1..=MAX_FLATTEN_DEPTH {
        match ty.as_ref() {
            TypeData::MergedReference(merged) => {
                match (&merged.ty, &merged.value_ty, &merged.namespace_ty) {
                    (Some(ty1), Some(ty2), Some(ty3)) if ty1 == ty2 && ty1 == ty3 => {
                        ty = Arc::new(TypeData::Reference(ty1.clone()));
                    }
                    (Some(ty1), Some(ty2), None)
                    | (Some(ty1), None, Some(ty2))
                    | (None, Some(ty1), Some(ty2))
                        if ty1 == ty2 =>
                    {
                        ty = Arc::new(TypeData::Reference(ty1.clone()));
                    }
                    _ => return ty,
                }
            }
            TypeData::InstanceOf(instance_of) => match &instance_of.ty {
                TypeReference::Unknown => return Arc::new(TypeData::unknown()),
                reference => match resolver.resolve_and_get(reference) {
                    Some(resolved) => match resolved.as_raw_data() {
                        TypeData::InstanceOf(resolved_instance) => {
                            return Arc::new(resolved.apply_module_id_to_data(
                                TypeData::instance_of(TypeInstance {
                                    ty: resolved_instance.ty.clone(),
                                    type_parameters: TypeReference::merge_parameters(
                                        &resolved_instance.type_parameters,
                                        &instance_of.type_parameters,
                                    ),
                                }),
                            ));
                        }
                        TypeData::Reference(reference) => {
                            return match reference {
                                TypeReference::Unknown => Arc::new(TypeData::unknown()),
                                _ => Arc::new(resolved.apply_module_id_to_data(
                                    TypeData::instance_of(TypeInstance {
                                        ty: reference.clone(),
                                        type_parameters: instance_of.type_parameters.clone(),
                                    }),
                                )),
                            };
                        }
                        TypeData::Global
                        | TypeData::Function(_)
                        | TypeData::Literal(_)
                        | TypeData::Object(_) => ty = Arc::new(resolved.to_data()),
                        _ => return ty,
                    },
                    None => return ty,
                },
            },
            TypeData::Intersection(intersection) => {
                ty = Arc::new(flattened_intersection(intersection, resolver));
            }
            TypeData::Reference(reference) => match reference {
                TypeReference::Unknown => return Arc::new(TypeData::unknown()),
                _ => match resolver.resolve_and_get(reference) {
                    Some(reference) => match reference.as_raw_data() {
                        TypeData::InstanceOf(instance_of) => {
                            ty = Arc::new(reference.apply_module_id_to_data(TypeData::InstanceOf(
                                instance_of.clone(),
                            )));
                        }
                        TypeData::Reference(target) => {
                            ty = Arc::new(TypeData::Reference(
                                reference.apply_module_id_to_reference(target).into_owned(),
                            ));
                        }
                        _ => return ty,
                    },
                    None => return ty,
                },
            },
            TypeData::TypeofExpression(expr) => match flattened_expression(expr, resolver, depth) {
                Some(flattened_ty) => {
                    ty = flattened_ty;
                }
                None => return ty,
            },
            TypeData::TypeofType(reference) => {
                match resolver.resolve_reference(reference.as_ref()) {
                    Some(resolved) => ty = Arc::new(TypeData::reference(resolved)),
                    None => return ty,
                }
            }
            TypeData::TypeofValue(value) if value.ty.is_known() => {
                match resolver.resolve_reference(&value.ty) {
                    Some(resolved) => ty = Arc::new(TypeData::reference(resolved)),
                    None => return ty,
                }
            }
            _ => return ty,
        }
    }

    debug_assert!(false, "max flattening depth reached");
    Arc::new(TypeData::unknown())
}

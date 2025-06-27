use crate::{TypeData, TypeInstance, TypeReference, TypeResolver};

mod conditionals;
mod expressions;
mod intersections;

use expressions::flattened_expression;
use intersections::flattened_intersection;

pub const MAX_FLATTEN_DEPTH: usize = 10; // Arbitrary depth, may require tweaking.

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
    pub fn flattened(&self, resolver: &mut dyn TypeResolver) -> Option<Self> {
        match self {
            Self::MergedReference(merged) => {
                match (&merged.ty, &merged.value_ty, &merged.namespace_ty) {
                    (Some(ty1), Some(ty2), Some(ty3)) if ty1 == ty2 && ty1 == ty3 => {
                        Some(Self::Reference(ty1.clone()))
                    }
                    (Some(ty1), Some(ty2), None)
                    | (Some(ty1), None, Some(ty2))
                    | (None, Some(ty1), Some(ty2))
                        if ty1 == ty2 =>
                    {
                        Some(Self::Reference(ty1.clone()))
                    }
                    _ => None,
                }
            }
            Self::InstanceOf(instance) => match &instance.ty {
                TypeReference::Unknown => Some(Self::unknown()),
                reference => match resolver.resolve_and_get(reference) {
                    Some(resolved) => match resolved.as_raw_data() {
                        Self::InstanceOf(resolved_instance) => Some(
                            resolved.apply_module_id_to_data(Self::instance_of(TypeInstance {
                                ty: resolved_instance.ty.clone(),
                                type_parameters: TypeReference::merge_parameters(
                                    &resolved_instance.type_parameters,
                                    &instance.type_parameters,
                                ),
                            })),
                        ),
                        Self::Reference(reference) => match reference {
                            TypeReference::Unknown => Some(Self::unknown()),
                            _ => Some(resolved.apply_module_id_to_data(Self::instance_of(
                                TypeInstance {
                                    ty: reference.clone(),
                                    type_parameters: instance.type_parameters.clone(),
                                },
                            ))),
                        },
                        _ => resolved
                            .should_flatten_instance(instance)
                            .then(|| resolved.to_data()),
                    },
                    None => None,
                },
            },
            Self::Intersection(intersection) => {
                Some(flattened_intersection(intersection, resolver))
            }
            Self::Reference(reference) => match reference {
                TypeReference::Unknown => Some(Self::unknown()),
                _ => match resolver.resolve_and_get(reference) {
                    Some(reference) => match reference.as_raw_data() {
                        Self::InstanceOf(instance_of) => Some(
                            reference
                                .apply_module_id_to_data(Self::InstanceOf(instance_of.clone())),
                        ),
                        Self::Reference(target) => Some(Self::Reference(
                            reference.apply_module_id_to_reference(target).into_owned(),
                        )),
                        _ => None,
                    },
                    None => None,
                },
            },
            Self::TypeofExpression(expr) => flattened_expression(expr, resolver),
            Self::TypeofType(reference) => resolver
                .resolve_reference(reference.as_ref())
                .map(Self::reference),
            Self::TypeofValue(value) if value.ty.is_known() => {
                resolver.resolve_reference(&value.ty).map(Self::reference)
            }
            _ => None,
        }
    }
}

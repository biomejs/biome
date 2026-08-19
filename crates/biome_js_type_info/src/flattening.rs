use crate::{TypeData, TypeInstance, TypeReference, TypeResolver};

mod expressions;
mod intersections;
mod unions;

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
            Self::InstanceOf(instance) => flattened_instance(instance, &instance.ty, resolver),
            Self::Intersection(intersection) => {
                Some(flattened_intersection(intersection, resolver))
            }
            Self::Reference(reference) => match resolver.resolve_and_get(reference) {
                Some(reference) => match reference.as_raw_data() {
                    Self::InstanceOf(instance_of) => Some(
                        reference.apply_module_id_to_data(Self::InstanceOf(instance_of.clone())),
                    ),
                    Self::Reference(target) => Some(Self::Reference(
                        reference.apply_module_id_to_reference(target).into_owned(),
                    )),
                    Self::Unknown => Some(Self::unknown()),
                    _ => None,
                },
                None => None,
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

fn flattened_instance(
    instance: &TypeInstance,
    reference: &TypeReference,
    resolver: &dyn TypeResolver,
) -> Option<TypeData> {
    let resolved = resolver.resolve_and_get(reference)?;
    match resolved.as_raw_data() {
        TypeData::InstanceOf(resolved_instance) => Some(TypeData::instance_of(TypeInstance {
            ty: resolved
                .apply_module_id_to_reference(&resolved_instance.ty)
                .into_owned(),
            type_parameters: {
                resolved_instance
                    .type_parameters
                    .iter()
                    .enumerate()
                    .map(|(i, param)| {
                        instance.type_parameters.get(i).cloned().unwrap_or_else(|| {
                            resolved.apply_module_id_to_reference(param).into_owned()
                        })
                    })
                    .collect()
            },
        })),
        TypeData::Reference(reference) if reference.is_known() => {
            Some(TypeData::instance_of(TypeInstance {
                ty: resolved
                    .apply_module_id_to_reference(reference)
                    .into_owned(),
                type_parameters: instance.type_parameters.clone(),
            }))
        }
        TypeData::Reference(_) => Some(TypeData::unknown()),
        _ => resolved
            .should_flatten_instance(instance)
            .then(|| resolved.to_data()),
    }
}

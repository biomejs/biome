use crate::{MappedType, TypeData, TypeInstance, TypeOperator, TypeReference, TypeResolver};

mod expressions;
mod intersections;
mod mapped_types;
mod unions;

use crate::flattening::mapped_types::flattened_mapped_type_with_substitutions;
use expressions::flattened_expression;
use intersections::flattened_intersection;
use mapped_types::{flattened_indexed_access, flattened_keyof, flattened_mapped_type};

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
            Self::TypeOperator(op) if matches!(op.operator, TypeOperator::Keyof) => {
                flattened_keyof(op, resolver)
            }
            Self::IndexedAccessType(access) => flattened_indexed_access(access, resolver),
            Self::MappedType(mapped) => flattened_mapped_type(mapped, resolver),
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
        TypeData::InstanceOf(resolved_instance) => {
            let inner_ref = resolved
                .apply_module_id_to_reference(&resolved_instance.ty)
                .into_owned();

            // Check if the inner type is a MappedType with concrete type parameters.
            if let Some(inner_resolved) = resolver.resolve_and_get(&inner_ref)
                && let TypeData::MappedType(mapped) = inner_resolved.as_raw_data()
            {
                // Build the concrete type parameters by merging outer and inner.
                let concrete_params: Box<_> = resolved_instance
                    .type_parameters
                    .iter()
                    .enumerate()
                    .map(|(i, param)| {
                        instance.type_parameters.get(i).cloned().unwrap_or_else(|| {
                            resolved.apply_module_id_to_reference(param).into_owned()
                        })
                    })
                    .collect();

                return flattened_mapped_type_with_generics(
                    mapped,
                    &resolved_instance.type_parameters,
                    &concrete_params,
                    resolver,
                );
            }

            Some(TypeData::instance_of(TypeInstance {
                ty: inner_ref,
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
            }))
        }
        TypeData::MappedType(mapped) if !instance.type_parameters.is_empty() => {
            // Direct InstanceOf(MappedType, [params]) — try a generic substitution.
            // We need to find the generic parameter references. The instance's
            // type_parameters correspond to the generic params of the type alias.
            // Look at the generic definitions to find what to substitute.
            flattened_mapped_type_instance(mapped, instance, resolver)
        }
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

/// Flattens a mapped type instance by substituting generic type parameters
/// with concrete types and then evaluating the mapped type.
fn flattened_mapped_type_instance(
    _mapped: &MappedType,
    instance: &TypeInstance,
    resolver: &dyn TypeResolver,
) -> Option<TypeData> {
    // Find the generic parameter references in the instance's type_parameters.
    // These are the parameters defined on the type alias (e.g., `T` in `type Foo<T>`).
    // If all params are still generics (not yet substituted with concrete types),
    // we can't flatten — we need concrete types from an outer instantiation.
    let all_generic = instance.type_parameters.iter().all(|param_ref| {
        resolver
            .resolve_and_get(param_ref)
            .is_some_and(|r| matches!(r.as_raw_data(), TypeData::Generic(_)))
    });

    if all_generic {
        return None;
    }

    // TODO: Handle the case where InstanceOf(MappedType, [concrete_refs])
    // appears without an outer InstanceOf wrapper. This requires building
    // the generic-to-concrete mapping from context.
    None
}

/// Flattens a mapped type when we have both the generic parameter references
/// and their concrete substitutions.
///
/// Instead of trying to deep-substitute inside referenced types (which would
/// require mutable access to the resolver), this function directly evaluates
/// the mapped type by:
/// 1. Resolving the keys via the generic→concrete substitution
/// 2. Building the object members with the substituted value types
///
/// This avoids the need for multi-pass flattening of intermediate MappedTypes.
fn flattened_mapped_type_with_generics(
    mapped: &MappedType,
    generic_param_refs: &[TypeReference],
    concrete_type_refs: &[TypeReference],
    resolver: &dyn TypeResolver,
) -> Option<TypeData> {
    let substitutions: Vec<(TypeReference, TypeReference)> = generic_param_refs
        .iter()
        .zip(concrete_type_refs.iter())
        .filter(|(g, c)| g != c)
        .map(|(generic, concrete)| (generic.clone(), concrete.clone()))
        .collect();

    if substitutions.is_empty() {
        return None;
    }

    flattened_mapped_type_with_substitutions(mapped, &substitutions, resolver)
}

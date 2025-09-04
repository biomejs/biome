use biome_rowan::Text;

use crate::{
    Literal, MergedReference, ResolvedTypeData, TypeData, TypeReference, TypeResolver,
    literal::NumberLiteral,
};

use super::MAX_FLATTEN_DEPTH;

/// Provides semantic information about types for use in conditionals.
///
/// If you want to know whether a type is truthy, falsy, nullish, or
/// non-nullish, see the [`Self::is_truthy()`], [`Self::is_falsy()`],
/// [`Self::is_nullish()`] and [`Self::is_non_nullish()`] methods. Please
/// refrain from matching the enum variants directly.
///
/// You can access the conditional information from a [`Type`](crate::Type)
/// instance using its [`conditional()`](crate::Type::conditional) method.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConditionalType {
    /// A type for which no semantic value can be determined statically.
    Anything,
    /// Covers any type that is considered falsy, including `null` or
    /// `undefined`.
    Falsy,
    /// Covers any type that is considered falsy, except `null` and `undefined`.
    FalsyButNotNullish,
    /// Anything except `null` and `undefined`.
    NonNullish,
    /// `null` or `undefined`.
    Nullish,
    /// Covers any type that is considered truthy.
    Truthy,
    /// Indicates we don't know, either because of a failure in our inference,
    /// or simply because the type hasn't been fully flattened yet. In either
    /// case, we shouldn't make assumptions (yet). We don't want unknown values
    /// to spread unnecessarily, and it's possible that after flattening
    /// referenced types, we may know more.
    Unknown,
}

impl ConditionalType {
    /// Returns the conditional type from the given resolved data, `ty`.
    ///
    /// Resolves references as necessary.
    pub fn from_resolved_data(ty: ResolvedTypeData, resolver: &dyn TypeResolver) -> Self {
        fn derive_conditional_type(
            ty: ResolvedTypeData,
            resolver: &dyn TypeResolver,
            mut depth: usize,
        ) -> ConditionalType {
            depth += 1;
            if depth > MAX_FLATTEN_DEPTH {
                return ConditionalType::Unknown;
            }

            let derive_from_resolved_reference = |reference: &TypeReference| -> ConditionalType {
                match resolver.resolve_and_get(reference) {
                    Some(ty) => derive_conditional_type(ty, resolver, depth),
                    None => ConditionalType::Unknown,
                }
            };

            let derive_from_reference = |reference: &TypeReference| -> ConditionalType {
                let reference = ty.apply_module_id_to_reference(reference);
                derive_from_resolved_reference(&reference)
            };

            match ConditionalType::from_data_shallow(ty.as_raw_data()) {
                Some(conditional) => conditional,
                None => match ty.as_raw_data() {
                    TypeData::InstanceOf(instance) => derive_from_reference(&instance.ty),
                    TypeData::Intersection(intersection) => {
                        let mut conditional = ConditionalType::Unknown;
                        for ty in intersection.types() {
                            let next = derive_from_reference(ty);
                            conditional = if conditional == ConditionalType::Unknown {
                                next
                            } else {
                                conditional.merged_with(next)
                            };
                            if !conditional.is_mergeable() {
                                break;
                            }
                        }
                        conditional
                    }
                    TypeData::MergedReference(reference) => {
                        let conditional_ty = reference.ty.as_ref().map(derive_from_reference);
                        let conditional_value_ty =
                            reference.value_ty.as_ref().map(derive_from_reference);
                        let conditional_namespace_ty =
                            reference.namespace_ty.as_ref().map(derive_from_reference);
                        match (
                            conditional_ty,
                            conditional_value_ty,
                            conditional_namespace_ty,
                        ) {
                            (None, None, None) => ConditionalType::Unknown,
                            (Some(conditional), None, None)
                            | (None, Some(conditional), None)
                            | (None, None, Some(conditional)) => conditional,
                            (Some(c1), Some(c2), None)
                            | (None, Some(c1), Some(c2))
                            | (Some(c1), None, Some(c2)) => c1.merged_with(c2),
                            (Some(c1), Some(c2), Some(c3)) => c1.merged_with(c2).merged_with(c3),
                        }
                    }
                    TypeData::Reference(reference) => derive_from_reference(reference),
                    TypeData::Union(_) => {
                        let mut conditional = ConditionalType::Unknown;
                        for ty in ty.flattened_union_variants(resolver) {
                            let next = derive_from_resolved_reference(&ty);
                            conditional = if conditional == ConditionalType::Unknown {
                                next
                            } else {
                                conditional.merged_with(next)
                            };
                            if !conditional.is_mergeable() {
                                break;
                            }
                        }
                        conditional
                    }
                    _ => unreachable!(),
                },
            }
        }

        derive_conditional_type(ty, resolver, 0)
    }

    /// Returns the conditional type from the given data, `ty`.
    ///
    /// Returns `None` if `ty` contains references that we cannot resolve.
    fn from_data_shallow(ty: &TypeData) -> Option<Self> {
        match ty {
            TypeData::AnyKeyword
            | TypeData::Conditional
            | TypeData::NeverKeyword
            | TypeData::ThisKeyword
            | TypeData::Unknown
            | TypeData::UnknownKeyword => Some(Self::Anything),
            TypeData::BigInt
            | TypeData::Boolean
            | TypeData::Interface(_)
            | TypeData::Number
            | TypeData::String => Some(Self::NonNullish),
            TypeData::Class(_)
            | TypeData::Constructor(_)
            | TypeData::Function(_)
            | TypeData::Global
            | TypeData::ImportNamespace(_)
            | TypeData::Module(_)
            | TypeData::Namespace(_)
            | TypeData::Object(_)
            | TypeData::ObjectKeyword
            | TypeData::Symbol
            | TypeData::Tuple(_) => Some(Self::Truthy),
            TypeData::InstanceOf(_)
            | TypeData::Intersection(_)
            | TypeData::MergedReference(_)
            | TypeData::Reference(_)
            | TypeData::Union(_) => {
                // IMPORTANT: If you add a variant to this branch, make sure to
                //            handle it in other matches in this file.
                None
            }
            TypeData::Literal(literal) => Some(match literal.as_ref() {
                Literal::BigInt(text) => match text.text() {
                    "0n" | "-0n" => Self::FalsyButNotNullish,
                    _ => Self::Truthy,
                },
                Literal::Boolean(boolean) => match boolean.as_bool() {
                    true => Self::Truthy,
                    false => Self::FalsyButNotNullish,
                },
                Literal::Number(number) => match number.to_f64() {
                    Some(n) if n == 0. || n.is_nan() => Self::FalsyButNotNullish,
                    Some(_n) => Self::Truthy,
                    None => Self::Anything,
                },
                Literal::Object(_) | Literal::RegExp(_) => Self::Truthy,
                Literal::String(string) => match string.as_str().is_empty() {
                    true => Self::FalsyButNotNullish,
                    false => Self::Truthy,
                },
                Literal::Template(_) => Self::Anything,
            }),
            TypeData::Null | TypeData::Undefined | TypeData::VoidKeyword => Some(Self::Nullish),
            TypeData::Generic(_)
            | TypeData::TypeOperator(_)
            | TypeData::TypeofExpression(_)
            | TypeData::TypeofType(_)
            | TypeData::TypeofValue(_) => Some(Self::Unknown),
        }
    }

    /// Returns `true` if this type is known to represent a falsy value.
    ///
    /// Returns `false` otherwise.
    pub fn is_falsy(self) -> bool {
        matches!(self, Self::Falsy | Self::FalsyButNotNullish | Self::Nullish)
    }

    /// Returns whether this conditionally has been semantically inferred.
    pub fn is_inferred(self) -> bool {
        !matches!(self, Self::Unknown)
    }

    /// Returns `true` if this type is known to be anything except `null` or
    /// `undefined`.
    ///
    /// Returns `false` otherwise.
    pub fn is_non_nullish(self) -> bool {
        matches!(
            self,
            Self::FalsyButNotNullish | Self::NonNullish | Self::Truthy
        )
    }

    /// Returns `true` if this type is known to be `null` or `undefined`.
    ///
    /// Returns `false` otherwise.
    pub fn is_nullish(self) -> bool {
        matches!(self, Self::Nullish)
    }

    /// Returns `true` if this type is known to represent a truthy value.
    ///
    /// Returns `false` otherwise.
    pub fn is_truthy(self) -> bool {
        matches!(self, Self::Truthy)
    }

    /// Returns whether this conditional type is semantically mergeable.
    ///
    /// Some conditional types always erase the semantic meaning of whichever
    /// other conditionals they are merged with, so there would be no point in
    /// merging further.
    fn is_mergeable(self) -> bool {
        !matches!(self, Self::Anything | Self::Unknown)
    }

    /// Merged this conditional with another.
    ///
    /// This attempts to preserve as much semantic information of both types as
    /// we can.
    fn merged_with(self, other: Self) -> Self {
        match (self, other) {
            (Self::Anything, _)
            | (_, Self::Anything)
            | (Self::Falsy | Self::Nullish, Self::NonNullish)
            | (Self::Falsy | Self::FalsyButNotNullish | Self::Nullish, Self::Truthy)
            | (Self::NonNullish, Self::Falsy | Self::Nullish)
            | (Self::Truthy, Self::Falsy | Self::FalsyButNotNullish | Self::Nullish) => {
                Self::Anything
            }
            (Self::Falsy, Self::Falsy | Self::FalsyButNotNullish | Self::Nullish)
            | (Self::FalsyButNotNullish | Self::Nullish, Self::Falsy)
            | (Self::FalsyButNotNullish, Self::Nullish)
            | (Self::Nullish, Self::FalsyButNotNullish) => Self::Falsy,
            (Self::FalsyButNotNullish, Self::FalsyButNotNullish) => Self::FalsyButNotNullish,
            (Self::NonNullish, Self::FalsyButNotNullish | Self::NonNullish | Self::Truthy)
            | (Self::FalsyButNotNullish | Self::Truthy, Self::NonNullish) => Self::NonNullish,
            (Self::Nullish, Self::Nullish) => Self::Nullish,
            (Self::Truthy, Self::Truthy) => Self::Truthy,
            (Self::Unknown, _) | (_, Self::Unknown) => Self::Unknown,
        }
    }
}

/// Determines the smallest subset of the type `ty` for which we know that it
/// _may_ evaluate to a falsy value, and returns a reference to that type.
///
/// Returns `None` if the type cannot be made any more specific.
pub fn reference_to_falsy_subset_of(
    ty: &TypeData,
    resolver: &mut dyn TypeResolver,
) -> Option<TypeReference> {
    let filter = |ty: &TypeData| match ty {
        TypeData::BigInt => FilteredData::Mapped(Literal::BigInt(Text::new_static("0n")).into()),
        TypeData::Boolean => FilteredData::Mapped(Literal::Boolean(false.into()).into()),
        TypeData::Number => {
            FilteredData::Mapped(Literal::Number(NumberLiteral::new(Text::new_static("0"))).into())
        }
        TypeData::String => {
            FilteredData::Mapped(Literal::String(Text::new_static("").into()).into())
        }
        other => {
            if ConditionalType::from_data_shallow(other)
                .is_none_or(|conditional| !conditional.is_truthy())
            {
                FilteredData::Retained
            } else {
                FilteredData::Stripped
            }
        }
    };

    let ty = to_filtered_value(ty, &filter, resolver, 0)?;
    Some(resolver.reference_to_owned_data(ty))
}

/// Determines the smallest subset of the type `ty` for which we know that it
/// _may_ evaluate to a non-nullish value, and returns a reference to that type.
///
/// Returns `None` if the type cannot be made any more specific.
pub fn reference_to_non_nullish_subset_of(
    ty: &TypeData,
    resolver: &mut dyn TypeResolver,
) -> Option<TypeReference> {
    let filter = |ty: &TypeData| {
        if ConditionalType::from_data_shallow(ty)
            .is_none_or(|conditional| !conditional.is_nullish())
        {
            FilteredData::Retained
        } else {
            FilteredData::Stripped
        }
    };

    let ty = to_filtered_value(ty, &filter, resolver, 0)?;
    Some(resolver.reference_to_owned_data(ty))
}

/// Determines the smallest subset of the type `ty` for which we know that it
/// _may_ evaluate to a truthy value, and returns a reference to that type.
///
/// Returns `None` if the type cannot be made any more specific.
pub fn reference_to_truthy_subset_of(
    ty: &TypeData,
    resolver: &mut dyn TypeResolver,
) -> Option<TypeReference> {
    let filter = |ty: &TypeData| match ty {
        TypeData::Boolean => FilteredData::Mapped(Literal::Boolean(true.into()).into()),
        other => {
            if ConditionalType::from_data_shallow(other)
                .is_none_or(|conditional| !conditional.is_falsy())
            {
                FilteredData::Retained
            } else {
                FilteredData::Stripped
            }
        }
    };

    let ty = to_filtered_value(ty, &filter, resolver, 0)?;
    Some(resolver.reference_to_owned_data(ty))
}

enum FilteredData {
    Mapped(TypeData),
    Retained,
    Stripped,
}

fn to_filtered_value(
    resolved: &TypeData,
    filter: &impl Fn(&TypeData) -> FilteredData,
    resolver: &mut dyn TypeResolver,
    mut depth: usize,
) -> Option<TypeData> {
    depth += 1;
    if depth > MAX_FLATTEN_DEPTH {
        return None;
    }

    let mut reference_to_non_nullish_value = |reference: &TypeReference| -> Option<TypeData> {
        resolver
            .resolve_and_get(reference)
            .map(ResolvedTypeData::to_data)
            .and_then(|ty| to_filtered_value(&ty, filter, resolver, depth))
    };

    match filter(resolved) {
        FilteredData::Mapped(ty) => Some(ty),
        FilteredData::Retained => match resolved {
            TypeData::InstanceOf(instance) => match resolver.resolve_and_get(&instance.ty) {
                Some(resolved) if resolved.should_flatten_instance(instance) => {
                    to_filtered_value(&resolved.to_data(), filter, resolver, depth)
                }
                _ => None,
            },
            TypeData::MergedReference(reference) => {
                let ty = reference.ty.as_ref().and_then(|reference| {
                    let ty = resolver
                        .resolve_and_get(reference)
                        .map(ResolvedTypeData::to_data)
                        .and_then(|ty| to_filtered_value(&ty, filter, resolver, depth))?;
                    Some(resolver.reference_to_owned_data(ty))
                });
                let value_ty = reference.value_ty.as_ref().and_then(|reference| {
                    let ty = resolver
                        .resolve_and_get(reference)
                        .map(ResolvedTypeData::to_data)
                        .and_then(|ty| to_filtered_value(&ty, filter, resolver, depth))?;
                    Some(resolver.reference_to_owned_data(ty))
                });
                let namespace_ty = reference.namespace_ty.as_ref().and_then(|reference| {
                    let ty = resolver
                        .resolve_and_get(reference)
                        .map(ResolvedTypeData::to_data)
                        .and_then(|ty| to_filtered_value(&ty, filter, resolver, depth))?;
                    Some(resolver.reference_to_owned_data(ty))
                });
                match (ty, value_ty, namespace_ty) {
                    (None, None, None) => None,
                    (Some(reference), None, None)
                    | (None, Some(reference), None)
                    | (None, None, Some(reference)) => Some(TypeData::Reference(reference)),
                    (ty, value_ty, namespace_ty) => Some(TypeData::from(MergedReference {
                        ty,
                        value_ty,
                        namespace_ty,
                    })),
                }
            }
            TypeData::Reference(reference) => reference_to_non_nullish_value(reference),
            TypeData::Union(_) => {
                let types: Vec<_> = resolved.flattened_union_variants(resolver).collect();
                let types = types
                    .iter()
                    .filter_map(|reference| {
                        let ty = resolver
                            .resolve_and_get(reference)
                            .map(ResolvedTypeData::to_data)
                            .and_then(|ty| to_filtered_value(&ty, filter, resolver, depth))?;
                        Some(resolver.reference_to_owned_data(ty))
                    })
                    .collect();
                Some(TypeData::union_of(resolver, types))
            }
            _ => Some(resolved.clone()),
        },
        FilteredData::Stripped => None,
    }
}

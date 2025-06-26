use std::{
    borrow::Cow,
    cmp::Ordering,
    hash::{Hash, Hasher},
};

use biome_rowan::Text;
use hashbrown::{HashTable, hash_table::Entry};
use rustc_hash::FxHasher;

use crate::{
    BindingId, Class, Interface, Literal, MAX_FLATTEN_DEPTH, MergedReference, Module, Namespace,
    Object, Resolvable, ResolvedTypeData, ResolvedTypeId, ResolvedTypeMember, ResolverId, Type,
    TypeData, TypeInstance, TypeMember, TypeReference, TypeResolver, Union,
    globals::{GLOBAL_ARRAY_ID, GLOBAL_PROMISE_ID, GLOBAL_TYPE_MEMBERS},
};

impl<'a> ResolvedTypeData<'a> {
    /// Iterates all member fields, including those belonging to extended
    /// classes or prototype objects.
    ///
    /// Note that members which are inherited and overridden may appear multiple
    /// times, but the member that is closest to the current type is guaranteed
    /// to come first.
    pub fn all_members(self, resolver: &'a dyn TypeResolver) -> AllTypeMemberIterator<'a> {
        AllTypeMemberIterator {
            resolver,
            resolver_id: self.resolver_id(),
            owner: TypeMemberOwner::from_type_data(self.as_raw_data()),
            seen_types: Vec::new(),
            index: 0,
            excluded_binding_id: None,
        }
    }

    /// Returns the type of an array's elements, if this object is an instance of `Array`.
    pub fn find_array_element_type(self, resolver: &'a dyn TypeResolver) -> Option<Self> {
        if self.is_instance_of(resolver, GLOBAL_ARRAY_ID) {
            self.get_type_parameter(0)
                .and_then(|reference| resolver.resolve_and_get(&reference))
        } else {
            None
        }
    }

    /// Returns the promised type, if this object is an instance of `Promise`.
    pub fn find_promise_type(self, resolver: &'a dyn TypeResolver) -> Option<Self> {
        if self.is_instance_of(resolver, GLOBAL_PROMISE_ID) {
            self.get_type_parameter(0)
                .and_then(|reference| resolver.resolve_and_get(&reference))
        } else {
            None
        }
    }

    pub fn get_type_parameter(self, index: usize) -> Option<Cow<'a, TypeReference>> {
        self.as_raw_data()
            .type_parameters()
            .and_then(|params| params.get(index))
            .map(|param| self.apply_module_id_to_reference(param))
    }

    pub fn has_members(self) -> bool {
        TypeMemberOwner::from_type_data(self.as_raw_data()).is_some()
    }

    #[inline]
    pub fn is_inferred(self) -> bool {
        self.as_raw_data().is_inferred()
    }

    /// Returns whether this type data is an instance of a type matching the
    /// given `predicate`.
    pub fn is_instance_matching(
        self,
        resolver: &'a dyn TypeResolver,
        predicate: impl Fn(TypeInstance) -> bool,
    ) -> bool {
        let apply_predicate = |owner: Self, instance: &TypeInstance| {
            let mut instance = instance.clone();
            instance.update_all_references(|reference| {
                if let Cow::Owned(updated_reference) = owner.apply_module_id_to_reference(reference)
                {
                    *reference = updated_reference;
                }
            });
            predicate(instance)
        };

        match self.as_raw_data() {
            TypeData::InstanceOf(instance) => apply_predicate(self, instance),
            TypeData::Reference(TypeReference::Resolved(resolved_id)) => resolver
                .get_by_resolved_id(self.apply_module_id(*resolved_id))
                .is_some_and(|resolved_data| match resolved_data.as_raw_data() {
                    TypeData::InstanceOf(instance) => apply_predicate(resolved_data, instance),
                    _ => false,
                }),
            _ => false,
        }
    }

    pub fn is_instance_of(self, resolver: &dyn TypeResolver, id: ResolvedTypeId) -> bool {
        let mut seen_types = Vec::new();
        let mut current_object = Some(self);
        while let Some(current) = current_object {
            let Some(prototype) = current.prototype(resolver) else {
                match current.as_raw_data() {
                    TypeData::Reference(TypeReference::Resolved(resolved_id)) => {
                        return *resolved_id == id;
                    }
                    _ => break,
                }
            };

            let Some(next_id) = resolver.resolve_reference(&prototype) else {
                break;
            };

            if next_id == id {
                return true;
            }

            if seen_types.contains(&next_id) {
                break;
            }

            seen_types.push(next_id);
            current_object = resolver.get_by_resolved_id(next_id);
        }

        false
    }

    /// Returns whether this type is an instance of a `Promise`.
    pub fn is_promise_instance(self, resolver: &dyn TypeResolver) -> bool {
        self.is_instance_of(resolver, GLOBAL_PROMISE_ID)
    }

    /// Returns a reference to the type's prototype, if any.
    pub fn prototype(self, resolver: &'a dyn TypeResolver) -> Option<Cow<'a, TypeReference>> {
        match self.as_raw_data() {
            TypeData::InstanceOf(instance_of) => {
                Some(self.apply_module_id_to_reference(&instance_of.ty))
            }
            TypeData::Object(object) => object
                .prototype
                .as_ref()
                .map(|reference| self.apply_module_id_to_reference(reference)),
            TypeData::Reference(reference) => resolver
                .resolve_and_get(&self.apply_module_id_to_reference(reference))
                .and_then(|ty| ty.prototype(resolver)),
            _ => None,
        }
    }
}

impl TypeData {
    /// Returns the type of an element at a given index, if this object is an
    /// array or a tuple.
    pub fn find_element_type_at_index<'a>(
        &'a self,
        resolver_id: ResolverId,
        resolver: &'a mut dyn TypeResolver,
        index: usize,
    ) -> Option<ResolvedTypeData<'a>> {
        match self {
            Self::Tuple(tuple) => Some(tuple.get_ty(resolver, index)),
            _ => {
                let resolved = ResolvedTypeData::from((resolver_id, self));
                if resolved.is_instance_of(resolver, GLOBAL_ARRAY_ID) {
                    resolved
                        .get_type_parameter(0)
                        .map(|reference| reference.into_owned())
                        .map(|reference| resolver.optional(reference))
                        .map(|id| {
                            ResolvedTypeData::from((
                                ResolvedTypeId::new(resolver.level(), id),
                                resolver.get_by_id(id),
                            ))
                        })
                } else {
                    None
                }
            }
        }
    }

    /// Returns the type of elements from a given index, if this object is an
    /// array or a tuple.
    pub fn find_type_of_elements_from_index<'a>(
        &'a self,
        resolver_id: ResolverId,
        resolver: &'a mut dyn TypeResolver,
        index: usize,
    ) -> Option<ResolvedTypeData<'a>> {
        let data = match self {
            Self::Tuple(tuple) => Some(Self::Tuple(Box::new(tuple.slice_from(index)))),
            _ => {
                let resolved = ResolvedTypeData::from((resolver_id, self));
                if resolved.is_instance_of(resolver, GLOBAL_ARRAY_ID) {
                    match resolved.get_type_parameter(0) {
                        Some(elem_ty) => Some(Self::instance_of(TypeInstance {
                            ty: GLOBAL_ARRAY_ID.into(),
                            type_parameters: Box::new([elem_ty.into_owned()]),
                        })),
                        None => return resolver.get_by_resolved_id(GLOBAL_ARRAY_ID),
                    }
                } else {
                    None
                }
            }
        }?;

        let id = resolver.register_and_resolve(data);
        resolver.get_by_resolved_id(id)
    }

    /// Turns this [`TypeData`] into an instance of itself.
    pub fn into_instance(self, resolver: &mut dyn TypeResolver) -> Self {
        match self {
            Self::InstanceOf(instance) => Self::InstanceOf(instance),
            Self::Reference(reference) => Self::instance_of(reference),
            other => Self::instance_of(TypeReference::from(resolver.register_and_resolve(other))),
        }
    }

    /// Returns whether this data is known to have a falsy value.
    pub fn is_falsy(&self, resolver: &dyn TypeResolver) -> bool {
        fn is_falsy(ty: &TypeData, resolver: &dyn TypeResolver, mut depth: usize) -> bool {
            depth += 1;
            if depth > MAX_FLATTEN_DEPTH {
                return false;
            }

            let is_falsy_reference = |reference: &TypeReference| -> bool {
                resolver
                    .resolve_and_get(reference)
                    .is_some_and(|ty| is_falsy(&ty.to_data(), resolver, depth))
            };

            match ty {
                TypeData::Null | TypeData::Undefined | TypeData::VoidKeyword => true,
                TypeData::AnyKeyword
                | TypeData::BigInt
                | TypeData::Boolean
                | TypeData::Class(_)
                | TypeData::Conditional
                | TypeData::Constructor(_)
                | TypeData::Function(_)
                | TypeData::Generic(_)
                | TypeData::Global
                | TypeData::Module(_)
                | TypeData::Namespace(_)
                | TypeData::ImportNamespace(_)
                | TypeData::Interface(_)
                | TypeData::NeverKeyword
                | TypeData::Number
                | TypeData::ObjectKeyword
                | TypeData::Object(_)
                | TypeData::String
                | TypeData::Symbol
                | TypeData::ThisKeyword
                | TypeData::Tuple(_)
                | TypeData::TypeOperator(_)
                | TypeData::TypeofExpression(_)
                | TypeData::TypeofType(_)
                | TypeData::TypeofValue(_)
                | TypeData::Unknown
                | TypeData::UnknownKeyword => false,
                TypeData::Literal(literal) => match literal.as_ref() {
                    Literal::BigInt(text) => text.text() == "0n" || text.text() == "-0n",
                    Literal::Boolean(boolean) => !boolean.as_bool(),
                    Literal::Number(number) => {
                        number.to_f64().is_some_and(|n| n == 0. || n.is_nan())
                    }
                    Literal::String(string) => string.as_str().is_empty(),
                    Literal::Object(_) | Literal::RegExp(_) | Literal::Template(_) => false,
                },
                TypeData::InstanceOf(instance) => is_falsy_reference(&instance.ty),
                TypeData::Intersection(intersection) => {
                    intersection.types().iter().all(is_falsy_reference)
                }
                TypeData::MergedReference(reference) => {
                    reference.ty.as_ref().is_none_or(is_falsy_reference)
                        && reference.ty.as_ref().is_none_or(is_falsy_reference)
                        && reference.ty.as_ref().is_none_or(is_falsy_reference)
                }
                TypeData::Reference(reference) => is_falsy_reference(reference),
                TypeData::Union(union) => union.types().iter().all(is_falsy_reference),
            }
        }
        is_falsy(self, resolver, 0)
    }

    /// Returns whether this data is known to be neither `null` nor `undefined`.
    pub fn is_non_nullish(&self, resolver: &dyn TypeResolver) -> bool {
        fn is_non_nullish(ty: &TypeData, resolver: &dyn TypeResolver, mut depth: usize) -> bool {
            depth += 1;
            if depth > MAX_FLATTEN_DEPTH {
                return false;
            }

            let is_non_nullish_reference = |reference: &TypeReference| -> bool {
                resolver
                    .resolve_and_get(reference)
                    .is_some_and(|ty| is_non_nullish(&ty.to_data(), resolver, depth))
            };

            match ty {
                TypeData::BigInt
                | TypeData::Boolean
                | TypeData::Global
                | TypeData::Number
                | TypeData::String
                | TypeData::Symbol
                | TypeData::ImportNamespace(_)
                | TypeData::Class(_)
                | TypeData::Constructor(_)
                | TypeData::Function(_)
                | TypeData::Interface(_)
                | TypeData::Module(_)
                | TypeData::Namespace(_)
                | TypeData::Object(_)
                | TypeData::Tuple(_)
                | TypeData::Literal(_)
                | TypeData::ObjectKeyword
                | TypeData::ThisKeyword => true,
                TypeData::AnyKeyword
                | TypeData::Conditional
                | TypeData::Generic(_)
                | TypeData::NeverKeyword
                | TypeData::Null
                | TypeData::TypeOperator(_)
                | TypeData::TypeofExpression(_)
                | TypeData::TypeofType(_)
                | TypeData::TypeofValue(_)
                | TypeData::Undefined
                | TypeData::Unknown
                | TypeData::UnknownKeyword
                | TypeData::VoidKeyword => false,
                TypeData::InstanceOf(instance) => is_non_nullish_reference(&instance.ty),
                TypeData::Intersection(intersection) => {
                    intersection.types().iter().all(is_non_nullish_reference)
                }
                TypeData::MergedReference(reference) => {
                    reference.ty.as_ref().is_none_or(is_non_nullish_reference)
                        && reference.ty.as_ref().is_none_or(is_non_nullish_reference)
                        && reference.ty.as_ref().is_none_or(is_non_nullish_reference)
                }
                TypeData::Reference(reference) => is_non_nullish_reference(reference),
                TypeData::Union(union) => union.types().iter().all(is_non_nullish_reference),
            }
        }
        is_non_nullish(self, resolver, 0)
    }

    /// Returns whether this data is known to have a truthy value.
    pub fn is_truthy(&self, resolver: &dyn TypeResolver) -> bool {
        fn is_truthy(ty: &TypeData, resolver: &dyn TypeResolver, mut depth: usize) -> bool {
            depth += 1;
            if depth > MAX_FLATTEN_DEPTH {
                return false;
            }

            let is_truthy_reference = |reference: &TypeReference| -> bool {
                resolver
                    .resolve_and_get(reference)
                    .is_some_and(|ty| is_truthy(&ty.to_data(), resolver, depth))
            };

            match ty {
                TypeData::Global
                | TypeData::Symbol
                | TypeData::ImportNamespace(_)
                | TypeData::Class(_)
                | TypeData::Constructor(_)
                | TypeData::Function(_)
                | TypeData::Module(_)
                | TypeData::Namespace(_)
                | TypeData::Object(_)
                | TypeData::Tuple(_)
                | TypeData::ObjectKeyword
                | TypeData::ThisKeyword => true,
                TypeData::AnyKeyword
                | TypeData::BigInt
                | TypeData::Boolean
                | TypeData::Conditional
                | TypeData::Generic(_)
                | TypeData::Interface(_)
                | TypeData::NeverKeyword
                | TypeData::Null
                | TypeData::Number
                | TypeData::String
                | TypeData::TypeOperator(_)
                | TypeData::TypeofExpression(_)
                | TypeData::TypeofType(_)
                | TypeData::TypeofValue(_)
                | TypeData::Undefined
                | TypeData::Unknown
                | TypeData::UnknownKeyword
                | TypeData::VoidKeyword => false,
                TypeData::Literal(literal) => match literal.as_ref() {
                    Literal::BigInt(text) => text.text() != "0n" && text.text() != "-0n",
                    Literal::Boolean(boolean) => boolean.as_bool(),
                    Literal::Number(number) => {
                        number.to_f64().is_some_and(|n| n != 0. && !n.is_nan())
                    }
                    Literal::Object(_) | Literal::RegExp(_) => true,
                    Literal::String(string) => !string.as_str().is_empty(),
                    Literal::Template(_) => false,
                },
                TypeData::InstanceOf(instance) => is_truthy_reference(&instance.ty),
                TypeData::Intersection(intersection) => {
                    intersection.types().iter().all(is_truthy_reference)
                }
                TypeData::MergedReference(reference) => {
                    reference.ty.as_ref().is_none_or(is_truthy_reference)
                        && reference.ty.as_ref().is_none_or(is_truthy_reference)
                        && reference.ty.as_ref().is_none_or(is_truthy_reference)
                }
                TypeData::Reference(reference) => is_truthy_reference(reference),
                TypeData::Union(union) => union.types().iter().all(is_truthy_reference),
            }
        }
        is_truthy(self, resolver, 0)
    }

    /// Iterates own member fields.
    ///
    /// This iterator does not return members of [`TypeData::InstanceOf`] or
    /// [`TypeData::Reference`] variants. If that's what you want, you will need
    /// to dereference them first.
    pub fn own_members(&self) -> OwnTypeMemberIterator {
        OwnTypeMemberIterator {
            owner: TypeMemberOwner::from_type_data(self),
            index: 0,
        }
    }

    /// Converts the type data to a value that is falsy.
    ///
    /// If the data itself may hold both truthy and falsy values, new data may
    /// be returned that can be falsy only.
    ///
    /// Returns `None` if the value is known to be always truthy.
    pub fn to_falsy_value(self, resolver: &mut dyn TypeResolver) -> Option<Self> {
        fn to_falsy_value(
            ty: TypeData,
            resolver: &mut dyn TypeResolver,
            mut depth: usize,
        ) -> Option<TypeData> {
            depth += 1;
            if depth > MAX_FLATTEN_DEPTH {
                return Some(ty);
            }

            let mut reference_to_falsy_value = |reference: &TypeReference| -> Option<TypeData> {
                resolver
                    .resolve_and_get(reference)
                    .map(ResolvedTypeData::to_data)
                    .and_then(|ty| to_falsy_value(ty, resolver, depth))
            };

            let new_ty = match &ty {
                TypeData::BigInt => Literal::BigInt(Text::Static("0n")).into(),
                TypeData::Boolean => Literal::Boolean(false.into()).into(),
                TypeData::Conditional => TypeData::Unknown,
                TypeData::InstanceOf(instance) => match resolver.resolve_and_get(&instance.ty) {
                    Some(resolved) if resolved.should_flatten_instance(instance) => {
                        return to_falsy_value(resolved.to_data(), resolver, depth);
                    }
                    _ => ty,
                },
                TypeData::Literal(literal) => match literal.as_ref() {
                    Literal::BigInt(text) => match text.text() {
                        "0n" | "-0n" => ty,
                        _ => return None,
                    },
                    Literal::Boolean(boolean) => match boolean.as_bool() {
                        false => ty,
                        true => return None,
                    },
                    Literal::Number(number) => match number.to_f64() {
                        Some(0.) => ty,
                        Some(n) if n.is_nan() => ty,
                        _ => return None,
                    },
                    Literal::Object(_) | Literal::RegExp(_) => return None,
                    Literal::String(string) => match string.as_str() {
                        "" => ty,
                        _ => return None,
                    },
                    Literal::Template(_) => ty,
                },
                TypeData::MergedReference(reference) => {
                    let ty = reference.ty.as_ref().and_then(|reference| {
                        let ty = resolver
                            .resolve_and_get(reference)
                            .map(ResolvedTypeData::to_data)
                            .and_then(|ty| to_falsy_value(ty, resolver, depth))?;
                        Some(resolver.reference_to_owned_data(ty))
                    });
                    let value_ty = reference.value_ty.as_ref().and_then(|reference| {
                        let ty = resolver
                            .resolve_and_get(reference)
                            .map(ResolvedTypeData::to_data)
                            .and_then(|ty| to_falsy_value(ty, resolver, depth))?;
                        Some(resolver.reference_to_owned_data(ty))
                    });
                    let namespace_ty = reference.namespace_ty.as_ref().and_then(|reference| {
                        let ty = resolver
                            .resolve_and_get(reference)
                            .map(ResolvedTypeData::to_data)
                            .and_then(|ty| to_falsy_value(ty, resolver, depth))?;
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
                    }?
                }
                TypeData::Reference(reference) => reference_to_falsy_value(reference)?,
                TypeData::String => Literal::String(Text::Static("").into()).into(),
                TypeData::Union(union) => {
                    let types = union
                        .types()
                        .iter()
                        .filter_map(|reference| {
                            let ty = resolver
                                .resolve_and_get(reference)
                                .map(ResolvedTypeData::to_data)
                                .and_then(|ty| to_falsy_value(ty, resolver, depth))?;
                            Some(resolver.reference_to_owned_data(ty))
                        })
                        .collect();
                    TypeData::union_of(resolver, types)
                }
                TypeData::Class(_)
                | TypeData::Constructor(_)
                | TypeData::Function(_)
                | TypeData::Global
                | TypeData::ImportNamespace(_)
                | TypeData::Module(_)
                | TypeData::Namespace(_)
                | TypeData::Object(_)
                | TypeData::Symbol
                | TypeData::Tuple(_) => return None,
                _unchanged => ty,
            };

            Some(new_ty)
        }

        to_falsy_value(self, resolver, 0)
    }

    /// Converts the type data to a value that is not `null` or `undefined`.
    ///
    /// If the data itself may be either nullish or non-nullish, new data may be
    /// returned that can be non-nullish only.
    ///
    /// Returns `None` if the value is known to be always nullish.
    pub fn to_non_nullish_value(self, resolver: &mut dyn TypeResolver) -> Option<Self> {
        fn to_non_nullish_value(
            ty: TypeData,
            resolver: &mut dyn TypeResolver,
            mut depth: usize,
        ) -> Option<TypeData> {
            depth += 1;
            if depth > MAX_FLATTEN_DEPTH {
                return Some(ty);
            }

            let mut reference_to_non_nullish_value =
                |reference: &TypeReference| -> Option<TypeData> {
                    resolver
                        .resolve_and_get(reference)
                        .map(ResolvedTypeData::to_data)
                        .and_then(|ty| to_non_nullish_value(ty, resolver, depth))
                };

            let new_ty = match &ty {
                TypeData::Null | TypeData::Undefined => return None,
                TypeData::InstanceOf(instance) => match resolver.resolve_and_get(&instance.ty) {
                    Some(resolved) if resolved.should_flatten_instance(instance) => {
                        return to_non_nullish_value(resolved.to_data(), resolver, depth);
                    }
                    _ => ty,
                },
                TypeData::MergedReference(reference) => {
                    let ty = reference.ty.as_ref().and_then(|reference| {
                        let ty = resolver
                            .resolve_and_get(reference)
                            .map(ResolvedTypeData::to_data)
                            .and_then(|ty| to_non_nullish_value(ty, resolver, depth))?;
                        Some(resolver.reference_to_owned_data(ty))
                    });
                    let value_ty = reference.value_ty.as_ref().and_then(|reference| {
                        let ty = resolver
                            .resolve_and_get(reference)
                            .map(ResolvedTypeData::to_data)
                            .and_then(|ty| to_non_nullish_value(ty, resolver, depth))?;
                        Some(resolver.reference_to_owned_data(ty))
                    });
                    let namespace_ty = reference.namespace_ty.as_ref().and_then(|reference| {
                        let ty = resolver
                            .resolve_and_get(reference)
                            .map(ResolvedTypeData::to_data)
                            .and_then(|ty| to_non_nullish_value(ty, resolver, depth))?;
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
                    }?
                }
                TypeData::Reference(reference) => reference_to_non_nullish_value(reference)?,
                TypeData::Union(union) => {
                    let types = union
                        .types()
                        .iter()
                        .filter_map(|reference| {
                            let ty = resolver
                                .resolve_and_get(reference)
                                .map(ResolvedTypeData::to_data)
                                .and_then(|ty| to_non_nullish_value(ty, resolver, depth))?;
                            Some(resolver.reference_to_owned_data(ty))
                        })
                        .collect();
                    TypeData::union_of(resolver, types)
                }
                _unchanged => ty,
            };

            Some(new_ty)
        }

        to_non_nullish_value(self, resolver, 0)
    }

    /// Converts the type data to a value that is truthy.
    ///
    /// If the data itself may hold both truthy and falsy values, new data may
    /// be returned that can be truthy only.
    ///
    /// Returns `None` if the value is known to be always falsy.
    pub fn to_truthy_value(self, resolver: &mut dyn TypeResolver) -> Option<Self> {
        fn to_truthy_value(
            ty: TypeData,
            resolver: &mut dyn TypeResolver,
            mut depth: usize,
        ) -> Option<TypeData> {
            depth += 1;
            if depth > MAX_FLATTEN_DEPTH {
                return Some(ty);
            }

            let mut reference_to_truthy_value = |reference: &TypeReference| -> Option<TypeData> {
                resolver
                    .resolve_and_get(reference)
                    .map(ResolvedTypeData::to_data)
                    .and_then(|ty| to_truthy_value(ty, resolver, depth))
            };

            let new_data = match &ty {
                TypeData::Boolean => Literal::Boolean(true.into()).into(),
                TypeData::Conditional => TypeData::Unknown,
                TypeData::InstanceOf(instance) => match resolver.resolve_and_get(&instance.ty) {
                    Some(resolved) if resolved.should_flatten_instance(instance) => {
                        return to_truthy_value(resolved.to_data(), resolver, depth);
                    }
                    _ => ty,
                },
                TypeData::Literal(literal) => match literal.as_ref() {
                    Literal::BigInt(text) => match text.text() {
                        "0n" | "-0n" => return None,
                        _ => ty,
                    },
                    Literal::Boolean(boolean) => match boolean.as_bool() {
                        false => return None,
                        true => ty,
                    },
                    Literal::Number(number) => match number.to_f64() {
                        Some(0.) => return None,
                        Some(n) if n.is_nan() => return None,
                        _ => ty,
                    },
                    Literal::Object(_) | Literal::RegExp(_) => return None,
                    Literal::String(string) => match string.as_str() {
                        "" => return None,
                        _ => ty,
                    },
                    Literal::Template(_) => ty,
                },
                TypeData::MergedReference(reference) => {
                    let ty = reference.ty.as_ref().and_then(|reference| {
                        let ty = resolver
                            .resolve_and_get(reference)
                            .map(ResolvedTypeData::to_data)
                            .and_then(|ty| to_truthy_value(ty, resolver, depth))?;
                        Some(resolver.reference_to_owned_data(ty))
                    });
                    let value_ty = reference.value_ty.as_ref().and_then(|reference| {
                        let ty = resolver
                            .resolve_and_get(reference)
                            .map(ResolvedTypeData::to_data)
                            .and_then(|ty| to_truthy_value(ty, resolver, depth))?;
                        Some(resolver.reference_to_owned_data(ty))
                    });
                    let namespace_ty = reference.namespace_ty.as_ref().and_then(|reference| {
                        let ty = resolver
                            .resolve_and_get(reference)
                            .map(ResolvedTypeData::to_data)
                            .and_then(|ty| to_truthy_value(ty, resolver, depth))?;
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
                    }?
                }
                TypeData::Null | TypeData::Undefined | TypeData::VoidKeyword => return None,
                TypeData::Reference(reference) => reference_to_truthy_value(reference)?,
                TypeData::Union(union) => {
                    let types = union
                        .types()
                        .iter()
                        .filter_map(|reference| {
                            let ty = resolver
                                .resolve_and_get(reference)
                                .map(ResolvedTypeData::to_data)
                                .and_then(|ty| to_truthy_value(ty, resolver, depth))?;
                            Some(resolver.reference_to_owned_data(ty))
                        })
                        .collect();
                    TypeData::union_of(resolver, types)
                }
                _unchanged => ty,
            };

            Some(new_data)
        }

        to_truthy_value(self, resolver, 0)
    }

    /// Creates a union of type references.
    ///
    /// References are automatically deduplicated. If only a single type
    /// remains, an instance of `Self::Reference` is returned instead of
    /// `Self::Union`.
    pub fn union_of(resolver: &dyn TypeResolver, types: Box<[TypeReference]>) -> Self {
        // We use a hash table separately of a vector to quickly check for
        // duplicates, without messing with the original order.
        let mut table: HashTable<usize> = HashTable::with_capacity(types.len());
        let mut vec = Vec::with_capacity(types.len());
        for ty in types {
            if let Some(resolved) = resolver.resolve_and_get(&ty) {
                match resolved.as_raw_data() {
                    Self::AnyKeyword => {
                        // `any` poisons the entire union.
                        return Self::AnyKeyword;
                    }
                    Self::NeverKeyword => {
                        // No point in adding `never` to the union.
                        continue;
                    }
                    Self::Union(union) => {
                        // Flatten existing union into the new one:
                        for ty in union.types() {
                            let entry = table.entry(
                                hash_reference(ty),
                                |i| &vec[*i] == ty,
                                |i| hash_reference(&vec[*i]),
                            );
                            if let Entry::Vacant(entry) = entry {
                                let index = vec.len();
                                vec.push(ty.clone());
                                entry.insert(index);
                            }
                        }
                    }
                    _ => {}
                }
            }

            let entry = table.entry(
                hash_reference(&ty),
                |i| vec[*i] == ty,
                |i| hash_reference(&vec[*i]),
            );
            if let Entry::Vacant(entry) = entry {
                let index = vec.len();
                vec.push(ty);
                entry.insert(index);
            }
        }

        match vec.len().cmp(&1) {
            Ordering::Greater => Self::Union(Box::new(Union(vec.into()))),
            Ordering::Equal => Self::reference(vec.remove(0)),
            Ordering::Less => Self::NeverKeyword,
        }
    }
}

#[inline(always)]
fn hash_reference(reference: &TypeReference) -> u64 {
    let mut hash = FxHasher::default();
    reference.hash(&mut hash);
    hash.finish()
}

pub struct AllTypeMemberIterator<'a> {
    resolver: &'a dyn TypeResolver,
    resolver_id: ResolverId,
    owner: Option<TypeMemberOwner<'a>>,
    seen_types: Vec<ResolvedTypeId>,
    index: usize,
    excluded_binding_id: Option<BindingId>,
}

impl AllTypeMemberIterator<'_> {
    pub fn with_excluded_binding_id(mut self, binding_id: BindingId) -> Self {
        self.excluded_binding_id = Some(binding_id);
        self
    }
}

impl<'a> Iterator for AllTypeMemberIterator<'a> {
    type Item = ResolvedTypeMember<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_reference = match &self.owner {
            Some(TypeMemberOwner::Class(class)) => {
                match (class.members.get(self.index), class.extends.as_ref()) {
                    (Some(member), _) => {
                        self.index += 1;
                        return Some((self.resolver_id, member).into());
                    }
                    (None, Some(extends)) => extends,
                    (None, None) => {
                        self.owner = None;
                        return None;
                    }
                }
            }
            Some(TypeMemberOwner::Global) => {
                if let Some(member) = GLOBAL_TYPE_MEMBERS.get(self.index) {
                    self.index += 1;
                    return Some((self.resolver_id, member).into());
                } else {
                    self.owner = None;
                    return None;
                }
            }
            Some(TypeMemberOwner::InstanceOf(instance_of)) => &instance_of.ty,
            Some(TypeMemberOwner::Interface(interface)) => {
                match interface.members.get(self.index) {
                    Some(member) => {
                        self.index += 1;
                        return Some((self.resolver_id, member).into());
                    }
                    None => {
                        self.owner = None;
                        return None;
                    }
                }
            }
            Some(TypeMemberOwner::Module(module)) => match module.members.get(self.index) {
                Some(member) => {
                    self.index += 1;
                    return Some((self.resolver_id, member).into());
                }
                None => {
                    self.owner = None;
                    return None;
                }
            },
            Some(TypeMemberOwner::Namespace(namespace)) => {
                match namespace.members.get(self.index) {
                    Some(member) => {
                        self.index += 1;
                        return Some((self.resolver_id, member).into());
                    }
                    None => {
                        self.owner = None;
                        return None;
                    }
                }
            }
            Some(TypeMemberOwner::Object(object)) => {
                match (object.members.get(self.index), object.prototype.as_ref()) {
                    (Some(member), _) => {
                        self.index += 1;
                        return Some((self.resolver_id, member).into());
                    }
                    (None, Some(prototype)) => prototype,
                    (None, None) => {
                        self.owner = None;
                        return None;
                    }
                }
            }
            None => return None,
        };

        // If there are any references in the inheritance chain, we need to keep
        // resolving them until we find an "actual" type again.
        let mut next_resolved_id = None;
        let mut next_reference = Cow::Borrowed(next_reference);
        while next_resolved_id.is_none() {
            if let TypeReference::Resolved(id) = next_reference.as_ref() {
                next_reference = Cow::Owned(TypeReference::Resolved(
                    self.resolver_id.apply_module_id(*id),
                ));
            }

            if let Some(excluded_binding_id) = self.excluded_binding_id {
                next_reference = Cow::Owned(
                    next_reference
                        .into_owned()
                        .with_excluded_binding_id(excluded_binding_id),
                );
            }

            let Some(resolved_id) = self.resolver.resolve_reference(&next_reference) else {
                break;
            };

            let Some(ty) = self.resolver.get_by_resolved_id(resolved_id) else {
                break;
            };

            match ty.as_raw_data() {
                TypeData::Reference(reference) => {
                    self.resolver_id = ty.resolver_id();
                    next_reference = Cow::Borrowed(reference);
                }
                _ => next_resolved_id = Some(resolved_id),
            }
        }

        let Some(next_resolved_id) = next_resolved_id else {
            self.owner = None;
            return None;
        };

        if self.seen_types.contains(&next_resolved_id) {
            self.owner = None;
            return None;
        }

        self.seen_types.push(next_resolved_id);

        let data = self.resolver.get_by_resolved_id(next_resolved_id);
        if let Some(data) = &data {
            self.owner = TypeMemberOwner::from_type_data(data.as_raw_data());
            self.resolver_id = data.resolver_id();
        } else {
            self.owner = None;
        }

        self.index = 0;
        self.next()
    }
}

pub struct OwnTypeMemberIterator<'a> {
    owner: Option<TypeMemberOwner<'a>>,
    index: usize,
}

impl<'a> Iterator for OwnTypeMemberIterator<'a> {
    type Item = &'a TypeMember;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match &self.owner {
            Some(TypeMemberOwner::Class(class)) => class.members.get(self.index),
            Some(TypeMemberOwner::Global) => GLOBAL_TYPE_MEMBERS.get(self.index),
            Some(TypeMemberOwner::Interface(interface)) => interface.members.get(self.index),
            Some(TypeMemberOwner::Module(module)) => module.members.get(self.index),
            Some(TypeMemberOwner::Namespace(namespace)) => namespace.members.get(self.index),
            Some(TypeMemberOwner::Object(object)) => object.members.get(self.index),
            None | Some(TypeMemberOwner::InstanceOf(_)) => None,
        };

        if next.is_some() {
            self.index += 1;
        } else {
            self.owner = None;
        }

        next
    }
}

#[derive(Debug)]
enum TypeMemberOwner<'a> {
    Class(&'a Class),
    Global,
    InstanceOf(&'a TypeInstance),
    Interface(&'a Interface),
    Module(&'a Module),
    Namespace(&'a Namespace),
    Object(&'a Object),
}

impl<'a> TypeMemberOwner<'a> {
    #[inline]
    fn from_type_data(type_data: &'a TypeData) -> Option<Self> {
        match type_data {
            TypeData::Class(class) => Some(Self::Class(class)),
            TypeData::Global => Some(Self::Global),
            TypeData::InstanceOf(type_instance) => Some(Self::InstanceOf(type_instance)),
            TypeData::Interface(interface) => Some(Self::Interface(interface)),
            TypeData::Module(module) => Some(Self::Module(module)),
            TypeData::Namespace(namespace) => Some(Self::Namespace(namespace)),
            TypeData::Object(object) => Some(Self::Object(object)),
            _ => None,
        }
    }
}

macro_rules! generate_resolved_matcher {
    ($name:ident) => {
        impl ResolvedTypeData<'_> {
            #[inline]
            pub fn $name(self) -> bool {
                self.as_raw_data().$name()
            }
        }
    };
}
macro_rules! generate_type_matcher {
    ($name:ident) => {
        impl Type {
            #[inline]
            pub fn $name(&self) -> bool {
                self.as_raw_data().is_some_and(TypeData::$name)
            }
        }
    };
}
macro_rules! generate_matcher {
    ($name:ident, $variant:ident) => {
        impl TypeData {
            #[inline]
            pub fn $name(&self) -> bool {
                matches!(self, Self::$variant)
            }
        }

        generate_resolved_matcher!($name);
        generate_type_matcher!($name);
    };
    ($name:ident, $variant:ident, $data:pat) => {
        impl TypeData {
            #[inline]
            pub fn $name(&self) -> bool {
                matches!(self, Self::$variant($data))
            }
        }

        generate_resolved_matcher!($name);
        generate_type_matcher!($name);
    };
}

generate_matcher!(is_any_keyword, AnyKeyword);
generate_matcher!(is_class, Class, _);
generate_matcher!(is_conditional, Conditional);
generate_matcher!(is_expression, TypeofExpression, _);
generate_matcher!(is_function, Function, _);
generate_matcher!(is_generic, Generic, _);
generate_matcher!(is_interface, Interface, _);
generate_matcher!(is_null, Null);
generate_matcher!(is_reference, Reference, _);
generate_matcher!(is_never_keyword, NeverKeyword);
generate_matcher!(is_unknown_keyword, UnknownKeyword);
generate_matcher!(is_void_keyword, VoidKeyword);

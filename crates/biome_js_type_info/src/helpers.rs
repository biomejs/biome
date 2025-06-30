use std::{
    borrow::Cow,
    cmp::Ordering,
    hash::{Hash, Hasher},
};

use hashbrown::{HashTable, hash_table::Entry};
use rustc_hash::FxHasher;

use crate::{
    BindingId, Class, Interface, Intersection, Module, Namespace, Object, Resolvable,
    ResolvedTypeData, ResolvedTypeId, ResolvedTypeMember, ResolverId, Type, TypeData, TypeInstance,
    TypeMember, TypeReference, TypeResolver, Union,
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

    /// Returns the type of an element at a given index, if this object is an
    /// array or a tuple.
    pub fn find_element_type_at_index(
        self,
        resolver: &'a dyn TypeResolver,
        index: usize,
    ) -> Option<ElementTypeReference> {
        match self.as_raw_data() {
            TypeData::Tuple(tuple) => {
                let element = tuple.get_element(index)?;
                Some(ElementTypeReference {
                    ty: self.apply_module_id_to_reference(&element.ty).into_owned(),
                    is_optional: element.is_optional || element.is_rest,
                })
            }
            _ if self.is_instance_of(resolver, GLOBAL_ARRAY_ID) => {
                self.get_type_parameter(0).map(|ty| ElementTypeReference {
                    ty: ty.into_owned(),
                    is_optional: true,
                })
            }
            _ => None,
        }
    }

    /// Convenience method for finding a type member of kind index signature.
    pub fn find_index_signature_with_ty(
        self,
        resolver: &'a dyn TypeResolver,
        predicate: impl Fn(Self) -> bool,
    ) -> Option<ResolvedTypeMember<'a>> {
        self.find_member(resolver, |member| {
            member.is_index_signature_with_ty(|ty| {
                resolver.resolve_and_get(ty).is_some_and(&predicate)
            })
        })
    }

    /// Convenience method for `.all_members().find()`.
    pub fn find_member(
        self,
        resolver: &'a dyn TypeResolver,
        predicate: impl Fn(&ResolvedTypeMember) -> bool,
    ) -> Option<ResolvedTypeMember<'a>> {
        self.all_members(resolver).find(predicate)
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

    /// Returns the type of elements from a given index, if this object is an
    /// array or a tuple.
    pub fn find_type_of_elements_from_index(
        self,
        resolver: &'a dyn TypeResolver,
        index: usize,
    ) -> Option<TypeData> {
        match self.as_raw_data() {
            TypeData::Tuple(tuple) => {
                Some(TypeData::from(tuple.slice_from(self.resolver_id(), index)))
            }
            _ if self.is_instance_of(resolver, GLOBAL_ARRAY_ID) => {
                match self.get_type_parameter(0) {
                    Some(elem_ty) => Some(TypeData::instance_of(TypeInstance {
                        ty: GLOBAL_ARRAY_ID.into(),
                        type_parameters: [elem_ty.into_owned()].into(),
                    })),
                    None => Some(TypeData::instance_of(TypeReference::from(GLOBAL_ARRAY_ID))),
                }
            }
            _ => None,
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
    /// Turns this [`TypeData`] into an instance of itself.
    pub fn into_instance(self, resolver: &mut dyn TypeResolver) -> Self {
        match self {
            Self::InstanceOf(instance) => Self::InstanceOf(instance),
            Self::Reference(reference) => Self::instance_of(reference),
            other => Self::instance_of(TypeReference::from(resolver.register_and_resolve(other))),
        }
    }

    /// Creates an intersection of type references.
    ///
    /// References are automatically deduplicated. If only a single type
    /// remains, an instance of `Self::Reference` is returned instead of
    /// `Self::Intersection`.
    pub fn intersection_of(mut types: Vec<TypeReference>) -> Self {
        types.dedup();
        match types.len().cmp(&1) {
            Ordering::Greater => Self::Intersection(Box::new(Intersection(types.into()))),
            Ordering::Equal => Self::reference(types.remove(0)),
            Ordering::Less => Self::unknown(),
        }
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
                            let ty = resolved.apply_module_id_to_reference(ty);
                            let entry = table.entry(
                                hash_reference(&ty),
                                |i| &vec[*i] == ty.as_ref(),
                                |i| hash_reference(&vec[*i]),
                            );
                            if let Entry::Vacant(entry) = entry {
                                let index = vec.len();
                                vec.push(ty.into_owned());
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

/// A reference to an element that is either optional or not.
pub struct ElementTypeReference {
    ty: TypeReference,
    is_optional: bool,
}

impl ElementTypeReference {
    pub fn into_reference(self, resolver: &mut dyn TypeResolver) -> TypeReference {
        if self.is_optional {
            let id = resolver.optional(self.ty);
            let resolved_id = ResolvedTypeId::new(resolver.level(), id);
            TypeReference::from(resolved_id)
        } else {
            self.ty
        }
    }
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
generate_matcher!(is_big_int, BigInt);
generate_matcher!(is_class, Class, _);
generate_matcher!(is_conditional, Conditional);
generate_matcher!(is_expression, TypeofExpression, _);
generate_matcher!(is_function, Function, _);
generate_matcher!(is_generic, Generic, _);
generate_matcher!(is_interface, Interface, _);
generate_matcher!(is_never_keyword, NeverKeyword);
generate_matcher!(is_null, Null);
generate_matcher!(is_number, Number);
generate_matcher!(is_reference, Reference, _);
generate_matcher!(is_string, String);
generate_matcher!(is_unknown_keyword, UnknownKeyword);
generate_matcher!(is_void_keyword, VoidKeyword);

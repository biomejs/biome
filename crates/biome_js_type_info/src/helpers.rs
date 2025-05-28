use std::borrow::Cow;

use biome_rowan::Text;

use crate::{
    BindingId, Class, GenericTypeParameter, Interface, Module, Namespace, Object, ResolvedTypeData,
    ResolvedTypeId, ResolvedTypeMember, ResolverId, TypeData, TypeInstance, TypeReference,
    TypeResolver,
    globals::{GLOBAL_ARRAY_ID, GLOBAL_PROMISE_ID, GLOBAL_TYPE_MEMBERS},
};

impl<'a> ResolvedTypeData<'a> {
    /// Iterates all member fields, including those belonging to extended
    /// classes or prototype objects.
    ///
    /// Note that members which are inherited and overridden may appear multiple
    /// times, but the member that is closest to the current type is guaranteed
    /// to come first.
    pub fn all_members(self, resolver: &'a dyn TypeResolver) -> TypeMemberIterator<'a> {
        TypeMemberIterator {
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
            self.find_type_parameter(resolver, "T")
                .and_then(|reference| resolver.resolve_and_get(&reference))
        } else {
            None
        }
    }

    /// Returns the promised type, if this object is an instance of `Promise`.
    pub fn find_promise_type(self, resolver: &'a dyn TypeResolver) -> Option<Self> {
        if self.is_instance_of(resolver, GLOBAL_PROMISE_ID) {
            self.find_type_parameter(resolver, "T")
                .and_then(|reference| resolver.resolve_and_get(&reference))
        } else {
            None
        }
    }

    pub fn find_type_parameter(
        self,
        resolver: &'a dyn TypeResolver,
        parameter_name: &str,
    ) -> Option<Cow<'a, TypeReference>> {
        let mut seen_types = Vec::new();
        let mut current_object = Some(self);
        while let Some(current) = current_object {
            if let Some(argument) = current
                .as_raw_data()
                .type_parameters()
                .iter()
                .flat_map(|params| params.iter())
                .find(|param| param.name == parameter_name && param.ty.is_known())
            {
                return Some(current.apply_module_id_to_reference(&argument.ty));
            }

            let Some(next_object) = current
                .prototype(resolver)
                .and_then(|prototype| resolver.resolve_reference(&prototype))
            else {
                break;
            };

            if seen_types.contains(&next_object) {
                break;
            }

            seen_types.push(next_object);
            current_object = resolver.get_by_resolved_id(next_object);
        }

        None
    }

    /// Returns whether this object is an instance of the type with the given ID.
    pub fn is_instance_of(self, resolver: &dyn TypeResolver, id: ResolvedTypeId) -> bool {
        let mut seen_types = Vec::new();
        let mut current_object = Some(self);
        while let Some(current) = current_object {
            let Some(prototype) = current.prototype(resolver) else {
                break;
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
                        .find_type_parameter(resolver, "T")
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
                    match resolved.find_type_parameter(resolver, "T") {
                        Some(elem_ty) => Some(Self::instance_of(TypeInstance {
                            ty: GLOBAL_ARRAY_ID.into(),
                            type_parameters: Box::new([GenericTypeParameter {
                                name: Text::Static("T"),
                                ty: elem_ty.into_owned(),
                            }]),
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
}

pub struct TypeMemberIterator<'a> {
    resolver: &'a dyn TypeResolver,
    resolver_id: ResolverId,
    owner: Option<TypeMemberOwner<'a>>,
    seen_types: Vec<ResolvedTypeId>,
    index: usize,
    excluded_binding_id: Option<BindingId>,
}

impl TypeMemberIterator<'_> {
    pub fn with_excluded_binding_id(mut self, binding_id: BindingId) -> Self {
        self.excluded_binding_id = Some(binding_id);
        self
    }
}

impl<'a> Iterator for TypeMemberIterator<'a> {
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

        let mut next_reference = self
            .resolver_id
            .apply_module_id_to_reference(next_reference);
        if let Some(excluded_binding_id) = self.excluded_binding_id {
            next_reference = Cow::Owned(
                next_reference
                    .into_owned()
                    .with_excluded_binding_id(excluded_binding_id),
            );
        }

        let Some(next_resolved_id) = self.resolver.resolve_reference(&next_reference) else {
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

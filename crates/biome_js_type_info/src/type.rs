//! Data structures for tracking type information across JS/TS files and scopes.
//!
//! Note that while our type inference is based on TypeScript and aims to be
//! compatible with TypeScript's types, we apply the same inference to
//! JavaScript files as well.
//!
//! This can be used by type-informed lint rules such as `noFloatingPromises`.
//!
//! The type information is instantiated and updated inside the Workspace
//! Server.

use std::fmt::Debug;
use std::{ops::Deref, sync::Arc};

use crate::conditionals::ConditionalType;
use crate::{
    GLOBAL_RESOLVER, Literal, ResolvedTypeData, ResolvedTypeId, ResolvedTypeMember, TypeData,
    TypeId, TypeReference, TypeResolver, UNKNOWN_DATA,
    globals::{
        GLOBAL_ARRAY_ID, GLOBAL_NUMBER_ID, GLOBAL_PROMISE_ID, GLOBAL_STRING_ID, GLOBAL_UNKNOWN_ID,
    },
};

/// Wrapper used to refer to type information stored in the `ModuleGraph`.
///
/// Type information is stored as part of `TypeData` structures that are stored
/// inside [`TypeStore`](crate::TypeStore) instances. However, there are many
/// type stores: One for each module in a project. As such, it can be difficult
/// to refer to a type if you don't know where to find it. [`Type`] solves this
/// problem by encapsulating both the resolver used for type lookups, as well as
/// an identifier for an individual type.
///
/// You should not create `Type` instances yourself. Instead, use the
/// `TypedService` to retrieve them.
#[derive(Clone)]
pub struct Type {
    resolver: Arc<dyn TypeResolver>,
    id: ResolvedTypeId,
}

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.deref(), f)
    }
}

impl Default for Type {
    fn default() -> Self {
        Self {
            resolver: GLOBAL_RESOLVER.clone(),
            id: GLOBAL_UNKNOWN_ID,
        }
    }
}

impl Deref for Type {
    type Target = TypeData;

    fn deref(&self) -> &Self::Target {
        self.resolved_data()
            .map_or(&UNKNOWN_DATA, |resolved| resolved.as_raw_data())
    }
}

impl Type {
    pub fn from_id(resolver: Arc<dyn TypeResolver>, id: ResolvedTypeId) -> Self {
        Self { resolver, id }
    }

    /// Returns this type's [`TypeId`].
    ///
    /// **Warning:** Type IDs can only be safely compared with other IDs from
    ///              the same module.
    pub fn id(&self) -> TypeId {
        self.id.id()
    }

    /// Returns semantic information about the type for use in conditionals.
    ///
    /// This gives you access to methods such as `is_truthy()`, `is_falsy()`,
    /// `is_nullish()` and `is_non_nullish()`.
    pub fn conditional_semantics(&self) -> ConditionalType {
        self.resolved_data()
            .map_or(ConditionalType::Unknown, |resolved| {
                ConditionalType::from_resolved_data(resolved, self.resolver.as_ref())
            })
    }

    /// Returns an iterator over the variants of this type, while deduplicating
    /// variants and flattening nested unions in the process.
    ///
    /// Returns an iterator that yields no elements if the type is not a union.
    pub fn flattened_union_variants(&self) -> impl Iterator<Item = Self> {
        self.resolved_data()
            .unwrap_or_else(|| ResolvedTypeData::from((GLOBAL_UNKNOWN_ID, &UNKNOWN_DATA)))
            .flattened_union_variants(self.resolver.as_ref())
            .filter_map(|ty| {
                self.resolver
                    .resolve_reference(&ty)
                    .map(|resolved_id| self.with_resolved_id(resolved_id))
            })
    }

    /// Returns `true` if this type represents a **union type** that has a
    /// variant for which the given `predicate` returns `true`.
    ///
    /// Returns `false` otherwise.
    pub fn has_variant(&self, predicate: impl Fn(Self) -> bool) -> bool {
        if self.is_union() {
            self.flattened_union_variants().any(predicate)
        } else {
            false
        }
    }

    /// Returns whether if this type is an instance of a type matching the given
    /// `predicate`.
    pub fn is_array_of(&self, predicate: impl Fn(Self) -> bool) -> bool {
        match self.as_raw_data() {
            Some(TypeData::InstanceOf(instance)) => {
                instance.ty == GLOBAL_ARRAY_ID.into()
                    && instance
                        .type_parameters
                        .first()
                        .and_then(|type_param| self.resolve(type_param))
                        .is_some_and(predicate)
            }
            _ => false,
        }
    }

    /// Returns whether this type is a boolean with the given `value`.
    pub fn is_boolean_literal(&self, value: bool) -> bool {
        self.as_raw_data().is_some_and(|ty| match ty {
            TypeData::Literal(literal) => match literal.as_ref() {
                Literal::Boolean(literal) => literal.as_bool() == value,
                _ => false,
            },
            _ => false,
        })
    }

    /// Returns whether `self` is a function with a return type matching the
    /// given `predicate`.
    pub fn is_function_with_return_type(&self, predicate: impl Fn(Self) -> bool) -> bool {
        match self.as_raw_data() {
            Some(TypeData::Function(function)) => function
                .return_type
                .as_type()
                .and_then(|ty| self.resolve(ty))
                .is_some_and(predicate),
            _ => false,
        }
    }

    /// Returns whether if this type is an instance of a type matching the given
    /// `predicate`.
    pub fn is_instance_of(&self, predicate: impl Fn(Self) -> bool) -> bool {
        match self.as_raw_data() {
            Some(TypeData::InstanceOf(instance)) => {
                self.resolve(&instance.ty).is_some_and(predicate)
            }
            _ => false,
        }
    }

    /// Returns whether this type is an interface that has a member matching the
    /// given `predicate`.
    pub fn is_interface_with_member(&self, predicate: impl Fn(ResolvedTypeMember) -> bool) -> bool {
        match self.as_raw_data() {
            Some(TypeData::Interface(interface)) => interface
                .members
                .iter()
                .map(|member| ResolvedTypeMember::from((self.id.resolver_id(), member)))
                .any(predicate),
            _ => false,
        }
    }

    /// Returns whether this type is a number or a literal number.
    pub fn is_number_or_number_literal(&self) -> bool {
        self.id == GLOBAL_NUMBER_ID
            || self.as_raw_data().is_some_and(|ty| match ty {
                TypeData::Number => true,
                TypeData::Literal(literal) => matches!(literal.as_ref(), Literal::Number(_)),
                _ => false,
            })
    }

    /// Returns whether this type is a number with the given `value`.
    pub fn is_number_literal(&self, value: f64) -> bool {
        self.as_raw_data().is_some_and(|ty| match ty {
            TypeData::Literal(literal) => match literal.as_ref() {
                Literal::Number(literal) => literal
                    .to_f64()
                    .is_some_and(|literal_value| literal_value == value),
                _ => false,
            },
            _ => false,
        })
    }

    /// Returns whether this type is the `Promise` class.
    pub fn is_promise(&self) -> bool {
        self.id == GLOBAL_PROMISE_ID
    }

    /// Returns whether this type is an instance of a `Promise`.
    pub fn is_promise_instance(&self) -> bool {
        self.resolved_data()
            .is_some_and(|ty| ty.is_instance_of(self.resolver.as_ref(), GLOBAL_PROMISE_ID))
    }

    /// Returns whether this type is a string.
    pub fn is_string_or_string_literal(&self) -> bool {
        self.id == GLOBAL_STRING_ID
            || self.as_raw_data().is_some_and(|ty| match ty {
                TypeData::String => true,
                TypeData::Literal(literal) => matches!(literal.as_ref(), Literal::String(_)),
                _ => false,
            })
    }

    /// Returns whether this type is a string with the given `value`.
    pub fn is_string_literal(&self, value: &str) -> bool {
        self.as_raw_data().is_some_and(|ty| match ty {
            TypeData::Literal(literal) => match literal.as_ref() {
                Literal::String(literal) => literal.as_str() == value,
                _ => false,
            },
            _ => false,
        })
    }

    pub fn resolve(&self, ty: &TypeReference) -> Option<Self> {
        self.resolver
            .resolve_reference(&self.id.apply_module_id_to_reference(ty))
            .map(|resolved_id| self.with_resolved_id(resolved_id))
    }

    #[inline]
    pub(super) fn as_raw_data(&self) -> Option<&TypeData> {
        self.resolved_data().map(ResolvedTypeData::as_raw_data)
    }

    #[inline]
    pub fn resolved_data(&self) -> Option<ResolvedTypeData<'_>> {
        self.resolver.get_by_resolved_id(self.id)
    }

    fn with_resolved_id(&self, id: ResolvedTypeId) -> Self {
        let mut id = id;
        loop {
            let Some(resolved_data) = self.resolver.get_by_resolved_id(id) else {
                break;
            };
            match resolved_data.as_raw_data() {
                TypeData::Reference(TypeReference::Resolved(resolved_id)) => {
                    id = resolved_data.apply_module_id(*resolved_id);
                }
                _ => break,
            }
        }

        Self {
            resolver: self.resolver.clone(),
            id,
        }
    }
}

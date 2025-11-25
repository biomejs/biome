//! Builder for constructing GlobalsResolver with forward references support.

use std::sync::Arc;

use crate::{TypeData, TypeId, TypeStore};

use super::globals::GlobalsResolver;

/// Builder for constructing a GlobalsResolver with support for forward references.
///
/// This builder allows types to reference each other through TypeIds without
/// requiring the types to be defined in a specific order. This decouples how we
/// define types from how we use them in the type inference phase (aka TypeStore's type lookup).
///
/// # Example
///
/// ```ignore
/// let mut builder = GlobalsResolverBuilder::new();
///
/// // Reserve IDs
/// let array_id = builder.reserve_id();
/// let filter_id = builder.reserve_id();
///
/// // Fill in types (Note that we can use the reserved IDs in any order)
/// builder.set_type(filter_id, TypeData::from(Function { ... }));
/// builder.set_type(array_id, TypeData::Class(Box::new(Class {
///     members: Box::new([method("filter", filter_id)]),
/// })));
///
/// // Build
/// let resolver = builder.build();
/// ```
pub struct GlobalsResolverBuilder {
    /// Types being built. None = reserved but not yet filled.
    types: Vec<Option<TypeData>>,
}

impl GlobalsResolverBuilder {
    /// Create a new empty builder.
    pub fn new() -> Self {
        Self { types: Vec::new() }
    }

    /// Reserve a slot in the type store and return its TypeId.
    ///
    /// The returned TypeId can be used in type references before the actual
    /// type data is provided via `set_type`.
    pub fn reserve_id(&mut self) -> TypeId {
        let id = TypeId::new(self.types.len());
        self.types.push(None);
        id
    }

    /// Fill a previously reserved type slot with actual type data.
    ///
    /// # Panics
    ///
    /// Panics if the TypeId is out of bounds or if the slot was already filled.
    pub fn set_type(&mut self, id: TypeId, data: TypeData) {
        let index = id.index();
        assert!(
            index < self.types.len(),
            "TypeId {index} out of bounds (len: {})",
            self.types.len()
        );
        assert!(
            self.types[index].is_none(),
            "Type at index {index} already set"
        );
        self.types[index] = Some(data);
    }

    /// Build the final GlobalsResolver.
    ///
    /// # Panics
    ///
    /// Panics if any reserved type slots were not filled.
    pub fn build(self) -> GlobalsResolver {
        let types: Vec<Arc<TypeData>> = self
            .types
            .into_iter()
            .enumerate()
            .map(|(i, opt)| {
                Arc::new(opt.unwrap_or_else(|| {
                    panic!("Type at index {i} was reserved but never filled")
                }))
            })
            .collect();

        GlobalsResolver {
            types: TypeStore::from_types(types),
        }
    }
}

impl Default for GlobalsResolverBuilder {
    fn default() -> Self {
        Self::new()
    }
}
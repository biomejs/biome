//! Builder for constructing GlobalsResolver with forward references support.

use std::sync::Arc;

use crate::{NUM_PREDEFINED_TYPES, TypeData, TypeId, TypeStore};

use super::globals::GlobalsResolver;

/// Builder for constructing a GlobalsResolver
pub struct GlobalsResolverBuilder {
    /// Types being built. None = reserved but not yet filled.
    types: Vec<Option<TypeData>>,
}

impl GlobalsResolverBuilder {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            types: vec![None; capacity],
        }
    }

    /// Fill a previously reserved type slot with actual type data.
    pub fn set_type_data(&mut self, id: TypeId, data: TypeData) {
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
    pub fn build(self) -> GlobalsResolver {
        let types: Vec<Arc<TypeData>> = self
            .types
            .into_iter()
            .enumerate()
            .map(|(i, opt)| {
                Arc::new(opt.unwrap_or_else(|| {
                    // Make sure we only reserve just enough for what we need
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
        Self::with_capacity(NUM_PREDEFINED_TYPES)
    }
}

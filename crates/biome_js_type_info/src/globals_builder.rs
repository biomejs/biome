use std::sync::Arc;

use crate::generated::global_types::MIGRATED_PREDEFINED_IDS;
use crate::{NUM_PREDEFINED_TYPES, TypeData, TypeStore};

use super::{globals::GlobalsResolver, globals_ids::GlobalTypeId};

/// Requires every predefined manifest slot to be populated before producing a [`GlobalsResolver`].
pub(crate) struct GlobalsResolverBuilder {
    /// `None` reserves a manifest row until its `TypeData` is written.
    types: Vec<Option<TypeData>>,
}

impl GlobalsResolverBuilder {
    /// Creates an empty builder that reserves `capacity` manifest slots as `None`.
    fn with_capacity(capacity: usize) -> Self {
        Self {
            types: vec![None; capacity],
        }
    }

    /// Writes the `TypeData` for the predefined manifest slot identified by `id`.
    pub(crate) fn set_type_data(&mut self, id: GlobalTypeId, data: TypeData) {
        let index = id.index();
        debug_assert!(
            index < self.types.len(),
            "GlobalsResolverBuilder::set_type_data: TypeId index {index} out of bounds (len {})",
            self.types.len()
        );

        let slot = &mut self.types[index];
        debug_assert!(
            slot.is_none(),
            "GlobalsResolverBuilder::set_type_data: double-write at index {index}"
        );
        *slot = Some(data);
    }

    /// Skips manual data for IDs owned by the generated module, so codegen remains
    /// the source of truth. `MIGRATED_PREDEFINED_IDS` must stay sorted and unique
    /// because this lookup uses `binary_search`.
    pub(crate) fn set_manual_type_data<F>(&mut self, id: GlobalTypeId, build: F)
    where
        F: FnOnce() -> TypeData,
    {
        if MIGRATED_PREDEFINED_IDS.binary_search(&id).is_err() {
            self.set_type_data(id, build());
        }
    }

    /// Consumes the builder and produces the immutable [`GlobalsResolver`].
    pub(crate) fn build(self) -> GlobalsResolver {
        let types: Vec<Arc<TypeData>> = self
            .types
            .into_iter()
            .map(|slot| Arc::new(slot.unwrap_or(TypeData::Unknown)))
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

#[cfg(all(test, debug_assertions))]
mod tests {
    use super::*;
    use crate::globals_ids::UNKNOWN_ID_GLOBAL_TYPE_ID;

    #[test]
    #[should_panic(expected = "TypeId index")]
    fn set_type_data_panics_on_out_of_bounds() {
        let mut builder = GlobalsResolverBuilder::default();
        builder.set_type_data(
            GlobalTypeId::new_for_test(NUM_PREDEFINED_TYPES),
            TypeData::Unknown,
        );
    }

    #[test]
    #[should_panic(expected = "double-write at index")]
    fn set_type_data_panics_on_double_write() {
        let mut builder = GlobalsResolverBuilder::default();
        builder.set_type_data(UNKNOWN_ID_GLOBAL_TYPE_ID, TypeData::Unknown);
        builder.set_type_data(UNKNOWN_ID_GLOBAL_TYPE_ID, TypeData::Unknown);
    }
}

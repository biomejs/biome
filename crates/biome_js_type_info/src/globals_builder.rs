use std::sync::Arc;

use crate::codegen::global_types::MIGRATED_PREDEFINED_IDS;
use crate::{NUM_PREDEFINED_TYPES, TypeData, TypeId, TypeStore};

use super::{
    globals::{GlobalsResolver, global_type_name},
    globals_ids::GlobalTypeId,
};

/// Returns the manifest name for `id` for use in panic messages, falling back
/// to a placeholder when the slot has no registered name yet.
fn manifest_name_for(id: GlobalTypeId) -> &'static str {
    global_type_name(id.as_type_id()).unwrap_or("<unregistered manifest slot>")
}

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
    ///
    /// # Panics
    ///
    /// - If `id.index() >= NUM_PREDEFINED_TYPES`.
    /// - If the slot at `id` has already been written.
    pub(crate) fn set_type_data(&mut self, id: GlobalTypeId, data: TypeData) {
        let index = id.index();
        let len = self.types.len();
        assert!(
            index < NUM_PREDEFINED_TYPES,
            "GlobalsResolverBuilder::set_type_data: TypeId index {index} (manifest name {name}) is out of bounds; builder capacity is {len} (NUM_PREDEFINED_TYPES = {num})",
            name = manifest_name_for(id),
            num = NUM_PREDEFINED_TYPES,
        );

        let slot = &mut self.types[index];
        let Some(prior) = slot.as_ref() else {
            *slot = Some(data);
            return;
        };

        panic!(
            "GlobalsResolverBuilder::set_type_data: double-write at index {index} (manifest name {name}); prior = {prior:?}; new = {new:?}",
            name = manifest_name_for(id),
            new = data,
        );
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
    ///
    /// # Panics
    ///
    /// Panics if any slot in `0..NUM_PREDEFINED_TYPES` is still unset. The
    /// panic message lists the offending index along with its manifest name so
    /// missing wiring can be located without a debugger.
    pub(crate) fn build(self) -> GlobalsResolver {
        for (index, slot) in self.types.iter().enumerate() {
            if slot.is_none() {
                panic!(
                    "GlobalsResolverBuilder::build: predefined slot at index {index} (manifest name {name}) was never initialized via set_type_data",
                    name = global_type_name(TypeId::new(index))
                        .unwrap_or("<unregistered manifest slot>"),
                );
            }
        }

        let types: Vec<Arc<TypeData>> = self
            .types
            .into_iter()
            .map(|slot| Arc::new(slot.expect("slot validated by build() pre-check above")))
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

#[cfg(test)]
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

    #[test]
    #[should_panic(expected = "predefined slot at index 0")]
    fn build_panics_on_unset_slot() {
        let builder = GlobalsResolverBuilder::default();
        let _ = builder.build();
    }
}

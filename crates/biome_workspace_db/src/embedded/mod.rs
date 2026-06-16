pub(crate) use crate::embedded::bindings::EmbeddedBinding;
pub(crate) use crate::embedded::references::EmbeddedValueReference;
use biome_rowan::{TextRange, TokenText};

pub mod bindings;
pub mod references;

#[salsa::db]
pub trait EmbeddedDb: biome_db::Db {
    fn bindings(&self) -> Vec<Vec<EmbeddedBinding>>;

    fn references(&self) -> Vec<Vec<EmbeddedValueReference>>;

    fn binding_by_name(&self, name: &str) -> Option<EmbeddedBinding> {
        for bindings in self.bindings() {
            for binding in bindings {
                if binding.text(self).text() == name {
                    return Some(binding);
                }
            }
        }
        None
    }

    fn service_references(&self) -> Vec<Vec<(TextRange, TokenText)>> {
        self.references()
            .iter()
            .map(|refs| {
                refs.into_iter()
                    .map(|this_ref| (this_ref.range(self), this_ref.text(self)))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }
}

pub(crate) use crate::embedded::bindings::EmbeddedBinding;
pub(crate) use crate::embedded::references::EmbeddedValueReference;
use biome_rowan::{TextRange, TokenText};
use camino::Utf8Path;

pub mod bindings;
pub mod references;

#[salsa::db]
pub trait EmbeddedDb: biome_db::Db {
    fn bindings(&self, path: &Utf8Path) -> Vec<Vec<EmbeddedBinding>>;

    fn references(&self, path: &Utf8Path) -> Vec<Vec<EmbeddedValueReference>>;

    fn binding_by_name(&self, path: &Utf8Path, name: &str) -> Option<EmbeddedBinding> {
        for bindings in self.bindings(path) {
            for binding in bindings {
                if binding.text(self).text() == name {
                    return Some(binding);
                }
            }
        }
        None
    }

    fn service_references(&self, path: &Utf8Path) -> Vec<Vec<(TextRange, TokenText)>> {
        self.references(path)
            .iter()
            .map(|refs| {
                refs.iter()
                    .map(|this_ref| (this_ref.range(self), this_ref.text(self)))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }
}

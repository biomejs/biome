use crate::DocumentFileSource;

#[salsa::db]
pub trait LanguageDb: biome_db::Db {
    /// Returns a previously inserted file source by index.
    fn source_from_index(&self, index: usize) -> Option<DocumentFileSource>;
}

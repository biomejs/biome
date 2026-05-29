use crate::AnyFileSource;

#[salsa::db]
pub trait LanguageDb: biome_db::Db {
    /// Returns a previously inserted file source by index.
    ///
    /// File sources can be inserted using `insert_source()`.
    fn source_from_index(&self, index: usize) -> Option<AnyFileSource>;
}

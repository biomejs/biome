use crate::DocumentFileSource;
use camino::Utf8Path;

#[salsa::db]
pub trait LanguageDb: biome_db::Db {
    /// Returns a previously inserted file source by index.
    fn source_from_index(&self, index: usize) -> Option<DocumentFileSource>;

    /// Return the [DocumentFileSource] for the given path.
    fn source_from_path(&self, path: &Utf8Path) -> Option<DocumentFileSource> {
        self.file_source_for_path(path)
            .and_then(|file| self.source_from_index(file.document_source_index(self)))
    }
}
